use crate::fix::checksum::checksum_is_valid;
use crate::fix::generated::{MsgType, SessionRejectReason, Tags};
use crate::fix::mem::MsgBuf;
use crate::fix::{GarbledMessageType, SessionError};

use chrono::{DateTime, Duration, Utc};

#[allow(clippy::too_many_arguments)]
pub(super) fn validate_msg<'a>(
    expected_sender_comp_id: &str,
    expected_target_comp_id: &str,
    msg_type: char,
    msg_seq_num: u32,
    target_comp_id: Option<&'a [u8]>,
    sender_comp_id: Option<&'a [u8]>,
    sending_time: Option<DateTime<Utc>>,
    poss_dup_flag: Option<char>,
    orig_sending_time: Option<DateTime<Utc>>,
    begin_seq_no: Option<u32>,
    end_seq_no: Option<u32>,
) -> Result<(), SessionError> {
    if <char as TryInto<MsgType>>::try_into(msg_type).is_err() {
        return Err(SessionError::new_message_rejected(
            Some(SessionRejectReason::INVALID_MSGTYPE),
            msg_seq_num,
            Some(Tags::MsgType.into()),
            Some(msg_type),
        ));
    }
    if Some(expected_target_comp_id.as_bytes()) != target_comp_id {
        return Err(SessionError::new_message_rejected(
            Some(SessionRejectReason::COMPID_PROBLEM),
            msg_seq_num,
            Some(Tags::TargetCompID.into()),
            Some(msg_type),
        ));
    }

    if Some(expected_sender_comp_id.as_bytes()) != sender_comp_id {
        return Err(SessionError::new_message_rejected(
            Some(SessionRejectReason::COMPID_PROBLEM),
            msg_seq_num,
            Some(Tags::SenderCompID.into()),
            Some(msg_type),
        ));
    }

    if sending_time.is_none() {
        return Err(SessionError::new_message_rejected(
            Some(SessionRejectReason::REQUIRED_TAG_MISSING),
            msg_seq_num,
            Some(Tags::SendingTime.into()),
            Some(msg_type),
        ));
    }

    if !valid_sending_time(sending_time.unwrap(), Duration::seconds(10)) {
        return Err(SessionError::new_message_rejected(
            Some(SessionRejectReason::SENDINGTIME_ACCURACY_PROBLEM),
            msg_seq_num,
            Some(Tags::SendingTime.into()),
            Some(msg_type),
        ));
    }

    match poss_dup_flag {
        Some('Y') => {
            validate_duplicate(
                msg_seq_num,
                msg_type,
                sending_time.unwrap(),
                orig_sending_time,
            )?;
        }
        Some('N') | None => {}
        Some(_) => {
            return Err(SessionError::new_message_rejected(
                Some(SessionRejectReason::VALUE_IS_INCORRECT),
                msg_seq_num,
                Some(Tags::PossDupFlag.into()),
                Some(msg_type),
            ));
        }
    }

    if msg_type == MsgType::RESEND_REQUEST.into() && !valid_resend_request(begin_seq_no, end_seq_no)
    {
        return Err(SessionError::new_message_rejected(
            Some(SessionRejectReason::REQUIRED_TAG_MISSING),
            msg_seq_num,
            None,
            Some(msg_type),
        ));
    }

    Ok(())
}

fn valid_resend_request(begin_seq_no: Option<u32>, end_seq_no: Option<u32>) -> bool {
    begin_seq_no.is_some() && end_seq_no.is_some()
}

pub(super) fn validate_checksum(msg_buf: &MsgBuf) -> Result<(), SessionError> {
    if !checksum_is_valid(&msg_buf.0) {
        return Err(SessionError::new_garbled_message(
            String::from("Checksum invalid"),
            GarbledMessageType::ChecksumIssue,
        ));
    }
    Ok(())
}

pub(super) fn validate_msg_length(msg_buf: &[u8], msg_length: usize) -> Result<(), SessionError> {
    if &msg_buf[msg_length - 7..msg_length - 4] != b"10=".as_slice() {
        return Err(SessionError::GarbledMessage {
            text: String::from("BodyLength(9) was incorrect"),
            garbled_msg_type: GarbledMessageType::BodyLengthIssue,
        });
    }
    Ok(())
}

fn valid_sending_time(sending_time: DateTime<Utc>, sending_time_threshold: Duration) -> bool {
    Utc::now() - sending_time < sending_time_threshold
        && sending_time - Utc::now() < sending_time_threshold
}

fn validate_duplicate(
    msg_seq_num: u32,
    msg_type: char,
    sending_time: DateTime<Utc>,
    orig_sending_time: Option<DateTime<Utc>>,
) -> Result<(), SessionError> {
    if orig_sending_time.is_none() {
        return Err(SessionError::new_message_rejected(
            Some(SessionRejectReason::REQUIRED_TAG_MISSING),
            msg_seq_num,
            Some(Tags::OrigSendingTime.into()),
            Some(msg_type),
        ));
    }

    if orig_sending_time.unwrap() > sending_time {
        return Err(SessionError::new_message_rejected(
            Some(SessionRejectReason::SENDINGTIME_ACCURACY_PROBLEM),
            msg_seq_num,
            None,
            Some(msg_type),
        ));
    }

    Ok(())
}
