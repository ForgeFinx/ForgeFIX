use anyhow::Result;

use crate::SessionSettings;
use crate::fix::mem::MsgBuf;

use std::sync::Arc;
use std::time::Instant; 

use chrono::offset::Utc; 
use chrono::naive::NaiveDateTime; 
use chrono::{DateTime, Duration}; 
use tokio::sync::{mpsc, oneshot};
use tokio_rusqlite::Connection;
use rusqlite::{OptionalExtension, OpenFlags};

const SQL_ENTER_WAL_MODE: &str = "PRAGMA journal_mode=WAL;";
const SQL_VACUUM: &str = "VACUUM;";
const SQL_CREATE_INCOMING_TABLE :&str="CREATE TABLE IF NOT EXISTS incoming_messages (key INTEGER PRIMARY KEY AUTOINCREMENT, epoch_guid VARCHAR, msg_seq_num INT, message BLOB);";
const SQL_CREATE_OUTGOING_TABLE :&str=
    "CREATE TABLE IF NOT EXISTS outgoing_messages (key INTEGER PRIMARY KEY AUTOINCREMENT, epoch_guid VARCHAR, msg_seq_num INT, send_time VARCHAR, message BLOB);";
const SQL_CREATE_SEQUENCES: &str =
    "CREATE TABLE IF NOT EXISTS sequences (epoch_guid VARCHAR, next_incoming INTEGER, next_outgoing INTEGER)";
const SQL_ENSURE_SEQUENCE_ROW: &str = "INSERT INTO sequences(epoch_guid, next_incoming, next_outgoing) SELECT ?1,1,1 WHERE NOT EXISTS (SELECT * FROM sequences WHERE epoch_guid = ?1);";
const SQL_INSERT_OUTGOING_MESSAGE: &str =
    "INSERT INTO outgoing_messages (epoch_guid, msg_seq_num, send_time, message) VALUES (?,?,?,?)";
const SQL_LAST_SEND_TIME: &str =
    "SELECT send_time FROM outgoing_messages WHERE epoch_guid = ? ORDER BY send_time DESC LIMIT 1";
const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

enum StoreRequest {
    StoreOutgoing(Arc<String>, u32, Instant, Arc<MsgBuf>),
    #[allow(clippy::type_complexity)]
    GetPrevMessages(
        Arc<String>,
        u32,
        u32,
        u32,
        oneshot::Sender<Result<Vec<(u32, Vec<u8>)>>>,
    ),
    GetSequences(Arc<String>, oneshot::Sender<Result<(u32, u32)>>),
    SetSequences(Arc<String>, u32, u32, oneshot::Sender<Result<()>>),   
    LastSendTime(Arc<String>, oneshot::Sender<Result<Option<DateTime<Utc>>>>),
    Disconnect(oneshot::Sender<Result<()>>),
}

pub struct Store {
    sender: mpsc::UnboundedSender<StoreRequest>,
}

impl Store {
    pub async fn build(settings: &SessionSettings) -> Result<Store> {
        let conn = Connection::open_with_flags(settings.store_path.clone(), OpenFlags::default()).await?;
        let epoch = settings.epoch.clone();
        setup(&conn, epoch).await?;
        let (sender, mut receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let begin_time = Utc::now(); 
            let begin_instant = Instant::now(); 
            while let Some(req) = receiver.recv().await {
                match req {
                    StoreRequest::StoreOutgoing(epoch, msg_seq_num, send_instant, msg) => {
                        let send_time = match Duration::from_std(send_instant.duration_since(begin_instant)) {
                            Ok(d) => begin_time + d, 
                            Err(_) => Utc::now(),
                        };
                        if store_outgoing(&conn, epoch, msg_seq_num, send_time, msg)
                            .await
                            .is_err()
                        {
                            eprintln!("error storing outgoing messages");
                        }
                    }
                    StoreRequest::GetPrevMessages(epoch, begin, end, last, sender) => {
                        let resp = get_prev_messages(&conn, epoch, begin, end, last).await;
                        let _ = sender.send(resp);
                    }
                    StoreRequest::GetSequences(epoch, sender) => {
                        let resp = get_sequences(&conn, epoch).await;
                        let _ = sender.send(resp);
                    }
                    StoreRequest::SetSequences(epoch, outgoing, incoming, sender) => {
                        let resp = set_sequences(&conn, epoch, outgoing, incoming).await;
                        let _ = sender.send(resp);
                    }
                    StoreRequest::LastSendTime(epoch, sender) => {
                        let resp = last_send_time(&conn, epoch).await; 
                        let _ = sender.send(resp); 
                    }
                    StoreRequest::Disconnect(sender) => {
                        let resp = vacuum(&conn).await;
                        let _ = sender.send(resp);
                        drop(conn);
                        break;
                    }
                }
            }
        });

        Ok(Store { sender })
    }

    pub fn store_outgoing(
        &self,
        epoch: Arc<String>,
        msg_seq_num: u32,
        send_instant: Instant, 
        msg: Arc<MsgBuf>,
    ) -> Result<()> {
        let req = StoreRequest::StoreOutgoing(epoch, msg_seq_num, send_instant, msg);
        self.sender.send(req)?;
        Ok(())
    }

    pub async fn get_sequences(&self, epoch: Arc<String>) -> Result<(u32, u32)> {
        let (sender, receiver) = oneshot::channel();
        let req = StoreRequest::GetSequences(epoch, sender);
        self.sender.send(req)?;
        receiver.await?
    }

    pub async fn get_prev_messages(
        &self,
        epoch: Arc<String>,
        begin: u32,
        end: u32,
        last: u32,
    ) -> Result<Vec<(u32, Vec<u8>)>> {
        let (sender, receiver) = oneshot::channel();
        let req = StoreRequest::GetPrevMessages(epoch, begin, end, last, sender);
        self.sender.send(req)?;
        receiver.await?
    }

    pub async fn set_sequences(
        &self,
        epoch: Arc<String>,
        next_outgoing: u32,
        next_incoming: u32,
    ) -> Result<()> {
        let (sender, receiver) = oneshot::channel();
        let req = StoreRequest::SetSequences(epoch, next_outgoing, next_incoming, sender);
        self.sender.send(req)?;
        let _ = receiver.await?;
        Ok(())
    }

    pub async fn last_send_time(
        &self,
        epoch: Arc<String>, 
    ) -> Result<Option<DateTime<Utc>>> {
        let (sender, receiver) = oneshot::channel(); 
        let req = StoreRequest::LastSendTime(epoch, sender); 
        self.sender.send(req)?;
        receiver.await?
    }

    pub async fn disconnect(&self) -> Result<()> {
        let (sender, receiver) = oneshot::channel();
        let req = StoreRequest::Disconnect(sender);
        self.sender.send(req)?;
        let _ = receiver.await?;
        Ok(())
    }
}

async fn setup(conn: &tokio_rusqlite::Connection, epoch: Arc<String>) -> Result<(u32, u32)> {
    conn.call(move |conn| {
        conn.query_row(SQL_ENTER_WAL_MODE, (), |_| Ok(()))?;
        conn.execute(SQL_CREATE_SEQUENCES, ())?;
        conn.execute(SQL_ENSURE_SEQUENCE_ROW, (Arc::clone(&epoch),))?;
        conn.execute(SQL_CREATE_INCOMING_TABLE, ())?;
        conn.execute(SQL_CREATE_OUTGOING_TABLE, ())?;

        conn.query_row(
            "SELECT next_incoming, next_outgoing FROM sequences where epoch_guid = ?;",
            (Arc::clone(&epoch),),
            |r| {
                let next_incoming: u32 = r.get(0)?;
                let next_outgoing: u32 = r.get(1)?;
                Ok((next_incoming, next_outgoing))
            },
        )
    })
    .await
    .map_err(|err| err.into())
}

async fn vacuum(conn: &tokio_rusqlite::Connection) -> Result<()> {
    conn.call(move |conn| {
        conn.execute(SQL_VACUUM, [])
    })
    .await
    .map(|_| ())
    .map_err(|e| e.into())
}

async fn get_sequences(
    conn: &tokio_rusqlite::Connection,
    epoch: Arc<String>,
) -> Result<(u32, u32)> {
    conn.call(move |conn| {
        conn.query_row(
            "SELECT next_incoming, next_outgoing FROM sequences where epoch_guid = ?;",
            (Arc::clone(&epoch),),
            |r| {
                let next_incoming: u32 = r.get(0)?;
                let next_outgoing: u32 = r.get(1)?;
                Ok((next_incoming, next_outgoing))
            },
        )
    })
    .await
    .map_err(|err| err.into())
}

async fn set_sequences(
    conn: &tokio_rusqlite::Connection,
    epoch: Arc<String>,
    new_outgoing: u32,
    new_incoming: u32,
) -> Result<()> {
    conn.call(move |conn| {
        conn.execute(
            "UPDATE sequences SET next_outgoing = ?1, next_incoming = ?2 WHERE epoch_guid = ?3",
            (new_outgoing, new_incoming, Arc::clone(&epoch)),
        )
    })
    .await
    .map(|_| ())
    .map_err(|err| err.into())
}

async fn store_outgoing(
    conn: &tokio_rusqlite::Connection,
    epoch: Arc<String>,
    msg_seq_num: u32,
    send_time: DateTime<Utc>,
    msg: Arc<MsgBuf>,
) -> Result<()> {
    conn.call(move |conn| {
        conn.execute(
            SQL_INSERT_OUTGOING_MESSAGE,
            (epoch, msg_seq_num, format!("{}", send_time.format(TIME_FORMAT)), &msg.as_ref()[..]),
        )
    })
    .await
    .map(|_| ())
    .map_err(|err| err.into())
}

async fn get_prev_messages(
    conn: &tokio_rusqlite::Connection,
    epoch: Arc<String>,
    begin_seq_no: u32,
    end_seq_no: u32,
    last_seq_no: u32,
) -> Result<Vec<(u32, Vec<u8>)>> {
    let mut output: Vec<(u32, Vec<u8>)> = Vec::new();
    output = conn.call(move |conn| -> Result<Vec<(u32, Vec<u8>)>> {
        let mut stmt = conn.prepare("SELECT msg_seq_num, message FROM (SELECT * FROM outgoing_messages WHERE epoch_guid = ?1 ORDER BY key DESC LIMIT ?2) WHERE msg_seq_num BETWEEN ?3 AND ?4;")?;
        let rows = stmt.query_map(
            rusqlite::params![Arc::clone(&epoch), &last_seq_no, &begin_seq_no, &end_seq_no], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?;
        for row in rows {
            output.push(row?);
        }
        Ok(output)
    }).await?;
    Ok(output)
}

async fn last_send_time(
    conn: &tokio_rusqlite::Connection, 
    epoch: Arc<String>, 
) -> Result<Option<DateTime<Utc>>> {
    let send_time = conn.call(move |conn| -> rusqlite::Result<Option<NaiveDateTime>> {
        conn.query_row(
           SQL_LAST_SEND_TIME,
           [epoch],
           |row| row.get(0)
        )
        .optional()
    }).await?; 
    Ok(send_time.map(|n| n.and_utc()))
}
