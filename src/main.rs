mod filters;
mod handlers;

use warp::Filter;
use deadpool_postgres::{Pool, Manager};
use deadpool_postgres::tokio_postgres::{Config, NoTls};

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

#[tokio::main]
async fn main() {
    let pool = create_pool();
    print_message_count(&pool).await;

    pretty_env_logger::init();

    let routes = filters::hello()
        .or(filters::get_messages(pool.clone()))
        .or(filters::post_message(pool.clone()))
        .or(filters::root())
        .or(filters::socket());

    warp::serve(routes.with(warp::log("chat")))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
