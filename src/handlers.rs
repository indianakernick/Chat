use askama::Template;
use std::convert::Infallible;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

impl HelloTemplate {
    fn new(name: String) -> HelloTemplate {
        HelloTemplate { name }
    }
}

pub async fn hello(name: String) -> Result<impl warp::Reply, Infallible> {
    Ok(HelloTemplate::new(name))
}

macro_rules! try_reply {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Ok(warp::reply::json(&format!("{}", err)))
        }
    };
}

pub async fn get_messages(pool: Pool) -> Result<impl warp::Reply, Infallible> {
    let client = try_reply!(pool.get().await);
    let rows = try_reply!(client.query(
        "SELECT content FROM Message",
        &[]
    ).await);
    let messages = rows
        .iter()
        .map(|row| -> String { row.get(0) })
        .collect::<Vec<String>>();
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
