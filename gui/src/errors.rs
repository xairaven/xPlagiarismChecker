use crate::config::ConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Configuration. {0}")]
    Config(#[from] ConfigError),
}
