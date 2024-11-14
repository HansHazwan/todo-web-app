mod error;

use crate::error::Error;
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

    println!("Server Address: {}", server_address);
    println!("Server Port: {}", server_port);
    println!("Database Username: {}", database_username);
    println!("Database Password: {}", database_password);
    println!("Database Name: {}", database_name);
    println!("Database Address: {}", database_address);

    Ok(())
}
