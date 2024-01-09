mod fields;
pub use fields::*;
impl MsgType {
    pub fn is_session(&self) -> bool {
        matches!(
            self,
            MsgType::HEARTBEAT
                | MsgType::TEST_REQUEST
                | MsgType::RESEND_REQUEST
                | MsgType::REJECT
                | MsgType::SEQUENCE_RESET
                | MsgType::LOGOUT
                | MsgType::LOGON
        )
    }
    pub fn is_application(&self) -> bool {
        !self.is_session()
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for &SessionRejectReason {
    fn into(self) -> u32 {
        match *self {
            SessionRejectReason::INVALID_TAG_NUMBER => 0,
            SessionRejectReason::REQUIRED_TAG_MISSING => 1,
            SessionRejectReason::SENDINGTIME_ACCURACY_PROBLEM => 10,
            SessionRejectReason::INVALID_MSGTYPE => 11,
            SessionRejectReason::TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE => 2,
            SessionRejectReason::UNDEFINED_TAG => 3,
            SessionRejectReason::TAG_SPECIFIED_WITHOUT_A_VALUE => 4,
            SessionRejectReason::VALUE_IS_INCORRECT => 5,
            SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE => 6,
            SessionRejectReason::DECRYPTION_PROBLEM => 7,
            SessionRejectReason::SIGNATURE_PROBLEM => 8,
            SessionRejectReason::COMPID_PROBLEM => 9,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for &SessionRejectReason {
    fn into(self) -> String {
        match *self {
            SessionRejectReason::INVALID_TAG_NUMBER => String::from("Invalid tag number"),
            SessionRejectReason::REQUIRED_TAG_MISSING => String::from("Required tag missing"),
            SessionRejectReason::SENDINGTIME_ACCURACY_PROBLEM => {
                String::from("SendingTime accuracy problem")
            }
            SessionRejectReason::INVALID_MSGTYPE => String::from("Invalid MsgType"),
            SessionRejectReason::TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE => {
                String::from("Tag not defined for this message type")
            }
            SessionRejectReason::UNDEFINED_TAG => String::from("Undefined tag"),
            SessionRejectReason::TAG_SPECIFIED_WITHOUT_A_VALUE => {
                String::from("Tag specified without a value")
            }
            SessionRejectReason::VALUE_IS_INCORRECT => String::from("Value is incorrect"),
            SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE => {
                String::from("Incorrect data format for value")
            }
            SessionRejectReason::DECRYPTION_PROBLEM => String::from("Decryption problem"),
            SessionRejectReason::SIGNATURE_PROBLEM => String::from("Signature problem"),
            SessionRejectReason::COMPID_PROBLEM => String::from("CompID problem"),
        }
    }
}
