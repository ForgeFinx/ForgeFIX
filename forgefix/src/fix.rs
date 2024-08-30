//! Modules implementing the FIX spec for [encoding] and [decoding] messages
//!
//! [encoding]: crate::fix::encode
//! [decoding]: crate::fix::decode

use chrono::naive::NaiveDateTime;
use chrono::{DateTime, Utc};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};

use anyhow::{bail, Result};
use thiserror::Error;

use crate::fix::decode::{parse_field, parse_sending_time};
use crate::fix::encode::{AdditionalHeaders, MessageBuilder, SerializedInt};
use crate::fix::generated::{
    is_session_message, GapFillFlag, PossDupFlag, SessionRejectReason, Tags,
};
use crate::fix::log::{FileLogger, Logger};
use crate::fix::resend::Transformer;
use crate::fix::session::{Event, MyStateMachine};
use crate::fix::stopwatch::FixTimeouts;
use crate::fix::store::Store;
use crate::fix::validate::validate_msg;
use crate::{FixEngineType, Request, SessionSettings};

use generated::MsgType;
use generated::MsgType::*;
use mem::MsgBuf;

use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub mod decode;
pub mod encode;
pub mod generated;
pub mod mem;

mod checksum;
mod log;
mod resend;
mod session;
mod stopwatch;
mod store;
mod stream;
mod validate;

#[derive(Debug, Error)]
enum SessionError {
    #[error("An I/O error occured: {0}")]
    IoError(#[from] io::Error),
    #[error("A garbled message was received")]
    GarbledMessage {
        text: String,
        garbled_msg_type: GarbledMessageType,
    },
    #[error("A message was received without a sequence number")]
    MissingMsgSeqNum { text: String },
    #[error("A message was rejected because: {text}")]
    MessageRejected {
        text: String,
        reject_reason: Option<SessionRejectReason>,
        msg_seq_num: u32,
        ref_tag_id: Option<u32>,
        ref_msg_type: Option<char>,
    },
    #[error("Tried to resend a malformed message")]
    ResendError,
    #[error("TCP peer closed their half of the connection")]
    TcpDisconnection,
}

#[derive(Debug)]
enum GarbledMessageType {
    BeginStringIssue,
    BodyLengthIssue,
    MsgTypeIssue,
    ChecksumIssue,
    Other,
}

impl SessionError {
    fn new_message_rejected(
        reason: Option<SessionRejectReason>,
        seq_num: u32,
        tag_id: Option<u32>,
        msg_type: Option<char>,
    ) -> SessionError {
        SessionError::MessageRejected {
            text: reason.as_ref().map_or(String::from(""), |r| r.into()),
            reject_reason: reason,
            msg_seq_num: seq_num,
            ref_tag_id: tag_id,
            ref_msg_type: msg_type,
        }
    }

    fn new_garbled_message(text: String, t: GarbledMessageType) -> SessionError {
        SessionError::GarbledMessage {
            text,
            garbled_msg_type: t,
        }
    }
}

#[derive(Default)]
struct SessionParserCallback<'a> {
    msg_type: char,
    msg_seq_num: u32,
    sender_comp_id: Option<&'a [u8]>,
    target_comp_id: Option<&'a [u8]>,
    poss_dup_flag: Option<char>,
    gap_fill: Option<char>,
    new_seq_no: Option<u32>,
    test_req_id: Option<&'a [u8]>,
    begin_seq_no: Option<u32>,
    end_seq_no: Option<u32>,
    heart_bt_int: Option<u32>,
    sending_time: Option<i64>,
    orig_sending_time: Option<i64>,
    encrypt_method: Option<u32>,
    reset_seq_num_flag: Option<char>,
}

impl<'a> crate::fix::decode::ParserCallback<'a> for SessionParserCallback<'a> {
    type Err = SessionError;
    fn header(&mut self, key: u32, value: &'a [u8]) -> Result<bool, Self::Err> {
        match key.try_into() {
            Ok(Tags::MsgType) => {
                if value.len() == 1 {
                    self.msg_type = value[0] as char;
                } else {
                    return Err(self.create_message_reject(
                        SessionRejectReason::INVALID_MSGTYPE,
                        Tags::MsgType,
                    ));
                }
            }
            Ok(Tags::MsgSeqNum) => {
                self.msg_seq_num = parse_field::<u32>(value).or_else(|_| {
                    Err(SessionError::MissingMsgSeqNum {
                        text: String::from("Missing MsgSeqNum"),
                    })
                })?;
            }
            Ok(Tags::TargetCompID) => {
                self.target_comp_id = Some(value);
            }
            Ok(Tags::SenderCompID) => {
                self.sender_comp_id = Some(value);
            }
            Ok(Tags::PossDupFlag) => {
                if value.len() == 1 {
                    self.poss_dup_flag = Some(value[0] as char);
                } else {
                    return Err(self.create_message_reject(
                        SessionRejectReason::VALUE_IS_INCORRECT,
                        Tags::PossDupFlag,
                    ));
                }
            }
            Ok(Tags::SendingTime) => match speedate::DateTime::parse_bytes(value) {
                Ok(sending_time) => {
                    self.sending_time = Some(sending_time.timestamp());
                }
                Err(_) => {
                    return Err(self.create_message_reject(
                        SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE,
                        Tags::SendingTime,
                    ));
                }
            },
            Ok(Tags::OrigSendingTime) => match speedate::DateTime::parse_bytes(value) {
                Ok(sending_time) => {
                    self.orig_sending_time = Some(sending_time.timestamp());
                }
                Err(_) => {
                    return Err(self.create_message_reject(
                        SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE,
                        Tags::SendingTime,
                    ));
                }
            },
            _ => (),
        }
        Ok(true)
    }

    fn body(&mut self, key: u32, value: &'a [u8]) -> Result<bool, Self::Err> {
        if !is_session_message(self.msg_type) {
            return Ok(false);
        }
        match key.try_into() {
            Ok(Tags::GapFillFlag) => {
                if value.len() == 1 {
                    self.gap_fill = Some(value[0] as char);
                } else {
                    return Err(self.create_message_reject(
                        SessionRejectReason::VALUE_IS_INCORRECT,
                        Tags::GapFillFlag,
                    ));
                }
            }
            Ok(Tags::NewSeqNo) => {
                self.new_seq_no =
                    Some(parse_field::<u32>(value).or(Err(self.create_message_reject(
                        SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE,
                        Tags::NewSeqNo,
                    )))?);
            }
            Ok(Tags::TestReqID) => {
                self.test_req_id = Some(value);
            }
            Ok(Tags::BeginSeqNo) => {
                self.begin_seq_no =
                    Some(parse_field::<u32>(value).or(Err(self.create_message_reject(
                        SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE,
                        Tags::BeginSeqNo,
                    )))?);
            }
            Ok(Tags::EndSeqNo) => {
                self.end_seq_no =
                    Some(parse_field::<u32>(value).or(Err(self.create_message_reject(
                        SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE,
                        Tags::EndSeqNo,
                    )))?);
            }
            Ok(Tags::HeartBtInt) => {
                self.heart_bt_int =
                    Some(parse_field::<u32>(value).or(Err(self.create_message_reject(
                        SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE,
                        Tags::HeartBtInt,
                    )))?)
            }
            Ok(Tags::EncryptMethod) => {
                self.encrypt_method =
                    Some(parse_field::<u32>(value).or(Err(self.create_message_reject(
                        SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE,
                        Tags::EndSeqNo,
                    )))?);
            }
            Ok(Tags::ResetSeqNumFlag) => {
                if value.len() == 1 {
                    self.reset_seq_num_flag = Some(value[0] as char);
                } else {
                    return Err(self.create_message_reject(
                        SessionRejectReason::VALUE_IS_INCORRECT,
                        Tags::ResetSeqNumFlag,
                    ));
                }
            }
            _ => (),
        }
        Ok(true)
    }

    fn trailer(&mut self, _key: u32, _value: &'a [u8]) -> Result<bool, Self::Err> {
        Ok(false)
    }

    fn parse_error(&mut self, err: decode::MessageParseError) -> Result<(), Self::Err> {
        match err {
            decode::MessageParseError::BadLengthField(tag, _) => {
                Err(SessionError::new_message_rejected(
                    Some(SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE),
                    self.msg_seq_num,
                    Some(tag),
                    None,
                ))
            }
            decode::MessageParseError::UnexpectedByte(..) => Err(SessionError::GarbledMessage {
                text: "invalid character in message".to_string(),
                garbled_msg_type: GarbledMessageType::Other,
            }),
        }
    }
}

impl SessionParserCallback<'_> {
    fn create_message_reject(&self, reason: SessionRejectReason, reg_tag: Tags) -> SessionError {
        SessionError::new_message_rejected(
            Some(reason),
            self.msg_seq_num,
            Some(reg_tag.into()),
            Some(self.msg_type),
        )
    }
}

pub(super) async fn spin_session(
    mut stream: TcpStream,
    mut request_receiver: mpsc::UnboundedReceiver<Request>,
    mut message_received_event_sender: rtrb::Producer<Arc<MsgBuf>>,
    settings: SessionSettings,
) -> Result<()> {
    // SETUP

    let additional_headers = AdditionalHeaders::build(&settings);
    let store = Store::build(&settings).await?;
    let mut logger = FileLogger::build(&settings).await?;
    let sequences = store.get_sequences(settings.epoch.clone()).await?;
    let mut state_machine = MyStateMachine::new(&settings, sequences);

    let logon_resp_sender = receive_logon_request(&mut request_receiver).await;

    let start_new_session = is_new_session(&store, &settings).await?;
    match settings.engine_type {
        FixEngineType::Server => {
            state_machine.set_logon_resp_sender(logon_resp_sender);
            state_machine.handle(&crate::fix::session::Event::Accept);
        }
        FixEngineType::Client => {
            state_machine.set_logon_resp_sender(logon_resp_sender);
            state_machine.handle(&crate::fix::session::Event::Connect(start_new_session));
        }
    }

    let epoch = settings.epoch.clone();
    let heartbt_dur = &settings.heartbeat_timeout;
    let tr_dur = test_request_duration(heartbt_dur);
    let logout_dur = logout_duration(heartbt_dur);
    let mut fix_timeouts = FixTimeouts::new(*heartbt_dur, tr_dur, logout_dur);

    let mut header_buf: stream::HeaderBuf<{ stream::PEEK_LEN }> = stream::HeaderBuf::new();

    let mut recv_from_channel_times: Vec<Duration> = Vec::with_capacity(100);
    let mut to_tcp_stream_times: Vec<Duration> = Vec::with_capacity(100);
    let mut recv_times: Vec<Duration> = Vec::with_capacity(100);

    let mut created_instants: Vec<Instant> = Vec::with_capacity(200);

    // LOOP

    loop {
        send_outgoing_messages(
            &mut state_machine,
            &mut stream,
            &additional_headers,
            &store,
            Arc::clone(&epoch),
            &mut logger,
            &mut fix_timeouts,
            &mut to_tcp_stream_times,
            &mut created_instants,
        )
        .await?;

        if session::should_disconnect(&state_machine) {
            let resp = disconnect(
                request_receiver,
                store,
                epoch,
                &state_machine,
                stream,
                logger,
            )
            .await;
            let logout_success = !session::in_error_state(&state_machine);
            state_machine.send_logout_response(logout_success && resp.is_ok());
            resp?;
            break;
        }

        let next_timeout = fix_timeouts.next_expiring_timeout();
        let (timeout_fut, timeout_event) = next_timeout.timeout();

        tokio::select! {
            biased;

            Some(req) = request_receiver.recv() => {
                handle_req(req, &mut state_machine, &mut recv_from_channel_times)
            }
            maybe_err = stream::read_header(&mut stream, &mut header_buf) => {
                let maybe_message = match maybe_err {
                    Ok(()) => stream::read_message(&mut stream, &mut header_buf, &mut logger).await,
                    Err(SessionError::IoError(e)) => bail!("{e:?}"),
                    Err(e) => Err(e),
                };

                if recv_times.len() != 0 || created_instants.len() != 0 {
                    recv_times.push(created_instants[recv_times.len()].elapsed());
                }

                if let Err(SessionError::IoError(e)) = maybe_message {
                    bail!("{e:?}");
                }

                handle_msg(
                    maybe_message,
                    &mut state_machine,
                    &mut fix_timeouts,
                    &store,
                    &settings,
                    &mut stream,
                    &mut logger,
                    &additional_headers,
                    &mut message_received_event_sender,
                ).await?;
            }
            _ = timeout_fut => {
                state_machine.handle(timeout_event);
                next_timeout.reset_timeout();
            }
        };
    }

    recv_from_channel_times.sort();
    to_tcp_stream_times.sort();
    recv_times.sort();

    println!(
        "time from building message to received by engine: {:?}",
        (
            recv_from_channel_times[0],
            recv_from_channel_times[24],
            recv_from_channel_times[49],
            recv_from_channel_times[74],
            recv_from_channel_times[99],
        )
    );
    println!(
        "time from building message to writing to TCP: {:?}",
        (
            to_tcp_stream_times[0],
            to_tcp_stream_times[24],
            to_tcp_stream_times[49],
            to_tcp_stream_times[74],
            to_tcp_stream_times[99]
        )
    );

    println!(
        "time from building message to receiving something on TCP: {:?}",
        (
            recv_times[0],
            recv_times[24],
            recv_times[49],
            recv_times[74],
            recv_times[99]
        )
    );

    Ok(())
}

fn test_request_duration(timeout_dur: &Duration) -> Duration {
    (*timeout_dur * 17) / 10
}

fn logout_duration(timeout_dur: &Duration) -> Duration {
    *timeout_dur * 2
}

fn handle_req(
    req: Request,
    state_machine: &mut MyStateMachine,
    recv_from_channel_times: &mut Vec<Duration>,
) {
    match req {
        Request::SendMessage {
            resp_sender,
            builder,
        } => {
            recv_from_channel_times.push(builder.created.elapsed());
            state_machine.outbox_push_with_sender(builder, resp_sender);
        }
        Request::Logout { resp_sender } => {
            let begin_string = Arc::clone(&state_machine.begin_string);
            state_machine.outbox_push_with_sender(
                crate::fix::session::build_logout_message(&begin_string),
                resp_sender,
            );
        }
        Request::Logon { resp_sender } => {
            let _ = resp_sender.send(true);
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn handle_msg(
    maybe_msg: Result<MsgBuf, SessionError>,
    state_machine: &mut MyStateMachine,
    fix_timeouts: &mut FixTimeouts,
    store: &Store,
    settings: &SessionSettings,
    stream: &mut TcpStream,
    logger: &mut impl Logger,
    additional_headers: &AdditionalHeaders,
    message_received_event_sender: &mut rtrb::Producer<Arc<MsgBuf>>,
) -> Result<()> {
    fix_timeouts.reset_test_request();

    let msg = match maybe_msg {
        Ok(b) => Arc::new(b),
        Err(error) => {
            state_machine.handle(&Event::SessionErrorReceived { error });
            return Ok(());
        }
    };

    // PARSE

    let mut cb: SessionParserCallback = Default::default();

    if let Err(error) = crate::fix::decode::parse(&msg.as_ref()[..], &mut cb) {
        state_machine.handle(&Event::SessionErrorReceived { error });
        return Ok(());
    };

    // VALIDATE

    if let Err(error) = validate_msg(
        settings.expected_sender_comp_id(),
        settings.expected_target_comp_id(),
        cb.msg_type,
        cb.msg_seq_num,
        cb.target_comp_id,
        cb.sender_comp_id,
        cb.sending_time,
        cb.poss_dup_flag,
        cb.orig_sending_time,
        cb.begin_seq_no,
        cb.end_seq_no,
    ) {
        state_machine.handle(&Event::SessionErrorReceived { error });
        return Ok(());
    }

    if let Err(error) = validate::validate_checksum(&msg) {
        state_machine.handle(&Event::SessionErrorReceived { error });
        return Ok(());
    }

    // HANDLE

    let msg_seq_num = cb.msg_seq_num;
    let maybe_msg_type = cb.msg_type.try_into();

    match maybe_msg_type {
        Ok(LOGON) => {
            let mut heartbt_secs = settings.heartbeat_timeout.as_secs() as u32;
            if let Some(i) = cb.heart_bt_int {
                heartbt_secs = i;
                let heartbt_dur = tokio::time::Duration::from_secs(i as u64);
                fix_timeouts.set_durations(
                    heartbt_dur,
                    test_request_duration(&heartbt_dur),
                    logout_duration(&heartbt_dur),
                );
            }
            state_machine.handle(&Event::LogonReceived(
                msg_seq_num,
                heartbt_secs,
                cb.encrypt_method,
                cb.reset_seq_num_flag.map(|f| f == 'Y').unwrap_or(false),
                to_poss_dup_flag(cb.poss_dup_flag),
            ));
        }
        Ok(LOGOUT) => {
            state_machine.handle(&Event::LogoutReceived(
                msg_seq_num,
                to_poss_dup_flag(cb.poss_dup_flag),
            ));
        }
        Ok(HEARTBEAT) => {
            state_machine.handle(&Event::HeartbeatReceived(
                msg_seq_num,
                to_poss_dup_flag(cb.poss_dup_flag),
            ));
        }
        Ok(SEQUENCE_RESET) => {
            if let Some(nsn) = cb.new_seq_no {
                let maybe_gap_fill = cb
                    .gap_fill
                    .map(GapFillFlag::try_from)
                    .transpose()
                    .map_err(anyhow::Error::msg)?;
                state_machine.handle(&Event::SequenceResetReceived {
                    msg_seq_num,
                    gap_fill: maybe_gap_fill,
                    new_seq_no: nsn,
                    poss_dup: to_poss_dup_flag(cb.poss_dup_flag),
                })
            }
        }
        Ok(REJECT) => state_machine.handle(&Event::RejectReceived(
            msg_seq_num,
            to_poss_dup_flag(cb.poss_dup_flag),
        )),
        Ok(TEST_REQUEST) => {
            if let Some(test_req_id) = cb.test_req_id {
                state_machine.handle(&Event::TestRequestReceived {
                    msg_seq_num,
                    test_req_id: test_req_id.to_owned(),
                    poss_dup: to_poss_dup_flag(cb.poss_dup_flag),
                })
            }
        }
        Ok(RESEND_REQUEST) => {
            let e = match cb.end_seq_no {
                Some(n) if n > 0 => n,
                _ => state_machine.sequences.peek_outgoing() - 1,
            };
            let b = cb.begin_seq_no.unwrap_or(e);

            if session::should_resend(state_machine) {
                let prev_messages = store
                    .get_prev_messages(
                        Arc::clone(&settings.epoch),
                        b,
                        e,
                        state_machine.sequences.peek_outgoing() - 1,
                    )
                    .await?;
                resend_messages(prev_messages, stream, additional_headers, logger).await?;
            }
            state_machine.handle(&Event::ResendRequestReceived(
                cb.msg_seq_num,
                b,
                e,
                to_poss_dup_flag(cb.poss_dup_flag),
            ));
        }
        Ok(ref msg_type) if msg_type.is_application() => {
            if session::should_pass_app_message(state_machine, msg_seq_num) {
                let _ = message_received_event_sender.push(Arc::clone(&msg));
            }
            state_machine.handle(&Event::ApplicationMessageReceived(
                msg_seq_num,
                to_poss_dup_flag(cb.poss_dup_flag),
            ));
        }
        _ => {
            let error = SessionError::new_message_rejected(
                Some(SessionRejectReason::INVALID_MSGTYPE),
                cb.msg_seq_num,
                None,
                None,
            );
            state_machine.handle(&Event::SessionErrorReceived { error });
        }
    }
    Ok(())
}

async fn disconnect(
    mut request_receiver: mpsc::UnboundedReceiver<Request>,
    store: Store,
    epoch: Arc<String>,
    state_machine: &MyStateMachine,
    stream: TcpStream,
    mut logger: FileLogger,
) -> Result<()> {
    request_receiver.close();
    store
        .set_sequences(
            epoch,
            state_machine.sequences.peek_outgoing(),
            state_machine.sequences.peek_incoming(),
        )
        .await?;
    store.disconnect().await?;
    logger.disconnect().await?;
    stream::disconnect(stream).await;
    Ok(())
}

async fn receive_logon_request(
    request_receiver: &mut mpsc::UnboundedReceiver<Request>,
) -> Option<oneshot::Sender<bool>> {
    loop {
        match request_receiver.recv().await {
            Some(Request::Logon { resp_sender }) => {
                return Some(resp_sender);
            }
            Some(Request::SendMessage { resp_sender, .. }) => {
                let _ = resp_sender.send(false);
            }
            Some(Request::Logout { resp_sender, .. }) => {
                let _ = resp_sender.send(true);
            }
            None => {
                return None;
            }
        }
    }
}

async fn send_outgoing_messages(
    state_machine: &mut MyStateMachine,
    stream: &mut TcpStream,
    additional_headers: &AdditionalHeaders,
    store: &Store,
    epoch: Arc<String>,
    logger: &mut impl Logger,
    fix_timeouts: &mut FixTimeouts,
    to_tcp_stream_times: &mut Vec<Duration>,
    created_instants: &mut Vec<Instant>,
) -> Result<(), SessionError> {
    if !state_machine.outbox.is_empty() {
        fix_timeouts.reset_heartbeat();
    }
    while let Some((msg, maybe_resp_sender)) = state_machine.outbox_pop() {
        let is_logout = msg.msg_type() == MsgType::LOGOUT.into();

        let created = msg.created;
        let msg_seq_num = state_machine.sequences.next_outgoing();
        let msg_buf = build_message_with_headers(msg, msg_seq_num, additional_headers).await?;
        stream::send_message(&msg_buf, stream, logger).await?;
        to_tcp_stream_times.push(created.elapsed());
        created_instants.push(created);

        store
            .store_outgoing(
                epoch.clone(),
                msg_seq_num,
                Instant::now(),
                Arc::new(msg_buf),
            )
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        if is_logout {
            state_machine.outbox_clear();
            state_machine.set_logout_resp_sender(maybe_resp_sender);
            state_machine.handle(&Event::LogoutSent);
            fix_timeouts.start_logout_timeout();
            break;
        } else if let Some(resp_sender) = maybe_resp_sender {
            let _ = resp_sender.send(true);
        }
    }
    Ok(())
}

async fn resend_messages(
    mut messages: Vec<(u32, Vec<u8>)>,
    stream: &mut TcpStream,
    additional_headers: &AdditionalHeaders,
    logger: &mut impl Logger,
) -> Result<(), SessionError> {
    messages.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut session_msg_count = 0;
    for (_, (msg_seq_num, msg)) in messages.iter().enumerate() {
        let transformer = Transformer::try_from(msg.clone())?;
        let msg_type =
            MsgType::try_from(transformer.msg_type).or(Err(SessionError::ResendError))?;
        if msg_type.is_session() {
            session_msg_count += 1;
            continue;
        }
        if session_msg_count > 0 {
            let msg_buf = build_gap_fill_msg(
                msg_seq_num - session_msg_count,
                *msg_seq_num,
                additional_headers,
            )
            .await?;
            stream::send_message(&msg_buf, stream, logger).await?;
            session_msg_count = 0;
        }
        let msg_buf = transform_message(transformer).await?;
        stream::send_message(&msg_buf, stream, logger).await?;
    }
    if session_msg_count > 0 {
        let last_seq_num = messages[messages.len() - 1].0;
        let msg_buf = build_gap_fill_msg(
            last_seq_num - session_msg_count + 1,
            last_seq_num + 1,
            additional_headers,
        )
        .await?;
        stream::send_message(&msg_buf, stream, logger).await?;
    }
    Ok(())
}

async fn build_message_with_headers(
    msg: MessageBuilder,
    msg_seq_num: u32,
    additional_headers: &AdditionalHeaders,
) -> Result<MsgBuf, SessionError> {
    let mut buf = Vec::with_capacity(1024);

    msg.build_async(&mut buf, msg_seq_num, additional_headers, Utc::now())
        .await?;
    Ok(buf.into())
}

async fn build_gap_fill_msg(
    msg_seq_num: u32,
    new_seq_num: u32,
    additional_headers: &AdditionalHeaders,
) -> Result<MsgBuf, SessionError> {
    let builder = MessageBuilder::new("FIX.4.2", MsgType::SEQUENCE_RESET.into())
        .push(Tags::NewSeqNo, SerializedInt::from(new_seq_num).as_bytes())
        .push(Tags::GapFillFlag, b"Y");
    let msg = build_message_with_headers(builder, msg_seq_num, additional_headers).await?;
    let transformer = Transformer::try_from(msg.0)?;
    transform_message(transformer).await
}

async fn transform_message(transformer: Transformer) -> Result<MsgBuf, SessionError> {
    let mut buf = Vec::new();
    let mut cur = tokio::io::BufWriter::new(&mut buf);
    transformer
        .build_async(&mut cur)
        .await
        .or(Err(SessionError::ResendError))?;
    cur.flush().await?;
    Ok(buf.into())
}

fn to_poss_dup_flag(maybe_flag: Option<char>) -> Option<PossDupFlag> {
    maybe_flag.map(|f| PossDupFlag::try_from(f).unwrap_or(PossDupFlag::NO))
}

async fn is_new_session(store: &Store, settings: &SessionSettings) -> Result<bool> {
    if matches!(settings.engine_type, FixEngineType::Server) {
        return Ok(false);
    }
    let last_send_time = store.last_send_time(settings.epoch.clone()).await?;
    let start_time = NaiveDateTime::new(Utc::now().date_naive(), settings.start_time).and_utc();
    Ok(last_send_time < Some(start_time))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fix::decode::ParsedPeek;
    use tokio::io::{AsyncReadExt, BufReader};
    #[tokio::test]
    async fn my_test() {
        let message_bytes = vec![
        &b"8=FIX.4.2\x019=77\x0135=A\x0134=1\x0149=GSLLDMAUAT\x0152=20220920-17:01:58.896\x0156=AMLRLLDMAUAT\x0198=0\x01108=30\x0110=126\x01"[..],
        &b"8=FIX.4.2\x019=77\x0135=A\x0134=2\x0149=AMLRLLDMAUAT\x0152=20220920-17:11:00.860\x0156=GSLLDMAUAT\x0198=0\x01108=30\x0110=106\x01"[..],
        &b"8=FIX.4.2\x019=0287\x0135=8\x0134=15\x0149=GSLLDMAUAT\x0152=20220920-17:24:23.974\x0156=AMLRLLDMAUAT\x011=\x016=0.000000\x0111=the-01GDDYVW95KB9XTD2GA7EC4CY2\x0114=0\x0117=BzdLZDozKgI=\x0120=0\x0122=J\x0137=BzdLZDkzKgI=\x0138=2\x0139=8\x0140=2\x0144=0.890000\x0148=RBLX  220930P00034000\x0154=1\x0158=GS:InvalidAccount\x0159=3\x0160=20220920-17:24:23.974\x01103=0\x01150=8\x01151=0\x0110=008\x01"[..],
        ];
        for data in message_bytes.iter() {
            let mut r = BufReader::new(*data);
            let ParsedPeek {
                len_start,
                body_length,
                len_end,
                ..
            } = crate::fix::decode::parse_peeked_prefix(&data[..32]).unwrap();
            assert_eq!(len_start, 12);
            // assert_eq!(len_end, 15);
            let mut msg_buf = vec![0; 32 + (body_length - (32 - (len_end + 1)) + 7)];
            r.read_exact(&mut msg_buf[..])
                .await
                .expect("expected no error");
            assert_eq!(&data[..], msg_buf);
        }
    }

    #[test]
    fn test_validate_msg_length() {
        let correct = &b"8=FIX.4.2\x019=21\x0134=0\x0149=send\x0156=rec\x0110=000\x01"[..];
        let short = &b"8=FIX.4.2\x019=14\x0134=0\x0149=send\x0156=rec\x01"[..];
        let long = &b"8=FIX.4.2\x019=23\x0134=0\x0149=send\x0156=rec\x0110=000\x018="[..];

        assert_eq!(
            validate::validate_msg_length(correct, correct.len()).is_ok(),
            true
        );
        assert_eq!(
            validate::validate_msg_length(short, short.len()).is_err(),
            true
        );
        assert_eq!(
            validate::validate_msg_length(long, long.len()).is_err(),
            true
        );
    }

    #[test]
    fn test_parser_callback() {
        let mut cb: SessionParserCallback = Default::default();
        assert_eq!(
            crate::fix::decode::parse(
                &b"8=FIX.4.2\x019=21\x0195=10\x0196=123\x01456789\x0110=000\x01"[..],
                &mut cb
            )
            .is_err(),
            false
        );
        let mut cb: SessionParserCallback = Default::default();
        assert_eq!(
            crate::fix::decode::parse(
                &b"8=FIX.4.2\x019=21\x0195=1a\x0196=123\x01456789\x0110=000\x01"[..],
                &mut cb
            )
            .is_err(),
            true
        );
        let mut cb: SessionParserCallback = Default::default();
        assert_eq!(
            crate::fix::decode::parse(
                &b"8=FIX.4.2\x019=21\x0195=10\x0196=123\x0145678910\x0110=000\x01"[..],
                &mut cb
            )
            .is_err(),
            false
        );
        let mut cb: SessionParserCallback = Default::default();
        assert_eq!(
            crate::fix::decode::parse(
                &b"8=FIX.4.2\x019=21\x0195=10\x0134=1\x0196=123\x01456789\x0110=000\x01"[..],
                &mut cb
            )
            .is_err(),
            false
        );
    }
}
