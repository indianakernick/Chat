use std::fs;
use std::io::Write;
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    name: String,
}

impl HelloTemplate {
    pub fn new(name: String) -> HelloTemplate {
        HelloTemplate { name }
    }
}

pub fn read_database() -> Result<Vec<String>, std::io::Error> {
    Ok(fs::read_to_string("database.txt")?
        .lines()
        .map(String::from)
        .collect::<Vec<String>>())
}

pub fn append_to_database(item: String) -> Result<(), std::io::Error> {
    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("database.txt")?
        .write_all((item + "\n").as_bytes())
}
