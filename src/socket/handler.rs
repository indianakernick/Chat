use log::error;
use warp::ws::Message;
use std::time::SystemTime;
use crate::database as db;
use serde::{Serialize, Deserialize};
use deadpool_postgres::{Pool, PoolError};
use super::upgrade::{ConnID, Sender, Group, Groups};

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
    #[serde(rename="rename channel")]
    RenameChannel { channel_id: db::ChannelID, name: String },
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

// TODO: For naming enums, it might be better to set the naming convention
// using #[serde(rename_all="snake_case")] or to just use the Rust names in
// JavaScript.

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
    #[serde(rename="channel renamed")]
    ChannelRenamed { channel_id: db::ChannelID, name: &'a String },
    // Might remove OnlineUserList and include this information in the HTML
    // bundle or fetch the UserList on startup.
    #[serde(rename="online user list")]
    OnlineUserList { users: Vec<db::UserID> },
    #[serde(rename="user list")]
    UserList { users: Vec<User> },
    // Perhaps include the user's name and picture in this too
    #[serde(rename="user status changed")]
    UserStatusChanged { user_id: db::UserID, status: UserStatus },
}

fn as_timestamp(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

fn send_message(ch_tx: &Sender, message: String) {
    if ch_tx.send(Ok(Message::text(message))).is_err() {
        // the connection handler will handle the possible error
    }
}

impl Group {
    fn find_channel(&self, channel_id: db::ChannelID) -> usize {
        match self.channels.binary_search_by(|ch| ch.channel_id.cmp(&channel_id)) {
            Ok(i) => i,
            Err(_) => usize::MAX
        }
    }

    fn contains_channel(&self, channel_id: db::ChannelID) -> bool {
        self.find_channel(channel_id) != usize::MAX
    }

    /// Send a message to all connections.
    fn send_all(&self, message: ServerMessage) {
        let response = serde_json::to_string(&message).unwrap();
        for (_, ch_tx) in self.connections.iter() {
            send_message(ch_tx, response.clone());
        }
    }

    /// Send a peer message to all connections but the current connection.
    /// Send a reply message to the current connection.
    fn send_peer_reply(&self, conn_id: ConnID, peer: ServerMessage, reply: ServerMessage) {
        let peer_response = serde_json::to_string(&peer).unwrap();
        let reply_response = serde_json::to_string(&reply).unwrap();
        for (&other_conn_id, ch_tx) in self.connections.iter() {
            if other_conn_id == conn_id {
                send_message(ch_tx, reply_response.clone());
            } else {
                send_message(ch_tx, peer_response.clone());
            }
        }
    }

    /// Send a reply message to the current connection.
    fn send_reply(&self, conn_id: ConnID, message: ServerMessage) {
        let sender = &self.connections[&conn_id];
        send_message(sender, serde_json::to_string(&message).unwrap());
    }

    /// Send a reply error to the current connection
    fn send_reply_error(&self, conn_id: ConnID, error: &'static str) {
        self.send_reply(conn_id, ServerMessage::Error {
            message: error
        });
    }

    fn send_user_status(&self, user_id: db::UserID, status: UserStatus) {
        self.send_all(ServerMessage::UserStatusChanged {
            user_id,
            status
        });
    }

    pub fn send_user_online(&self, user_id: db::UserID) {
        self.send_user_status(user_id, UserStatus::Online);
    }

    pub fn send_user_offline(&self, user_id: db::UserID) {
        self.send_user_status(user_id, UserStatus::Offline);
    }
}

pub struct MessageContext<'a> {
    pub user_id: db::UserID,
    pub group_id: db::GroupID,
    pub conn_id: ConnID,
    pub groups: &'a Groups,
    pub pool: &'a Pool,
}

impl<'a> MessageContext<'a> {
    pub async fn handle(&self, message: Message) {
        let message = match message.to_str() {
            Ok(m) => m,
            Err(_) => return,
        };

        let client_message = match serde_json::from_str::<ClientMessage>(message) {
            Ok(m) => m,
            Err(e) => {
                error!("{}", e);
                let group = &self.groups.read().await[&self.group_id];
                group.send_reply_error(self.conn_id, "JSON");
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
            ClientMessage::RenameChannel { channel_id, name } => {
                self.rename_channel(channel_id, name).await
            },
        };

        if let Err(e) = result {
            error!("{}", e);
            let group = &self.groups.read().await[&self.group_id];
            group.send_reply_error(self.conn_id, "Database");
        }
    }

    async fn post_message(&self, content: String, channel_id: db::ChannelID)
        -> Result<(), PoolError>
    {
        let time = SystemTime::now();
        let timestamp = as_timestamp(time);

        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.group_id];

        if !group.contains_channel(channel_id) {
            group.send_reply_error(self.conn_id, "Invalid channel_id");
            return Ok(());
        }

        let peer = ServerMessage::RecentMessage(RecentMessage {
            timestamp,
            author: self.user_id,
            content: content.clone(),
            channel_id,
        });

        let echo = ServerMessage::MessageReceipt {
            timestamp,
            channel_id,
        };

        group.send_peer_reply(self.conn_id, peer, echo);

        db::create_message(self.pool.clone(), time, self.user_id, content, channel_id).await
    }

    async fn request_recent_messages(&self, channel_id: db::ChannelID)
        -> Result<(), PoolError>
    {
        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.group_id];

        if !group.contains_channel(channel_id) {
            group.send_reply_error(self.conn_id, "Invalid channel_id");
            return Ok(());
        }

        let rows = db::recent_messages(self.pool.clone(), channel_id).await?;

        group.send_reply(self.conn_id, ServerMessage::RecentMessageList {
            channel_id,
            messages: rows.iter()
                .map(|row| GenericRecentMessage {
                    timestamp: as_timestamp(row.get(0)),
                    author: row.get(1),
                    content: row.get(2)
                })
                .collect()
        });

        Ok(())
    }

    async fn create_channel(&self, name: String) -> Result<(), PoolError> {
        let mut groups_guard = self.groups.write().await;
        let group = &mut groups_guard.get_mut(&self.group_id).unwrap();

        if !db::valid_channel_name(&name) {
            // This shouldn't happen unless someone is bypassing the JavaScript
            // validation.
            group.send_reply_error(self.conn_id, "Channel name invalid");
            return Ok(());
        }

        let channel_id = match db::create_channel(self.pool.clone(), self.group_id, &name).await? {
            Some(id) => id,
            None => {
                group.send_reply_error(self.conn_id, "Channel name exists");
                return Ok(());
            }
        };

        group.send_all(ServerMessage::ChannelCreated {
            channel_id,
            name: &name,
        });

        group.channels.push(db::Channel {
            channel_id,
            name
        });

        Ok(())
    }

    async fn request_channels(&self) -> Result<(), PoolError> {
        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.group_id];

        group.send_reply(self.conn_id, ServerMessage::ChannelList {
            channels: &group.channels
        });

        Ok(())
    }

    async fn delete_channel(&self, channel_id: db::ChannelID) -> Result<(), PoolError> {
        let mut groups_guard = self.groups.write().await;
        let group = &mut groups_guard.get_mut(&self.group_id).unwrap();

        if group.channels.len() == 1 {
            group.send_reply_error(self.conn_id, "Cannot delete lone channel");
            return Ok(());
        }

        let channel_index = group.find_channel(channel_id);
        if channel_index == usize::MAX {
            group.send_reply_error(self.conn_id, "Channel not in group");
            return Ok(());
        }

        if !db::delete_channel(self.pool.clone(), channel_id).await? {
            // If the above checks pass then this cannot happen
            group.send_reply_error(self.conn_id, "Channel already deleted");
            return Ok(());
        }

        group.channels.remove(channel_index);

        group.send_all(ServerMessage::ChannelDeleted {
            channel_id
        });

        Ok(())
    }

    async fn request_online(&self) -> Result<(), PoolError> {
        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.group_id];

        let users = group.online_users.iter()
            .map(|(user_id, _)| user_id)
            .cloned()
            .collect();
        group.send_reply(self.conn_id, ServerMessage::OnlineUserList {
            users
        });

        Ok(())
    }

    async fn request_users(&self) -> Result<(), PoolError> {
        let groups_guard = self.groups.read().await;
        let group = &groups_guard[&self.group_id];

        let group_users = db::group_users(self.pool.clone(), self.group_id).await?;
        let mut users = Vec::new();

        for user in group_users.iter() {
            let status = if group.online_users.contains_key(&user.user_id) {
                UserStatus::Online
            } else {
                UserStatus::Offline
            };
            users.push(User {
                user_id: user.user_id,
                name: user.name.clone(),
                picture: user.picture.clone(),
                status
            });
        }

        group.send_reply(self.conn_id, ServerMessage::UserList {
            users
        });

        Ok(())
    }

    // Perhaps a dedicated message for the status of operations.
    // Sort of like message receipt.
    // A message that indicates whether renaming a channel succeeded
    // or failed and why it failed.
    // If someone else deletes a channel, we need to close the window.
    // So it might be better to have an error message type for each operation.
    //
    async fn rename_channel(&self, channel_id: db::ChannelID, name: String) -> Result<(), PoolError> {
        let mut groups_guard = self.groups.write().await;
        let group = &mut groups_guard.get_mut(&self.group_id).unwrap();

        if !db::valid_channel_name(&name) {
            // This shouldn't happen unless someone is bypassing the JavaScript
            // validation.
            group.send_reply_error(self.conn_id, "rename channel");
            return Ok(());
        }

        let channel_index = group.find_channel(channel_id);
        if channel_index == usize::MAX {
            group.send_reply_error(self.conn_id, "rename channel");
            return Ok(());
        }

        if !db::rename_channel(self.pool.clone(), self.group_id, channel_id, &name).await? {
            group.send_reply_error(self.conn_id, "rename channel");
            return Ok(());
        }

        group.send_all(ServerMessage::ChannelRenamed {
            channel_id,
            name: &name,
        });

        group.channels[channel_index].name = name;

        Ok(())
    }
}
