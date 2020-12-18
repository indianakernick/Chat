use warp::ws::Message;
use log::{error, debug};
use std::time::SystemTime;
use crate::database as db;
use serde::{Serialize, Deserialize};
use deadpool_postgres::{Pool, PoolError};
use super::upgrade::{Sender, Group, Groups, ConnectionContext};

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
#[serde(tag="type")]
enum ServerMessage<'a> {
    #[serde(rename="error")]
    Error { message: &'static str },
    #[serde(rename="message receipt")]
    MessageReceipt { timestamp: u64, channel_id: db::ChannelID },
    #[serde(rename="recent message")]
    RecentMessage(RecentMessage),
    #[serde(rename="recent message list")]
    RecentMessageList { channel_id: db::ChannelID, messages: Vec<GenericRecentMessage> },
    #[serde(rename="channel created")]
    ChannelCreated { channel_id: db::ChannelID, name: &'a String },
    #[serde(rename="channel list")]
    ChannelList { channels: &'a Vec<db::Channel> }
}

fn as_timestamp(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub struct MessageContext<'a> {
    pub ctx: &'a ConnectionContext,
    pub groups: &'a Groups,
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

fn contains_channel(channels: &Vec<db::Channel>, channel_id: db::ChannelID) -> bool {
    for channel in channels.iter() {
        if channel.channel_id == channel_id {
            return true;
        }
    }
    return false;
}

impl<'a> MessageContext<'a> {
    fn reply_error(&self, group: &Group, error: &'static str) {
        send_error(&group.users[&self.ctx.conn_id], error);
    }

    fn reply_message(&self, group: &Group, message: String) {
        send_message(&group.users[&self.ctx.conn_id], message);
    }

    pub async fn handle(self) {
        let message = match self.message.to_str() {
            Ok(m) => m,
            Err(_) => return,
        };

        if message == "a" {
            let group = &self.groups.read().await[&self.ctx.group_id];
            self.reply_message(group, String::from("b"));
            return;
        }

        let client_message = match serde_json::from_str::<ClientMessage>(message) {
            Ok(m) => m,
            Err(e) => {
                error!("{}", e);
                let group = &self.groups.read().await[&self.ctx.group_id];
                self.reply_error(group, "JSON");
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
            let group = &self.groups.read().await[&self.ctx.group_id];
            self.reply_error(group, "Database");
        }
    }

    async fn post_message(&self, content: String, channel_id: db::ChannelID)
        -> Result<(), PoolError>
    {
        let time = SystemTime::now();
        let timestamp = as_timestamp(time);

        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.ctx.group_id];

        if !contains_channel(&group.channels, channel_id) {
            self.reply_error(group, "Invalid channel_id");
            return Ok(());
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

        for (&other_conn_id, ch_tx) in group.users.iter() {
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
        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.ctx.group_id];

        if !contains_channel(&group.channels, channel_id) {
            self.reply_error(group, "Invalid channel_id");
            return Ok(());
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

        self.reply_message(group, response);

        Ok(())
    }

    async fn create_channel(&self, name: String) -> Result<(), PoolError> {
        let mut groups_guard = self.groups.write().await;
        let group = &mut groups_guard.get_mut(&self.ctx.group_id).unwrap();

        if !db::valid_channel_name(&name) {
            // This shouldn't happen unless someone is bypassing the JavaScript
            // validation.
            self.reply_error(group, "Channel name invalid");
            return Ok(());
        }

        let channel_id = match db::create_channel(self.pool.clone(), self.ctx.group_id, &name).await? {
            Some(id) => id,
            None => {
                // JavaScript can check for this but the JavaScript can be wrong
                // if two people try to create channels at the same time.
                self.reply_error(group, "Channel name exists");
                return Ok(());
            }
        };

        let response = serde_json::to_string(&ServerMessage::ChannelCreated {
            channel_id,
            name: &name,
        }).unwrap();

        group.channels.push(db::Channel {
            channel_id,
            name
        });

        for (_, ch_tx) in group.users.iter() {
            send_message(ch_tx, response.clone());
        }

        Ok(())
    }

    async fn request_channels(&self) -> Result<(), PoolError> {
        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.ctx.group_id];

        let response = serde_json::to_string(&ServerMessage::ChannelList {
            channels: &group.channels
        }).unwrap();

        self.reply_message(group, response);

        Ok(())
    }
}
