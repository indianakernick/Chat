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
    // connection in a way. It exists for the entire lifetime of the connection.

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
        let handler = MessageHandler {
            conn_id,
            message,
            conns: &*conns_guard,
            pool: &pool
        };
        handler.handle().await;
    }

    disconnected(conn_id, &conns).await;
}

async fn disconnected(conn_id: usize, conns: &Connections) {
    eprintln!("Disconnected: {}", conn_id);
    conns.write().await.remove(&conn_id);
}

macro_rules! try_string {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.to_string())
        }
    };
}

#[derive(Deserialize)]
#[serde(tag="type")]
enum ClientMessage {
    #[serde(rename="post message")]
    PostMessage { content: String },
    #[serde(rename="request recent messages")]
    RequestRecentMessages
}

#[derive(Serialize)]
struct RecentMessage {
    timestamp: u64,
    author: i32,
    content: String
}

#[derive(Serialize)]
#[serde(tag="type")]
enum ServerMessage {
    #[serde(rename="error")]
    Error { message: String },
    #[serde(rename="message receipt")]
    MessageReceipt { timestamp: u64 },
    #[serde(rename="recent message")]
    RecentMessage(RecentMessage),
    #[serde(rename="recent message list")]
    RecentMessageList { messages: Vec<RecentMessage> }
}

fn as_timestamp(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

struct MessageHandler<'a> {
    conn_id: usize,
    message: Message,
    conns: &'a ConnectionMap,
    pool: &'a Pool,
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

impl<'a> MessageHandler<'a> {
    fn reply_error(&self, error: String) {
        if let Some(ch_tx) = self.conns.get(&self.conn_id) {
            send_error(ch_tx, error);
        }
    }

    fn reply_message(&self, message: String) {
        if let Some(ch_tx) = self.conns.get(&self.conn_id) {
            send_message(ch_tx, message);
        }
    }

    async fn handle(self) {
        if let Err(e) = self.handle_error().await {
            self.reply_error(e);
        }
    }

    async fn handle_error(&self) -> Result<(), String> {
        let message = match self.message.to_str() {
            Ok(m) => m,
            Err(_) => return Ok(())
        };
        let client_message = try_string!(serde_json::from_str::<ClientMessage>(message));

        match client_message {
            ClientMessage::PostMessage { content } => {
                self.handle_post_message(content).await
            },
            ClientMessage::RequestRecentMessages => {
                self.handle_request_recent_messages().await
            }
        }
    }

    async fn handle_post_message(&self, content: String) -> Result<(), String> {
        let time = SystemTime::now();
        let timestamp = as_timestamp(time);

        let echo_response = serde_json::to_string(&ServerMessage::MessageReceipt {
            timestamp
        }).unwrap();

        let peer_response = serde_json::to_string(&ServerMessage::RecentMessage(RecentMessage {
            timestamp,
            author: self.conn_id as i32,
            content: content.clone(),
        })).unwrap();

        // Artificial delay for testing
        // tokio::time::delay_for(std::time::Duration::from_secs(1)).await

        for (&other_conn_id, ch_tx) in self.conns.iter() {
            if other_conn_id == self.conn_id {
                println!("Echoing back to ({}): {}", self.conn_id, echo_response);
                send_message(ch_tx, echo_response.clone());
            } else {
                println!("Forwarding message from ({}) to ({}): {}", self.conn_id, other_conn_id, peer_response);
                send_message(ch_tx, peer_response.clone());
            }
        }

        let db_conn = try_string!(self.pool.get().await);
        let stmt = try_string!(db_conn.prepare(
            "INSERT INTO Message (timestamp, author, content) VALUES ($1, $2, $3)"
        ).await);
        try_string!(db_conn.query(&stmt, &[&time, &(self.conn_id as i32), &content]).await);

        Ok(())
    }

    async fn handle_request_recent_messages(&self) -> Result<(), String> {
        let db_conn = try_string!(self.pool.get().await);
        let stmt = try_string!(db_conn.prepare(
            "SELECT timestamp, author, content FROM Message"
        ).await);
        let rows = try_string!(db_conn.query(&stmt, &[]).await);

        let response = serde_json::to_string(&ServerMessage::RecentMessageList {
            messages: rows.iter()
                .map(|row| RecentMessage {
                    timestamp: as_timestamp(row.get(0)),
                    author: row.get::<_, i32>(1),
                    content: row.get(2)
                })
                .collect()
        }).unwrap();

        //tokio::time::delay_for(std::time::Duration::from_secs(5)).await;

        self.reply_message(response);

        Ok(())
    }
}
