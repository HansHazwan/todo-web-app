use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error Bind Listener: {0}")]
    BindListener(tokio::io::Error),

    #[error("Error Load Config: {0}")]
    LoadConfig(std::env::VarError),

    #[error("Error Run Server: {0}")]
    RunServer(Axum::Error),
}
