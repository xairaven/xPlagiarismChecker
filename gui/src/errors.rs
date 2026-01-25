use crate::logs::LogError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("GUI Framework. {0}")]
    EFrame(#[from] eframe::Error),

    #[error("I/O. {0}")]
    Io(#[from] IoError),

    #[error("JSON Handling. {0}")]
    Json(#[from] JsonError),

    #[error("Logging. {0}")]
    LogError(#[from] LogError),
}

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Failed to get current directory. {0}")]
    CurrentDirectory(std::io::Error),

    #[error("Failed to create parent directories. {0}")]
    CreateParentDirectories(std::io::Error),

    #[error("Failed to open file. {0}")]
    Open(std::io::Error),

    #[error("Failed to write content into file. {0}")]
    Write(std::io::Error),
}

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("Serialization. {0}")]
    Serialization(#[from] toml::ser::Error),

    #[error("Deserialization. {0}")]
    Deserialization(#[from] toml::de::Error),
}
