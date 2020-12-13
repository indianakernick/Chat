use log::debug;
use warp::ws::Message;
use std::time::SystemTime;
use deadpool_postgres::Pool;
use serde::{Serialize, Deserialize};
use crate::error::{Error, DatabaseError};
use super::upgrade::{Sender, ConnectionMap};
use crate::database::{UserID, ChannelID, GroupID, create_message, recent_messages, valid_group_channel};

#[derive(Deserialize)]
#[serde(tag="type")]
enum ClientMessage {
    #[serde(rename="post message")]
    PostMessage { content: String, channel_id: ChannelID },
    #[serde(rename="request recent messages")]
    RequestRecentMessages { channel_id: ChannelID }
}

#[derive(Serialize)]
struct RecentMessage {
    timestamp: u64,
    author: UserID,
    content: String,
    channel_id: ChannelID
}

#[derive(Serialize)]
#[serde(tag="type")]
enum ServerMessage {
    #[serde(rename="error")]
    Error { message: String },
    #[serde(rename="message receipt")]
    MessageReceipt { timestamp: u64, channel_id: ChannelID },
    #[serde(rename="recent message")]
    RecentMessage(RecentMessage),
    // TODO: each one of the channel_ids on the recent messages is the same.
    #[serde(rename="recent message list")]
    RecentMessageList { messages: Vec<RecentMessage> }
}

fn as_timestamp(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub struct MessageHandler<'a> {
    pub conn_id: usize,
    pub user_id: UserID,
    pub group_id: GroupID,
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
            ClientMessage::PostMessage { content, channel_id } => {
                self.handle_post_message(content, channel_id).await?
            },
            ClientMessage::RequestRecentMessages { channel_id } => {
                self.handle_request_recent_messages(channel_id).await?
            }
        })
    }

    async fn handle_post_message(&self, content: String, channel_id: ChannelID) -> Result<(), DatabaseError> {
        let time = SystemTime::now();
        let timestamp = as_timestamp(time);

        // TODO: Is there something we can do about this latency?
        // We need to check that the channel_id is valid before we can continue.
        // Perhaps use a memory cache on top of the database...?
        if !valid_group_channel(self.pool.clone(), self.group_id, channel_id).await? {
            self.reply_error("Invalid channel_id".to_owned());
        }

        let echo_response = serde_json::to_string(&ServerMessage::MessageReceipt {
            timestamp,
            channel_id,
        }).unwrap();

        let peer_response = serde_json::to_string(&ServerMessage::RecentMessage(RecentMessage {
            timestamp,
            author: self.user_id,
            content: content.clone(),
            channel_id,
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

        create_message(self.pool.clone(), time, self.user_id, content, channel_id).await
    }

    async fn handle_request_recent_messages(&self, channel_id: ChannelID) -> Result<(), DatabaseError> {
        if !valid_group_channel(self.pool.clone(), self.group_id, channel_id).await? {
            self.reply_error("Invalid channel_id".to_owned());
        }

        let rows = recent_messages(self.pool.clone(), channel_id).await?;

        let response = serde_json::to_string(&ServerMessage::RecentMessageList {
            messages: rows.iter()
                .map(|row| RecentMessage {
                    timestamp: as_timestamp(row.get(0)),
                    author: row.get(1),
                    content: row.get(2),
                    channel_id
                })
                .collect()
        }).unwrap();

        self.reply_message(response);

        Ok(())
    }
}
