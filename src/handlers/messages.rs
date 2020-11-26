use serde::Serialize;
use std::time::SystemTime;
use deadpool_postgres::Pool;
use std::convert::Infallible;

macro_rules! try_reply {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Ok(warp::reply::json(&format!("{}", err)))
        }
    };
}

#[derive(Serialize)]
struct Message {
    content: String,
    creation_time: u64
}

impl Message {
    fn from_row(row: &deadpool_postgres::tokio_postgres::Row) -> Message {
        let time: SystemTime = row.get(1);
        Message {
            content: row.get(0),
            creation_time: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
        }
    }
}

pub async fn get_messages(pool: Pool) -> Result<impl warp::Reply, Infallible> {
    let client = try_reply!(pool.get().await);
    let rows = try_reply!(client.query(
        "SELECT content, creation_time FROM Message",
        &[]
    ).await);
    let messages: Vec<_> = rows
        .iter()
        .map(Message::from_row)
        .collect();
    Ok(warp::reply::json(&messages))
}

pub async fn post_message(message: String, pool: Pool) -> Result<impl warp::Reply, Infallible> {
    let client = try_reply!(pool.get().await);
    try_reply!(client.query(
        "INSERT INTO Message (content) VALUES ($1)",
        &[&message]
    ).await);
    Ok(warp::reply::json(&""))
}
