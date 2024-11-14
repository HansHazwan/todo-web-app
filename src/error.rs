use thiserror::Error;

#[derive(Debug, Error)]
pub struct Error {
    #[error("Error Bind Listener: {0}")]
    BindListener(tokio::io::Error),
}
