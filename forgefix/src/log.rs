//! Generic logging traits and [FileLoggerFactory]
//!
//! [FileLoggerFactory]: crate::log::FileLoggerFactory

use crate::SessionSettings;
use crate::fix::mem::MsgBuf;

use chrono::offset::Local;
use chrono::{DateTime, Duration};

use std::fs::{File, OpenOptions};
use std::future::Future;
use std::io::{self, Write};
use std::sync::mpsc;
use std::time::Instant;

const LOG_FILE_TYPE: &str = "txt";

/// A sink for FIX message traffic produced by a running engine.
///
/// The engine calls [`log_message`] for every message it sends or receives, and calls
/// [`disconnect`] once when the connection ends.
///
/// # Implementing `Logger`
///
/// [`log_message`] is called on every inbound and outbound message, so it sits squarely in the
/// hot path of the engine loop. Implementations should be **fast and non-blocking** — do not
/// perform synchronous I/O, acquire a contended lock, or do any meaningful computation inside
/// this method. The recommended pattern is to forward the formatted message to a background
/// worker via a channel (as [`FileLoggerFactory`] does) and let the worker handle I/O off the
/// critical path.
///
/// [`disconnect`] is awaited once at the end of a session. It is the right place to flush
/// buffers and release resources.
///
/// [`log_message`]: Logger::log_message
/// [`disconnect`]: Logger::disconnect
pub trait Logger: Send + 'static {
    /// Record a single FIX message.
    ///
    /// Called for every message sent or received by the engine. Must be fast and non-blocking —
    /// see the [trait-level documentation](Logger) for guidance.
    fn log_message(&mut self, msg: &MsgBuf) -> Result<(), io::Error>;

    /// Flush and release any resources held by the logger.
    ///
    /// Called once when the session ends. The engine `await`s the returned future before
    /// completing teardown, so it is safe to perform I/O here.
    fn disconnect(self) -> impl Future<Output = Result<(), io::Error>> + Send;
}

/// A factory that constructs a [`Logger`] instance for each FIX connection.
///
/// Implement this trait to plug a custom logging backend into an [`EngineFactory`]. The
/// [`build`] method is called once per [`connect`] call, just before the engine task is
/// spawned, and receives the session's [`SessionSettings`] so the logger can be named or
/// configured per-session.
///
/// The built-in implementation is [`FileLoggerFactory`], which writes timestamped messages to
/// a plain-text file in the directory specified by [`SessionSettings`].
///
/// [`build`]: LoggerFactory::build
/// [`connect`]: crate::EngineFactory::connect
/// [`EngineFactory`]: crate::EngineFactory
pub trait LoggerFactory {
    /// Build a [`Logger`] for the session described by `settings`.
    fn build(&self, settings: &SessionSettings) -> Result<impl Logger, io::Error>;
}

enum LoggerRequest {
    Log(String, Instant),
    Disconnect,
}

/// A [`LoggerFactory`] that writes FIX message traffic to a plain-text log file.
///
/// Each session gets its own file named `<TargetCompID>-<SenderCompID>.txt` inside the
/// directory configured by
/// [`SessionSettingsBuilder::with_log_dir`](crate::SessionSettingsBuilder::with_log_dir). The file is opened in append
/// mode so logs persist across restarts.
///
/// I/O is performed on a dedicated `spawn_blocking` thread so that logging never blocks the
/// async engine loop.
///
/// # Runtime requirement
///
/// [`LoggerFactory::build`] must be called from within a Tokio runtime.
pub struct FileLoggerFactory;

impl LoggerFactory for FileLoggerFactory {
    fn build(&self, settings: &SessionSettings) -> Result<impl Logger, io::Error> {
        FileLogger::build(settings)
    }
}

pub(super) struct FileLogger {
    sender: mpsc::Sender<LoggerRequest>,
    handle: tokio::task::JoinHandle<Result<(), io::Error>>,
}

impl Logger for FileLogger {
    fn log_message(&mut self, buf: &MsgBuf) -> Result<(), io::Error> {
        let req = LoggerRequest::Log(format!("{}", buf), Instant::now());
        self.sender.send(req).map_err(io_err)?;
        Ok(())
    }

    async fn disconnect(self) -> Result<(), io::Error> {
        let _ = self.sender.send(LoggerRequest::Disconnect);
        self.handle.await.map_err(|_| io_err(()))?
    }
}

impl FileLogger {
    pub(super) fn build(settings: &SessionSettings) -> Result<FileLogger, io::Error> {
        let log_path = &settings.log_dir;
        let sendercompid = settings.expected_sender_comp_id();
        let targetcompid = settings.expected_target_comp_id();
        std::fs::create_dir_all(log_path)?;
        let mut logs = OpenOptions::new().create(true).append(true).open(
            log_path
                .join(format!("{}-{}", sendercompid, targetcompid))
                .with_extension(LOG_FILE_TYPE),
        )?;

        let (sender, receiver) = mpsc::channel();

        let handle = tokio::task::spawn_blocking(move || {
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
                    LoggerRequest::Disconnect => {
                        break;
                    }
                }
            }
            do_disconnect(&mut logs)
        });

        Ok(FileLogger { sender, handle })
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
