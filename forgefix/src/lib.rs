//! An opinionated FIX 4.2 client library for the buy-side.
//!
//! ForgeFIX is an engine that implements a subset of the FIX protocol which allows users to connect
//! to brokers or exchanges to send and receive messages.
//!
//! ## Terminology
//! * `FIX Connection` -- A single connection to a FIX Session. A network connection is made over TCP,
//! then a FIX logon handshake is performed to establish the FIX connection. The FIX connection
//! ends properly with a FIX logout, but is considered ended if the TCP connection breaks.
//!     * Note, the term 'connection' is overloaded and can also mean TCP connection. When unclear, a
//! 'connection' will be specified as TCP or FIX.
//!
//! * `FIX Session` -- A conceptual construct that represents the bidirectional stream of ordered
//! messages between two peers. A FIX Session can live across multiple instances of a FIX
//! connection.
//!
//! * `FIX Engine` -- A sub-process running in the background that manages a single FIX connection
//! to a FIX Session. The engine starts, runs, and ends the FIX connection as defined by the FIX
//! protocol, and manages all resources that support the connection.
//!
//! ## Examples
//!
//! ### Asynchronous API
//! ```no_run
//! use forgefix::{
//!     SessionSettings, FixApplicationHandle, FixApplicationInitiator, ApplicationError,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), ApplicationError> {
//!     
//!     // build session settings
//!     let settings = SessionSettings::builder()
//!         .with_sender_comp_id("my_id")
//!         .with_target_comp_id("peer_id")
//!         .with_store_path("./store".into())
//!         .with_log_dir("./log".into())
//!         .with_socket_addr("127.0.0.1:0".parse().unwrap())
//!         .build()?;
//!
//!     // create a FIX engine and intiate TCP connection
//!     let (fix_handle, mut event_receiver) = FixApplicationInitiator::build(settings)?
//!         .initiate()
//!         .await?;
//!
//!     // handle incoming messages in the background...
//!     tokio::spawn(async move {
//!         while let Some(msg) = event_receiver.recv().await {
//!             println!("got an application message: {}", msg);
//!         }
//!     });
//!
//!     // start the FIX connection
//!     fix_handle.start_async().await?;
//!
//!     // send messages here...
//!
//!     // end the FIX connection
//!     fix_handle.end_async().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Synchronous API*
//! ```no_run
//! use forgefix::{
//!     SessionSettings, FixApplicationHandle, FixApplicationInitiator, ApplicationError,
//! };
//!
//! fn main() -> Result<(), ApplicationError> {
//!
//!     let settings = SessionSettings::builder()
//!         .with_sender_comp_id("my_id")
//!         .with_target_comp_id("peer_id")
//!         .with_store_path("./store".into())
//!         .with_log_dir("./log".into())
//!         .with_socket_addr("127.0.0.1:0".parse().unwrap())
//!         .build()?;
//!
//!     let (fix_handle, mut event_receiver) = FixApplicationInitiator::build(settings)?
//!         .initiate_sync()?;
//!
//!     std::thread::spawn(move || {
//!         while let Some(msg) = event_receiver.blocking_recv() {
//!             println!("got an application message: {}", msg);
//!         }
//!     });
//!
//!     fix_handle.start_sync()?;
//!
//!     // send messages here...
//!
//!     fix_handle.end_sync()?;
//!     
//!     Ok(())
//! }
//! ```
//! *When using synchronous API, a tokio runtime is still created internally (see
//! [`FixApplicationInitiator`])

pub mod fix;
use fix::encode::MessageBuilder;
use fix::mem::MsgBuf;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use thiserror::Error;
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use tokio::sync::{mpsc, oneshot};

use chrono::naive::NaiveTime;

use rtrb::{Consumer, Producer, RingBuffer};

enum Request {
    Logon {
        resp_sender: oneshot::Sender<bool>,
    },
    SendMessage {
        resp_sender: oneshot::Sender<bool>,
        builder: MessageBuilder,
    },
    Logout {
        resp_sender: oneshot::Sender<bool>,
    },
}

/// Errors that can occur while running ForgeFIX.
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("An I/O error occured: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Session ended unexpectedly")]
    SessionEnded,
    #[error("Logon has failed")]
    LogonFailed,
    #[error("Logout has failed")]
    LogoutFailed,
    #[error("MessageSend has failed")]
    SendMessageFailed,
    #[error("setting `{0}` is required")]
    SettingRequired(String),
}

/// A collection of settings used to configurate a FIX session.
///
/// `SessionSettings` can be constructed using the [`SessionSettingsBuilder`], or can be constructed explicitly.
#[derive(Clone)]
pub struct SessionSettings {
    begin_string: Arc<String>,
    engine_type: FixEngineType,
    sender_comp_id: String,
    target_comp_id: String,
    addr: SocketAddr,
    epoch: Arc<String>,
    store_path: PathBuf,
    log_dir: PathBuf,
    heartbeat_timeout: Duration,
    start_time: NaiveTime,
}

/// A builder for easily configuring all the fields of a [`SessionSettings`]
///
/// The following settings are required to be set:
/// * sender comp id
/// * target comp id
/// * addr
/// * store path
/// * log dir
#[derive(Default)]
pub struct SessionSettingsBuilder {
    sender_comp_id: Option<String>,
    target_comp_id: Option<String>,
    addr: Option<SocketAddr>,
    begin_string: Option<String>,
    epoch: Option<String>,
    store_path: Option<PathBuf>,
    log_dir: Option<PathBuf>,
    heartbeat_timeout: Option<Duration>,
    start_time: Option<NaiveTime>,
}

impl SessionSettingsBuilder {
    pub fn new() -> SessionSettingsBuilder {
        Default::default()
    }

    /// The time the FIX session starts each day.
    pub fn with_start_time(mut self, start_time: NaiveTime) -> Self {
        self.set_start_time(start_time);
        self
    }
    pub fn set_start_time(&mut self, start_time: NaiveTime) {
        self.start_time = Some(start_time);
    }

    /// The `SenderCompID(49)` that will be included in each message.
    pub fn with_sender_comp_id(mut self, sender_comp_id: &str) -> Self {
        self.set_sender_comp_id(sender_comp_id);
        self
    }
    pub fn set_sender_comp_id(&mut self, sender_comp_id: &str) {
        self.sender_comp_id = Some(sender_comp_id.to_string());
    }

    /// The `TargetCompID(56)` that will be included in each message.
    pub fn with_target_comp_id(mut self, target_comp_id: &str) -> Self {
        self.set_target_comp_id(target_comp_id);
        self
    }
    pub fn set_target_comp_id(&mut self, target_comp_id: &str) {
        self.target_comp_id = Some(target_comp_id.to_string());
    }

    /// The address to initiate a connection to, or accept connections on.
    pub fn with_socket_addr(mut self, addr: SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }
    pub fn set_socket_addr(&mut self, addr: SocketAddr) {
        self.addr = Some(addr);
    }

    /// The `BeginString(8)` that will be included in each message.
    pub fn with_begin_string(mut self, begin_string: &str) -> Self {
        self.set_begin_string(begin_string);
        self
    }
    pub fn set_begin_string(&mut self, begin_string: &str) {
        self.begin_string = Some(begin_string.to_string());
    }

    /// A local unique identifier for this FIX session.
    pub fn with_epoch(mut self, epoch: &str) -> Self {
        self.set_epoch(epoch);
        self
    }
    pub fn set_epoch(&mut self, epoch: &str) {
        self.epoch = Some(epoch.to_string());
    }

    /// The file that should be used as the sqlite database file.
    pub fn with_store_path(mut self, store_path: PathBuf) -> Self {
        self.set_store_path(store_path);
        self
    }
    pub fn set_store_path(&mut self, store_path: PathBuf) {
        self.store_path = Some(store_path);
    }

    /// The directory that should be used to create log files.
    pub fn with_log_dir(mut self, log_dir: PathBuf) -> Self {
        self.set_log_dir(log_dir);
        self
    }
    pub fn set_log_dir(&mut self, log_dir: PathBuf) {
        self.log_dir = Some(log_dir);
    }

    /// The timeout length used for sending `Heartbeat<0>` messages.
    pub fn with_heartbeat_timeout(mut self, hb_timeout: Duration) -> Self {
        self.set_heartbeat_timeout(hb_timeout);
        self
    }
    pub fn set_heartbeat_timeout(&mut self, hb_timeout: Duration) {
        self.heartbeat_timeout = Some(hb_timeout);
    }

    /// Build the [`SessionSettings`] struct.
    ///
    /// Returns an `Err(ApplicationError::SettingRequired)` if not all of the required fields
    /// were set.
    pub fn build(self) -> Result<SessionSettings, ApplicationError> {
        let sender_comp_id = self
            .sender_comp_id
            .ok_or(ApplicationError::SettingRequired(
                "sender_comp_id".to_string(),
            ))?;
        let target_comp_id = self
            .target_comp_id
            .ok_or(ApplicationError::SettingRequired(
                "target_comp_id".to_string(),
            ))?;
        let addr = self
            .addr
            .ok_or(ApplicationError::SettingRequired("addr".to_string()))?;
        let store_path = self
            .store_path
            .ok_or(ApplicationError::SettingRequired("store_path".to_string()))?;
        let log_dir = self
            .log_dir
            .ok_or(ApplicationError::SettingRequired("log_dir".to_string()))?;

        Ok(SessionSettings {
            engine_type: FixEngineType::Client,
            begin_string: Arc::new(self.begin_string.unwrap_or(String::from("FIX.4.2"))),
            epoch: Arc::new(
                self.epoch
                    .unwrap_or(format!("{}_{}", &sender_comp_id, &target_comp_id)),
            ),
            heartbeat_timeout: self.heartbeat_timeout.unwrap_or(Duration::from_secs(30)),
            start_time: self.start_time.unwrap_or_default(),
            sender_comp_id,
            target_comp_id,
            addr,
            store_path,
            log_dir,
        })
    }
}

impl SessionSettings {
    /// Creates a new [`SessionSettingsBuilder`]
    pub fn builder() -> SessionSettingsBuilder {
        SessionSettingsBuilder::new()
    }

    fn expected_sender_comp_id(&self) -> &str {
        &self.target_comp_id
    }

    fn expected_target_comp_id(&self) -> &str {
        &self.sender_comp_id
    }
}

/// A handle on a FIX engine instance.
///
/// The [`FixApplicationHandle`] allows for requesting the basic operations of starting the FIX connection, sending
/// a message to the peer, and ending the connection.
///
/// The handle offers asynchronous and synchronous APIs for these operations. As well as functions
/// that return immedietly with a [`oneshot::Receiver`] that will eventually return the result of the
/// operation.
///
/// The underlying engine could stop running at any moment for a variety of reasons. Only until you
/// attempt an operation, will you learn the engine has stopped by receiving an
/// [`ApplicationError::SessionEnded`].
///
/// [`FixApplicationHandle`] `impl`'s [`Clone`], [`Send`] and [`Sync`] and therefore multiple
/// copies of the handle can be made and passed to different threads that can all request messages
/// to be sent. Only one thread has to call [`end`] for the engine to terminate the connection.
///
/// [`oneshot::Receiver`]: https://docs.rs/tokio/latest/tokio/sync/oneshot/struct.Receiver.html
/// [`end`]: FixApplicationHandle::end
///
/// # Example - Multiple Threads
///
///```no_run
/// use forgefix::{
///     SessionSettings, FixApplicationInitiator, ApplicationError
/// };
/// use forgefix::fix::{encode::MessageBuilder, generated::MsgType};
/// # use anyhow::Result;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// #    let settings = SessionSettings::builder()
/// #        .with_sender_comp_id("my_id")
/// #        .with_target_comp_id("peer_id")
/// #        .with_store_path("./store".into())
/// #        .with_log_dir("./log".into())
/// #        .with_socket_addr("127.0.0.1:0".parse().unwrap())
/// #        .build()?;
///
/// let (handle, mut receiver) = FixApplicationInitiator::build(settings)?
///     .initiate()
///     .await?;
/// receiver.close();
///
/// // FixApplicationHandle can be cloned
/// let handle1 = handle.clone();
/// let handle2 = handle.clone();
///
/// // FixApplicationHandle clones can be sent across threads and tasks
/// let h1 = tokio::spawn(async move {
///
///     // thread logic here...
///
///     let builder = MessageBuilder::new(
///         &handle1.begin_string(),
///         MsgType::ORDER_SINGLE.into()
///     );
///     handle1.send_message_async(builder).await
///
///     // ...
/// });
///
/// // send to multiple tasks...
/// let h2 = tokio::spawn(async move {
///     let builder = MessageBuilder::new(
///         &handle2.begin_string(),
///         MsgType::ORDER_SINGLE.into()
///     );
///     handle2.send_message_async(builder).await
/// });
///
/// // wait for all threads to finish...
/// let (res1, res2) = tokio::join!(h1, h2);
/// res1??;
/// res2??;
///     
/// // end the FIX connection
/// handle.end_async().await?;
///  #   Ok(())
/// # }
///
///```
#[derive(Clone)]
pub struct FixApplicationHandle {
    request_sender: mpsc::UnboundedSender<Request>,
    begin_string: Arc<String>,
}

impl FixApplicationHandle {
    /// Send a request to the engine to start the connection and return immediately.
    ///
    /// The receiver will eventually yield `true` if a connection was successfully established, or
    /// `false` othersize.
    pub fn start(&self) -> Result<oneshot::Receiver<bool>, ApplicationError> {
        if self.request_sender.is_closed() {
            return Err(ApplicationError::SessionEnded);
        }
        let (resp_sender, resp_receiver) = oneshot::channel();
        let logon_request = Request::Logon { resp_sender };
        let _ = self.request_sender.send(logon_request);
        Ok(resp_receiver)
    }
    /// Send a request to the engine to start the connection and await asynchronously.
    pub async fn start_async(&self) -> Result<(), ApplicationError> {
        let resp_sender = self.start()?;
        if Ok(true) != resp_sender.await {
            return Err(ApplicationError::LogonFailed);
        }
        Ok(())
    }
    /// Send a request to the engine to start a connection, and block until a result is returned.
    pub fn start_sync(&self) -> Result<(), ApplicationError> {
        let resp_receiver = self.start()?;
        if Ok(true) != resp_receiver.blocking_recv() {
            return Err(ApplicationError::LogonFailed);
        }
        Ok(())
    }

    /// Send a request to the engine to send the message in the [`MessageBuilder`] to the peer, and return immediately.
    ///
    /// If the request was successfully sent to the engine, a [`oneshot::Receiver`] will be
    /// returned.
    ///
    /// The receiver will yield `true` once the message has successfully sent over the TCP
    /// connection. It will yeild `false` if a message cannot be sent.
    ///
    /// [`oneshot::Receiver`]: https://docs.rs/tokio/latest/tokio/sync/oneshot/struct.Receiver.html
    pub fn send_message(
        &self,
        builder: MessageBuilder,
    ) -> Result<oneshot::Receiver<bool>, ApplicationError> {
        if self.request_sender.is_closed() {
            return Err(ApplicationError::SessionEnded);
        }
        let (resp_sender, resp_receiver) = oneshot::channel();
        let send_message_request = Request::SendMessage {
            resp_sender,
            builder,
        };
        let _ = self.request_sender.send(send_message_request);
        Ok(resp_receiver)
    }
    /// Send a request to the engine to send the message in `builder` and await asynchronously.
    pub async fn send_message_async(
        &self,
        builder: MessageBuilder,
    ) -> Result<(), ApplicationError> {
        let resp_sender = self.send_message(builder)?;
        if Ok(true) != resp_sender.await {
            return Err(ApplicationError::SendMessageFailed);
        }
        Ok(())
    }
    /// Send a request to the engine to send the message in `builder` and block until a result is
    /// returned.
    pub fn send_message_sync(&self, builder: MessageBuilder) -> Result<(), ApplicationError> {
        let resp_receiver = self.send_message(builder)?;
        if Ok(true) != resp_receiver.blocking_recv() {
            return Err(ApplicationError::SendMessageFailed);
        }
        Ok(())
    }

    /// Send a request to the engine to end the FIX connection, and return immediately.
    ///
    /// If the request was successfully send to the engine, a [`oneshot::Receiver`] will be
    /// returned.
    ///
    /// The receiver will yield `true` is the FIX connection is over, and ended without any issues.
    /// Otherwise it will be `false`.
    ///
    /// [`oneshot::Receiver`]: https://docs.rs/tokio/latest/tokio/sync/oneshot/struct.Receiver.html
    pub fn end(&self) -> Result<oneshot::Receiver<bool>, ApplicationError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let logout_request = Request::Logout { resp_sender };
        let _ = self.request_sender.send(logout_request);
        Ok(resp_receiver)
    }
    /// Send a request to the engine to end the FIX connection, and await asynchronously.
    pub async fn end_async(&self) -> Result<(), ApplicationError> {
        let resp_sender = self.end()?;
        if Ok(true) != resp_sender.await {
            return Err(ApplicationError::LogoutFailed);
        }
        Ok(())
    }
    /// Send a request to the engine to end the FIX connection, and block until a result is
    /// returned.
    pub fn end_sync(&self) -> Result<(), ApplicationError> {
        let resp_receiver = self.end()?;
        if Ok(true) != resp_receiver.blocking_recv() {
            return Err(ApplicationError::LogoutFailed);
        }
        Ok(())
    }

    /// Get the `BeginString(8)` of this FIX Session. Should generally be `"FIX.4.2"`.
    pub fn begin_string(&self) -> Arc<String> {
        Arc::clone(&self.begin_string)
    }
}

/// A struct that can initiate the TCP connection to the peer and create a FIX engine instance.
pub struct FixApplicationInitiator {
    settings: SessionSettings,
    stream_factory: StreamFactory,
}

impl FixApplicationInitiator {
    /// Build a `FixApplicationInitiator` that will create a FIX engine using `settings`.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        mut settings: SessionSettings,
    ) -> Result<FixApplicationInitiator, ApplicationError> {
        settings.engine_type = FixEngineType::Client;
        let stream_factory = StreamFactory::build(&settings)?;
        let fix_app_client = FixApplicationInitiator {
            settings,
            stream_factory,
        };
        Ok(fix_app_client)
    }

    /// Initiate a TCP connection and start the FIX engine with the current asynchronous runtime.
    ///
    /// If the connection is successfully made, a [`FixApplicationHandle`] will be returned, and an
    /// `UnboundedReceiver<Arc<MsgBuf>>` will be returned.
    ///
    /// The application handle can be used to start the FIX connection, send messages and end the
    /// connection.
    ///
    /// The receiver is a channel where all incoming, valid application messages can be received.
    /// If you do not want to use the channel, it is recommended you call [`close`].
    ///
    /// [`close`]: tokio::sync::mpsc::UnboundedReceiver::close
    pub async fn initiate(
        self,
    ) -> Result<(FixApplicationHandle, Consumer<Arc<MsgBuf>>), ApplicationError> {
        let stream = self.stream_factory.stream().await?;
        let (request_sender, request_receiver) = mpsc::unbounded_channel::<Request>();
        let begin_string = Arc::clone(&self.settings.begin_string);

        let (app_message_event_sender, app_message_event_receiver) = RingBuffer::new(1000);

        tokio::spawn(async move {
            if let Err(e) = fix::spin_session(
                stream,
                request_receiver,
                app_message_event_sender,
                self.settings,
            )
            .await
            {
                eprintln!("{e:?}");
            }
        });

        let handle = FixApplicationHandle {
            request_sender,
            begin_string,
        };

        Ok((handle, app_message_event_receiver))
    }

    /// Initiate a TCP connection and start the FIX engine that will be driven by `runtime`.
    pub fn initiate_with_runtime(
        self,
        runtime: tokio::runtime::Runtime,
    ) -> Result<(FixApplicationHandle, Consumer<Arc<MsgBuf>>), ApplicationError> {
        let (request_sender, request_receiver) = mpsc::unbounded_channel::<Request>();
        let (app_message_event_sender, app_message_event_receiver) = RingBuffer::new(1000);
        let begin_string = Arc::clone(&self.settings.begin_string);
        let stream = runtime.block_on(self.stream_factory.stream())?;

        std::thread::spawn(move || {
            if let Err(e) = runtime.block_on(fix::spin_session(
                stream,
                request_receiver,
                app_message_event_sender,
                self.settings,
            )) {
                eprintln!("{e:?}");
            }
        });
        let handle = FixApplicationHandle {
            request_sender,
            begin_string,
        };

        Ok((handle, app_message_event_receiver))
    }

    /// Initiate a TCP connection, and a runtime will be created internally to drive the engine.
    pub fn initiate_sync(
        self,
    ) -> Result<(FixApplicationHandle, Consumer<Arc<MsgBuf>>), ApplicationError> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        self.initiate_with_runtime(runtime)
    }
}

/// A struct that can accept TCP connections, and create a FIX engine instance for each connection.
pub struct FixApplicationAcceptor {
    settings: SessionSettings,
    stream_factory: StreamFactory,
}

impl FixApplicationAcceptor {
    /// Build a `FixApplicationAcceptor` from `settings`.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        mut settings: SessionSettings,
    ) -> Result<FixApplicationAcceptor, ApplicationError> {
        settings.engine_type = FixEngineType::Server;
        let stream_factory = StreamFactory::build(&settings)?;
        let fix_app_server = FixApplicationAcceptor {
            settings,
            stream_factory,
        };
        Ok(fix_app_server)
    }

    /// Accept an incoming TCP connection and create a FIX engine.
    ///
    /// Returns the handle to the created engine, and a channel to receive all valid, incoming application
    /// messages.
    pub async fn accept(
        &mut self,
    ) -> Result<(FixApplicationHandle, Consumer<Arc<MsgBuf>>), ApplicationError> {
        let stream = self.stream_factory.stream().await?;
        let settings = self.settings.clone();
        let (request_sender, request_receiver) = mpsc::unbounded_channel::<Request>();
        let (app_message_event_sender, app_message_event_receiver) = RingBuffer::new(1000);
        let begin_string = Arc::clone(&self.settings.begin_string);

        tokio::task::spawn(async move {
            if let Err(e) =
                fix::spin_session(stream, request_receiver, app_message_event_sender, settings)
                    .await
            {
                eprintln!("{e:?}");
            }
        });

        let handle = FixApplicationHandle {
            request_sender,
            begin_string,
        };

        Ok((handle, app_message_event_receiver))
    }
}

#[derive(Clone)]
enum FixEngineType {
    Client,
    Server,
}

enum StreamFactory {
    Server(TcpListener),
    Client(std::net::SocketAddr),
}

impl StreamFactory {
    fn build(settings: &SessionSettings) -> Result<Self, std::io::Error> {
        match settings.engine_type {
            FixEngineType::Client => Ok(StreamFactory::Client(settings.addr)),
            FixEngineType::Server => {
                let socket = TcpSocket::new_v4()?;
                socket.bind(settings.addr)?;
                let listener = socket.listen(1024)?;
                Ok(StreamFactory::Server(listener))
            }
        }
    }
    async fn stream(&self) -> Result<TcpStream, std::io::Error> {
        match self {
            StreamFactory::Server(listener) => {
                let (mut stream, _from_addr) = listener.accept().await?;
                stream.set_nodelay(true)?;
                Ok(stream)
            }
            StreamFactory::Client(addr) => {
                let socket = TcpSocket::new_v4()?;
                let mut stream = socket.connect(*addr).await?;
                stream.set_nodelay(true)?;
                Ok(stream)
            }
        }
    }
}
