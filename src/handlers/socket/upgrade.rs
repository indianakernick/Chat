use log::{debug, error};
use tokio::sync::mpsc;
use deadpool_postgres::Pool;
use futures::{FutureExt, StreamExt};
use warp::ws::{Ws, WebSocket, Message};
use crate::handlers::{UserID, get_session_user_id};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

pub type Sender = mpsc::UnboundedSender<Result<Message, warp::Error>>;
pub type ConnectionMap = std::collections::HashMap<usize, Sender>;
pub type Connections = Arc<tokio::sync::RwLock<ConnectionMap>>;

// Atomic int for tracking connection IDs
static NEXT_CONNECTION_ID: AtomicUsize = AtomicUsize::new(1);

pub async fn upgrade(ws: Ws, session_id: String, pool: Pool, conns: Connections)
    -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = get_session_user_id(pool.clone(), session_id).await?;
    // Upgrade the HTTP connection to a WebSocket connection
    Ok(ws.on_upgrade(move |socket: WebSocket| {
        connected(socket, user_id, pool, conns)
    }))
}

async fn connected(ws: WebSocket, user_id: UserID, pool: Pool, conns: Connections) {
    let conn_id = NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);
    debug!("Socket connected: {}", conn_id);

    // Splitting the web socket into separate sinks and streams.
    // This is our means of sending and receiving messages over the socket.
    let (ws_tx, mut ws_rx) = ws.split::<Message>();

    // Channel used as a queue for messages.
    let (ch_tx, ch_rx) = mpsc::unbounded_channel::<Result<Message, warp::Error>>();

    // Pull messages off the end of the queue and send them over the socket.
    tokio::task::spawn(ch_rx.forward(ws_tx).map(move |result: Result<(), warp::Error>| {
        if let Err(e) = result {
            error!("Error sending over socket ({}): {}", conn_id, e);
        }
    }));

    // Add the connection to the hashmap, saving the sending end of the queue.
    // Putting messages onto the queue will cause them to eventually be
    // processed above and sent over the socket.
    conns.write().await.insert(conn_id, ch_tx);

    // The future returned by this function acts as a state machine for the
    // connection in a way. It exists for the entire lifetime of the connection.

    // Handle each message received from the socket.
    while let Some(result) = ws_rx.next().await {
        // result: Result<Message, warp::Error>
        let message = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("Error receiving from socket ({}): {}", conn_id, e);
                break;
            }
        };

        let conns_guard = conns.read().await;
        let handler = super::handler::MessageHandler {
            conn_id,
            user_id,
            message,
            conns: &*conns_guard,
            pool: &pool
        };
        handler.handle().await;
    }

    debug!("Socket disconnected: {}", conn_id);
    conns.write().await.remove(&conn_id);
}
