use crate::fix::checksum::AsyncChecksumWriter;
use crate::fix::decode::ParsedPeek;
use crate::fix::encode::{SerializedInt, SOH, TIME_FORMAT};
use crate::fix::SessionError;
use anyhow::Result;
use chrono::offset::Utc;
use std::str;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub(super) struct Transformer {
    msg: Vec<u8>,
    len_start: usize,
    len_end: usize,
    sending_time_start: usize,
    sending_time_end: usize,
    fixed_fields_end: usize,
    pub msg_type: char,
}

const POSS_DUP_FLAG_EQ_Y: &[u8] = b"43=Y\x01";
const ORIG_SENDING_TIME_TAG: &[u8] = b"122=";

impl Transformer {
    fn original_sending_time(&self) -> &[u8] {
        &self.msg[self.sending_time_start..self.sending_time_end]
    }

    pub(super) async fn build_async<'a, W>(self, sink: W) -> Result<(), SessionError>
    where
        W: AsyncWrite + Unpin,
    {
        let mut writer = AsyncChecksumWriter::new(sink);

        let len_bytes = &self.msg[self.len_start..self.len_end];
        let old_len: u32 = str::from_utf8(len_bytes)
            .or(Err(SessionError::ResendError))?
            .parse()
            .or(Err(SessionError::ResendError))?;

        // get the original sending time and new sending time
        let orig_sending_time: &[u8] = self.original_sending_time();
        let new_sending_time = format!("{}", Utc::now().format(TIME_FORMAT));

        // calc the new sending time len
        let new_sending_time_len = new_sending_time.len() as u32;

        // update the Length
        let new_len = old_len
            + new_sending_time_len
            + POSS_DUP_FLAG_EQ_Y.len() as u32
            + ORIG_SENDING_TIME_TAG.len() as u32
            + 1;

        // add the new Length checksum and the POSS dup flag to the total checksom
        let new_len_bytes = SerializedInt::from(new_len);

        // we subtracted out old checksum contribution, so it could be negative

        writer.write_all(&self.msg[..self.len_start]).await?;
        writer.write_all(new_len_bytes.as_bytes()).await?;
        writer.write_all(SOH).await?;
        writer
            .write_all(&self.msg[self.len_end + 1..self.fixed_fields_end])
            .await?;
        writer.write_all(POSS_DUP_FLAG_EQ_Y).await?;
        writer
            .write_all(&self.msg[self.fixed_fields_end..self.sending_time_start])
            .await?;
        writer.write_all(new_sending_time.as_bytes()).await?;
        writer.write_all(SOH).await?;
        writer.write_all(ORIG_SENDING_TIME_TAG).await?;
        writer.write_all(orig_sending_time).await?;
        writer.write_all(SOH).await?;
        writer
            .write_all(&self.msg[self.sending_time_end + 1..self.msg.len() - 7])
            .await?;
        let checksum_str = format!("{:0>3}", writer.checksum());
        writer.write_all(b"10=").await?;
        writer.write_all(checksum_str.as_bytes()).await?;
        writer.write_all(SOH).await?;
        Ok(())
    }
}

impl TryFrom<Vec<u8>> for Transformer {
    type Error = crate::fix::SessionError;

    fn try_from(msg: Vec<u8>) -> Result<Transformer, SessionError> {
        let ParsedPeek {
            msg_type,
            len_start,
            len_end,
            fixed_fields_end,
            ..
        } = crate::fix::decode::parse_peeked_prefix(&msg[..])?;
        let (sending_time_start, sending_time_end) = sending_time_indices(&msg);
        Ok(Transformer {
            msg,
            msg_type,
            len_start,
            len_end,
            sending_time_start,
            sending_time_end,
            fixed_fields_end,
        })
    }
}

fn sending_time_indices(msg: &[u8]) -> (usize, usize) {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut found_start = false;
    for (i, b) in msg.iter().enumerate().skip(4) {
        if !found_start && &msg[(i - 4)..i] == b"\x0152=" {
            found_start = true;
            start = i;
        }
        if found_start && b == &b'\x01' {
            end = i;
            break;
        }
    }
    (start, end)
}

#[cfg(test)]
mod test {
    use super::*;

    const POSS_DUP_FLAG_EQ_Y_CHECKSUM: i32 = 254;

    #[test]
    fn test_poss_dup_flag_const() {
        let sum: i32 = POSS_DUP_FLAG_EQ_Y.iter().map(|c| *c as i32).sum();
        assert_eq!(sum, POSS_DUP_FLAG_EQ_Y_CHECKSUM);
    }

    #[tokio::test]
    async fn test_transformer() {
        //checksum = (/*oldcheck*/ 55 - (50+53) + (51+48) + /* possdup */ 2+51+61+89+1) = 328
        let data = vec![(
            b"8=FIX.4.2\x019=25\x0135=Q\x0152=20230808-13:19:54.537\x0134=0\x0144=fqwe\x0188=43\x0110=055\x01",
            b"8=FIX.4.2\x019=56\x0135=Q\x0143=Y\x0152=00000000-00:00:00.000\x01122=20230808-13:19:54.537\x0134=0\x0144=fqwe\x0188=43\x0110=049\x01",
        )];
        for (orig, expected) in data {
            let in_msg = &orig[..];
            let t: Transformer = in_msg.to_vec().try_into().unwrap();
            let mut buf = Vec::new();
            let mut cur = tokio::io::BufWriter::new(&mut buf);
            t.build_async(&mut cur).await.expect("building");
            cur.flush().await.unwrap();

            assert_eq!(
                std::str::from_utf8(&buf[..buf.len() - 7]).unwrap()[..25],
                std::str::from_utf8(&expected[..expected.len() - 7]).unwrap()[0..25]
            );

            assert_eq!(
                std::str::from_utf8(&buf[..buf.len() - 7]).unwrap()[49..],
                std::str::from_utf8(&expected[..buf.len() - 7]).unwrap()[49..],
            );
        }
    }
}
