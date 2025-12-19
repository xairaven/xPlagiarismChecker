use crate::PROJECT_TITLE;
use crate::config::{Config, ConfigError};
use crate::errors::ProjectError;
use crate::logs::{LogError, Logger};
use directories::ProjectDirs;
use std::env;
use std::path::{Path, PathBuf};

pub const QUALIFIER: &str = "dev";
pub const ORGANIZATION: &str = "xairaven";
pub const APPLICATION: &str = PROJECT_TITLE;

pub fn create_parent_directories(path: &Path) -> Result<(), std::io::Error> {
    if let Some(parent_path) = path.parent() {
        return std::fs::create_dir_all(parent_path);
    }

    Ok(())
}

impl Config {
    const FILENAME: &str = "config.toml";

    fn path() -> Result<PathBuf, ProjectError> {
        let dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION);
        match dirs {
            Some(project_dirs) => Ok(project_dirs.config_dir().join(Self::FILENAME)),
            None => {
                let mut current_dir =
                    env::current_exe().map_err(ConfigError::CurrentDirectory)?;
                current_dir.pop(); // Remove executable name
                current_dir.push(Self::FILENAME);
                Ok(current_dir)
            },
        }
    }

    pub fn from_file() -> Result<Self, ProjectError> {
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

    pub fn save_to_file(&self) -> Result<(), ProjectError> {
        let data = toml::to_string(&self).map_err(ConfigError::Serialization)?;
        let path = Self::path()?;

        create_parent_directories(&path)
            .map_err(ConfigError::ParentDirectoriesCreation)?;
        std::fs::write(path, data).map_err(ConfigError::Write)?;

        Ok(())
    }
}

impl Logger {
    pub fn path(file_name: String) -> Result<PathBuf, LogError> {
        const LOG_DIR: &str = "logs";
        let mut current_dir = env::current_exe().map_err(LogError::CurrentDirectory)?;
        current_dir.pop(); // Remove executable name
        current_dir.push(LOG_DIR);

        std::fs::create_dir_all(&current_dir).map_err(LogError::LogDirectory)?;

        Ok(current_dir.join(file_name))
    }
}
