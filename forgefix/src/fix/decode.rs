//! Message decoding and parsing
//!
//! This module provides tools for decoding information from an array of bytes representing a FIX
//! message. One of the design choices of ForgeFIX was for message parsing to occur on-demend
//! instead of automatically. This way, only when necessary are messages decoded, and only what is
//! necessary will be parsed from the message. 
//!
//! # Terminalogy
//! * `message` - An entire FIX message, which is represented as an array of bytes in a [`MsgBuf`].
//! There are two types of messages: Session and Application. Session message control the FIX
//! session and are managed entirely by the FIX engine. Application messages are what peers create
//! and send to each other, and are almost entirely managed by the user. The FIX engine only
//! gurantees that the user receives the application messages in the correct order.
//!
//! * `fields` -- A tag/value pair. The tag and value are connected with an `=`. Multiple fields
//! are delimited with an `SOH`. A message is just list of fields.
//!
//! * `tags` --  A tag is on the left side of the `=` and describes what kind of information the value
//! represents. All valid FIX tags can be found in the [FIX dictionary], and are represented in the
//! [`Tags`] enum. 
//!
//! * `values` -- A value is on the right side of the `=` and contains the actual information for the
//! field. FIX values are Utf8 encoded and have one of the following types: int, float, String,
//! char or data (see [FIX dictionary] for more info). Furthermore, for some fields, only a subset
//! of values for the type are considered valid. These are called value sets for that field. All
//! value sets are represented by enums in the [generated] module. 
//!
//! * `SOH` -- The character that delimits the fields in a message. An SOH is represented with ascii
//! code 1. For displaying, a `|` is often used to show an SOH. In rust, an SOH is represented as a
//! byte: `b'\x01'`. 
//!
//! # Decoding
//!
//! Parsing of messages is done with the [`parse`] function which depends on a user defined
//! [`ParserCallback`]. The parse function splits a message into fields, and then tag/value pairs.
//! These pairs are sent to the callback, where the user defines which to parse. 
//!
//! The [`Tags`] enum and [`parse_field`] function are tools to support parsing of tags and values. 
//!
//! # Errors
//!
//! If a message is malformed or contains invalid data, then decoding the message will likely cause an error. 
//! The FIX specification recommends being fault tolerant when processing application level
//! messages. ForgeFIX follows this recommendation which is reflected in the decode error types. 
//!
//! [`MessageParseError`]: errors that occur when a message fails to meet the FIX spec for message
//! structure, and the [`parse`] function is not able to split the message into its fields. This
//! error will always be tripped if any part of the message is malformed. 
//!
//! [`DecodeError`]: errors for invalid tags and values. A tag can be invalid if it is not a known tag number. 
//! A value can be invalid because it: is invalid UTF-8, cannot be parsed into a rust type, or does not exist in 
//! a value set. This error will only occur when a user attempts to parse an invalid tag or value. 
//!
//! [`MsgBuf`]: crate::fix::mem::MsgBuf
//! [FIX dictionary]: https://btobits.com/fixopaedia/fixdic42/index.html
//! [`Tags`]: crate::fix::generated::Tags
//! [generated]: crate::fix::generated
//!
//! # Example
//!
//! ```no_run
//! use anyhow::{Error, bail, Result}; 
//! use forgefix::fix::decode::{ParserCallback, parse_field, parse, MessageParseError}; 
//! use forgefix::fix::generated::{Tags, MsgType, ExecType, OrdStatus};
//!
//! #[derive(Debug)]
//! struct ExecutionReportParser<'a> {
//!     order_id: &'a str,
//!     order_status: OrdStatus,
//!     exec_type: ExecType,
//!     qty_filled: f32,
//! }
//!
//! impl<'a> Default for ExecutionReportParser<'a> {
//!     fn default() -> Self {
//!         ExecutionReportParser {
//!             order_id: Default::default(),
//!             order_status: OrdStatus::NEW,
//!             exec_type: ExecType::NEW,
//!             qty_filled: Default::default(),
//!         }
//!     }
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
//!     fn parse_error(&mut self, err: MessageParseError) -> Result<(), Self::Err> {
//!         Err(err.into())
//!     }
//! }
//!
//! # use forgefix::{SessionSettings, FixApplicationInitiator}; 
//! #[tokio::main]
//! async fn main() -> Result<()> {
//! 
//!     // create SessionSettings...
//!
//! #    let settings = SessionSettings::builder()
//! #        .with_sender_comp_id("my_id")
//! #        .with_target_comp_id("peer_id")
//! #        .with_store_path("./store".into())
//! #        .with_log_dir("./log".into())
//! #        .with_socket_addr("127.0.0.1:0".parse().unwrap())
//! #        .build()?; 
//!     let (handle, mut receiver) = FixApplicationInitiator::build(settings)?
//!         .initiate()
//!         .await?;
//!
//!     tokio::spawn(async move {
//!         while let Some(msg) = receiver.recv().await {
//!             let mut callback: ExecutionReportParser = Default::default(); 
//!             match parse(&msg[..], &mut callback) {
//!                 Ok(()) => println!("Received execution report: {:?}", callback),
//!                 Err(e) => println!("Error parsing execution report: {:?}", e),
//!             }
//!         }
//!     }); 
//!
//!     // run application ...
//!     # Ok(())
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

/// Errors that can occur while splitting a message into fields.
#[derive(Error, Debug)]
pub enum MessageParseError {
    /// The message contained an unexpected byte. 
    ///
    /// The [`usize`] is the index of the unexpected byte, and the [`Vec<u8>`] will contain the
    /// entire message. 
    #[error("the value at index {0:?} was unexpected in message {1:?}")]
    UnexpectedByte(usize, Vec<u8>), 
    /// A length field's value could not be parsed. 
    ///
    /// The [`u32`] will be the length tag, and the [`Vec<u8>`] will contain its value that could
    /// not be parsed. 
    #[error("could not parse value {1:?} of length field {0:?}")]
    BadLengthField(u32, Vec<u8>), 
}

/// Errors that can occur while decoding a FIX message. 
#[derive(Error, Debug)]
pub enum DecodeError {
    /// The Message could not be parsed into fields 
    #[error("Message could not be parsed into fields: {0:?}")]
    BadMessage(#[from] MessageParseError),
    /// A field contained an unknown tag
    ///
    /// The [`u32`] contains the tag value
    #[error("{0:?} does not match a known Tag")]
    UnknownTag(u32), 
    /// A field contained invalid utf8
    #[error("FIX message contained invalid utf8: {0:?}")]
    Utf8Error(#[from] std::str::Utf8Error),
    /// A field's value could not be parsed
    ///
    /// The [`Vec<u8>`] contains the value
    #[error("Value {0:?} could not be parsed")]
    BadValue(Vec<u8>),
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

/// A trait that defines parsing of tag/values in a [`MsgBuf`], and is required to call the [`parse`]
/// function.
///
/// The `ParserCallback` defines methods that get called for certain parsing events. 
///
/// ## Events
///
/// * Header field found -- the [`header`] function is called 
/// * Body field found -- the [`body`] function is called 
/// * Trailer field found -- the [`trailer`] function is called 
/// * A [`MessageParseError`] occurs -- the [`parse_error`] function is called 
///
/// To see which fields are headers or trailers, see [FIX dictionary]. All other fields are
/// considered body fields. 
///
/// ## Return Values 
///
/// * [`header`], [`body`] and [`trailer`] -- Return `Ok(true)` to signal that parsing should
/// continue. Return `Ok(false)` to signal that parsing should end. Return `Err` if an error
/// occured that should cause parsing to stop. 
///
/// * [`parse_error`] -- Convert the [`MessageParseError`] into a `Result<(), Self::Err>`
///
/// [FIX dictionary]: https://btobits.com/fixopaedia/fixdic42/index.html
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

    /// Called if a [`MessageParseError`] occurs
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

/// Parse a [`MsgBuf`] and store values in a [`ParserCallback`]
///
/// [`MsgBuf`]: crate::fix::mem::MsgBuf
///
/// # Notes
///
/// The `parse` function iterates over each field and splits each field into a tag/value pair. Then, 
/// each tag/value pair is passed to the `callback`'s methods. 
///
/// In the event that splitting a message into fields causes a [`MessageParseError`], the created
/// error will be passed to the callback.
///
/// [`parse`] will return early with `Ok(())` if at any point the callback returns `Ok(false)`.
/// `Ok(())` will also be returned once all the fields have been iterated over. 
///
/// # Errors
///
/// If at any point the `callback` return an `Err`, [`parse`] will end and return the err. 
///
/// If at any point a [`MessageParseError`] occurs,
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

pub(super) fn parse_header(header: &[u8]) -> Result<usize, SessionError> {
    let prefix = parse_peeked_prefix(header)?; 
    // body_length does not account for the 7 byte checksum (10=xxx|) 
    // and len_end is 1 less that we would like 
    Ok(prefix.body_length - (header.len() - (prefix.len_end + 1)) + 7)
}

pub(super) struct ParsedPeek {
    pub msg_type: char,
    #[allow(dead_code)]
    pub msg_length: usize,
    pub len_start: usize,
    pub len_end: usize,
    pub fixed_fields_end: usize,
    pub body_length: usize,
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
        body_length,
    })
}

/// Attempts to parse a FIX value into any type that `impl`'s [`FromStr`]
///
/// # Primitives
///
/// Rust primitives generally `impl` [`FromStr`]. And most FIX data type can be represented by rust primitives. Consider 
/// using the following for each FIX type: 
/// * `int` -- [`i32`], [`u32`]
/// * `float` -- [`f32`]
/// * `char` -- [`char`]
/// * `String` -- [`&str`]*, [`String`]
/// * `data` -- `&[u8]`
///
/// *[`&str`] does not itself `impl` [`FromStr`], so just use [`from_utf8`]. Since [`DecodeError`]
/// `impl`'s [`From<std::str::Utf8Error>`], the result can easily be converted
///
/// # Tags 
///
/// FIX tags are automatically converted into a [`u32`]. The [`Tags`] enum `impl`'s
/// [`TryFrom<u32>`]. 
///
/// # Value Sets 
///
/// All FIX value sets are implemented as enums in the `generated` module. To convert a value into
/// its enum, first convert to the corresponding primitive ([`char`] or [`u8`]). And then
/// all enums `impl` [`TryFrom`] for either [`char`] or [`u8`]. 
///
///
/// [`FromStr`]: std::str::FromStr
/// [`MsgType`]: crate::fix::generated::MsgType
/// [`from_utf8`]: std::str::from_utf8
///
/// # Example
///
/// ```rust
/// # use forgefix::fix::generated::{EncryptMethod, OrdStatus, MsgType}; 
/// # use forgefix::fix::decode::parse_field;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     // MsgType is specified as a char and has a value set
///     let msg_type_field = b"A"; 
///     let msg_type: MsgType = parse_field::<char>(msg_type_field)?.try_into()?; 
///     assert_eq!(msg_type, MsgType::LOGON); 
///
///     // Prices are floats, so parse into an f32
///     let price_field = b"1.13"; 
///     let price = parse_field::<f32>(price_field)?; 
///     assert_eq!(price, 1.13f32); 
///
///     // OrdStatus is also specified as a char and has a value set 
///     let ord_status_field = b"0"; 
///     let ord_status: OrdStatus = parse_field::<char>(ord_status_field)?.try_into()?;
///     assert_eq!(ord_status, OrdStatus::NEW); 
///
///     // To parse into a &str, just use std::str::from_utf8
///     let order_id_field = b"abc123"; 
///     let order_id: &str = std::str::from_utf8(order_id_field)?; 
///     assert_eq!(order_id, "abc123"); 
///
///     // EncryptMethod is specified as an int and has a value set
///     let encrypt_method_field = b"0"; 
///     let encrypt_method: EncryptMethod  = parse_field::<u8>(encrypt_method_field)?.try_into()?; 
///     assert_eq!(encrypt_method, EncryptMethod::NONE);
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
