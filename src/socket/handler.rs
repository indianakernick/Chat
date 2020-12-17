use warp::ws::Message;
use log::{error, debug};
use std::time::SystemTime;
use crate::database as db;
use serde::{Serialize, Deserialize};
use deadpool_postgres::{Pool, PoolError};
use super::upgrade::{Sender, Group, ConnectionContext};

#[derive(Deserialize)]
#[serde(tag="type")]
enum ClientMessage {
    #[serde(rename="post message")]
    PostMessage { content: String, channel_id: db::ChannelID },
    #[serde(rename="request recent messages")]
    RequestRecentMessages { channel_id: db::ChannelID },
    #[serde(rename="create channel")]
    CreateChannel { name: String },
    #[serde(rename="request channels")]
    RequestChannels,
}

#[derive(Serialize)]
struct RecentMessage {
    timestamp: u64,
    author: db::UserID,
    content: String,
    channel_id: db::ChannelID,
}

#[derive(Serialize)]
struct GenericRecentMessage {
    timestamp: u64,
    author: db::UserID,
    content: String,
}

#[derive(Serialize)]
struct Channel {
    channel_id: db::ChannelID,
    name: String,
}

#[derive(Serialize)]
#[serde(tag="type")]
enum ServerMessage {
    #[serde(rename="error")]
    Error { message: &'static str },
    #[serde(rename="message receipt")]
    MessageReceipt { timestamp: u64, channel_id: db::ChannelID },
    #[serde(rename="recent message")]
    RecentMessage(RecentMessage),
    #[serde(rename="recent message list")]
    RecentMessageList { channel_id: db::ChannelID, messages: Vec<GenericRecentMessage> },
    #[serde(rename="channel created")]
    ChannelCreated { channel_id: db::ChannelID, name: String },
    #[serde(rename="channel list")]
    ChannelList { channels: Vec<Channel> }
}

fn as_timestamp(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub struct MessageContext<'a> {
    pub ctx: &'a ConnectionContext,
    pub group: &'a Group,
    pub pool: &'a Pool,
    pub message: Message,
}

fn send_message(ch_tx: &Sender, message: String) {
    if ch_tx.send(Ok(Message::text(message))).is_err() {
        // the connection handler will handle the possible error
    }
}

fn send_error(ch_tx: &Sender, error: &'static str) {
    let response = serde_json::to_string(&ServerMessage::Error {
        message: error
    }).unwrap();
    send_message(ch_tx, response);
}

impl<'a> MessageContext<'a> {
    fn reply_error(&self, error: &'static str) {
        send_error(&self.group.users[&self.ctx.conn_id], error);
    }

    fn reply_message(&self, message: String) {
        send_message(&self.group.users[&self.ctx.conn_id], message);
    }

    pub async fn handle(self) {
        let message = match self.message.to_str() {
            Ok(m) => m,
            Err(_) => return,
        };

        let client_message = match serde_json::from_str::<ClientMessage>(message) {
            Ok(m) => m,
            Err(e) => {
                error!("{}", e);
                self.reply_error("JSON");
                return;
            }
        };

        let result = match client_message {
            ClientMessage::PostMessage { content, channel_id } => {
                self.post_message(content, channel_id).await
            },
            ClientMessage::RequestRecentMessages { channel_id } => {
                self.request_recent_messages(channel_id).await
            },
            ClientMessage::CreateChannel { name } => {
                self.create_channel(name).await
            },
            ClientMessage::RequestChannels => {
                self.request_channels().await
            }
        };

        if let Err(e) = result {
            error!("{}", e);
            self.reply_error("Database");
        }
    }

    async fn post_message(&self, content: String, channel_id: db::ChannelID)
        -> Result<(), PoolError>
    {
        let time = SystemTime::now();
        let timestamp = as_timestamp(time);

        if !self.group.channels.contains(&channel_id) {
            self.reply_error("Invalid channel_id");
        }

        let echo_response = serde_json::to_string(&ServerMessage::MessageReceipt {
            timestamp,
            channel_id,
        }).unwrap();

        let peer_response = serde_json::to_string(&ServerMessage::RecentMessage(RecentMessage {
            timestamp,
            author: self.ctx.user_id,
            content: content.clone(),
            channel_id,
        })).unwrap();

        for (&other_conn_id, ch_tx) in self.group.users.iter() {
            if other_conn_id == self.ctx.conn_id {
                debug!("Echoing back to ({}): {}", self.ctx.conn_id, echo_response);
                send_message(ch_tx, echo_response.clone());
            } else {
                debug!(
                    "Forwarding message from ({}) to ({}): {}",
                    self.ctx.conn_id, other_conn_id, peer_response
                );
                send_message(ch_tx, peer_response.clone());
            }
        }

        db::create_message(self.pool.clone(), time, self.ctx.user_id, content, channel_id).await
    }

    async fn request_recent_messages(&self, channel_id: db::ChannelID)
        -> Result<(), PoolError>
    {
        if !self.group.channels.contains(&channel_id) {
            self.reply_error("Invalid channel_id");
        }

        let rows = db::recent_messages(self.pool.clone(), channel_id).await?;

        let response = serde_json::to_string(&ServerMessage::RecentMessageList {
            channel_id,
            messages: rows.iter()
                .map(|row| GenericRecentMessage {
                    timestamp: as_timestamp(row.get(0)),
                    author: row.get(1),
                    content: row.get(2)
                })
                .collect()
        }).unwrap();

        self.reply_message(response);

        Ok(())
    }

    async fn create_channel(&self, name: String) -> Result<(), PoolError> {
        let channel_id = db::create_channel(self.pool.clone(), self.ctx.group_id, &name).await?;

        // TODO: Update self.group.channels
        // self.group.channels.push(channel_id);

        let response = serde_json::to_string(&ServerMessage::ChannelCreated {
            channel_id,
            name
        }).unwrap();

        for (_, ch_tx) in self.group.users.iter() {
            send_message(ch_tx, response.clone());
        }

        Ok(())
    }

    async fn request_channels(&self) -> Result<(), PoolError> {
        // TODO: Read this from self.group.channels

        let channels = db::group_channels(self.pool.clone(), self.ctx.group_id)
            .await
            .unwrap() // Unwrapping because of different error types
            .iter() // this is temporary so it doesn't matter
            .map(|row| Channel {
                channel_id: row.get(0),
                name: row.get(1),
            })
            .collect();

        let response = serde_json::to_string(&ServerMessage::ChannelList {
            channels
        }).unwrap();

        self.reply_message(response);

        Ok(())
    }
}
