use clap::{Parser, ValueHint};
use forgefix::{
    fix,
    fix::generated::{MsgType, Tags},
    SessionSettings, FixApplicationAcceptor, FixApplicationHandle, FixApplicationInitiator,
    fix::decode::parse_field,
};
use std::error::Error;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use chrono::naive::NaiveTime; 

fn parse_duration(s: &str) -> Result<Duration, std::num::ParseIntError> {
    let seconds = s.parse()?;
    Ok(std::time::Duration::from_secs(seconds))
}

fn parse_time(s: &str) -> Result<NaiveTime, chrono::format::ParseError> {
    let res = NaiveTime::parse_from_str(s, "%H:%M:%S")?; 
    Ok(res)
}

const ID_SOURCE: &str = "A";

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// SenderCompId
    #[arg(short, long)]
    sender_comp_id: String,

    /// TargetCompId
    #[arg(short, long)]
    target_comp_id: String,

    /// Address to listen or connect
    #[arg(short, long)]
    addr: SocketAddr,

    /// Listen (opposite is connect)
    #[arg(short, long)]
    listen: bool,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Location of datastore
    #[arg(short = 'r', long, value_hint = ValueHint::FilePath)]
    store: PathBuf,

    /// Location of log files
    #[arg(short = 'o', long, value_hint = ValueHint::FilePath)]
    log: PathBuf,

    /// Unique identifier of FIX session
    #[arg(short, long, default_value = "999")]
    epoch: Arc<String>,

    /// Heartbeat timeout duration in seconds
    #[arg(long, default_value = "30", value_parser = parse_duration)]
    heartbeat_timeout: Duration,

    /// Time session should start each day in format HH:MM:SS
    #[arg(long, default_value = "23:59:59", value_parser = parse_time)]
    start: NaiveTime, 
}

impl Opts {
    #[allow(dead_code)]
    fn additional_headers(&self) -> Vec<(u32, Vec<u8>)> {
        Vec::from([
            (49u32, self.sender_comp_id.as_bytes().to_vec()),
            (56u32, self.target_comp_id.as_bytes().to_vec()),
        ])
    }
}

#[derive(Default)]
struct ApplicationParserCallback<'a> {
    _msg_type: char,
    msg_seq_num: u32,
    cl_order_id: Option<&'a [u8]>,
}

impl<'a> fix::decode::ParserCallback<'a> for ApplicationParserCallback<'a> {
    fn header(&mut self, key: u32, value: &'a [u8]) -> Result<bool, fix::SessionError> {
        if let Ok(fix::generated::Tags::MsgSeqNum) = key.try_into() {
            self.msg_seq_num =
                parse_field::<u32>(value).or(Err(fix::SessionError::MissingMsgSeqNum {
                    text: String::from("Missing MsgSeqNum"),
                }))?;
        }
        Ok(true)
    }
    fn body(&mut self, key: u32, value: &'a [u8]) -> Result<bool, fix::SessionError> {
        if let Ok(fix::generated::Tags::ClOrdID) = key.try_into() {
            self.cl_order_id = Some(value);
        }
        Ok(true)
    }
    fn trailer(&mut self, _key: u32, _value: &'a [u8]) -> Result<bool, fix::SessionError> {
        Ok(false)
    }
    fn sequence_num(&self) -> u32 {
        self.msg_seq_num
    }
}

#[tokio::main]
async fn main() -> Result<(), forgefix::ApplicationError> {
    let opts = Opts::parse();
    // let addr = "138.8.53.226:12189".parse().unwrap();
    let is_server = opts.listen;

    let settings = SessionSettings::builder()
        .with_sender_comp_id(opts.sender_comp_id.as_str())
        .with_target_comp_id(opts.target_comp_id.as_str())
        .with_socket_addr(opts.addr)
        .with_begin_string("FIX.4.2")
        .with_epoch(&opts.epoch)
        .with_store_path(opts.store.clone())
        .with_log_dir(opts.log.clone())
        .with_heartbeat_timeout(opts.heartbeat_timeout)
        .with_start_time(opts.start)
        .build()?;

    if is_server {
        let mut fix_server = FixApplicationAcceptor::build(settings)?;

        loop {
            let (fix_handle, mut event_receiver) = fix_server.accept().await?;
            let h = tokio::spawn(async move {
                let _ = fix_handle.start_async().await;
                while event_receiver.recv().await.is_some() {
                    let default_msg_type: char = fix::generated::MsgType::ORDER_SINGLE.into();
                    let builder = fix::encode::MessageBuilder::new(
                        fix_handle.begin_string().as_str(),
                        default_msg_type,
                    )
                    .push(Tags::ClOrdID, b"ID")
                    .push(Tags::HandlInst, b"1")
                    .push(Tags::OrdType, b"1")
                    .push(Tags::Side, b"1")
                    .push(Tags::Symbol, b"AAA")
                    .push(Tags::TransactTime, b"00000000-00:00:00");
                    let _ = fix_handle.send_message(builder);
                }
            });
            let _ = h.await;
        }
    } else {
        // forgefix PUBLIC API IN USE HERE

        let (fix_handle, mut event_receiver) = FixApplicationInitiator::build(settings)?
            .initiate()
            .await?;

        tokio::spawn(async move {
            while let Some(msg) = event_receiver.recv().await {
                println!("got an application message: {}", msg);
            }
        });

        fix_handle.start_async().await?;

        let _ = send_order(
            &fix_handle,
            "ID1",
            1,
            "AAPL  230803P00100000",
            "2.31",
            true,
            "ELMD",
            "ABCD1234",
        )
        .await;
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let _ = send_order(
            &fix_handle,
            "ID2",
            1,
            "AAPL  230803P00100000",
            "2.31",
            true,
            "ELMD",
            "ABCD1234",
        )
        .await;
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        fix_handle.end_async().await?;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn send_order(
    fix_app_client: &FixApplicationHandle,
    sguid: &str,
    qty: u32,
    symbol: &str,
    price: &str,
    is_buy: bool,
    exchange: &str,
    account: &str,
) -> Result<(), forgefix::ApplicationError> {
    let msg_type = MsgType::ORDER_SINGLE;

    let side = if is_buy {
        fix::generated::Side::BUY
    } else {
        fix::generated::Side::SELL
    };
    let qty = fix::encode::SerializedInt::from(qty);
    let transact_time = fix::encode::formatted_time();

    let builder = fix::encode::MessageBuilder::new(&fix_app_client.begin_string(), msg_type.into())
        .push(Tags::Account, account.as_bytes())
        .push(Tags::ClOrdID, sguid.as_bytes())
        .push(Tags::IDSource, ID_SOURCE.as_bytes())
        .push(Tags::OrderQty, qty.as_bytes())
        .push(Tags::OrdType, fix::generated::OrdType::LIMIT.into())
        .push(Tags::Price, price.as_bytes())
        .push(Tags::SecurityID, symbol.as_bytes())
        .push(Tags::Side, side.into())
        .push(
            Tags::TimeInForce,
            fix::generated::TimeInForce::IMMEDIATE_OR_CANCEL.into(),
        )
        .push(Tags::TransactTime, transact_time.as_bytes())
        .push(Tags::OpenClose, fix::generated::OpenClose::OPEN.into())
        .push(Tags::ExDestination, exchange.as_bytes());

    fix_app_client.send_message_async(builder).await
}

fn _confirm_order(msg: Option<Arc<fix::mem::MsgBuf>>, id: &str) -> Result<(), Box<dyn Error>> {
    match msg {
        Some(msg) => {
            let mut cb: ApplicationParserCallback = Default::default();
            let msg = Arc::new(&msg);

            fix::decode::parse(&msg.as_ref()[..], &mut cb)?;

            if cb.cl_order_id == Some(id.as_bytes()) {
                Ok(())
            } else {
                Err(Box::<dyn Error>::from("ClOrdID did not match!"))
            }
        }
        None => Err(Box::<dyn Error>::from("No message received")),
    }
}
