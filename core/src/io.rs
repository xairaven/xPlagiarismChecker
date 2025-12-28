use thiserror::Error;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Failed to create file: {0}")]
    Create(std::io::Error),

    #[error("Failed to open file: {0}")]
    Open(std::io::Error),

    #[error("Failed to read content from file: {0}")]
    Read(std::io::Error),

    #[error("Failed to write content into file: {0}")]
    Write(std::io::Error),
}
