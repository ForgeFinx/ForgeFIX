use crate::SessionSettings;
use crate::fix::SessionError;
use crate::fix::mem::MsgBuf;

use chrono::offset::Local;
use chrono::{DateTime, Duration};

use tokio::sync::oneshot;

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::mpsc;
use std::time::Instant;

use anyhow::Result;

const LOG_FILE_TYPE: &str = "txt";

enum LoggerRequest {
    Log(String, Instant),
    Disconnect(oneshot::Sender<Result<(), io::Error>>),
}

pub(super) struct FileLogger {
    sender: mpsc::Sender<LoggerRequest>,
}

pub(super) trait Logger {
    fn log_message(&mut self, msg: &MsgBuf) -> Result<(), io::Error>;
}

impl Logger for FileLogger {
    fn log_message(&mut self, buf: &MsgBuf) -> Result<(), io::Error> {
        let req = LoggerRequest::Log(format!("{}", buf), Instant::now());
        self.sender.send(req).map_err(io_err)?;
        Ok(())
    }
}

impl FileLogger {
    pub(super) async fn build(settings: &SessionSettings) -> Result<FileLogger> {
        let log_path = &settings.log_dir;
        let sendercompid = settings.expected_sender_comp_id();
        let targetcompid = settings.expected_target_comp_id();
        std::fs::create_dir_all(log_path)?;
        let mut logs = OpenOptions::new().create(true).append(true).open(
            log_path
                .join(format!("{}-{}", sendercompid, targetcompid))
                .with_extension(LOG_FILE_TYPE),
        )?;

        let (sender, mut receiver) = mpsc::channel();

        tokio::task::spawn_blocking(move || {
            let begin_time = Local::now();
            let begin_instant = Instant::now();
            while let Ok(req) = receiver.recv() {
                match req {
                    LoggerRequest::Log(msg, instant) => {
                        let send_time =
                            match Duration::from_std(instant.duration_since(begin_instant)) {
                                Ok(d) => begin_time + d,
                                Err(_) => Local::now(),
                            };
                        if let Err(e) = do_log_message(&mut logs, msg, send_time) {
                            eprintln!("error logging message: {e:?}")
                        }
                    }
                    LoggerRequest::Disconnect(sender) => {
                        let resp = do_disconnect(&mut logs);
                        let _ = sender.send(resp);
                    }
                }
            }
        });

        Ok(FileLogger { sender })
    }

    pub(super) async fn disconnect(&mut self) -> Result<(), io::Error> {
        let (sender, receiver) = oneshot::channel();
        let req = LoggerRequest::Disconnect(sender);
        self.sender.send(req).map_err(io_err)?;
        receiver.await.map_err(io_err)?
    }
}

fn do_log_message(logs: &mut File, buf: String, time: DateTime<Local>) -> Result<(), io::Error> {
    writeln!(logs, "{} : {}", time.format("%Y%m%d-%H:%M:%S%.9f"), buf)
}

fn do_disconnect(logs: &mut File) -> Result<(), io::Error> {
    Ok(logs.flush()?)
}

fn io_err<T>(_: T) -> io::Error {
    io::Error::other("logger thread failed")
}
