use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use std::time::SystemTime;
use warp::ws::{WebSocket, Message};
use futures::{FutureExt, StreamExt};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use deadpool_postgres::Pool;

// Atomic int for tracking connection IDs
static NEXT_CONNECTION_ID: AtomicUsize = AtomicUsize::new(1);

type Sender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

type ConnectionMap = std::collections::HashMap<usize, Sender>;

pub type Connections = Arc<tokio::sync::RwLock<ConnectionMap>>;

pub fn upgrade(ws: warp::ws::Ws, conns: Connections, pool: Pool) -> impl warp::Reply {
    // Upgrade the HTTP connection to a WebSocket connection
    ws.on_upgrade(move |socket: warp::ws::WebSocket| {
        connected(socket, conns, pool)
    })
}

async fn connected(ws: WebSocket, conns: Connections, pool: Pool) {
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
    conns.write().await.insert(conn_id, ch_tx);

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

        let conns_guard = conns.read().await;
        if let Err(e) = message_received(conn_id, message, &*conns_guard, &pool).await {
            send_error_to(&*conns_guard, conn_id, e);
        }
    }

    disconnected(conn_id, &conns).await;
}

async fn disconnected(conn_id: usize, conns: &Connections) {
    eprintln!("Disconnected: {}", conn_id);
    conns.write().await.remove(&conn_id);
}

#[derive(Deserialize)]
#[serde(tag="type")]
enum ClientMessage {
    #[serde(rename="send message")]
    SendMessage { content: String },
    #[serde(rename="request messages")]
    RequestMessages
}

#[derive(Serialize)]
struct NewMessage {
    timestamp: u64,
    content: String,
    from: usize
}

impl NewMessage {
    fn from_row(row: &tokio_postgres::Row) -> NewMessage {
        NewMessage {
            timestamp: as_timestamp(row.get(1)),
            content: row.get(0),
            from: 0
        }
    }
}

#[derive(Serialize)]
#[serde(tag="type")]
enum ServerMessage {
    #[serde(rename="error")]
    Error { message: String },
    #[serde(rename="message sent")]
    MessageSent { timestamp: u64 },
    #[serde(rename="new message")]
    NewMessage(NewMessage),
    #[serde(rename="message list")]
    MessageList { messages: Vec<NewMessage> }
}

fn as_timestamp(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

fn send_message(ch_tx: &Sender, message: String) {
    if ch_tx.send(Ok(Message::text(message))).is_err() {
        // disconnected will handle the possible error
    }
}

fn send_error(ch_tx: &Sender, error: String) {
    let response = serde_json::to_string(&ServerMessage::Error {
        message: error
    }).unwrap();
    send_message(ch_tx, response);
}

fn send_error_to(conns: &ConnectionMap, conn_id: usize, error: String) {
    if let Some(ch_tx) = conns.get(&conn_id) {
        send_error(ch_tx, error);
    }
}

macro_rules! try_string {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.to_string())
        }
    };
}

async fn message_received(conn_id: usize, message: Message, conns: &ConnectionMap, pool: &Pool) -> Result<(), String> {
    // TODO: This function is turning into a bit of a mess

    let time = SystemTime::now();
    let timestamp = as_timestamp(time);
    let message = match message.to_str() {
        Ok(m) => m,
        Err(_) => return Ok(())
    };
    let client_message = try_string!(serde_json::from_str::<ClientMessage>(message));

    match client_message {
        ClientMessage::SendMessage{ content: client_message_content } => {
            let echo_response = serde_json::to_string(&ServerMessage::MessageSent {
                timestamp
            }).unwrap();

            let peer_response = serde_json::to_string(&ServerMessage::NewMessage (NewMessage {
                timestamp,
                content: client_message_content.clone(),
                from: conn_id
            })).unwrap();

            // Artificial delay for testing
            // tokio::time::delay_for(std::time::Duration::from_secs(1)).await

            for (&other_conn_id, ch_tx) in conns.iter() {
                if other_conn_id == conn_id {
                    println!("Echoing back to ({}): {}", conn_id, echo_response);
                    send_message(ch_tx, echo_response.clone());
                } else {
                    println!("Forwarding message from ({}) to ({}): {}", conn_id, other_conn_id, peer_response);
                    send_message(ch_tx, peer_response.clone());
                }
            }

            let db_conn = try_string!(pool.get().await);
            let stmt = try_string!(db_conn.prepare(
                "INSERT INTO Message (content, creation_time) VALUES ($1, $2)"
            ).await);
            try_string!(db_conn.query(&stmt, &[&client_message_content, &time]).await);

            Ok(())
        },

        ClientMessage::RequestMessages => {
            let db_conn = try_string!(pool.get().await);
            let stmt = try_string!(db_conn.prepare(
                "SELECT content, creation_time FROM Message"
            ).await);
            let rows = try_string!(db_conn.query(&stmt, &[]).await);
            let response = serde_json::to_string(&ServerMessage::MessageList {
                messages: rows.iter()
                    .map(NewMessage::from_row)
                    .collect()
            }).unwrap();
            if let Some(ch_tx) = conns.get(&conn_id) {
                send_message(ch_tx, response);
            }

            Ok(())
        }
    }
}
