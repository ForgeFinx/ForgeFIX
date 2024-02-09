use crate::SessionSettings;
use crate::fix::mem::MsgBuf;
use crate::fix::SessionError;
use tokio::fs::{File, OpenOptions};
use tokio::io::{BufWriter, AsyncWriteExt};

pub(super) struct Logger {
    logs: BufWriter<File>,
}

impl Logger {
    pub(super) async fn build(settings: &SessionSettings) -> Result<Logger, SessionError> {
        let log_path = &settings.log_dir; 
        let sender_comp_id = settings.expected_sender_comp_id();
        let target_comp_id = settings.expected_target_comp_id(); 
        let filetype = "txt";
        std::fs::create_dir_all(log_path)?;
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(
                log_path
                    .join(format!("{}-{}", sender_comp_id, target_comp_id))
                    .with_extension(filetype),
            )
            .await?;
        Ok(Logger { logs: BufWriter::new(file) })
    }

    pub(super) async fn log_message(&mut self, buf: &MsgBuf) -> Result<(), SessionError> {
        self.logs
            .write_all(format!("{} : {}\n", message_stamp(), buf).as_bytes())
            .await?;
        Ok(())
    }

    pub(super) async fn disconnect(&mut self) -> Result<(), SessionError> {
        self.logs.flush().await?;
        Ok(())
    }
}

fn message_stamp() -> String {
    chrono::offset::Local::now()
        .format("%Y%m%d-%H:%M:%S%.9f")
        .to_string()
}
