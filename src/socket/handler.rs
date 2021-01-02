use log::error;
use warp::ws::Message;
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
    #[serde(rename="delete channel")]
    DeleteChannel { channel_id: db::ChannelID },
    #[serde(rename="request online")]
    RequestOnline,
    #[serde(rename="request users")]
    RequestUsers,
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
enum UserStatus {
    #[serde(rename="online")]
    Online,
    #[serde(rename="offline")]
    Offline,
    // #[serde(rename="left")]
    // Left,
}

#[derive(Serialize)]
struct User {
    user_id: db::UserID,
    name: String,
    picture: String,
    status: UserStatus,
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
    ChannelList { channels: &'a Vec<db::Channel> },
    #[serde(rename="channel deleted")]
    ChannelDeleted { channel_id: db::ChannelID },
    // Might remove OnlineUserList and include this information in the HTML
    // bundle or fetch the UserList on startup.
    #[serde(rename="online user list")]
    OnlineUserList { users: Vec<db::UserID> },
    #[serde(rename="user list")]
    UserList { users: Vec<User> },
    // Perhaps include the user's name and picture in this too
    #[serde(rename="user status changed")]
    UserStatusChanged { user_id: db::UserID, status: UserStatus }
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

fn find_channel(channels: &Vec<db::Channel>, channel_id: db::ChannelID) -> usize {
    for i in 0..channels.len() {
        if channels[i].channel_id == channel_id {
            return i;
        }
    }
    return usize::MAX;
}

fn contains_channel(channels: &Vec<db::Channel>, channel_id: db::ChannelID) -> bool {
    find_channel(channels, channel_id) != usize::MAX
}

fn send_to_all(group: &Group, response: String) {
    for (_, (ch_tx, _)) in group.users.iter() {
        send_message(ch_tx, response.clone());
    }
}

fn send_user_status(group: &Group, user_id: db::UserID, status: UserStatus) {
    send_to_all(group, serde_json::to_string(&ServerMessage::UserStatusChanged {
        user_id,
        status
    }).unwrap())
}

pub fn send_user_online(group: &Group, user_id: db::UserID) {
    send_user_status(group, user_id, UserStatus::Online);
}

pub fn send_user_offline(group: &Group, user_id: db::UserID) {
    send_user_status(group, user_id, UserStatus::Offline);
}

impl<'a> MessageContext<'a> {
    fn reply_error(&self, group: &Group, error: &'static str) {
        send_error(&group.users[&self.ctx.conn_id].0, error);
    }

    fn reply_message(&self, group: &Group, message: String) {
        send_message(&group.users[&self.ctx.conn_id].0, message);
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
            },
            ClientMessage::DeleteChannel { channel_id } => {
                self.delete_channel(channel_id).await
            },
            ClientMessage::RequestOnline => {
                self.request_online().await
            },
            ClientMessage::RequestUsers => {
                self.request_users().await
            },
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

        for (&other_conn_id, (ch_tx, _)) in group.users.iter() {
            if other_conn_id == self.ctx.conn_id {
                send_message(ch_tx, echo_response.clone());
            } else {
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

        send_to_all(group, response);

        Ok(())
    }

    async fn request_channels(&self) -> Result<(), PoolError> {
        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.ctx.group_id];

        self.reply_message(group, serde_json::to_string(&ServerMessage::ChannelList {
            channels: &group.channels
        }).unwrap());

        Ok(())
    }

    async fn delete_channel(&self, channel_id: db::ChannelID) -> Result<(), PoolError> {
        let mut groups_guard = self.groups.write().await;
        let group = &mut groups_guard.get_mut(&self.ctx.group_id).unwrap();

        if group.channels.len() == 1 {
            self.reply_error(group, "Cannot delete lone channel");
            return Ok(());
        }

        let channel_index = find_channel(&group.channels, channel_id);
        if channel_index == usize::MAX {
            self.reply_error(group, "Channel not in group");
            return Ok(());
        }

        if !db::delete_channel(self.pool.clone(), channel_id).await? {
            // If the above checks pass then this cannot happen
            self.reply_error(group, "Channel already deleted");
            return Ok(());
        }

        group.channels.remove(channel_index);

        send_to_all(group, serde_json::to_string(&ServerMessage::ChannelDeleted {
            channel_id
        }).unwrap());

        Ok(())
    }

    async fn request_online(&self) -> Result<(), PoolError> {
        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.ctx.group_id];

        let users = group.users.iter().map(|(_, (_, user_id))| user_id).cloned().collect();
        self.reply_message(group, serde_json::to_string(&ServerMessage::OnlineUserList {
            users
        }).unwrap());

        Ok(())
    }

    async fn request_users(&self) -> Result<(), PoolError> {
        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.ctx.group_id];

        let group_users = db::group_users(self.pool.clone(), self.ctx.group_id).await?;
        let mut users = Vec::new();

        for user in group_users.iter() {
            // TODO: Need a different data structure for online user IDs
            // Maybe we could keep their online status in the database...?
            // How should we handle the same user logging in on multiple clients?

            let mut status = UserStatus::Offline;
            for (_, (_, user_id)) in group.users.iter() {
                if user.user_id == *user_id {
                    status = UserStatus::Online;
                    break;
                }
            }
            users.push(User {
                user_id: user.user_id,
                name: user.name.clone(),
                picture: user.picture.clone(),
                status
            });
        }

        self.reply_message(group, serde_json::to_string(&ServerMessage::UserList {
            users
        }).unwrap());

        Ok(())
    }
}
