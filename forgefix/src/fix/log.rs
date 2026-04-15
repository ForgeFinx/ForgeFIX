use crate::SessionSettings;
use crate::fix::mem::MsgBuf;

use chrono::offset::Local;

use std::fs::{File, OpenOptions};
use std::io::{self, Write};

use anyhow::Result;

const LOG_FILE_TYPE: &str = "txt";

pub(super) struct FileLogger {
    file: File,
    buf: Vec<u8>,
}

pub(super) trait Logger {
    fn log_message(&mut self, msg: &MsgBuf) -> Result<(), io::Error>;
}

impl Logger for FileLogger {
    fn log_message(&mut self, msg: &MsgBuf) -> Result<(), io::Error> {
        self.buf.clear();
        write!(
            self.buf,
            "{} : ",
            Local::now().format("%Y%m%d-%H:%M:%S%.9f")
        )?;
        self.buf.extend_from_slice(&msg.0);
        self.buf.push(b'\n');
        self.file.write_all(&self.buf)?;
        self.file.flush()?;
        Ok(())
    }
}

impl FileLogger {
    pub(super) async fn build(settings: &SessionSettings) -> Result<FileLogger> {
        let log_path = &settings.log_dir;
        let sendercompid = settings.expected_sender_comp_id();
        let targetcompid = settings.expected_target_comp_id();
        std::fs::create_dir_all(log_path)?;
        let file = OpenOptions::new().create(true).append(true).open(
            log_path
                .join(format!("{}-{}", sendercompid, targetcompid))
                .with_extension(LOG_FILE_TYPE),
        )?;
        Ok(FileLogger {
            file,
            buf: Vec::with_capacity(1024),
        })
    }

    pub(super) async fn disconnect(&mut self) -> Result<(), io::Error> {
        self.file.flush()?;
        Ok(())
    }
}
