use crate::fix::mem::MsgBuf;
use crate::SessionSettings;
use anyhow::Result;
use chrono::offset::Utc;
use chrono::DateTime;

use std::sync::{Arc, Mutex};
use std::time::Instant;

struct Db {
    outgoing_messages: Vec<(Instant, u32, Arc<MsgBuf>)>,
    next_outgoing: u32,
    next_incoming: u32,
}

impl Db {
    fn new() -> Self {
        Self {
            outgoing_messages: Vec::new(),
            next_outgoing: 1,
            next_incoming: 1,
        }
    }
}

pub struct Store {
    db: Mutex<Db>,
    begin_time: DateTime<Utc>,
    begin_instant: Instant,
}

impl Store {
    pub fn build(_settings: &SessionSettings) -> Result<Store> {
        Ok(Self {
            db: Mutex::new(Db::new()),
            begin_instant: Instant::now(),
            begin_time: Utc::now(),
        })
    }

    pub fn store_outgoing(
        &self,
        _epoch: Arc<String>,
        msg_seq_num: u32,
        send_instant: Instant,
        msg: Arc<MsgBuf>,
    ) -> Result<()> {
        self.db
            .lock()
            .unwrap()
            .outgoing_messages
            .push((send_instant, msg_seq_num, msg));
        Ok(())
    }

    pub async fn get_sequences(&self, _epoch: Arc<String>) -> Result<(u32, u32)> {
        let db = self.db.lock().unwrap();
        Ok((db.next_incoming, db.next_outgoing))
    }

    pub async fn get_prev_messages(
        &self,
        _epoch: Arc<String>,
        begin: u32,
        end: u32,
        _last: u32,
    ) -> Result<Vec<(u32, Vec<u8>)>> {
        let mut prev_messages: Vec<_> = {
            let db = self.db.lock().unwrap();
            db.outgoing_messages
                .iter()
                .filter(|(_, sequence, _)| (begin..=end).contains(sequence))
                .map(|(_, sequence, msg)| (*sequence, msg.0.clone()))
                .collect()
        };

        // note, comparison is reversed to get descending order
        prev_messages.sort_by(|(seq1, _), (seq2, _)| seq2.cmp(seq1));
        Ok(prev_messages)
    }

    pub async fn set_sequences(
        &self,
        _epoch: Arc<String>,
        next_outgoing: u32,
        next_incoming: u32,
    ) -> Result<()> {
        let mut db = self.db.lock().unwrap();
        db.next_outgoing = next_outgoing;
        db.next_incoming = next_incoming;
        Ok(())
    }

    pub async fn last_send_time(&self, _epoch: Arc<String>) -> Result<Option<DateTime<Utc>>> {
        Ok(self
            .db
            .lock()
            .unwrap()
            .outgoing_messages
            .last()
            .map(|(send_instant, _, _)| {
                let since_begin = send_instant.duration_since(self.begin_instant);
                self.begin_time + since_begin
            }))
    }

    pub async fn disconnect(&self) -> Result<()> {
        Ok(())
    }
}
