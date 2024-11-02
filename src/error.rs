use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Custom Error: {0}")]
    Custom(String),

    #[error("I/O Error: {0}")]
    Io(#[from] tokio::io::Error),

    #[error("Loading Setting Error: Cannot Find {0}")]
    LoadingSettings(String),
}
