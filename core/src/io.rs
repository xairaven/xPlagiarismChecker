use thiserror::Error;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Failed to create file: {0}")]
    CreateFile(std::io::Error),

    #[error("Failed to write content into file: {0}")]
    Write(std::io::Error),
}
