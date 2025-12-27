use crate::config::ConfigError;
use crate::logs::LogError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Configuration. {0}")]
    Config(#[from] ConfigError),

    #[error("GUI Framework. {0}")]
    EFrame(#[from] eframe::Error),

    #[error("Logging. {0}")]
    LogError(#[from] LogError),
}
