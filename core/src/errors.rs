use crate::io::IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LibError {
    #[error("IO. {0}")]
    Io(#[from] IoError),
}
