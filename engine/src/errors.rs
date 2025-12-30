use crate::io::IoError;
use crate::models::database::DatabaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LibError {
    #[error("IO. {0}")]
    Io(#[from] IoError),

    #[error("ZIP. {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("JSON. {0}")]
    Json(#[from] serde_json::error::Error),

    #[error("Database. {0}")]
    Database(#[from] DatabaseError),
}
