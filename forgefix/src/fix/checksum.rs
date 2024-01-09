use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncWrite;

pub struct ChecksumWriter<W>(W, usize);
impl<W> Write for ChecksumWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for c in buf {
            self.1 += (*c) as usize;
        }
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
impl<W> ChecksumWriter<W> {
    #[allow(dead_code)]
    pub fn new(w: W) -> Self {
        ChecksumWriter(w, 0)
    }
    #[allow(dead_code)]
    pub fn checksum(&self) -> usize {
        self.1 % 256
    }
}

pub struct AsyncChecksumWriter<W>(W, usize);
impl<W> AsyncWrite for AsyncChecksumWriter<W>
where
    W: AsyncWrite + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let mut_self = &mut self.get_mut();
        for c in buf {
            mut_self.1 += (*c) as usize;
        }

        Pin::new(&mut mut_self.0).poll_write(cx, buf)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.get_mut().0).poll_flush(cx)
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.get_mut().0).poll_shutdown(cx)
    }
}
impl<W> AsyncChecksumWriter<W> {
    pub fn new(w: W) -> Self {
        AsyncChecksumWriter(w, 0)
    }
    pub fn checksum(&self) -> usize {
        self.1 % 256
    }
}

pub fn calc_checksum(bytes: &[u8]) -> i32 {
    bytes.iter().map(|c| *c as i32).sum::<i32>() % 256
}

pub fn checksum_is_valid(msg_buf: &[u8]) -> bool {
    if let Some(checksum) = parse_checksum(msg_buf) {
        return checksum_matches(&msg_buf[..msg_buf.len() - 7], checksum);
    }
    false
}

fn parse_checksum(msg_buf: &[u8]) -> Option<i32> {
    if msg_buf.len() < 7 {
        return None;
    }
    let tail = &msg_buf[msg_buf.len() - 7..];
    if &tail[0..3] != b"10="
        || !tail[3..6].iter().all(|&byte| byte.is_ascii_digit())
        || tail[6] != b'\x01'
    {
        return None;
    }

    match std::str::from_utf8(&tail[3..6]).unwrap_or("").parse() {
        Ok(v) => Some(v),
        _ => None,
    }
}

fn checksum_matches(msg: &[u8], checksum: i32) -> bool {
    let calculated = calc_checksum(msg);
    checksum == calculated
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_checksum_matches() {
        let tests: Vec<(&[u8], i32, bool)> = vec![
            (b"8=FIX.4.2\x019=98\x0135=5\x0134=2\x0149=ISLD5\x012=20230803-14:13:08.157\x0156=TW\x0158=MsgSeqNum too low, expecting 3 but received 2\x01", 81, true),
            (b"8=FIX.4.2\x019=98\x0135=5\x0134=2\x0149=ISLD5\x012=20230803-14:13:08.157\x0156=TW\x0158=MsgSeqNum too low, expecting 3 but received 2\x01", 0, false),
            (b"8=FIX.4.2\x019=57\x0135=A\x0134=1\x0149=TW\x0152=20230803-15:42:57\x0156=ISLD\x0198=0\x01108=30\x01", 19, true),
        ];
        for t in tests {
            assert_eq!(checksum_matches(t.0, t.1), t.2);
        }
    }

    #[test]
    fn test_parse_checksum() {
        let tests: Vec<(&[u8], bool)> = vec![
            (b"aaaaaaaaaaaaaaaa10=123\x01", true),
            (b"aaaaaaaa10=43\x01", false),
            (b"aaaaaaaa10=123", false),
            (b"aaaaaaaa11=123\x01", false),
        ];
        for t in tests {
            assert_eq!(
                parse_checksum(t.0).is_some(),
                t.1,
                "{:?} {}",
                parse_checksum(t.0),
                t.1
            );
        }
    }
}
