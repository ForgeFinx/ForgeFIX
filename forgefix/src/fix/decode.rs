//! Message decoding and parsing
//!
//! Application messages that come off the wire are stored in a [`MsgBuf`]. Message buffers are just wrappers
//! around a [`Vec<u8>`], and have yet to be parsed and verified. In order to extract the tag/value
//! pairs from a message, it must be parsed using the [`parse`] function which accepts a [`MsgBuf`]
//! and a [`ParserCallback`]. The callback defines which tags to parse, implements how to parse the value 
//! and can either save the value, or return an error. 
//!
//! [`MsgBuf`]: crate::fix::mem::MsgBuf

use crate::fix::generated::{get_data_ref, SessionRejectReason};
use crate::fix::{GarbledMessageType, SessionError};
use anyhow::{format_err, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use lazy_static::lazy_static;
use std::collections::{BTreeSet, HashMap};

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

#[derive(PartialEq, Eq, Debug)]
enum FieldState {
    Start,
    InTag,
    SeenEquals,
    InField,
    Error,
}
struct FieldIter<'a, 'i, 'x> {
    inner: std::iter::Enumerate<std::iter::Cloned<std::slice::Iter<'a, u8>>>,
    state: FieldState,
    tag_accum: u32,
    field_start: usize,
    include_fields: Option<&'i BTreeSet<u32>>,
    exclude_fields: Option<&'x BTreeSet<u32>>,
}
impl<'a, 'i, 'x> Iterator for FieldIter<'a, 'i, 'x> {
    type Item = Result<(u32, (usize, usize))>;

    fn next(&mut self) -> Option<Self::Item> {
        for (i, b) in &mut self.inner {
            let c = b as char;
            match (&self.state, c) {
                (&FieldState::Start, '0'..='9') | (&FieldState::InTag, '0'..='9') => {
                    if self.state == FieldState::Start {
                        self.tag_accum = 0;
                    } else {
                        self.tag_accum *= 10;
                    }
                    self.tag_accum += b as u32 - '0' as u32;
                    self.state = FieldState::InTag;
                }
                (&FieldState::InTag, '=') => {
                    self.field_start = i + 1;
                    self.state = FieldState::SeenEquals;
                }
                (&FieldState::SeenEquals, '\x01') | (&FieldState::InField, '\x01') => {
                    let should_stop = self
                        .include_fields
                        .as_ref()
                        .map(|fields| !fields.contains(&self.tag_accum))
                        .unwrap_or(false);
                    let reached_end = self
                        .exclude_fields
                        .as_ref()
                        .map(|fields| fields.contains(&self.tag_accum))
                        .unwrap_or(false);
                    if should_stop || reached_end {
                        break;
                    }
                    self.state = FieldState::Start;
                    return Some(Ok((
                        self.tag_accum,
                        (self.field_start, i - self.field_start),
                    )));
                }
                (&FieldState::SeenEquals, _) | (&FieldState::InField, _) => {
                    if self.state != FieldState::InField {
                        self.state = FieldState::InField;
                    }
                }
                (&FieldState::Error, _) => return None,
                _ => {
                    self.state = FieldState::Error;
                    return Some(Err(format_err!(
                        "{}: invalid char at {} while in {:?}",
                        c,
                        i,
                        self.state
                    )));
                }
            }
        }

        None
    }
}

impl<'a, 'i, 'x> FieldIter<'a, 'i, 'x> {
    #[allow(dead_code)]
    fn new_header(msg: &'a [u8]) -> Self {
        FieldIter {
            inner: msg.iter().cloned().enumerate(),
            state: FieldState::Start,
            tag_accum: 0,
            field_start: 0,
            include_fields: Some(&*HEADER_FIELDS),
            exclude_fields: None,
        }
    }

    #[allow(dead_code)]
    fn new_body(msg: &'a [u8]) -> Self {
        FieldIter {
            inner: msg.iter().cloned().enumerate(),
            state: FieldState::Start,
            tag_accum: 0,
            field_start: 0,
            include_fields: None,
            exclude_fields: Some(&*TRAILER_FIELDS),
        }
    }
}

/// A trait that allows custom parsing of a [`MsgBuf`] 
///
/// [`MsgBuf`]: crate::fix::mem::MsgBuf
pub trait ParserCallback<'a> {
    fn header(&mut self, key: u32, value: &'a [u8]) -> Result<bool, SessionError>;
    fn body(&mut self, key: u32, value: &'a [u8]) -> Result<bool, SessionError>;
    fn trailer(&mut self, key: u32, value: &'a [u8]) -> Result<bool, SessionError>;
    fn sequence_num(&self) -> u32;
}

/// A default implementation of [`ParserCallback`]
pub struct NullParserCallback;

impl<'a> ParserCallback<'a> for NullParserCallback {
    fn header(&mut self, _key: u32, _value: &'a [u8]) -> Result<bool, SessionError> {
        Ok(true)
    }
    fn body(&mut self, _key: u32, _value: &'a [u8]) -> Result<bool, SessionError> {
        Ok(true)
    }
    fn trailer(&mut self, _key: u32, _value: &'a [u8]) -> Result<bool, SessionError> {
        Ok(true)
    }
    fn sequence_num(&self) -> u32 {
        0
    }
}

/// Parse a [`MsgBuf`] with a [`ParserCallback`]
///
/// [`MsgBuf`]: crate::fix::mem::MsgBuf
pub fn parse<'a>(
    msg: &'a [u8],
    callbacks: &mut impl ParserCallback<'a>,
) -> Result<(), SessionError> {
    let mut field_lengths: HashMap<u32, u32> = HashMap::new();
    let mut state = FieldState::Start;
    let mut tag_accum: u32 = 0;
    let mut field_start: usize = 0;
    let mut iter = msg.iter().enumerate();
    while let Some((i, b)) = iter.next() {
        let c = *b as char;
        match (&state, c) {
            (&FieldState::Start, '0'..='9') | (&FieldState::InTag, '0'..='9') => {
                if state == FieldState::Start {
                    tag_accum = 0;
                } else {
                    tag_accum *= 10;
                }
                tag_accum += *b as u32 - '0' as u32;
                state = FieldState::InTag;
            }
            (&FieldState::InTag, '=') => {
                field_start = i + 1;
                if let Some(len) = field_lengths.get(&tag_accum) {
                    skip_ahead(&mut iter, len - 1);
                }
                state = FieldState::SeenEquals;
            }
            (&FieldState::SeenEquals, '\x01') | (&FieldState::InField, '\x01') => {
                if let Some(tag) = get_data_ref(tag_accum) {
                    field_lengths.insert(
                        tag,
                        bytes_to_u32(&msg[field_start..i]).ok_or(
                            SessionError::new_message_rejected(
                                Some(SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE),
                                callbacks.sequence_num(),
                                Some(tag_accum),
                                None,
                            ),
                        )?,
                    );
                }
                let cont =
                    if HEADER_FIELDS.contains(&tag_accum) || TRAILER_FIELDS.contains(&tag_accum) {
                        callbacks.header(tag_accum, &msg[field_start..i])?
                    } else {
                        callbacks.body(tag_accum, &msg[field_start..i])?
                    };
                if !cont {
                    break;
                }

                state = FieldState::Start;
            }
            (&FieldState::SeenEquals, _) | (&FieldState::InField, _) => {}
            _ => {
                return Err(SessionError::GarbledMessage {
                    text: format!("{}: invalid char at {} while in {:?}", c, i, state),
                    garbled_msg_type: GarbledMessageType::Other,
                });
            }
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
fn skip_ahead<T: Iterator>(iter: &mut T, n: u32) {
    for _ in 0..n {
        _ = iter.next();
    }
}

pub(super) struct ParsedPeek {
    pub msg_type: char,
    pub msg_length: usize,
    pub len_start: usize,
    pub len_end: usize,
    pub fixed_fields_end: usize,
}
pub(super) fn parse_peeked_prefix(peeked: &[u8]) -> Result<ParsedPeek, SessionError> {
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
/// [`FromStr`]: std::str::FromStr
pub fn parse_field<T>(field: &[u8]) -> Result<T> 
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    std::str::from_utf8(field)?.parse::<T>().map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) fn parse_sending_time(sending_time_bytes: &[u8]) -> Result<DateTime<Utc>> {
    let sending_time_str = std::str::from_utf8(sending_time_bytes)?;
    let sending_time = NaiveDateTime::parse_from_str(sending_time_str, TIME_FORMAT_SHORT).or(
        NaiveDateTime::parse_from_str(sending_time_str, TIME_FORMAT_LONG),
    )?;
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
}
