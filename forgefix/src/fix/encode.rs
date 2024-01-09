use crate::fix::checksum::AsyncChecksumWriter;
use crate::fix::generated::Tags;
use crate::SessionSettings;
use chrono::{DateTime, Utc};
use std::io::{Cursor, Write};
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub const TIME_FORMAT: &str = "%Y%m%d-%H:%M:%S%.3f";

pub fn formatted_time() -> String {
    format!("{}", Utc::now().format(TIME_FORMAT))
}

#[derive(Debug)]
pub struct MessageBuilder {
    preamble: Cursor<[u8; 32]>, // e.g. 8=FIX.4.2^9=_________________
    msg_type: char,
    main_buffer: Cursor<Vec<u8>>,
}

pub(super) const SOH: &[u8] = &[b'\x01'];

impl MessageBuilder {
    pub fn new(begin_string: &str, msg_type: char) -> Self {
        let mut writer = Cursor::new([0_u8; 32]);
        writer
            .write_fmt(format_args!("8={}\x019=", begin_string))
            .unwrap();
        let main_buffer = Cursor::new(Vec::with_capacity(1024));

        MessageBuilder {
            preamble: writer,
            msg_type,
            main_buffer,
        }
    }

    fn write_bytes(&mut self, buf: &[u8]) -> std::io::Result<()> {
        std::io::Write::write(&mut self.main_buffer, buf).map(|_| ())
    }

    pub fn push(mut self, tag_param: impl Into<u32>, value: &[u8]) -> Self {
        self.push_mut(tag_param, value);
        self
    }

    pub fn push_mut(&mut self, tag_param: impl Into<u32>, value: &[u8]) {
        let tag: u32 = tag_param.into();
        let _ = self.write_bytes(tag.to_string().as_bytes());
        let _ = self.write_bytes(b"=");
        let _ = self.write_bytes(value);
        let _ = self.write_bytes(SOH);
    }

    fn body_len(&self) -> usize {
        let body_len = self.main_buffer.position() as usize;
        let msg_type_len = 5;
        body_len + msg_type_len
    }

    pub(super) async fn build_async<'a, W>(
        &self,
        sink: W,
        msg_seq_num: u32,
        additional_headers: &AdditionalHeaders,
        sending_time: DateTime<Utc>,
    ) -> std::io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let mut writer = AsyncChecksumWriter::new(sink);
        let body_len = self.body_len();
        let msg_seq_num_str = format!("34={}\x01", msg_seq_num);

        writer
            .write_all(&self.preamble.get_ref()[..self.preamble.position() as usize])
            .await?;
        let body_len_str =
            (body_len + additional_headers.len() + msg_seq_num_str.len()).to_string();
        writer.write_all(body_len_str.as_bytes()).await?;
        writer.write_all(SOH).await?;
        let msg_type_str = format!("35={}\x01", self.msg_type);
        writer.write_all(msg_type_str.as_bytes()).await?;
        writer.write_all(msg_seq_num_str.as_bytes()).await?;

        additional_headers
            .write_all(&mut writer, sending_time)
            .await?;

        writer.write_all(self.main_buffer.get_ref()).await?;
        let checksum: usize = writer.checksum();
        let checksum_str = format!("{:0>3}", checksum);
        writer.write_all(b"10=").await?;
        writer.write_all(checksum_str.as_bytes()).await?;
        writer.write_all(SOH).await?;
        Ok(())
    }

    pub fn msg_type(&self) -> char {
        self.msg_type
    }
}
#[derive(Default)]
pub struct SerializedInt([u8; 32], usize);

impl SerializedInt {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[self.0.len() - self.1..]
    }
}
impl From<u32> for SerializedInt {
    fn from(u: u32) -> Self {
        Self::from(u as u64)
    }
}
impl From<u64> for SerializedInt {
    fn from(u: u64) -> Self {
        let mut ser: SerializedInt = Default::default();
        if u == 0 {
            ser.0[ser.0.len() - 1] = b'0';
            ser.1 = 1;
            return ser;
        }
        let mut n = u;
        let mut cursor = 0;
        while n > 0 {
            let quotient = n / 10;
            let remainder = n % 10;
            let at = ser.0.len() - 1 - cursor;
            ser.0[at] = b'0' + remainder as u8;
            n = quotient;
            cursor += 1;
        }
        ser.1 = cursor;
        ser
    }
}

#[derive(Default, Debug)]
pub struct AdditionalHeaders {
    prefix: Vec<u8>,
    suffix: Vec<u8>,
}

fn format_fields(fields: &[(u32, Vec<u8>)]) -> Vec<u8> {
    let mut buf = Cursor::new(vec![]);
    for (tag, value) in fields {
        std::io::Write::write_fmt(&mut buf, format_args!("{}=", tag)).unwrap();
        std::io::Write::write(&mut buf, value).unwrap();
        std::io::Write::write(&mut buf, b"\x01").unwrap();
    }
    buf.into_inner()
}

impl AdditionalHeaders {
    pub fn new(fields: Vec<(u32, Vec<u8>)>) -> Self {
        let mut at = 0;
        for (i, (k, _)) in fields.iter().enumerate() {
            at = i;
            if *k > Tags::SendingTime.into() {
                break;
            }
        }
        let (prefix_fields, suffix_fields) = fields.split_at(at);
        AdditionalHeaders {
            prefix: format_fields(prefix_fields),
            suffix: format_fields(suffix_fields),
        }
    }

    pub fn build(settings: &SessionSettings) -> Self {
        AdditionalHeaders::new(comp_id_headers(
            &settings.sender_comp_id,
            &settings.target_comp_id,
        ))
    }

    pub(super) async fn write_all<W>(
        &self,
        w: &mut W,
        sending_time: DateTime<Utc>,
    ) -> std::io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let sending_time_field = format!(
            "{}={}\x01",
            u32::from(Tags::SendingTime),
            sending_time.format(TIME_FORMAT)
        )
        .into_bytes();
        assert_eq!(sending_time_field.len(), 21 + 4);
        w.write_all(&self.prefix[..]).await?;
        w.write_all(&sending_time_field[..]).await?;
        w.write_all(&self.suffix[..]).await
    }
    pub(super) fn len(&self) -> usize {
        self.prefix.len() + 25 + self.suffix.len()
    }
}

fn comp_id_headers(sender_comp_id: &str, target_comp_id: &str) -> Vec<(u32, Vec<u8>)> {
    vec![
        (
            u32::from(Tags::SenderCompID),
            sender_comp_id.as_bytes().to_vec(),
        ),
        (
            u32::from(Tags::TargetCompID),
            target_comp_id.as_bytes().to_vec(),
        ),
    ]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fix::checksum::ChecksumWriter;

    #[test]
    fn test_serialized_int() {
        let tests = vec![(1u32, "1"), (1918230917, "1918230917"), (0, "0")];
        for (num, s) in tests.into_iter() {
            let si: SerializedInt = num.into();
            assert_eq!(si.as_bytes(), s.as_bytes());
        }
    }

    #[test]
    fn test_direct_push() {
        let b: MessageBuilder = MessageBuilder::new("FIX.4.2", 'Q');
        b.push(
            44u32,
            crate::fix::generated::TimeInForce::GOOD_TILL_CANCEL.into(),
        );
    }

    fn create_message_builder() -> MessageBuilder {
        let b: MessageBuilder = MessageBuilder::new("FIX.4.2", 'Q');
        let data = b"asdfqwer12343456";
        b.push(44u32, &data[3..7]).push(88u32, &data[11..13])
    }
    #[tokio::test]
    async fn test_builder() {
        let mut buf = Vec::new();
        let mut cur = tokio::io::BufWriter::new(&mut buf);
        let additional_headers: AdditionalHeaders = Default::default();
        create_message_builder()
            .build_async(
                &mut cur,
                1,
                &additional_headers,
                std::time::UNIX_EPOCH.into(),
            )
            .await
            .expect("building");
        cur.flush().await.unwrap();
        unsafe {
            assert_eq!(
                String::from_utf8_unchecked(buf),
                "8=FIX.4.2\x019=49\x0135=Q\x0134=1\x0152=19700101-00:00:00.000\x0144=fqwe\x0188=43\x0110=245\x01"
            );
        }
    }

    #[tokio::test]
    async fn test_checksum() {
        let datas = vec![
            (179, b"8=FIX.4.2\x019=206\x0135=D\x0134=296\x0149=AMLRLLDMAUAT\x0152=20230126-14:30:45.444\x0156=GSLLDMAUAT\x011=AVFT1209\x0111=the-01GQQ7SXY4KBTRXCPSG1VRHJXE\x0122=J\x0138=25\x0140=2\x0144=1.25\x0148=MBB   230217P00097000\x0154=1\x0159=3\x0160=20230126-14:30:45\x0177=O\x01100=EMLD\x01")
        ];
        for (checksum, d) in datas {
            let mut buf: Vec<u8> = Vec::new();
            let cur = std::io::Cursor::new(&mut buf);
            let mut cb = ChecksumWriter::new(cur);
            cb.write(d).expect("writing");
            assert_eq!(cb.checksum(), checksum);
        }
    }
    #[test]
    fn test_format_fields() {
        let fs = vec![(1, b"asdf".to_vec()), (2, b"qwer".to_vec())];
        assert_eq!(format_fields(&fs[..]), b"1=asdf\x012=qwer\x01");
    }

    #[test]
    fn test_additional_headers() {
        let fs = vec![
            (Tags::SenderCompID as u32, b"asdf".to_vec()),
            (Tags::TargetCompID as u32, b"qwer".to_vec()),
        ];
        let ah = AdditionalHeaders::new(fs);
        assert_eq!(b"49=asdf\x01", &ah.prefix[..]);
        assert_eq!(b"56=qwer\x01", &ah.suffix[..]);
    }
}
