use crate::SessionSettings;
use crate::fix::mem::MsgBuf;
use crate::fix::SessionError;

use chrono::offset::{Local};
use chrono::{Duration, DateTime};

use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt};
use tokio::sync::{oneshot, mpsc}; 

use std::time::Instant; 

use anyhow::Result;

const LOG_FILE_TYPE: &str = "txt";

enum LoggerRequest {
    Log(String, Instant),
    Disconnect(oneshot::Sender<Result<(), SessionError>>),
}

pub(super) struct FileLogger {
    sender: mpsc::UnboundedSender<LoggerRequest>,
}

pub(super) trait Logger {
    fn log_message(&mut self, msg: &MsgBuf) -> Result<(), SessionError>;
}

impl Logger for FileLogger {
    fn log_message(&mut self, buf: &MsgBuf) -> Result<(), SessionError> {
        let req = LoggerRequest::Log(format!("{}", buf), Instant::now()); 
        self.sender.send(req).map_err(to_io_err)?;
        Ok(())
    }
}


impl FileLogger {
    pub(super) async fn build(settings: &SessionSettings) -> Result<FileLogger> {
        let log_path = &settings.log_dir;
        let sendercompid = settings.expected_sender_comp_id();
        let targetcompid = settings.expected_target_comp_id();
        std::fs::create_dir_all(log_path)?;
        let mut logs = OpenOptions::new()
            .create(true)
            .append(true)
            .open(
                log_path
                    .join(format!("{}-{}", sendercompid, targetcompid))
                    .with_extension(LOG_FILE_TYPE)
            )
            .await?;

        let (sender, mut receiver) = mpsc::unbounded_channel(); 

        tokio::spawn(async move {
            let begin_time = Local::now();
            let begin_instant = Instant::now();
            while let Some(req) = receiver.recv().await {
                match req {
                    LoggerRequest::Log(msg, instant) => {
                        let send_time = match Duration::from_std(instant.duration_since(begin_instant)) {
                            Ok(d) => begin_time + d,
                            Err(_) => Local::now(),
                        };
                        if let Err(e) = log_message(&mut logs, msg, send_time).await {
                            eprintln!("error logging message: {e:?}")
                        }
                    }
                    LoggerRequest::Disconnect(sender) => {
                        let resp = disconnect(&mut logs).await;
                        let _ = sender.send(resp);
                    }
                }
            }
        }); 

        Ok(FileLogger { sender })
    }

    pub(super) async fn disconnect(&mut self) -> Result<(), SessionError> {
        let (sender, receiver) = oneshot::channel();
        let req = LoggerRequest::Disconnect(sender);
        self.sender.send(req).map_err(to_io_err)?;
        receiver.await.map_err(to_io_err)?
    }
}

async fn log_message(logs: &mut File, buf: String, time: DateTime<Local>) -> Result<(), SessionError> {
    logs
        .write_all(format!("{} : {}\n", message_stamp(time), buf).as_bytes())
        .await?;
    logs.flush().await?;
    Ok(())
}

async fn disconnect(logs: &mut File) -> Result<(), SessionError> {
    logs.flush().await?;
    Ok(())
}


fn message_stamp(time: DateTime<Local>) -> String {
    time
        .format("%Y%m%d-%H:%M:%S%.9f")
        .to_string()
}

fn to_io_err<E>(e: E) -> std::io::Error 
where E: Into<Box<dyn std::error::Error + Send + Sync>>
{
    std::io::Error::new(std::io::ErrorKind::Other, e)
}
