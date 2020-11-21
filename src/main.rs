mod filters;
mod handlers;

use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = filters::hello()
        .or(filters::get_messages())
        .or(filters::post_message());

    warp::serve(routes.with(warp::log("chat")))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
