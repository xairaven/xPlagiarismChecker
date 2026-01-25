use crate::errors::{IoError, JsonError, ProjectError};
use crate::files::FileType;
use crate::localization::Language;
use crate::logs::LogLevel;
use crate::ui::themes::Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub language: Language,
    pub log_level: LogLevel,
    pub theme: Theme,
}

pub const CONFIG_FILE_TYPE: FileType = FileType::Config;

impl Config {
    pub fn from_file() -> Result<Self, ProjectError> {
        match CONFIG_FILE_TYPE.path() {
            Ok(path) => {
                let text = std::fs::read_to_string(&path);
                match text {
                    Ok(text) => {
                        let config: Config =
                            toml::from_str(&text).map_err(JsonError::Deserialization)?;
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
        let data = toml::to_string(&self).map_err(JsonError::Serialization)?;
        let path = CONFIG_FILE_TYPE.path()?;

        if let Some(parent_path) = path.parent() {
            std::fs::create_dir_all(parent_path)
                .map_err(IoError::CreateParentDirectories)?;
        }

        std::fs::write(path, data).map_err(IoError::Write)?;

        Ok(())
    }
}
