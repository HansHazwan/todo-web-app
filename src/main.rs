mod error;
mod router;

use crate::error::Error;
use crate::router::initialize_router;

use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS")
        .map_err(|e| Error::LoadConfig(e))?;
    let server_port = env::var("SERVER_PORT")
        .map_err(|e| Error::LoadConfig(e))?;
    let database_username = env::var("DATABASE_USERNAME")
        .map_err(|e| Error::LoadConfig(e))?;
    let database_password = env::var("DATABASE_PASSWORD")
        .map_err(|e| Error::LoadConfig(e))?;
    let database_name = env::var("DATABASE_NAME")
        .map_err(|e| Error::LoadConfig(e))?;
    let database_address = env::var("DATABASE_ADDRESS")
        .map_err(|e| Error::LoadConfig(e))?;

    let ip_address = format!("{}:{}", server_address, server_port);
    let database_url = format!(
        "mysql://{}:{}@{}/{}",
        database_username,
        database_password,
        database_address,
        database_name,
    );

    let listener = TcpListener::bind(&ip_address)
        .await
        .map_err(|e| Error::BindListener(e))?;

    let router = initialize_router();

    axum::serve(listener, router)
        .await
        .map_err(|e| Error::RunServer(e))?;

    Ok(())
}
