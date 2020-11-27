use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use std::time::SystemTime;
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

#[derive(Deserialize)]
#[serde(tag="type")]
enum ClientMessage {
    #[serde(rename="send message")]
    SendMessage { content: String }
}

#[derive(Serialize)]
#[serde(tag="type")]
enum ServerMessage {
    #[serde(rename="error")]
    Error { message: String },
    #[serde(rename="message sent")]
    MessageSent { timestamp: u64 },
    #[serde(rename="new message")]
    NewMessage { timestamp: u64, content: String, from: usize },
}

fn as_timestamp(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

fn send_message(ch_tx: &mpsc::UnboundedSender<Result<Message, warp::Error>>, message: String) {
    if let Err(_) = ch_tx.send(Ok(Message::text(message))) {
        // disconnected will handle the possible error
    }
}

async fn parse_client_message(conn_id: usize, message: Message, connections: &Connections) -> Result<ClientMessage, ()> {
    let message = message.to_str()?;
    match serde_json::from_str::<ClientMessage>(message) {
        Ok(m) => Ok(m),
        Err(e) => {
            let hashmap = connections.read().await;
            if let Some(ch_tx) = hashmap.get(&conn_id) {
                let response = serde_json::to_string(&ServerMessage::Error {
                    message: e.to_string()
                }).unwrap();
                send_message(ch_tx, response);
            }
            Err(())
        }
    }
}

async fn message_received(conn_id: usize, message: Message, connections: &Connections) {
    let receive_timestamp = as_timestamp(SystemTime::now());
    let client_message = match parse_client_message(conn_id, message, connections).await {
        Ok(m) => m,
        Err(_) => return
    };
    let client_message_content = match client_message {
        ClientMessage::SendMessage{ content: c } => c,
    };

    let echo_response = serde_json::to_string(&ServerMessage::MessageSent {
        timestamp: receive_timestamp
    }).unwrap();

    let peer_response = serde_json::to_string(&ServerMessage::NewMessage {
        timestamp: receive_timestamp,
        content: client_message_content,
        from: conn_id
    }).unwrap();

    // Artificial delay for testing
    // tokio::time::delay_for(std::time::Duration::from_secs(1)).await;

    for (&other_conn_id, ch_tx) in connections.read().await.iter() {
        if other_conn_id == conn_id {
            println!("Echoing back to ({}): {}", conn_id, echo_response);
            send_message(ch_tx, echo_response.clone());
        } else {
            println!("Forwarding message from ({}) to ({}): {}", conn_id, other_conn_id, peer_response);
            send_message(ch_tx, peer_response.clone());
        }
    }
}
