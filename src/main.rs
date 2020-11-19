use warp::Filter;
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let hello = warp::get()
        .and(warp::path!("hello" / String))
        .map(move |name| {
            HelloTemplate {name}
        });

    // Can expand this to hello.or(something).or(something_else)
    let routes = hello;

    warp::serve(routes.with(warp::log("chat")))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
