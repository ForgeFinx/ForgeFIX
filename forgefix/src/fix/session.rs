use crate::{SessionSettings}; 
use crate::fix::encode::{MessageBuilder, SerializedInt};
use crate::fix::generated::{GapFillFlag, MsgType, PossDupFlag, SessionRejectReason, Tags};
use crate::fix::{GarbledMessageType, SessionError};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::oneshot;

enum Response {
    Handled,
    Transition(State), 
}

#[derive(Debug, Clone)]
pub enum State {
    Start,
    Connected,
    LogonSent,
    LoggedIn,
    ExpectingResends {
        return_state: Arc<State>, 
    },
    ExpectingTestResponse,
    LogoutSent,
    End,
    Error,
}

pub struct MyStateMachine {
    pub outbox: VecDeque<(MessageBuilder, Option<oneshot::Sender<bool>>)>, // https://github.com/mdeloof/statig/issues/7
    pub sequences: Sequences,
    pub begin_string: Arc<String>, 
    rereceive_range: Option<(u32, u32)>,
    logout_resp_sender: Option<oneshot::Sender<bool>>,
    logon_resp_sender: Option<oneshot::Sender<bool>>,
    state: State,
}

#[derive(Debug)]
pub enum Event {
    Connect(bool),
    Accept,
    LogonReceived(u32, u32, Option<u32>, bool, Option<PossDupFlag>),
    LogoutSent,
    LogoutReceived(u32, Option<PossDupFlag>),
    HeartbeatReceived(u32, Option<PossDupFlag>),
    SequenceResetReceived {
        msg_seq_num: u32,
        gap_fill: Option<GapFillFlag>,
        new_seq_no: u32,
        poss_dup: Option<PossDupFlag>,
    },
    TestRequestReceived {
        msg_seq_num: u32,
        test_req_id: Vec<u8>,
        poss_dup: Option<PossDupFlag>,
    },
    SessionErrorReceived {
        error: SessionError,
    },
    ApplicationMessageReceived(u32, Option<PossDupFlag>),
    SendHeartbeat,
    SendTestRequest(u32),
    ResendRequestReceived(u32, u32, u32, Option<PossDupFlag>),
    RejectReceived(u32, Option<PossDupFlag>),
    LogoutExpired,
}
impl Event {
    fn get_msg_seq_num(&self) -> Option<u32> {
        match self {
            Event::LogonReceived(n, ..) => Some(*n),
            Event::LogoutReceived(n, ..) => Some(*n),
            Event::HeartbeatReceived(n, ..) => Some(*n),
            Event::SequenceResetReceived {
                msg_seq_num,
                gap_fill: Some(GapFillFlag::YES),
                ..
            } => Some(*msg_seq_num),
            Event::TestRequestReceived { msg_seq_num, .. } => Some(*msg_seq_num),
            Event::ApplicationMessageReceived(n, ..) => Some(*n),
            Event::ResendRequestReceived(n, ..) => Some(*n),
            Event::RejectReceived(n, ..) => Some(*n),
            _ => None,
        }
    }

    fn is_poss_dup(&self) -> bool {
        let poss_dup_flag = match self {
            Event::LogonReceived(.., p) => p,
            Event::LogoutReceived(.., p) => p,
            Event::HeartbeatReceived(.., p) => p,
            Event::SequenceResetReceived { poss_dup, .. } => poss_dup,
            Event::TestRequestReceived { poss_dup, .. } => poss_dup,
            Event::ApplicationMessageReceived(.., p) => p,
            Event::ResendRequestReceived(.., p) => p,
            Event::RejectReceived(.., p) => p,
            _ => &None,
        };
        poss_dup_flag == &Some(PossDupFlag::YES)
    }

    fn is_logout(&self) -> bool {
        matches!(self, Event::LogoutReceived(..))
    }

    fn is_sequence_reset(&self) -> bool {
        matches!(
            self,
            Event::SequenceResetReceived {
                gap_fill: Some(GapFillFlag::NO),
                ..
            } | Event::SequenceResetReceived { gap_fill: None, .. }
        )
    }
}

impl MyStateMachine {
    pub fn new(settings: &SessionSettings, seqs: (u32, u32)) -> Self {
        MyStateMachine {
            outbox: VecDeque::new(),
            sequences: seqs.into(),
            begin_string: Arc::clone(&settings.begin_string),
            logon_resp_sender: None,
            logout_resp_sender: None,
            rereceive_range: None,
            state: State::Start,
        }
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn handle(&mut self, event: &Event) {
        if let Response::Transition(new_state) = match &self.state {
            State::Start => self.start(event),
            State::Connected => self.connected(event),
            State::LogonSent => self.logon_sent(event),
            State::LoggedIn => self.logged_in(event),
            State::ExpectingResends { return_state } => self.expecting_resends(event, return_state.clone()),
            State::ExpectingTestResponse => self.expecting_test_response(event),
            State::LogoutSent => self.logout_sent(event),
            State::End => self.end(event),
            State::Error => self.error(event), 
        } {
            self.state = new_state; 
        }
    }
    pub fn outbox_push(&mut self, builder: MessageBuilder) {
        self.outbox.push_back((builder, None));
    }
    pub fn outbox_push_with_sender(
        &mut self,
        builder: MessageBuilder,
        resp_sender: oneshot::Sender<bool>,
    ) {
        self.outbox.push_back((builder, Some(resp_sender)));
    }
    pub fn outbox_pop(&mut self) -> Option<(MessageBuilder, Option<oneshot::Sender<bool>>)> {
        self.outbox.pop_front()
    }
    pub fn outbox_clear(&mut self) {
        self.outbox.clear();
    }
    pub fn set_logon_resp_sender(&mut self, resp_sender: Option<oneshot::Sender<bool>>) {
        self.logon_resp_sender = resp_sender; 
    }
    pub fn set_logout_resp_sender(&mut self, resp_sender: Option<oneshot::Sender<bool>>) {
        self.logout_resp_sender = resp_sender; 
    }
    fn send_logon_response(&mut self, logon_status: bool) {
        if let Some(resp_sender) = self.logon_resp_sender.take() {
            let _ = resp_sender.send(logon_status);
        }
    }
    pub fn send_logout_response(&mut self, logout_status: bool) {
        if let Some(resp_sender) = self.logout_resp_sender.take() {
            let _ = resp_sender.send(logout_status);
        }
    }
    fn process_sequence(&mut self, event: &Event, return_state: State) -> Option<Response> {
        event.get_msg_seq_num().and_then(|incoming| {
            let expected = self.sequences.peek_incoming();
            if expected == incoming {
                self.sequences.incr_incoming();
                None
            } else if expected < incoming {
                self.rereceive_range = Some((expected, incoming));
                let message = MessageBuilder::new(&self.begin_string, MsgType::RESEND_REQUEST.into())
                    .push(Tags::BeginSeqNo, SerializedInt::from(expected).as_bytes())
                    .push(Tags::EndSeqNo, SerializedInt::from(0u32).as_bytes());
                self.outbox_push(message);
                Some(Response::Transition(State::ExpectingResends { return_state: Arc::new(return_state) }))
            } else if expected > incoming && !event.is_poss_dup() {
                let message = build_logout_message_with_text(
                    &self.begin_string,
                    format!(
                        "MsgSeqNum too low, expecting {} but received {}",
                        expected, incoming
                    )
                    .as_bytes(),
                );
                self.outbox_push(message);
                Some(Response::Transition(State::Error))
            } else {
                Some(Response::Handled)
            }
        })
    }
    fn reset_sequences(&mut self) {
        self.sequences = (1, 1).into()
    }
    fn reset_expected_incoming(&mut self, msg_seq_num: u32, new_seq_no: u32) {
        match self.sequences.reset_incoming(new_seq_no) {
            Ok(_) => {}
            Err(msg) => {
                let builder = build_message_reject(
                    &msg.to_string(),
                    &Some(SessionRejectReason::VALUE_IS_INCORRECT),
                    &msg_seq_num,
                    &None,
                    &Some(char::from(MsgType::SEQUENCE_RESET)),
                );
                self.outbox_push(builder);
            }
        }
    }
    // This function acts as a superstate: multiple states can defer exectution to their
    // superstate.
    fn post_logon(&mut self, event: &Event) -> Response {
        match event {
            Event::SessionErrorReceived { 
                error:
                    SessionError::GarbledMessage {
                        text,
                        garbled_msg_type: GarbledMessageType::BeginStringIssue,
                    },
            } => { 
                self.outbox_push(build_logout_message_with_text(&self.begin_string, text.as_bytes()));
                Response::Transition(State::Error)
            }
            Event::SessionErrorReceived {
                error: SessionError::TcpDisconnection,
            } => Response::Transition(State::Error),
            Event::LogoutReceived(..) => {
                let builder = build_logout_message(&self.begin_string); 
                self.outbox_push(builder); 
                Response::Transition(State::End)
            }
            Event::SendTestRequest(_) => {
                let builder = MessageBuilder::new(&self.begin_string, MsgType::TEST_REQUEST.into())
                    .push(Tags::TestReqID, b"TEST"); 
                self.outbox_push(builder);
                Response::Transition(State::ExpectingTestResponse)
            }
            Event::SendHeartbeat => {
                let builder = MessageBuilder::new(&self.begin_string, MsgType::HEARTBEAT.into());
                self.outbox_push(builder); 
                Response::Handled
            }
            Event::LogoutSent => Response::Transition(State::LogoutSent),
            Event::LogoutExpired => Response::Transition(State::Error),
            _ => Response::Handled,
        }
    }
    fn expecting_resends(&mut self, event: &Event, return_state: Arc<State>) -> Response {
        if self.rereceive_range.is_none() {
            return Response::Transition(State::Error);
        }
        let (next, end) = self.rereceive_range.as_mut().unwrap();

        if !event.is_poss_dup() {
            match event {
                Event::LogoutReceived(..) => {
                    let message = build_logout_message(&self.begin_string);
                    self.outbox_push(message);
                    Response::Transition(State::End)
                }
                _ => self.post_logon(event),
            }
        } else {
            if event.get_msg_seq_num() != Some(*next) && !event.is_sequence_reset() {
                return Response::Handled;
            }
            match event {
                Event::SequenceResetReceived {
                    msg_seq_num,
                    gap_fill,
                    new_seq_no,
                    ..
                } => {
                    if *gap_fill == Some(GapFillFlag::YES) {
                        *next = *new_seq_no;
                    } else {
                        self.reset_expected_incoming(*msg_seq_num, *new_seq_no);
                        return Response::Transition((*return_state).clone())
                    }
                }
                Event::ApplicationMessageReceived(..) => {
                    *next += 1;
                }
                _ => {
                    *next += 1;
                }
            }

            if next > end || Some(*end) == event.get_msg_seq_num() {
                let _ = self.sequences.reset_incoming(*end + 1);
                self.rereceive_range = None;
                if matches!(*return_state, State::End) {
                    let message = build_logout_message(&self.begin_string); 
                    self.outbox_push(message);
                }
                return Response::Transition((*return_state).clone());
            }
            Response::Handled
        }
    }
    fn expecting_test_response(&mut self, event: &Event) -> Response {
        match event {
            Event::HeartbeatReceived(..) => {
                if let Some(resp) = self.process_sequence(event, State::LoggedIn) {
                    return resp;
                }
                Response::Transition(State::LoggedIn)
            }
            Event::SendHeartbeat | Event::SendTestRequest(_) => {
                Response::Transition(State::Error)
            }
            _ => self.logged_in(event),
        }
    }
    fn logged_in(&mut self, event: &Event) -> Response {
        let next_state = if event.is_logout() {
            State::End
        } else {
            State::LoggedIn
        };

        if let Some(resp) = self.process_sequence(event, next_state) {
            return resp;
        }
        match event {
            Event::SessionErrorReceived {
                error: SessionError::MissingMsgSeqNum { text },
            } => {
                self.outbox_push(build_logout_message_with_text(&self.begin_string, text.as_bytes()));
                Response::Transition(State::Error)
            }
            Event::SequenceResetReceived {
                msg_seq_num,
                new_seq_no,
                ..
            } => {
                self.reset_expected_incoming(*msg_seq_num, *new_seq_no);
                Response::Handled 
            }
            Event::TestRequestReceived { test_req_id, .. } => {
                let builder: MessageBuilder =
                    MessageBuilder::new(&self.begin_string, MsgType::HEARTBEAT.into())
                        .push(Tags::TestReqID, test_req_id);
                self.outbox_push(builder);
                Response::Handled 
            }
            Event::ApplicationMessageReceived(..) => Response::Handled,
            Event::SessionErrorReceived {
                error:
                    SessionError::MessageRejected {
                        text,
                        reject_reason,
                        msg_seq_num,
                        ref_tag_id,
                        ref_msg_type,
                    },
            } => {
                self.sequences.incr_incoming();
                self.outbox_push(build_message_reject(
                    text,
                    reject_reason,
                    msg_seq_num,
                    ref_tag_id,
                    ref_msg_type,
                ));

                if *reject_reason == Some(SessionRejectReason::COMPID_PROBLEM)
                    || *reject_reason == Some(SessionRejectReason::SENDINGTIME_ACCURACY_PROBLEM)
                {
                    self.outbox_push(build_logout_message_with_text(&self.begin_string, text.as_bytes()));
                    return Response::Transition(State::Error);
                }
                Response::Handled 
            }
            Event::SessionErrorReceived {
                error: SessionError::TcpDisconnection,
            } => Response::Transition(State::Error),
            _ => self.post_logon(event),
        }
    }
    fn start(&mut self, event: &Event) -> Response {
        match event {
            Event::Connect(reset_seq_num) => {
                let mut builder: MessageBuilder =
                    MessageBuilder::new(&self.begin_string, MsgType::LOGON.into())
                        .push(Tags::EncryptMethod, b"0")
                        .push(Tags::HeartBtInt, 30.to_string().as_bytes());
                if *reset_seq_num {
                    builder = builder.push(Tags::ResetSeqNumFlag, b"Y");
                    self.reset_sequences();
                }
                self.outbox_push(builder);
                Response::Transition(State::LogonSent)
            }
            Event::Accept => Response::Transition(State::Connected),
            _ => Response::Handled,
        }
    }
    #[allow(unused_variables)]
    fn error(&mut self, event: &Event) -> Response {
        Response::Handled
    }
    #[allow(unused_variables)]
    fn end(&mut self, event: &Event) -> Response {
        Response::Handled 
    }
    fn connected(&mut self, event: &Event) -> Response {
        match event {
            Event::SessionErrorReceived { error } => match error {
                SessionError::GarbledMessage { .. } => {
                    self.send_logon_response(false);
                    Response::Transition(State::Error)
                }
                SessionError::MessageRejected {
                    text,
                    reject_reason,
                    ..
                } => {
                    if *reject_reason != Some(SessionRejectReason::COMPID_PROBLEM) {
                        let builder = build_logout_message_with_text(&self.begin_string, text.as_bytes());
                        self.outbox_push(builder);
                    }
                    self.send_logon_response(false);
                    Response::Transition(State::Error)
                }
                _ => {
                    self.send_logon_response(false);
                    Response::Transition(State::Error)
                }
            },
            Event::LogonReceived(_, heart_bt_int, maybe_encrypt_method, reset_seq_num, _) => {
                if *maybe_encrypt_method != Some(0) {
                    self.send_logon_response(false);
                    return Response::Transition(State::Error);
                }
                let mut builder: MessageBuilder =
                    MessageBuilder::new(&self.begin_string, MsgType::LOGON.into())
                        .push(Tags::EncryptMethod, b"0")
                        .push(
                            Tags::HeartBtInt,
                            SerializedInt::from(*heart_bt_int).as_bytes(),
                        );
                if *reset_seq_num {
                    builder = builder.push(Tags::ResetSeqNumFlag, b"Y");
                    self.reset_sequences();
                }
                self.outbox_push(builder);
                self.send_logon_response(true);
                if let Some(resp) = self.process_sequence(event, State::LoggedIn) {
                    return resp;
                }
                Response::Transition(State::LoggedIn)
            }
            _ => {
                self.send_logon_response(false);
                Response::Transition(State::Error)
            }
        }
    }
    fn logon_sent(&mut self, event: &Event) -> Response {
        match event {
            Event::LogonReceived(_, _, encrypt_method, _, _) => {
                if *encrypt_method != Some(0) {
                    return Response::Transition(State::Error);
                }
                self.send_logon_response(true);

                if let Some(resp) = self.process_sequence(event, State::LoggedIn) {
                    return resp;
                }

                Response::Transition(State::LoggedIn)
            }
            Event::SessionErrorReceived { error } => {
                match error {
                    SessionError::MessageRejected { ref_msg_type, .. }
                        if *ref_msg_type == Some(MsgType::LOGON.into()) =>
                    {
                        let builder = build_logout_message(&self.begin_string);
                        self.outbox_push(builder);
                    }
                    _ => {}
                }

                self.send_logon_response(false);
                Response::Transition(State::Error)
            }
            Event::LogoutSent => {
                self.send_logon_response(false);
                Response::Transition(State::LogoutSent)
            }
            _ => {
                self.send_logon_response(false);
                Response::Transition(State::Error)
            }
        }
    }
    fn logout_sent(&mut self, event: &Event) -> Response {
        if let Some(resp) = self.process_sequence(event, State::LogoutSent) {
            return resp;
        }

        match event {
            Event::LogoutReceived(..) => {
                Response::Transition(State::End)
            }
            Event::LogoutExpired => {
                Response::Transition(State::Error)
            }
            Event::SessionErrorReceived { .. }
            | Event::SendTestRequest { .. }
            | Event::SendHeartbeat => {
                Response::Transition(State::Error)
            }
            _ => Response::Handled,
        }
    }
}

pub fn should_pass_app_message(
    state_machine: &MyStateMachine,
    msg_seq_num: u32,
) -> bool {
    if let Some((next, _)) = state_machine.rereceive_range {
        return msg_seq_num == next;
    }
    msg_seq_num == state_machine.sequences.peek_incoming()
        && !matches!(
            state_machine.state(),
            State::Start {}
                | State::End {}
                | State::Error {}
                | State::Connected {}
                | State::LogonSent {}
        )
}

pub fn should_resend(state_machine: &MyStateMachine) -> bool {
    matches!(
        state_machine.state(),
        State::LoggedIn | State::ExpectingResends { .. } | State::LogoutSent
    )
}
pub fn should_disconnect(
    state_machine: &MyStateMachine,
) -> bool {
    matches!(
        state_machine.state(),
        State::End | State::Error
    )
}

pub fn in_error_state(state_machine: &MyStateMachine) -> bool {
    matches!(state_machine.state(), State::Error )
}

pub fn build_logout_message_with_text(begin_string: &str, text: &[u8]) -> MessageBuilder {
    MessageBuilder::new(begin_string, MsgType::LOGOUT.into()).push(Tags::Text, text)
}

pub fn build_logout_message(begin_string: &str) -> MessageBuilder {
    MessageBuilder::new(begin_string, MsgType::LOGOUT.into())
}

fn build_message_reject(
    text: &String,
    reject_reason: &Option<SessionRejectReason>,
    msg_seq_num: &u32,
    ref_tag_id: &Option<u32>,
    ref_msg_type: &Option<char>,
) -> MessageBuilder {
    let mut builder: MessageBuilder = MessageBuilder::new("FIX.4.2", MsgType::REJECT.into())
        .push(
            Tags::RefSeqNum,
            SerializedInt::from(*msg_seq_num).as_bytes(),
        )
        .push(Tags::Text, text.as_bytes());

    if let Some(t) = ref_tag_id {
        builder = builder.push(Tags::RefTagID, SerializedInt::from(*t).as_bytes());
    }
    if !ref_msg_type.is_none() && *ref_msg_type != Some('0') {
        builder = builder.push(
            Tags::RefMsgType,
            (*ref_msg_type).unwrap().to_string().as_bytes(),
        )
    }
    if let Some(r) = reject_reason {
        builder = builder.push(
            Tags::SessionRejectReason,
            SerializedInt::from(<&SessionRejectReason as Into<u32>>::into(r)).as_bytes(),
        );
    }
    builder
}

#[derive(Default)]
pub struct Sequences(AtomicU32, AtomicU32);

impl Sequences {
    pub fn next_outgoing(&mut self) -> u32 {
        self.0.fetch_add(1, Ordering::Relaxed)
    }
    pub fn incr_incoming(&mut self) -> u32 {
        self.1.fetch_add(1, Ordering::Relaxed)
    }
    pub fn peek_incoming(&self) -> u32 {
        self.1.load(Ordering::Relaxed)
    }
    pub fn peek_outgoing(&self) -> u32 {
        self.0.load(Ordering::Relaxed)
    }
    pub fn reset_incoming(&mut self, new: u32) -> std::result::Result<(), &'static str> {
        let old = self.1.fetch_max(new, Ordering::Relaxed);
        if old > new {
            Err("Value is incorrect (out of range) for this tag")
        } else {
            self.1 = new.into();
            Ok(())
        }
    }
}

impl From<(u32, u32)> for Sequences {
    fn from((incoming, outgoing): (u32, u32)) -> Self {
        Sequences(outgoing.into(), incoming.into())
    }
}
