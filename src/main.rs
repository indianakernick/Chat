use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::param())
        .map(move |name: String| {
            warp::reply::html(format!(r#"
                <!DOCTYPE html>
                <html>
                    <head>
                        <title>Chat</title>
                    </head>
                    <body>
                        <p style="font-family: Helvetica, sans-serif">Hello <b>{}</b></p>
                    </body>
                </html>
                "#,
                name
            ))
        });

    warp::serve(hello.with(warp::log("chat")))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
