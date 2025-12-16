use crate::localization::Language;
use crate::logs::LogLevel;
use crate::ui::themes::Theme;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub language: Language,
    pub log_level: LogLevel,
    pub theme: Theme,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to get current directory. {0}")]
    CurrentDirectory(std::io::Error),

    #[error("Failed to serialize. {0}")]
    Serialization(#[from] toml::ser::Error),

    #[error("Failed to deserialize. {0}")]
    Deserialization(#[from] toml::de::Error),

    #[error("Failed to create parent directories. {0}")]
    ParentDirectoriesCreation(std::io::Error),

    #[error("Failed to write to file. {0}")]
    Write(std::io::Error),
}
