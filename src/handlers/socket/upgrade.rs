use tokio::sync::mpsc;
use log::{debug, error};
use deadpool_postgres::Pool;
use std::collections::HashMap;
use futures::{FutureExt, StreamExt};
use warp::ws::{Ws, WebSocket, Message};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use crate::database::{UserID, GroupID, valid_group, SessionID, session_user_id};

pub type ConnID = usize;
static NEXT_CONNECTION_ID: AtomicUsize = AtomicUsize::new(1);

pub type Sender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

pub struct Group {
    // TODO: Store channels
    // channels: Vec<ChannelID>,
    pub users: HashMap<ConnID, Sender>,
}

type GroupMap = HashMap<GroupID, Group>;
type Groups = Arc<tokio::sync::RwLock<GroupMap>>;

fn group_map_insert(map: &mut GroupMap, group_id: GroupID, conn_id: ConnID, ch_tx: Sender) {
    map
        .entry(group_id)
        .or_insert_with(|| {
            // TODO: get channels
            Group { users: HashMap::new() }
        })
        .users
        .insert(conn_id, ch_tx);
}

fn group_map_remove(map: &mut GroupMap, group_id: GroupID, conn_id: ConnID) {
    let users = &mut map.get_mut(&group_id).unwrap().users;
    users.remove(&conn_id);
    if users.is_empty() {
        map.remove(&group_id);
    }
}

#[derive(Clone)]
pub struct SocketContext {
    pool: Pool,
    //conns: Connections,
    groups: Groups,
}

impl SocketContext {
    pub fn new(pool: Pool) -> SocketContext {
        SocketContext {
            pool,
            //conns: Connections::default(),
            groups: Groups::default()
        }
    }
}

pub struct ConnectionContext {
    pub user_id: UserID,
    pub group_id: GroupID,
    pub conn_id: ConnID,
}

pub async fn upgrade(group_id: GroupID, ws: Ws, session_id: SessionID, ctx: SocketContext)
    -> Result<Box<dyn warp::Reply>, warp::Rejection> {

    // The JavaScript that invokes this is only loaded when the session cookie
    // is valid. The only way that this error could happen is if the session
    // expires between loading the page and running the JavaScript. Another
    // possibility is someone directly accessing this endpoint but failing to
    // provide the cookie.
    let user_id = match session_user_id(ctx.pool.clone(), session_id).await? {
        Some(id) => id,
        None => return Ok(Box::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    };

    // Can only happen if someone is directly accessing the socket.
    if !valid_group(ctx.pool.clone(), group_id).await? {
        return Ok(Box::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    // TODO: Maybe join the above two database queries and run them simultaneously

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
    group_map_insert(&mut *sock_ctx.groups.write().await, conn_ctx.group_id, conn_ctx.conn_id, ch_tx);

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

        let guard = sock_ctx.groups.read().await;
        let msg_ctx = super::handler::MessageContext {
            ctx: &conn_ctx,
            group: &guard[&conn_ctx.group_id],
            pool: &sock_ctx.pool,
            message,
        };
        msg_ctx.handle().await;
    }

    debug!("Socket disconnected: {}", conn_ctx.conn_id);
    group_map_remove(&mut *sock_ctx.groups.write().await, conn_ctx.group_id, conn_ctx.conn_id);
}
