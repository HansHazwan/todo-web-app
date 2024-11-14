mod error;

use crate::error::Error;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    Ok(())
}
