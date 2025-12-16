use crate::config::{Config, ConfigError};
use directories::ProjectDirs;
use std::env;
use std::path::PathBuf;

pub const QUALIFIER: &str = "dev";
pub const ORGANIZATION: &str = "xairaven";
pub const APPLICATION: &str = "xPlagiarismChecker";

impl Config {
    const FILENAME: &str = "config.toml";

    fn path() -> Result<PathBuf, ConfigError> {
        let dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION);
        match dirs {
            Some(project_dirs) => Ok(project_dirs.config_dir().join(Self::FILENAME)),
            None => {
                let mut current_dir =
                    env::current_dir().map_err(ConfigError::CurrentDirectory)?;
                current_dir.push(Self::FILENAME);
                Ok(current_dir)
            },
        }
    }

    pub fn from_file() -> Result<Self, ConfigError> {
        match Self::path() {
            Ok(path) => {
                let text = std::fs::read_to_string(&path);
                match text {
                    Ok(text) => {
                        let config: Config = toml::from_str(&text)
                            .map_err(ConfigError::Deserialization)?;
                        Ok(config)
                    },
                    Err(_) => {
                        let config = Config::default();
                        config.save_to_file()?;
                        Ok(config)
                    },
                }
            },
            Err(_) => Ok(Self::default()),
        }
    }

    pub fn save_to_file(&self) -> Result<(), ConfigError> {
        let data = toml::to_string(&self).map_err(ConfigError::Serialization)?;
        let path = Self::path()?;

        std::fs::write(path, data).map_err(ConfigError::Write)?;

        Ok(())
    }
}
