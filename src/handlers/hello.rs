use askama::Template;
use std::convert::Infallible;

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
