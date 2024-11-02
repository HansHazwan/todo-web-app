use crate::prelude::*;

pub fn generate_response(mut request: Vec<String>) -> Result<String> {
    let request_line = request
        .get(0)
        .ok_or(Error::Custom("Invalid Http Request".to_string()))?;

    Ok(format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        request_line.len(),
        request_line,
    ))
}
