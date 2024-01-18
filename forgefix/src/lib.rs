pub mod fix;
use fix::encode::MessageBuilder;
use fix::mem::MsgBuf;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use thiserror::Error;
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use tokio::sync::{mpsc, oneshot};

use chrono::naive::NaiveTime; 

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

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("An I/O error occured")]
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

    pub fn with_start_time(mut self, start_time: NaiveTime) -> Self {
        self.set_start_time(start_time);
        self
    }
    pub fn set_start_time(&mut self, start_time: NaiveTime) {
        self.start_time = Some(start_time); 
    }

    pub fn with_sender_comp_id(mut self, sender_comp_id: &str) -> Self {
        self.set_sender_comp_id(sender_comp_id);
        self
    }
    pub fn set_sender_comp_id(&mut self, sender_comp_id: &str) {
        self.sender_comp_id = Some(sender_comp_id.to_string());
    }

    pub fn with_target_comp_id(mut self, target_comp_id: &str) -> Self {
        self.set_target_comp_id(target_comp_id);
        self
    }
    pub fn set_target_comp_id(&mut self, target_comp_id: &str) {
        self.target_comp_id = Some(target_comp_id.to_string());
    }

    pub fn with_socket_addr(mut self, addr: SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }
    pub fn set_socket_addr(&mut self, addr: SocketAddr) {
        self.addr = Some(addr);
    }

    pub fn with_begin_string(mut self, begin_string: &str) -> Self {
        self.set_begin_string(begin_string);
        self
    }
    pub fn set_begin_string(&mut self, begin_string: &str) {
        self.begin_string = Some(begin_string.to_string());
    }

    pub fn with_epoch(mut self, epoch: &str) -> Self {
        self.set_epoch(epoch);
        self
    }
    pub fn set_epoch(&mut self, epoch: &str) {
        self.epoch = Some(epoch.to_string());
    }

    pub fn with_store_path(mut self, store_path: PathBuf) -> Self {
        self.set_store_path(store_path);
        self
    }
    pub fn set_store_path(&mut self, store_path: PathBuf) {
        self.store_path = Some(store_path);
    }

    pub fn with_log_dir(mut self, log_dir: PathBuf) -> Self {
        self.set_log_dir(log_dir);
        self
    }
    pub fn set_log_dir(&mut self, log_dir: PathBuf) {
        self.log_dir = Some(log_dir); 
    }

    pub fn with_heartbeat_timeout(mut self, hb_timeout: Duration) -> Self {
        self.set_heartbeat_timeout(hb_timeout);
        self
    }
    pub fn set_heartbeat_timeout(&mut self, hb_timeout: Duration) {
        self.heartbeat_timeout = Some(hb_timeout);
    }

    pub fn build(self) -> Result<SessionSettings, ApplicationError> {
        let sender_comp_id = self.sender_comp_id.ok_or(ApplicationError::SettingRequired("sender_comp_id".to_string()))?;
        let target_comp_id = self.target_comp_id.ok_or(ApplicationError::SettingRequired("target_comp_id".to_string()))?;
        let addr = self.addr.ok_or(ApplicationError::SettingRequired("addr".to_string()))?;
        let store_path = self.store_path.ok_or(ApplicationError::SettingRequired("store_path".to_string()))?;
        let log_dir = self.log_dir.ok_or(ApplicationError::SettingRequired("log_dir".to_string()))?;

        Ok(SessionSettings {
            engine_type: FixEngineType::Client,
            begin_string: Arc::new(self.begin_string.unwrap_or(String::from("FIX.4.2"))),
            epoch: Arc::new(self.epoch.unwrap_or(format!("{}_{}", &sender_comp_id, &target_comp_id))),
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

#[derive(Clone)]
pub struct FixApplicationHandle {
    request_sender: mpsc::UnboundedSender<Request>,
    begin_string: Arc<String>,
}

impl FixApplicationHandle {
    pub fn start(&self) -> Result<oneshot::Receiver<bool>, ApplicationError> {
        if self.request_sender.is_closed() {
            return Err(ApplicationError::SessionEnded);
        }
        let (resp_sender, resp_receiver) = oneshot::channel();
        let logon_request = Request::Logon { resp_sender };
        let _ = self.request_sender.send(logon_request);
        Ok(resp_receiver)
    }
    pub async fn start_async(&self) -> Result<(), ApplicationError> {
        let resp_sender = self.start()?;
        if Ok(true) != resp_sender.await {
            return Err(ApplicationError::LogonFailed);
        }
        Ok(())
    }
    pub fn start_sync(&self) -> Result<(), ApplicationError> {
        let resp_receiver = self.start()?; 
        if Ok(true) != resp_receiver.blocking_recv() {
            return Err(ApplicationError::LogonFailed);
        }
        Ok(())
    }
    

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
    pub fn send_message_sync(
        &self,
        builder: MessageBuilder,
    ) -> Result<(), ApplicationError> {
        let resp_receiver = self.send_message(builder)?;
        if Ok(true) != resp_receiver.blocking_recv() {
            return Err(ApplicationError::SendMessageFailed);
        }
        Ok(())
    }

    pub fn end(&self) -> Result<oneshot::Receiver<bool>, ApplicationError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let logout_request = Request::Logout { resp_sender };
        let _ = self.request_sender.send(logout_request);
        Ok(resp_receiver)
    }
    pub async fn end_async(&self) -> Result<(), ApplicationError> {
        let resp_sender = self.end()?;
        if Ok(true) != resp_sender.await {
            return Err(ApplicationError::LogoutFailed);
        }
        Ok(())
    }
    pub fn end_sync(&self) -> Result<(), ApplicationError> {
        let resp_receiver = self.end()?;
        if Ok(true) != resp_receiver.blocking_recv() {
            return Err(ApplicationError::LogoutFailed);
        }
        Ok(())
    }

    pub fn begin_string(&self) -> Arc<String> {
        Arc::clone(&self.begin_string)
    }
}

pub struct FixApplicationInitiator {
    settings: SessionSettings,
    stream_factory: StreamFactory,
}

impl FixApplicationInitiator {
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

    pub async fn initiate(
        self,
    ) -> Result<(FixApplicationHandle, mpsc::UnboundedReceiver<Arc<MsgBuf>>), ApplicationError>
    {
        let stream = self.stream_factory.stream().await?;
        let (request_sender, request_receiver) = mpsc::unbounded_channel::<Request>();
        let (app_message_event_sender, app_message_event_receiver) =
            mpsc::unbounded_channel::<Arc<MsgBuf>>();
        let begin_string = Arc::clone(&self.settings.begin_string); 

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

    pub fn initiate_with_runtime(
        self,
        runtime: tokio::runtime::Runtime, 
    ) -> Result<(FixApplicationHandle, mpsc::UnboundedReceiver<Arc<MsgBuf>>), ApplicationError>
    {
        let (request_sender, request_receiver) = mpsc::unbounded_channel::<Request>();
        let (app_message_event_sender, app_message_event_receiver) =
            mpsc::unbounded_channel::<Arc<MsgBuf>>();
        let begin_string = Arc::clone(&self.settings.begin_string); 
        let stream = runtime.block_on(self.stream_factory.stream())?;
        
        std::thread::spawn(move || {
            if let Err(e) = runtime.block_on(fix::spin_session(
                    stream,
                    request_receiver,
                    app_message_event_sender,
                    self.settings,
            ))
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

    pub fn initiate_sync(
        self
    ) -> Result<(FixApplicationHandle, mpsc::UnboundedReceiver<Arc<MsgBuf>>), ApplicationError> 
    {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?; 
        self.initiate_with_runtime(runtime)
    }
}

pub struct FixApplicationAcceptor {
    settings: SessionSettings,
    stream_factory: StreamFactory,
}

impl FixApplicationAcceptor {
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

    pub async fn accept(
        &mut self,
    ) -> Result<(FixApplicationHandle, mpsc::UnboundedReceiver<Arc<MsgBuf>>), ApplicationError>
    {
        let stream = self.stream_factory.stream().await?;
        let settings = self.settings.clone();
        let (request_sender, request_receiver) = mpsc::unbounded_channel::<Request>();
        let (app_message_event_sender, app_message_event_receiver) =
            mpsc::unbounded_channel::<Arc<MsgBuf>>();
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
                let (stream, _from_addr) = listener.accept().await?;
                Ok(stream)
            }
            StreamFactory::Client(addr) => {
                let socket = TcpSocket::new_v4()?;
                Ok(socket.connect(*addr).await?)
            }
        }
    }
}
