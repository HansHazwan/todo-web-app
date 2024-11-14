mod error;

use crate::error::Error;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let port = env::var("PORT").unwrap();
    let address = env::var("ADDRESS").unwrap();

    println!("PORT: {}, ADDRESS: {}", port, address);

    Ok(())
}
