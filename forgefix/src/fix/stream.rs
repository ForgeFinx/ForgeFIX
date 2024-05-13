use crate::fix::log::Logger;
use crate::fix::mem::MsgBuf;
use crate::fix::{decode, validate, SessionError};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub(super) const PEEK_LEN: usize = 32;

pub(super) async fn preparse_stream(
    buf: &[u8],
    r: &mut TcpStream,
    logger: &mut impl Logger,
) -> Result<(char, MsgBuf), SessionError> {
    let res = decode::parse_peeked_prefix(buf);
    let peeked_info: decode::ParsedPeek = match res {
        Ok(info) => info,
        Err(e) => {
            flush_stream(r, logger).await?;
            return Err(e);
        }
    };

    let mut msg_buf = vec![0; peeked_info.msg_length];
    r.read_exact(&mut msg_buf[..]).await?;
    logger.log_message(&msg_buf.clone().into())?;
    if let Err(e) = validate::validate_msg_length(&msg_buf[..], peeked_info.msg_length) {
        flush_stream(r, logger).await?;
        return Err(e);
    }
    let m: MsgBuf = msg_buf.into();
    Ok((peeked_info.msg_type, m))
}

// needed to remove any garbled message from the stream that wasn't read
// to flush:
//  - peek next 3 bytes
//  - if next 3 bytes signal a new message AND not first pass, return
//  - else, remove another byte from the stream (add to sink so it can be logged)
//  - if that was the last byte, return
//  - else, repeat
pub(super) async fn flush_stream(r: &mut TcpStream, l: &mut impl Logger) -> Result<(), SessionError> {
    let mut sink = Vec::new();
    let mut buf: [u8; 1] = [0; 1];
    let mut peek_buf: [u8; 3] = [0; 3];
    let mut first_pass = true;

    loop {
        let n = r.peek(&mut peek_buf).await?;

        if &peek_buf == b"8=F" && !first_pass {
            break;
        }

        first_pass = false;

        match r.try_read(&mut buf) {
            Ok(0) => break,
            Ok(_) => sink.push(buf[0]),
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => break,
            Err(e) => return Err(e.into()),
        }

        if n == 1 {
            break;
        }
    }
    l.log_message(&sink.into())?;
    Ok(())
}

pub(super) async fn peek_stream(
    r: &mut TcpStream,
    buf: &mut [u8],
    _peek_len: usize,
) -> Result<usize, SessionError> {
    let mut num_read;
    loop {
        num_read = r.peek(&mut buf[..]).await?;
        if num_read == buf.len() {
            break;
        }
        if num_read == 0 {
            return Err(SessionError::TcpDisconnection);
        }
    }
    Ok(num_read)
}

pub(super) async fn disconnect(mut stream: TcpStream) {
    _ = stream.set_linger(Some(tokio::time::Duration::from_secs(0)));
    _ = stream.shutdown().await;
    std::mem::drop(stream);
}

pub(super) async fn send_message(
    msg_buf: &MsgBuf,
    r: &mut TcpStream,
    l: &mut impl Logger,
) -> Result<(), SessionError> {
    r.write_all(&msg_buf[..]).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            SessionError::TcpDisconnection
        } else {
            e.into()
        }
    })?;
    l.log_message(msg_buf)?;
    Ok(())
}
