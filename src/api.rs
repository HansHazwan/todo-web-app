use crate::prelude::*;
use std::fs;

fn invalid_request() -> String {
    let content = fs::read_to_string("views/404.html").unwrap();
    let content_len = content.len();

    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        content_len,
        content,
    )
}

fn home_page() -> String {
    let content = fs::read_to_string("views/index.html").unwrap();
    let content_len = content.len();

    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        content_len,
        content,
    )
}

pub fn generate_response(mut request: Vec<String>) -> String {
    let request_line = request.get(0).map(|x| x.as_str());

    match request_line {
        Some("GET / HTTP/1.1") => home_page(),
        None | _ => invalid_request(),
    }
}
