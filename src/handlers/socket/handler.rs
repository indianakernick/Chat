use log::debug;
use warp::ws::Message;
use std::time::SystemTime;
use deadpool_postgres::Pool;
use serde::{Serialize, Deserialize};
use crate::handlers::{UserID, ChannelID};
use crate::error::{Error, DatabaseError};
use super::upgrade::{Sender, ConnectionMap};

#[derive(Deserialize)]
#[serde(tag="type")]
enum ClientMessage {
    #[serde(rename="post message")]
    PostMessage { content: String },
    #[serde(rename="request recent messages")]
    RequestRecentMessages
}

#[derive(Serialize)]
struct RecentMessage {
    timestamp: u64,
    author: i32,
    content: String
}

#[derive(Serialize)]
#[serde(tag="type")]
enum ServerMessage {
    #[serde(rename="error")]
    Error { message: String },
    #[serde(rename="message receipt")]
    MessageReceipt { timestamp: u64 },
    #[serde(rename="recent message")]
    RecentMessage(RecentMessage),
    #[serde(rename="recent message list")]
    RecentMessageList { messages: Vec<RecentMessage> }
}

fn as_timestamp(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub struct MessageHandler<'a> {
    pub conn_id: usize,
    pub user_id: UserID,
    pub chan_id: ChannelID,
    pub message: Message,
    pub conns: &'a ConnectionMap,
    pub pool: &'a Pool,
}

fn send_message(ch_tx: &Sender, message: String) {
    if ch_tx.send(Ok(Message::text(message))).is_err() {
        // the connection handler will handle the possible error
    }
}

fn send_error(ch_tx: &Sender, error: String) {
    let response = serde_json::to_string(&ServerMessage::Error {
        message: error
    }).unwrap();
    send_message(ch_tx, response);
}

impl<'a> MessageHandler<'a> {
    fn reply_error(&self, error: String) {
        if let Some(ch_tx) = self.conns.get(&self.conn_id) {
            send_error(ch_tx, error);
        }
    }

    fn reply_message(&self, message: String) {
        if let Some(ch_tx) = self.conns.get(&self.conn_id) {
            send_message(ch_tx, message);
        }
    }

    pub async fn handle(self) {
        if let Err(e) = self.handle_error().await {
            self.reply_error(e.to_string());
        }
    }

    async fn handle_error(&self) -> Result<(), Error> {
        let message = match self.message.to_str() {
            Ok(m) => m,
            Err(_) => return Ok(())
        };
        let client_message = serde_json::from_str::<ClientMessage>(message)?;

        Ok(match client_message {
            ClientMessage::PostMessage { content } => {
                self.handle_post_message(content).await?
            },
            ClientMessage::RequestRecentMessages => {
                self.handle_request_recent_messages().await?
            }
        })
    }

    async fn handle_post_message(&self, content: String) -> Result<(), DatabaseError> {
        let time = SystemTime::now();
        let timestamp = as_timestamp(time);

        let echo_response = serde_json::to_string(&ServerMessage::MessageReceipt {
            timestamp
        }).unwrap();

        let peer_response = serde_json::to_string(&ServerMessage::RecentMessage(RecentMessage {
            timestamp,
            author: self.user_id,
            content: content.clone(),
        })).unwrap();

        for (&other_conn_id, ch_tx) in self.conns.iter() {
            if other_conn_id == self.conn_id {
                debug!("Echoing back to ({}): {}", self.conn_id, echo_response);
                send_message(ch_tx, echo_response.clone());
            } else {
                debug!("Forwarding message from ({}) to ({}): {}", self.conn_id, other_conn_id, peer_response);
                send_message(ch_tx, peer_response.clone());
            }
        }

        let db_conn = self.pool.get().await?;
        let stmt = db_conn.prepare("
            INSERT INTO Message (timestamp, author, content, channel_id)
            VALUES ($1, $2, $3, $4)
        ").await?;
        db_conn.execute(&stmt, &[&time, &self.user_id, &content, &self.chan_id]).await?;

        Ok(())
    }

    async fn handle_request_recent_messages(&self) -> Result<(), DatabaseError> {
        let db_conn = self.pool.get().await?;
        let stmt = db_conn.prepare("
            SELECT timestamp, COALESCE(author, 0), content
            FROM Message
            WHERE channel_id = $1
        ").await?;
        let rows = db_conn.query(&stmt, &[&self.chan_id]).await?;

        let response = serde_json::to_string(&ServerMessage::RecentMessageList {
            messages: rows.iter()
                .map(|row| RecentMessage {
                    timestamp: as_timestamp(row.get(0)),
                    author: row.get(1),
                    content: row.get(2)
                })
                .collect()
        }).unwrap();

        self.reply_message(response);

        Ok(())
    }
}
