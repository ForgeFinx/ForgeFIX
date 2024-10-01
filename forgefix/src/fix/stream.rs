use crate::fix::log::Logger;
use crate::fix::mem::MsgBuf;
use crate::fix::{decode, validate, SessionError};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;

pub(super) const PEEK_LEN: usize = 32;

pub(super) trait TryRead {
    fn try_read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>;
}

impl TryRead for TcpStream {
    fn try_read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        <TcpStream>::try_read(self, buf)
    }
}

pub(super) struct HeaderBuf<const N: usize> {
    inner: Box<[u8]>,
    filled_len: usize,
}

impl<const N: usize> HeaderBuf<N> {
    pub(super) fn new() -> Self {
        HeaderBuf {
            inner: vec![0; N].into_boxed_slice(),
            filled_len: 0,
        }
    }
    fn unfilled_mut(&mut self) -> &mut [u8] {
        &mut self.inner[self.filled_len..]
    }
    fn advance(&mut self, n: usize) {
        self.filled_len = std::cmp::min(self.filled_len + n, self.inner.len());
    }
    fn clear(&mut self) {
        self.filled_len = 0;
    }
    fn is_full(&self) -> bool {
        self.filled_len == N
    }
    fn filled(&self) -> &[u8] {
        &self.inner[0..self.filled_len]
    }
    // take() is "slow": rotate_left(n) is O(N), and an allocation is done.
    // Do not use in `hot-path`, only use in events assumed to be rare, such as
    // receiving a garbled message.
    fn take(&mut self, n: usize) -> Vec<u8> {
        let n = std::cmp::min(n, self.filled_len);
        let taken = self.inner[..n].to_vec();
        self.inner.rotate_left(n);
        self.filled_len -= n;
        taken
    }
}

pub(super) async fn read_message<const N: usize, T>(
    r: &mut T,
    header: &mut HeaderBuf<N>,
    logger: &mut impl Logger,
) -> Result<MsgBuf, SessionError>
where
    T: TryRead + AsyncRead + Unpin,
{
    let body_len = match decode::parse_header(header.filled()) {
        Ok(n) => n,
        Err(e) => {
            let junk = skip_to_next_message(r, header).await?;
            logger.log_message(&junk.into())?;
            return Err(e);
        }
    };

    let header_len = header.filled().len();
    let mut msg_vec = vec![0; header_len + body_len];
    msg_vec[..header_len].copy_from_slice(header.filled());
    header.clear();
    r.read_exact(&mut msg_vec[header_len..]).await?;

    let msg_buf: MsgBuf = msg_vec.into();
    // logger.log_message(&msg_buf)?;

    if let Err(e) = validate::validate_msg_length(msg_buf.0.as_slice(), msg_buf.len()) {
        let junk = skip_to_next_message(r, header).await?;
        logger.log_message(&junk.into())?;
        return Err(e);
    }

    Ok(msg_buf)
}

// Finds the position of the longest, if any, prefix of `target` that is also a
// suffix of `buf` using a simple brute force algorithm.
//
// Examples: a prefix of "8=F" exists in "xxx8=F", "xxx8=" and "xxx8", but one
// does not exists in "xxx8=x", or "xxxx8x".
fn partial_match_in_suffix(buf: &[u8], target: &[u8]) -> Option<usize> {
    let largest_match_len = std::cmp::min(target.len(), buf.len());
    for prefix_len in (1..=largest_match_len).rev() {
        let suffix_begin = buf.len() - prefix_len;
        if buf[suffix_begin..] == target[..prefix_len] {
            return Some(suffix_begin);
        }
    }
    None
}

// Trys to find an exact match of `target` in `buf`, and if that fails, trys to find a prefix of
// `target` in the suffix of `buf`.
//
// Uses a brute force algorithm instead of most optimal. This is okay because assuming `buf` and
// `target` are small, and this function is not called in the hot-path.
fn position_or_partial_match(buf: &[u8], target: &[u8]) -> Option<usize> {
    buf.windows(target.len())
        .position(|window| window == target)
        .or_else(|| partial_match_in_suffix(buf, target))
}

impl<const N: usize> HeaderBuf<N> {
    fn take_until_possible_match(&mut self, target: &[u8]) -> Vec<u8> {
        self.take(position_or_partial_match(self.filled(), target).unwrap_or(self.filled_len))
    }
}

const MESSAGE_BEGINNING: &[u8] = b"8=F";

async fn skip_to_next_message<const N: usize, T>(
    stream: &mut T,
    header: &mut HeaderBuf<N>,
) -> Result<Vec<u8>, SessionError>
where
    T: TryRead + AsyncRead + Unpin,
{
    // assumption is that the current message in header is garbled, remove first byte otherwise the
    // while-loop will think the garbled message is the next message
    let mut sink = header.take(1);

    while !header.filled().starts_with(MESSAGE_BEGINNING) {
        sink.extend(header.take_until_possible_match(MESSAGE_BEGINNING));
        match stream.try_read(header.unfilled_mut()) {
            Ok(0) => break,
            Ok(n) => header.advance(n),
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => break,
            Err(e) => return Err(e.into()),
        }
    }
    Ok(sink)
}

pub(super) async fn read_header<R: AsyncRead + Unpin, const N: usize>(
    r: &mut R,
    buf: &mut HeaderBuf<N>,
) -> Result<(), SessionError> {
    while !buf.is_full() {
        let num_read = r.read(buf.unfilled_mut()).await?;
        buf.advance(num_read);

        if num_read == 0 {
            return Err(SessionError::TcpDisconnection);
        }
    }
    Ok(())
}

pub(super) async fn disconnect(mut stream: TcpStream) {
    _ = stream.set_linger(Some(tokio::time::Duration::from_secs(0)));
    _ = stream.shutdown().await;
    std::mem::drop(stream);
}

pub(super) async fn send_message<W: AsyncWrite + Unpin>(
    msg_buf: &MsgBuf,
    r: &mut W,
    l: &mut impl Logger,
) -> Result<(), SessionError> {
    r.write_all(&msg_buf[..]).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            SessionError::TcpDisconnection
        } else {
            e.into()
        }
    })?;
    // l.log_message(msg_buf)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fix::GarbledMessageType;
    use std::io::Cursor;

    impl TryRead for Cursor<&[u8]> {
        fn try_read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
            std::io::Read::read(self, buf)
        }
    }

    struct MockLogger;
    impl Logger for MockLogger {
        fn log_message(&mut self, _: &MsgBuf) -> Result<(), SessionError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_skip_to_next_message() {
        let mut header_buf = HeaderBuf::<{ PEEK_LEN }>::new();

        let mut no_next_message = Cursor::new(b"8=FIX.4.2\x019=57\x0135=A\x0134=1\x0149=ISLD\x0152=20240506-13:59:15.021\x0156=TW\x0198=0\x01108=30\x01141=Y\x0110=003\x01".as_slice());

        read_header(&mut no_next_message, &mut header_buf)
            .await
            .unwrap();
        assert!(skip_to_next_message(&mut no_next_message, &mut header_buf)
            .await
            .is_ok());
        assert_eq!(
            no_next_message.position() as usize,
            no_next_message.get_ref().len()
        );
        assert_eq!(header_buf.filled(), &[]);

        header_buf.clear();
        let mut next_message_in_header =
            Cursor::new(b"8=FIX.5.2\x01xxxxxxxxxxxxxxxxxxx8=F".as_slice());
        read_header(&mut next_message_in_header, &mut header_buf)
            .await
            .unwrap();
        assert!(
            skip_to_next_message(&mut next_message_in_header, &mut header_buf)
                .await
                .is_ok()
        );
        assert_eq!(
            next_message_in_header.position() as usize,
            next_message_in_header.get_ref().len()
        );
        assert_eq!(header_buf.filled(), b"8=F".as_slice());

        header_buf.clear();
        let mut next_message_in_stream = Cursor::new(b"8=FIX.4.2\x019=57\x0135=A\x0134=1\x0149=ISLD\x0152=20240506-13:59:15.021\x0156=TW\x0198=0\x01108=30\x01141=Y\x0110=003\x018=F".as_slice());
        read_header(&mut next_message_in_stream, &mut header_buf)
            .await
            .unwrap();
        assert!(
            skip_to_next_message(&mut next_message_in_stream, &mut header_buf)
                .await
                .is_ok()
        );
        assert_eq!(
            next_message_in_stream.position() as usize,
            next_message_in_stream.get_ref().len()
        );
        assert_eq!(header_buf.filled(), b"8=F".as_slice());

        header_buf.clear();
        let mut next_msg_maybe = Cursor::new(b"8=FIX.5.2\x01xxxxxxxxxxxxxxxxxxxx8=".as_slice());
        read_header(&mut next_msg_maybe, &mut header_buf)
            .await
            .unwrap();
        assert!(skip_to_next_message(&mut next_msg_maybe, &mut header_buf)
            .await
            .is_ok());
        assert_eq!(
            next_msg_maybe.position() as usize,
            next_msg_maybe.get_ref().len()
        );
        assert_eq!(header_buf.filled(), b"8=".as_slice());

        header_buf.clear();
        let mut next_msg_maybe = Cursor::new(b"8=FIX.5.2\x01xxxxxxxxxxxxxxxxxxxx8=F".as_slice());
        read_header(&mut next_msg_maybe, &mut header_buf)
            .await
            .unwrap();
        assert!(skip_to_next_message(&mut next_msg_maybe, &mut header_buf)
            .await
            .is_ok());
        assert_eq!(
            next_msg_maybe.position() as usize,
            next_msg_maybe.get_ref().len()
        );
        assert_eq!(header_buf.filled(), b"8=F".as_slice());

        header_buf.clear();
        let mut next_msg_maybe = Cursor::new(b"8=FIX.5.2\x01xxxxxxxxxxxxxxxxxxxxxxxx8=".as_slice());
        read_header(&mut next_msg_maybe, &mut header_buf)
            .await
            .unwrap();
        assert!(skip_to_next_message(&mut next_msg_maybe, &mut header_buf)
            .await
            .is_ok());
        assert_eq!(
            next_msg_maybe.position() as usize,
            next_msg_maybe.get_ref().len()
        );
        assert_eq!(header_buf.filled(), b"8=".as_slice());
    }

    #[tokio::test]
    async fn test_read_message() {
        let mut mock_logger = MockLogger;
        let mut incoming_message = Cursor::new(b"8=FIX.4.2\x019=67\x0135=A\x0134=1\x0149=ISLD\x0152=20240506-13:59:15.021\x0156=TW\x0198=0\x01108=30\x01141=Y\x0110=003\x01".as_slice());
        let mut header_buf = HeaderBuf::<{ PEEK_LEN }>::new();
        assert!(read_header(&mut incoming_message, &mut header_buf)
            .await
            .is_ok());

        let expected = MsgBuf(incoming_message.get_ref().to_vec());
        assert_eq!(
            read_message(&mut incoming_message, &mut header_buf, &mut mock_logger)
                .await
                .unwrap()
                .0,
            expected.0,
        );

        let mut incoming_message_bad_header = Cursor::new(b"8=FIX.5.2\x019=67\x0135=A\x0134=1\x0149=ISLD\x0152=20240506-13:59:15.021\x0156=TW\x0198=0\x01108=30\x01141=Y\x0110=003\x01".as_slice());
        header_buf.clear();
        assert!(
            read_header(&mut incoming_message_bad_header, &mut header_buf)
                .await
                .is_ok()
        );
        assert!(matches!(
            read_message(
                &mut incoming_message_bad_header,
                &mut header_buf,
                &mut mock_logger
            )
            .await,
            Err(SessionError::GarbledMessage {
                garbled_msg_type: GarbledMessageType::BeginStringIssue,
                ..
            }),
        ));
        assert_eq!(
            incoming_message_bad_header.position(),
            incoming_message_bad_header.get_ref().len() as u64
        );
        assert_eq!(header_buf.filled(), &[]);

        let mut incoming_message_wrong_len = Cursor::new(b"8=FIX.4.2\x019=40\x0135=A\x0134=1\x0149=ISLD\x0152=20240506-13:59:15.021\x0156=TW\x0198=0\x01108=30\x01141=Y\x0110=003\x01".as_slice());
        header_buf.clear();
        assert!(
            read_header(&mut incoming_message_wrong_len, &mut header_buf)
                .await
                .is_ok()
        );
        assert!(matches!(
            read_message(
                &mut incoming_message_wrong_len,
                &mut header_buf,
                &mut mock_logger
            )
            .await,
            Err(SessionError::GarbledMessage {
                garbled_msg_type: GarbledMessageType::BodyLengthIssue,
                ..
            }),
        ));
        assert_eq!(
            incoming_message_wrong_len.position(),
            incoming_message_bad_header.get_ref().len() as u64
        );
        assert_eq!(header_buf.filled(), &[]);
    }

    #[tokio::test]
    async fn test_read_header() {
        const incoming_message: &[u8] = b"8=FIX.4.2\x019=54\x0135=A\x01".as_slice();
        let mut incoming_header = Cursor::new(incoming_message);
        let mut header_buf = HeaderBuf::<{ incoming_message.len() }>::new();

        assert!(read_header(&mut incoming_header, &mut header_buf)
            .await
            .is_ok());
        assert_eq!(&header_buf.filled(), incoming_header.get_ref());

        header_buf.clear();
        let mut tcp_disconnect = Cursor::new(b"".as_slice());

        assert!(matches!(
            read_header(&mut tcp_disconnect, &mut header_buf)
                .await
                .unwrap_err(),
            SessionError::TcpDisconnection,
        ));

        header_buf.unfilled_mut()[..3].copy_from_slice(b"8=F");
        header_buf.advance(3);
        incoming_header.set_position(3);
        assert!(read_header(&mut incoming_header, &mut header_buf)
            .await
            .is_ok());
        assert_eq!(&header_buf.filled(), incoming_header.get_ref());

        header_buf.clear();
        let mut empty_message = Cursor::new(b"");
        assert!(read_header(&mut empty_message, &mut header_buf)
            .await
            .is_err());

        let mut full_header: HeaderBuf<{ incoming_message.len() }> = HeaderBuf {
            inner: vec![0u8; incoming_message.len()].into_boxed_slice(),
            filled_len: incoming_message.len(),
        };
        assert!(read_header(&mut incoming_header, &mut full_header)
            .await
            .is_ok());
    }

    #[test]
    fn test_header_buf() {
        let mut buf = HeaderBuf::<5>::new();
        assert_eq!(buf.unfilled_mut(), vec![0; 5].as_slice());
        assert_eq!(buf.filled(), vec![].as_slice());

        buf.unfilled_mut()[..3].copy_from_slice(vec![1, 2, 3].as_slice());
        assert_eq!(buf.filled(), vec![].as_slice());
        buf.advance(3);
        assert_eq!(buf.unfilled_mut(), vec![0; 2].as_slice());
        assert_eq!(buf.filled(), vec![1, 2, 3].as_slice());

        assert_eq!(buf.take(1), vec![1]);
        assert_eq!(buf.unfilled_mut().len(), 3);
        assert_eq!(buf.filled(), vec![2, 3]);

        assert_eq!(buf.take(2), vec![2, 3]);
        assert_eq!(buf.unfilled_mut().len(), 5);
        assert_eq!(buf.filled(), vec![].as_slice());

        buf.unfilled_mut()[..3].copy_from_slice(vec![4, 5, 6].as_slice());
        buf.advance(3);
        assert_eq!(buf.take(100), vec![4, 5, 6]);
        assert_eq!(buf.filled().len(), 0);
        assert_eq!(buf.unfilled_mut().len(), 5);

        buf.advance(100);
        assert_eq!(buf.filled().len(), 5);
        assert_eq!(buf.unfilled_mut().len(), 0);
    }

    #[test]
    fn test_partial_match() {
        let partial_match_in_suffix_fields: Vec<(&[u8], &[u8], Option<usize>)> = vec![
            (b"xxx8", b"8=F", Some(3)),
            (b"xxx8=", b"8=F", Some(3)),
            (b"xxx8=F", b"8=F", Some(3)),
            (b"xxxxx", b"8=F", None),
            (b"xxx8=x", b"8=F", None),
            (b"xx8", b"8=F", Some(2)),
            (b"x8=", b"8=F", Some(1)),
            (b"x8", b"8=F", Some(1)),
            (b"8", b"8=F", Some(0)),
            (b"", b"8=F", None),
        ];
        for (buf, target, expected) in partial_match_in_suffix_fields {
            assert_eq!(partial_match_in_suffix(buf, target), expected);
        }

        let position_or_partial_match_fields: Vec<(&[u8], &[u8], Option<usize>)> = vec![
            (b"8=F", b"8=F", Some(0)),
            (b"xx8=Fxxx", b"8=F", Some(2)),
            (b"xx8=xxx", b"8=F", None),
            (b"x8", b"8=F", Some(1)),
            (b"x", b"8=F", None),
            (b"", b"8=F", None),
        ];
        for (buf, target, expected) in position_or_partial_match_fields {
            assert_eq!(position_or_partial_match(buf, target), expected);
        }
    }
}
