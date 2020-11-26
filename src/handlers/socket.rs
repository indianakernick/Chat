use tokio::sync::mpsc;
use warp::ws::{WebSocket, Message};
use futures::{FutureExt, StreamExt};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

// Atomic int for tracking connection IDs
static NEXT_CONNECTION_ID: AtomicUsize = AtomicUsize::new(1);

pub type Connections = Arc<
    tokio::sync::RwLock<
        std::collections::HashMap<
            // Connection ID. We need to map connection IDs to persistent client
            // IDs somehow.
            usize,
            // Sending end of an unbounded channel.
            // Unbounded channels use an unbounded amount of memory.
            mpsc::UnboundedSender<Result<Message, warp::Error>>
        >
    >
>;

pub async fn connected(ws: WebSocket, connections: Connections) {
    let conn_id = NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("Connected: {}", conn_id);

    // Splitting the web socket into separate sinks and streams.
    // This is our means of sending and receiving messages over the socket.
    let (ws_tx, mut ws_rx) = ws.split::<Message>();

    // Channel used as a queue for messages.
    let (ch_tx, ch_rx) = mpsc::unbounded_channel::<Result<Message, warp::Error>>();

    // Pull messages off the end of the queue and send them over the socket.
    tokio::task::spawn(ch_rx.forward(ws_tx).map(move |result: Result<(), warp::Error>| {
        if let Err(e) = result {
            eprintln!("Error sending over socket ({}): {}", conn_id, e);
        }
    }));

    // Add the connection to the hashmap, saving the sending end of the queue.
    // Putting messages onto the queue will cause them to eventually be
    // processed above and sent over the socket.
    connections.write().await.insert(conn_id, ch_tx);

    // Why does this need to happen up here?
    let connections_clone = Arc::clone(&connections);

    // The future returned by this function acts as a state machine for the
    // connection in a way. When we break out of this loop, we disconnect.

    // Handle each message received from the socket in some way.
    while let Some(result) = ws_rx.next().await {
        // result: Result<Message, warp::Error>
        let message = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Error receiving from socket ({}): {}", conn_id, e);
                break;
            }
        };
        message_received(conn_id, message, &connections).await;
    }

    disconnected(conn_id, &connections_clone).await;
}

async fn disconnected(conn_id: usize, connections: &Connections) {
    eprintln!("Disconnected: {}", conn_id);
    connections.write().await.remove(&conn_id);
}

async fn message_received(conn_id: usize, message: Message, connections: &Connections) {
    // Discarding messages that aren't Text
    let body = if let Ok(s) = message.to_str() {
        s
    } else {
        return;
    };

    // Sending the message back over every other connection except the one that
    // sent the message.

    let response = format!("<{}>: {}", conn_id, body);

    for (&other_conn_id, ch_tx) in connections.read().await.iter() {
        if other_conn_id != conn_id {
            println!("Forwarding message from ({}) to ({}): {}", conn_id, other_conn_id, response);

            if let Err(_) = ch_tx.send(Ok(Message::text(response.clone()))) {
                // disconnected will handle this so do nothing here.
            }
        }
    }
}
