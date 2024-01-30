//! Message decoding and parsing
//!
//! Application messages that come off the wire are stored in a [`MsgBuf`]. Message buffers are just wrappers
//! around a [`Vec<u8>`], and have yet to be parsed and verified. In order to extract the tag/value
//! pairs from a message, it must be parsed using the [`parse`] function which accepts a [`MsgBuf`]
//! and a [`ParserCallback`]. The callback defines which tags to parse, implements how to parse the value 
//! and can save values, and handle error cases. 
//!
//! [`MsgBuf`]: crate::fix::mem::MsgBuf
//!
//! # Example
//!
//! ```rust
//! use anyhow::{Error, bail}; 
//!
//! use forgefix::fix::decode::{ParserCallback, parse_field, parse, ParseError}; 
//! use forgefix::fix::generated::{Tags, MsgType, ExecType, OrdStatus};
//!
//! struct ExecutionReportParser<'a> {
//!     order_id: &'a str,
//!     order_status: OrdStatus,
//!     exec_type: ExecType,
//!     qty_filled: f32,
//! }
//!
//! impl<'a> ParserCallback<'a> for ExecutionReportParser<'a> {
//!     type Err = Error; 
//!
//!     // parse and save any header fields...
//!     fn header(&mut self, key: u32, value: &'a [u8]) -> Result<bool, Self::Err> {
//!         if let Ok(Tags::MsgType) = key.try_into() {
//!             let msg_type = parse_field::<char>(value)?.try_into()?; 
//!             if !matches!(msg_type, MsgType::EXECUTION_REPORT) {
//!                 bail!("not an execution report message");
//!             }
//!         }
//!         Ok(true)
//!     }
//!
//!     // parse and save any body fields...
//!     fn body(&mut self, key: u32, value: &'a [u8]) -> Result<bool, Self::Err> {
//!         match key.try_into() {
//!             Ok(Tags::OrderID) => self.order_id = std::str::from_utf8(value)?, 
//!             Ok(Tags::OrdStatus) => {
//!                 self.order_status = parse_field::<char>(value)?.try_into()?;
//!             }
//!             Ok(Tags::ExecType) => {
//!                 self.exec_type = parse_field::<char>(value)?.try_into()?;
//!             }
//!             Ok(Tags::CumQty) => self.qty_filled = parse_field::<f32>(value)?, 
//!             _ => {}
//!         }
//!         Ok(true)
//!     }
//!
//!     // parse and save any trailer fields...
//!     fn trailer(&mut self, key: u32, value: &'a [u8]) -> Result<bool, Self::Err> {
//!         Ok(true)
//!     }
//!
//!     // if the message is malformed, catch the error and handle it...
//!     fn parse_error(&mut self, _err: ParseError) -> Result<(), Self::Err> {
//!         bail!("message is malformed");
//!     }
//! }
//! ```

use crate::fix::generated::{get_data_ref, Tags};
use crate::fix::{GarbledMessageType, SessionError};
use chrono::{DateTime, NaiveDateTime, Utc};
use lazy_static::lazy_static;
use std::collections::{BTreeSet, HashMap};
use std::result;
use thiserror::Error;

const TIME_FORMAT_SHORT: &str = "%Y%m%d-%H:%M:%S";
const TIME_FORMAT_LONG: &str = "%Y%m%d-%H:%M:%S%.3f";

lazy_static! {
    static ref HEADER_FIELDS: BTreeSet<u32> = [
        8, 9, 35, 49, 56, 115, 128, 90, 91, 34, 50, 142, 57, 143, 116, 129, 145, 43, 97, 52, 122,
        212, 213, 347, 369, 370,
    ]
    .iter()
    .cloned()
    .collect();
    static ref TRAILER_FIELDS: BTreeSet<u32> = [93, 89, 10].iter().cloned().collect();
}

#[derive(Error, Debug)]
pub enum MessageParseError {
    #[error("the value at index {0:?} was unexpected in message {1:?}")]
    UnexpectedByte(usize, Vec<u8>), 
    #[error("could not parse value {1:?} of length field {0:?}")]
    BadLengthField(u32, Vec<u8>), 
}

/// Errors that can occur while extracting fields from a FIX message. 
#[derive(Error, Debug)]
pub enum DecodeError {
    /// The Message could not be parsed into fields 
    #[error("Message could not be parsed into fields: {0:?}")]
    BadMessage(#[from] MessageParseError),
    /// A field contained invalid utf8
    #[error("FIX message contained invalid utf8: {0:?}")]
    Utf8Error(#[from] std::str::Utf8Error),
    /// A field's value could not be parsed
    ///
    /// The [`Vec<u8>`] contains the value
    #[error("Value {0:?} could not be parsed")]
    BadValue(Vec<u8>),
    /// A field contained an unknown tag
    ///
    /// The [`u32`] contains the tag value
    #[error("{0:?} does not match a known Tag")]
    UnknownTag(u32), 
    /// A character field did not match any known variant of a tag
    ///
    /// The attempted [`Tags`] and [`char`] are contained in the error
    #[error("char {1:?} does not match a known variant of {0:?}")]
    UnknownChar(Tags, char),
    /// A int field did not match any known variant of a tag
    ///
    /// The attempted [`Tags`] and [`u8`] are contained in the error 
    #[error("int {1:?} does not match a known variant of {0:?}")]
    UnknownInt(Tags, u8),
}
    
#[derive(PartialEq, Eq, Debug)]
enum FieldState {
    Start,
    InTag,
    SeenEquals,
    InField,
    Error,
}
struct FieldIter<'a> {
    inner: std::iter::Enumerate<std::slice::Iter<'a, u8>>,
    msg: &'a [u8],
    state: FieldState,
    field_start: usize,
    tag_accum: u32, 
    field_lengths: HashMap<u32, u32>,
}

impl<'a> FieldIter<'a> {
    fn new(msg: &'a [u8]) -> Self {
        FieldIter {
            inner: msg.iter().enumerate(), 
            msg,
            state: FieldState::Start,
            field_start: 0,
            tag_accum: 0,
            field_lengths: HashMap::new(), 
        }
    }

    fn skip_ahead(&mut self, n: u32) {
        for _ in 0..n {
            _ = self.inner.next();
        }
    }
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = Result<(u32, &'a [u8]), MessageParseError>; 

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((i, b)) = self.inner.next() {
            let c = *b as char; 
            match (&self.state, c) {
                (&FieldState::Start, '0'..='9') | (&FieldState::InTag, '0'..='9') => {
                    if self.state == FieldState::Start {
                        self.tag_accum = 0;
                    } else {
                        self.tag_accum *= 10; 
                    }
                    self.tag_accum += *b as u32 - '0' as u32; 
                    self.state = FieldState::InTag;
                }
                (&FieldState::InTag, '=') => {
                    self.field_start = i + 1;
                    if let Some(len) = self.field_lengths.get(&self.tag_accum) {
                        self.skip_ahead(len - 1);
                    }
                    self.state = FieldState::SeenEquals; 
                }
                (&FieldState::SeenEquals, '\x01') | (&FieldState::InField, '\x01') => {
                    let curr_value = &self.msg[self.field_start..i]; 
                    if let Some(tag) = get_data_ref(self.tag_accum) {
                        match bytes_to_u32(curr_value) {
                            Some(val) => {
                                self.field_lengths.insert(tag, val);
                            }
                            None => {
                                self.state = FieldState::Error; 
                                return Some(Err(MessageParseError::BadLengthField(
                                    self.tag_accum,
                                    curr_value.to_vec(),
                                )));
                            }
                        }
                    }
                    self.state = FieldState::Start; 
                    return Some(Ok((self.tag_accum, &self.msg[self.field_start..i]))); 
                }
                (&FieldState::SeenEquals, _) => self.state = FieldState::InField, 
                (&FieldState::InField, _) => {}
                (&FieldState::Error, _) => return None,
                _ => {
                    self.state = FieldState::Error; 
                    return Some(Err(MessageParseError::UnexpectedByte(i, self.msg.to_vec())));
                }
            }
        }
        None
    }
}

/// A trait that allows custom parsing of a [`MsgBuf`]
///
/// The `ParserCallback` contains methods that get called for certain parsing events. For example,
/// if a header field is found, the [`header`] function will be called. In the event of a
/// [`ParseError`], the [`parse_error`] function is called. 
///
/// For the [`header`], [`body`] and [`trailer`] functions, the returned boolean is a signal to the parser
/// to continue or to stop. If `Ok(true)` is returned, the parser will continue. If `Ok(false)` is
/// returned, the parser will stop, and return `Ok(())`. If any of these functions return an `Err`,
/// the parser will stop and return the `Err`.
///
/// [`MsgBuf`]: crate::fix::mem::MsgBuf
/// [`header`]: ParserCallback::header
/// [`body`]: ParserCallback::body
/// [`trailer`]: ParserCallback::trailer
/// [`parse_error`]: ParserCallback::parse_error
pub trait ParserCallback<'a> {
    type Err; 

    /// Called for any fields in message that are header fields.
    fn header(&mut self, key: u32, value: &'a [u8]) -> result::Result<bool, Self::Err>;
    
    /// Called for any fields in message that are body fields 
    fn body(&mut self, key: u32, value: &'a [u8]) -> result::Result<bool, Self::Err>;

    /// Called for any fields in message that are trailer fields 
    fn trailer(&mut self, key: u32, value: &'a [u8]) -> result::Result<bool, Self::Err>;

    /// Called if a [`ParseError`] occurs
    ///
    /// If a [`ParseError`] occurs, the [`parse`] function will call `parse_error`, and return its
    /// result. This is the oppurtunity to control the return value of [`parse`] in the case of a
    /// message tripping a [`ParseError`]
    fn parse_error(&mut self, err: MessageParseError) -> result::Result<(), Self::Err>; 
}

/// A default implementation of [`ParserCallback`]
pub struct NullParserCallback;

impl<'a> ParserCallback<'a> for NullParserCallback {
    type Err = DecodeError; 
    fn header(&mut self, _key: u32, _value: &'a [u8]) -> Result<bool, DecodeError> {
        Ok(true)
    }
    fn body(&mut self, _key: u32, _value: &'a [u8]) -> Result<bool, DecodeError> {
        Ok(true)
    }
    fn trailer(&mut self, _key: u32, _value: &'a [u8]) -> Result<bool, DecodeError> {
        Ok(true)
    }
    fn parse_error(&mut self, err: MessageParseError) -> Result<(), DecodeError> {
        Err(err.into())
    }
}

/// Parse a [`MsgBuf`] and store fields in a [`ParserCallback`]
///
/// [`MsgBuf`]: crate::fix::mem::MsgBuf
///
/// # Notes
///
/// A FIX message is made up of many FIX fields. A FIX field is a tag/value pair connected with an
/// `=`. Fields are delimited by an `SOH`. This can be represented by `\x01` or `|`. 
///
/// There are a few special fields which are allowed to contain an `SOH` in the value. Thus, they require a corresponding length 
/// field to specify they bytes for that value. For example, `SignatureLength(93)` says how long the
/// `Signature(89)` value will be. Which makes the following valid in a FIX message:
/// `"93=5|89=12\x0145"`. 
///
/// The `parse` function works by iterating over each field, and passing each tag/value pair to the
/// `callback`'s methods. 
///
/// [`parse`] will return early with `Ok(())` if at any point the callback returns `Ok(false)`. 
///
/// # Errors
///
/// If at any point the `callback` return an `Err`, [`parse`] will end and return the err. 
///
/// If at any point the next field cannot be extracted  due to the message being malformed,
/// `parse` will call [`ParserCallback::parse_error`] and return its result. 
pub fn parse<'a, T: ParserCallback<'a>>(
    msg: &'a [u8],
    callbacks: &mut T,
) -> result::Result<(), T::Err> 
{
    let field_iter = FieldIter::new(msg); 
    for res in field_iter {
        let (tag, val) = match res {
            Ok((t, v)) => (t, v),
            Err(e) => return callbacks.parse_error(e),
        };
        let cont =
            if HEADER_FIELDS.contains(&tag) {
                callbacks.header(tag, val)?
            } else if TRAILER_FIELDS.contains(&tag) {
                callbacks.trailer(tag, val)?
            } else {
                callbacks.body(tag, val)?
            };
        if !cont {
            break;
        }
    }
    Ok(())
}

fn bytes_to_u32(bytes: &[u8]) -> Option<u32> {
    let mut accum: u32 = 0;
    for b in bytes.iter() {
        if *b < b'0' || b'9' < *b {
            return None;
        }
        accum = match accum
            .checked_mul(10_u32)
            .and_then(|r| r.checked_add((b - b'0').into()))
        {
            Some(v) => v,
            _ => {
                return None;
            }
        }
    }
    Some(accum)
}

pub(super) struct ParsedPeek {
    pub msg_type: char,
    pub msg_length: usize,
    pub len_start: usize,
    pub len_end: usize,
    pub fixed_fields_end: usize,
}
pub(super) fn parse_peeked_prefix(peeked: &[u8]) -> result::Result<ParsedPeek, SessionError> {
    const EXPECTED_PREFIX: &[u8] = b"8=FIX.4.2\x019=";
    if &peeked[..2] == b"8=" && &peeked[2..9] != b"FIX.4.2" {
        return Err(SessionError::new_garbled_message(
            String::from("Incorrect BeginString"),
            GarbledMessageType::BeginStringIssue,
        ));
    }

    if &peeked[..EXPECTED_PREFIX.len()] != EXPECTED_PREFIX {
        return Err(SessionError::new_garbled_message(
            String::from("BeginString not first"),
            GarbledMessageType::Other,
        ));
    }
    let mut at = EXPECTED_PREFIX.len();
    let mut body_length: usize = 0;
    let mut saw_end = false;
    for c in peeked[EXPECTED_PREFIX.len()..].iter() {
        at += 1;
        match *c as char {
            '0'..='9' => {
                body_length =
                    body_length
                        .checked_mul(10)
                        .ok_or(SessionError::new_garbled_message(
                            String::from("BodyLength too large"),
                            GarbledMessageType::BodyLengthIssue,
                        ))?;
                body_length = body_length.checked_add((*c - (b'0')) as usize).ok_or(
                    SessionError::new_garbled_message(
                        String::from("BodyLength too large"),
                        GarbledMessageType::BodyLengthIssue,
                    ),
                )?;
            }
            '\x01' => {
                saw_end = true;
                break;
            }
            _ => {
                return Err(SessionError::new_garbled_message(
                    String::from("Illegal character in BodyLength"),
                    GarbledMessageType::BodyLengthIssue,
                ));
            }
        }
    }
    let len_end = at - 1;
    if !saw_end {
        return Err(SessionError::new_garbled_message(
            String::from("BodyLength too large"),
            GarbledMessageType::BodyLengthIssue,
        ));
    }
    let msg_type = if &peeked[at..at + 3] == b"35=" && peeked[at + 4] == b'\x01' {
        peeked[at + 3]
    } else {
        return Err(SessionError::new_garbled_message(
            String::from("Missing MsgType"),
            GarbledMessageType::MsgTypeIssue,
        ));
    };
    let fixed_fields_end = at + 5;

    // "at" is at the first character counted by BodyLength
    // BodyLength is the count of all the bytes up until and including the SOH before the checksum
    // the checksum will always be 10=xxx| which is 7 bytes
    // the value of "at" also represents the number of bytes in the message before the first byte counted by body length
    //  Therefore, "at" + "body_length" + 7 = total_msg_length
    let msg_length = body_length + at + 7;
    Ok(ParsedPeek {
        msg_type: msg_type as char,
        msg_length,
        len_start: EXPECTED_PREFIX.len(),
        len_end,
        fixed_fields_end,
    })
}

/// Attempts to parse a FIX value into any type that `impl`'s [`FromStr`]
///
/// In order to parse a field into an enum like [`MsgType`], parse the field into an [`char`] and
/// then try converting the [`char`] into a [`MsgType`].
///
/// In order to parse a field into a [`String`], use `parse_field`. However, to parse into a
/// [`&str`], it is recommended to use [`from_utf8`]. 
///
/// [`FromStr`]: std::str::FromStr
/// [`MsgType`]: crate::fix::generated::MsgType
/// [`from_utf8`]: std::str::from_utf8
///
/// # Example
///
/// ```rust
/// # use forgefix::fix::generated::{OrdStatus, MsgType}; 
/// # use forgefix::fix::decode::parse_field;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let msg_type_field = b"A"; 
///     let msg_type: MsgType = parse_field::<char>(msg_type_field)?.try_into()?; 
///     assert_eq!(msg_type, MsgType::LOGON); 
///
///     let price_field = b"1.13"; 
///     let price = parse_field::<f32>(price_field)?; 
///     assert_eq!(price, 1.13f32); 
///
///     let ord_status_field = b"0"; 
///     let ord_status: OrdStatus = parse_field::<char>(ord_status_field)?.try_into()?;
///     assert_eq!(ord_status, OrdStatus::NEW); 
///
///     let order_id_field = b"abc123"; 
///     let order_id: &str = std::str::from_utf8(order_id_field)?; 
///     assert_eq!(order_id, "abc123"); 
///     # Ok(())
/// # }
/// ```
pub fn parse_field<T>(field: &[u8]) -> Result<T, DecodeError> 
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    std::str::from_utf8(field)?.parse::<T>()
        .map_err(|_| DecodeError::BadValue(field.to_vec()))
}

pub(super) fn parse_sending_time(sending_time_bytes: &[u8]) -> Result<DateTime<Utc>, DecodeError> {
    let sending_time_str = std::str::from_utf8(sending_time_bytes)?;
    let sending_time = NaiveDateTime::parse_from_str(sending_time_str, TIME_FORMAT_SHORT)
        .or_else(|_| NaiveDateTime::parse_from_str(sending_time_str, TIME_FORMAT_LONG))
        .map_err(|_| DecodeError::BadValue(sending_time_bytes.to_vec()))?; 
    Ok(sending_time.and_utc())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_body_length_too_long() {
        if let Ok(_) = parse_peeked_prefix(b"8=FIX.4.2\x019=33333333333333333333333") {
            assert!(false, "Expected error");
        };
    }

    #[test]
    fn test_bytes_to_u32() {
        assert_eq!(bytes_to_u32(b"234").unwrap(), 234);
        assert_eq!(bytes_to_u32(b"0").unwrap(), 0);
        assert_eq!(
            bytes_to_u32(b"11111111111111111111111111111111111111").is_none(),
            true
        );
        assert_eq!(bytes_to_u32(b"a").is_none(), true);
    }

    #[test]
    fn test_field_iter() {
        let messages: Vec<&[u8]> = vec![
            b"8=FIX.4.2\x019=44\x018=A\x0110=123\x01",
            b"8\x01=FIX.4.2",
            b"93=6\x018=A\x0189=12\x01456\x0110=123\x01",
            b"93=6A\x018=A\x0189=12\x01456\x0110=123\x01",
        ];

        let expected: Vec<Vec<Result<(u32, &[u8]), ()>>> = vec![
            vec![Ok((8, b"FIX.4.2")), Ok((9, b"44")), Ok((8, b"A")), Ok((10, b"123"))],
            vec![Err(())],
            vec![Ok((93, b"6")), Ok((8, b"A")), Ok((89, b"12\x01456")), Ok((10, b"123"))],
            vec![Err(())],
        ];

        for (msg, ex) in messages.iter().zip(expected.iter()) {
            let field_iter = FieldIter::new(&msg[..]); 
            for (got, exp) in field_iter.zip(ex.iter()) {
                if exp.is_err() {
                    assert!(got.is_err(), "Expected error");
                } else {
                    assert_eq!(got.unwrap(), *exp.as_ref().unwrap()); 
                }
            }
        }
    }
}
