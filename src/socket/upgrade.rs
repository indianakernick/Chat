use log::{debug, error};
use crate::database as db;
use deadpool_postgres::Pool;
use tokio::sync::{RwLock, mpsc};
use futures::{FutureExt, StreamExt};
use warp::ws::{Ws, WebSocket, Message};
use std::collections::hash_map::{HashMap, Entry};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

pub type ConnID = usize;
static NEXT_CONNECTION_ID: AtomicUsize = AtomicUsize::new(1);

pub type Sender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

pub struct Group {
    pub channels: Vec<db::Channel>,
    pub users: HashMap<ConnID, Sender>,
}

pub type GroupMap = HashMap<db::GroupID, Group>;
pub type Groups = Arc<RwLock<GroupMap>>;

#[derive(Clone)]
pub struct SocketContext {
    pool: Pool,
    groups: Groups,
}

impl SocketContext {
    pub fn new(pool: Pool) -> SocketContext {
        SocketContext {
            pool,
            groups: Groups::default()
        }
    }
}

pub struct ConnectionContext {
    pub user_id: db::UserID,
    pub group_id: db::GroupID,
    pub conn_id: ConnID,
}

pub async fn upgrade(group_id: db::GroupID, ws: Ws, session_id: db::SessionID, ctx: SocketContext)
    -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let (user_id, valid) = futures::future::join(
        db::session_user_id(ctx.pool.clone(), session_id),
        db::valid_group(ctx.pool.clone(), group_id)
    ).await;

    // The JavaScript that invokes this is only loaded when the session cookie
    // is valid. The only way that this error could happen is if the session
    // expires between loading the page and running the JavaScript. Another
    // possibility is someone directly accessing this endpoint but failing to
    // provide the cookie.
    let user_id = match user_id? {
        Some(id) => id,
        None => return Ok(Box::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    };

    // Can only happen if someone is directly accessing the socket.
    if !valid? {
        return Ok(Box::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    // Upgrade the HTTP connection to a WebSocket connection
    Ok(Box::new(ws.on_upgrade(move |socket: WebSocket| {
        let conn_ctx = ConnectionContext {
            user_id,
            group_id,
            conn_id: NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed)
        };
        connected(socket, ctx, conn_ctx)
    })))
}

async fn connected(ws: WebSocket, sock_ctx: SocketContext, conn_ctx: ConnectionContext) {
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
    {
        let mut guard = sock_ctx.groups.write().await;
        match guard.entry(conn_ctx.group_id) {
            Entry::Vacant(entry) => {
                let channels = match db::group_channels(sock_ctx.pool.clone(), conn_ctx.group_id).await {
                    Ok(c) => c,
                    Err(e) => {
                        error!("{}", e);
                        return;
                    }
                };
                let mut users = HashMap::new();
                users.insert(conn_ctx.conn_id, ch_tx);
                entry.insert(Group { channels, users });
            },
            Entry::Occupied(mut entry) => {
                entry.get_mut().users.insert(conn_ctx.conn_id, ch_tx);
            }
        }
    }

    // Handle each message received from the socket.
    while let Some(result) = ws_rx.next().await {
        // result: Result<Message, warp::Error>
        let message = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("Error receiving from socket ({}): {}", conn_ctx.conn_id, e);
                break;
            }
        };

        // perhaps don't lock the groups map
        // instead, pass it to the message handler as is
        // let the message handler decide whether it needs to lock for reading or writing
        // wrapping something in a lock adds 6 * 8 (48) bytes of overhead
        // probably pretty inefficient

        let msg_ctx = super::handler::MessageContext {
            ctx: &conn_ctx,
            groups: &sock_ctx.groups,
            pool: &sock_ctx.pool,
            message,
        };
        msg_ctx.handle().await;
    }

    debug!("Socket disconnected: {}", conn_ctx.conn_id);
    let mut guard = sock_ctx.groups.write().await;
    let mut entry = match guard.entry(conn_ctx.group_id) {
        Entry::Occupied(entry) => entry,
        Entry::Vacant(_) => panic!()
    };
    let users = &mut entry.get_mut().users;
    users.remove(&conn_id);
    if users.is_empty() {
        entry.remove();
    }
}
