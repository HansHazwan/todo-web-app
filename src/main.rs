mod error;
mod prelude;
mod application;
mod api;
mod utils;

use crate::prelude::*;
use crate::application::Application;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // configuration
    dotenv::dotenv().ok();
    env_logger::init();

    let server_ip = env::var("IP")
        .map_err(|_| Error::LoadingSettings("IP".to_string()))?;
    let server_port = env::var("PORT")
        .map_err(|_| Error::LoadingSettings("PORT".to_string()))?;
    let address = format!("{}:{}", server_ip, server_port);

    let listener = TcpListener::bind(&address).await?;

    // Main Logic
    Application::new(listener).run().await?;

    Ok(())
}
