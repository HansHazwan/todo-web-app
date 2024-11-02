use crate::prelude::*;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn read_socket(socket: &mut TcpStream) -> Result<Vec<String>> {
    let mut buffer = [0; 1024];

    let bytes_read = socket.read(&mut buffer).await?;

    Ok(String::from_utf8_lossy(&buffer[..bytes_read])
        .lines()
        .map(String::from)
        .collect::<Vec<String>>())
}

pub async fn write_socket(socket: &mut TcpStream, response: &str) -> Result<()> {
    Ok(socket.write_all(response.as_bytes()).await?)
}
