use crate::algorithm::file::FileNameParseError;
use crate::io::IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LibError {
    #[error("IO. {0}")]
    Io(#[from] IoError),

    #[error("ZIP. {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("JSON. {0}")]
    Json(#[from] serde_json::error::Error),

    #[error("Filename Parse. {0}")]
    FileNameParse(#[from] FileNameParseError),
}
