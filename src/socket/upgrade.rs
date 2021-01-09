use log::{debug, error};
use crate::error::Error;
use crate::database as db;
use deadpool_postgres::Pool;
use tokio::sync::{RwLock, mpsc};
use futures::{FutureExt, StreamExt};
use warp::ws::{Ws, WebSocket, Message};
use std::collections::hash_map::{HashMap, Entry};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

pub type ConnID = usize;
pub type AtomicConnID = AtomicUsize;
static NEXT_CONNECTION_ID: AtomicConnID = AtomicConnID::new(1);

pub type Sender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

struct ConnectionContext {
    user_id: db::UserID,
    group_id: db::GroupID,
    conn_id: ConnID,
}

pub struct Group {
    pub channels: Vec<db::Channel>,
    pub connections: HashMap<ConnID, Sender>,
    pub online_users: HashMap<db::UserID, Vec<ConnID>>,
}

pub type GroupMap = HashMap<db::GroupID, Group>;
pub type Groups = Arc<RwLock<GroupMap>>;
pub type UserGroupMap = HashMap<db::UserID, Vec<db::GroupID>>;
pub type UserGroups = Arc<RwLock<UserGroupMap>>;

impl Group {
    /// Create a new group and insert a connection
    async fn new(conn_ctx: &ConnectionContext, pool: Pool, ch_tx: Sender)
        -> Result<Self, Error>
    {
        let channels = db::group_channels(pool, conn_ctx.group_id).await?;
        let mut connections = HashMap::new();
        connections.insert(conn_ctx.conn_id, ch_tx);
        let mut online_users = HashMap::new();
        online_users.insert(conn_ctx.user_id, vec![conn_ctx.conn_id]);
        Ok(Self { channels, connections, online_users })
    }

    /// Insert a new connection into the group.
    /// Returns true if the user has one connection to the group.
    fn insert_connection(&mut self, conn_ctx: &ConnectionContext, ch_tx: Sender) -> bool {
        let conn_ids = self.online_users.entry(conn_ctx.user_id).or_default();
        conn_ids.push(conn_ctx.conn_id);
        let mut joined_group = false;
        if conn_ids.len() == 1 {
            self.send_user_online(conn_ctx.user_id);
            joined_group = true;
        }
        self.connections.insert(conn_ctx.conn_id, ch_tx);
        joined_group
    }

    /// Remove the current connection from the group.
    /// Returns true if the user has no connections to the group.
    fn remove_connection(&mut self, conn_ctx: &ConnectionContext) -> bool {
        self.connections.remove(&conn_ctx.conn_id);
        if self.connections.is_empty() {
            return true;
        }
        let mut user_entry = match self.online_users.entry(conn_ctx.user_id) {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(_) => panic!(),
        };
        let conn_ids = user_entry.get_mut();
        if conn_ids.len() == 1 {
            user_entry.remove();
            self.send_user_offline(conn_ctx.user_id);
            true
        } else {
            let index = conn_ids.iter().position(|id| *id == conn_ctx.conn_id).unwrap();
            conn_ids.swap_remove(index);
            false
        }
    }
}

#[derive(Clone)]
pub struct Context {
    pool: Pool,
    groups: Groups,
    user_groups: UserGroups,
}

impl Context {
    pub fn new(pool: Pool) -> Self {
        Self {
            pool,
            groups: Groups::default(),
            user_groups: UserGroups::default(),
        }
    }

    /// Insert a connection into the group map. Creates a new group if
    /// necessary, otherwise inserts into an existing group.
    async fn insert_connection(&self, conn_ctx: &ConnectionContext, ch_tx: Sender)
        -> Result<(), Error>
    {
        let joined_group;
        match self.groups.write().await.entry(conn_ctx.group_id) {
            Entry::Occupied(mut entry) => {
                joined_group = entry.get_mut().insert_connection(&conn_ctx, ch_tx);
            }
            Entry::Vacant(entry) => {
                entry.insert(Group::new(&conn_ctx, self.pool.clone(), ch_tx).await?);
                joined_group = true;
            }
        }
        if joined_group {
            match self.user_groups.write().await.entry(conn_ctx.user_id) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(conn_ctx.group_id);
                },
                Entry::Vacant(entry) => {
                    entry.insert(vec![conn_ctx.group_id]);
                }
            }
        }
        Ok(())
    }

    /// Remove a connection from the group map. Also removes the group if the
    /// group becomes empty.
    async fn remove_connection(&self, conn_ctx: &ConnectionContext) {
        let left_group;
        match self.groups.write().await.entry(conn_ctx.group_id) {
            Entry::Occupied(mut entry) => {
                if entry.get_mut().connections.len() == 1 {
                    entry.remove();
                    left_group = true;
                } else {
                    left_group = entry.get_mut().remove_connection(&conn_ctx);
                }
            },
            Entry::Vacant(_) => panic!()
        }
        if left_group {
            match self.user_groups.write().await.entry(conn_ctx.user_id) {
                Entry::Occupied(mut entry) => {
                    if entry.get_mut().len() == 1 {
                        entry.remove();
                    } else {
                        let pos = entry.get_mut().iter().position(|id| *id == conn_ctx.group_id).unwrap();
                        entry.get_mut().swap_remove(pos);
                    }
                },
                Entry::Vacant(_) => panic!()
            }
        }
    }

    pub async fn upgrade(group_id: db::GroupID, ws: Ws, session_id: db::SessionID, ctx: Self)
        -> Result<Box<dyn warp::Reply>, warp::Rejection>
    {
        // The JavaScript that invokes this is only loaded when the session cookie
        // is valid. The only way that this error could happen is if the session
        // expires between loading the page and running the JavaScript. Another
        // possibility is someone directly accessing this endpoint but failing to
        // provide the cookie.
        let user_id = match db::session_user_id(ctx.pool.clone(), &session_id).await? {
            Some(id) => id,
            None => return Ok(Box::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR))
        };

        // Can only happen if someone is directly accessing the socket.
        if !db::group_member(ctx.pool.clone(), user_id, group_id).await? {
            return Ok(Box::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR));
        }

        // Upgrade the HTTP connection to a WebSocket connection
        Ok(Box::new(ws.on_upgrade(move |socket: WebSocket| {
            ctx.connected(socket, ConnectionContext {
                user_id,
                group_id,
                conn_id: NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed)
            })
        })))
    }

    async fn connected(self, ws: WebSocket, conn_ctx: ConnectionContext) {
        debug!("Socket connected: {}", conn_ctx.conn_id);

        // Splitting the web socket into separate sinks and streams.
        // This is our means of sending and receiving messages over the socket.
        let (ws_tx, mut ws_rx) = ws.split::<Message>();

        // Channel used as a queue for messages.
        let (ch_tx, ch_rx) = mpsc::unbounded_channel::<Result<Message, warp::Error>>();

        // Pull messages off the end of the queue and send them over the socket.
        let conn_id = conn_ctx.conn_id;
        tokio::task::spawn(ch_rx.forward(ws_tx).map(move |result: Result<(), warp::Error>| {
            if let Err(e) = result {
                error!("Error sending over socket ({}): {}", conn_id, e);
            }
        }));

        // Add the connection to the hashmap, saving the sending end of the queue.
        // Putting messages onto the queue will cause them to eventually be
        // processed above and sent over the socket.
        if let Err(e) = self.insert_connection(&conn_ctx, ch_tx).await {
            error!("{}", e);
            return;
        }

        let message_ctx = super::handler::MessageContext {
            user_id: conn_ctx.user_id,
            group_id: conn_ctx.group_id,
            conn_id: conn_ctx.conn_id,
            groups: &self.groups,
            user_groups: &self.user_groups,
            pool: &self.pool,
        };

        // Handle each message received from the socket.
        while let Some(result) = ws_rx.next().await {
            // result: Result<Message, warp::Error>
            match result {
                Ok(message) => message_ctx.handle(message).await,
                Err(e) => {
                    error!("Error receiving from socket ({}): {}", conn_ctx.conn_id, e);
                    break;
                }
            }
        }

        self.remove_connection(&conn_ctx).await;
        debug!("Socket disconnected: {}", conn_ctx.conn_id);
    }

    pub async fn kick_user(self, user_id: db::UserID) {
        let groups_guard = self.groups.read().await;
        let user_groups_guard = self.user_groups.read().await;
        for group_id in user_groups_guard[&user_id].iter() {
            groups_guard[group_id].kick_user(user_id);
        }
    }

    pub async fn rename_user(&self, groups: Vec<db::GroupID>, user_id: db::UserID, name: &String, picture: &String) {
        let groups_guard = self.groups.read().await;
        for group_id in groups.iter() {
            if let Some(group) = groups_guard.get(group_id) {
                group.send_user_renamed(user_id, name, picture);
            }
        }
    }

    pub async fn delete_group(self, users: Vec<db::UserID>, deleted_group_id: db::GroupID) {
        let groups_guard = self.groups.read().await;
        let user_groups_guard = self.user_groups.read().await;
        for user_id in users.iter() {
            if let Some(groups) = user_groups_guard.get(&user_id) {
                for group_id in groups.iter() {
                    let group = &groups_guard[group_id];
                    if *group_id == deleted_group_id {
                        group.kick_user(*user_id);
                    } else {
                        group.send_delete_group(*user_id, deleted_group_id);
                    }
                }
            }
        }
    }
}
