mod filters;
mod handlers;

use warp::Filter;
use deadpool_postgres::{Pool, Manager};
use deadpool_postgres::tokio_postgres::{Config, NoTls};

use futures::{FutureExt, StreamExt};

// Why are strings not fixed size?
// let _a: &[u8; 5] = b"hello";
// let _b: &str = "hello";

fn create_pool() -> Pool {
    let mut config = Config::new();
    config.host("localhost");
    config.user("postgres");
    config.dbname("chat");

    let manager = Manager::new(config, NoTls);
    Pool::new(manager, 16)
}

async fn print_message_count(pool: &Pool) {
    let client = pool.get().await.unwrap();

    let init = std::fs::read_to_string("initialize.sql").unwrap();
    client.batch_execute(init.as_str()).await.unwrap();

    let rows = client
        .query("SELECT content FROM Message", &[])
        .await.unwrap();

    println!("Messages: {}", rows.len());
}

// Atomic int for tracking client IDs
static NEXT_CLIENT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

type Clients = std::sync::Arc<
    tokio::sync::RwLock<
        std::collections::HashMap<
            // Client ID. Unique to the connection. That is, if the same client
            // reconnects, they'll be assigned a new ID.
            // But that doesn't really matter though because we can map these
            // temporary IDs with permanent IDs via a separate mechanism.
            usize,
            // Sending end of an unbounded channel.
            // Unbounded channels use an unbounded amount of memory.
            tokio::sync::mpsc::UnboundedSender<
                // Why does this need to be a result type?
                Result<warp::ws::Message, warp::Error>
            >
        >
    >
>;

// Once the boiler-plate for pushing and popping the queue (inside
// client_connected) is done, usage is pretty simple. message_received is
// completely application specific.

#[tokio::main]
async fn main() {
    let pool = create_pool();
    print_message_count(&pool).await;

    pretty_env_logger::init();

    let clients = Clients::default();

    // Accepts a GET request with some particular headers
    let socket = warp::ws()
        .and(warp::path!("api" / "socket"))
        .map(move |ws: warp::ws::Ws| {
            let clients = std::sync::Arc::clone(&clients);
            // Upgrade the HTTP connection to a WebSocket connection
            ws.on_upgrade(move |socket: warp::ws::WebSocket| {
                client_connected(socket, clients)
            })
        });

    let routes = filters::hello()
        .or(filters::get_messages(pool.clone()))
        .or(filters::post_message(pool.clone()))
        .or(filters::root())
        .or(socket);

    warp::serve(routes.with(warp::log("chat")))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Connected to the client
async fn client_connected(ws: warp::ws::WebSocket, clients: Clients) {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    eprintln!("Connected to client: {}", client_id);

    // Splitting the web socket into separate sinks and streams.
    // This is our means of sending and receiving messages over the socket.
    // ws_tx: SplitSink<WebSocket, Message>
    // ws_rx: SplitStream<WebSocket>
    let (ws_tx, mut ws_rx) = ws.split();

    // Channel used as a queue for messages.
    let (ch_tx, ch_rx) = tokio::sync::mpsc::unbounded_channel();

    // Do we really need the queue?
    // What would happen if we tried to use the socket connection in place of
    // the queue?
    // I guess spreading the load...?

    // Pull messages off the end of the queue and send them over the socket.
    tokio::task::spawn(ch_rx.forward(ws_tx).map(|result| {
        // What is the type of result here?
        // Obviously it's a std::Result but of what?
        // Result<warp::ws::Message, warp::Error>?
        if let Err(e) = result {
            eprintln!("Error sending over websocket: {}", e);
        }
    }));

    // Add the client to the hashmap, saving the sending end of the queue.
    // Putting messages onto the queue will cause them to eventually be
    // processed above and sent over the socket.
    clients.write().await.insert(client_id, ch_tx);

    // Why does this need to happen up here?
    let clients_clone = std::sync::Arc::clone(&clients);

    // The future returned by this function acts as a state machine for the
    // client in a way. When we break out of this loop, we disconnect.

    // Handle each message received from the socket in some way.
    while let Some(result) = ws_rx.next().await {
        // Pretty sure result is a Result<warp::ws::Message, warp::Error>
        let message = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Error receiving from socket ({}): {}", client_id, e);
                break;
            }
        };
        message_received(client_id, message, &clients).await;
    }

    client_disconnected(client_id, &clients_clone).await;
}

// Disconnected from client
async fn client_disconnected(client_id: usize, clients: &Clients) {
    eprintln!("Disconnected from client: {}", client_id);
    clients.write().await.remove(&client_id);
}

// Message received from client
async fn message_received(client_id: usize, message: warp::ws::Message, clients: &Clients) {
    // Discarding messages that aren't Text
    let body = if let Ok(s) = message.to_str() {
        s
    } else {
        return;
    };

    // Here could do whatever we want.
    // Sending the message back to all other clients except the client that
    // sent it.

    let response = format!("<{}>: {}", client_id, body);

    for (&other_client_id, ch_tx) in clients.read().await.iter() {
        if other_client_id != client_id {
            if let Err(_) = ch_tx.send(Ok(warp::ws::Message::text(response.clone()))) {
                // client_disconnected will handle this so do nothing here.
            }
        }
    }
}
