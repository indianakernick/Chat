use std::fs;
use std::io::Write;
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

fn read_database() -> std::io::Result<Vec<String>> {
    Ok(fs::read_to_string("database.txt")?
        .lines()
        .map(String::from)
        .collect::<Vec<String>>())
}

fn append_to_database(item: String) -> std::io::Result<()> {
    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("database.txt")?
        .write_all((item + "\n").as_bytes())
}

pub async fn hello(name: String) -> Result<impl warp::Reply, Infallible> {
    Ok(HelloTemplate::new(name))
}

pub async fn get_messages() -> Result<impl warp::Reply, Infallible> {
    match read_database() {
        Ok(val) => Ok(warp::reply::json(&val)),
        Err(e) => Ok(warp::reply::json(&format!("{}", e)))
    }
}

pub async fn post_message(message: String) -> Result<impl warp::Reply, Infallible> {
    match append_to_database(message) {
        Ok(_) => Ok(String::new()),
        Err(e) => Ok(format!("{}", e))
    }
}
