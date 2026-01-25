use crate::PROJECT_TITLE;
use crate::errors::{IoError, ProjectError};
use crate::files::config::Config;
use crate::files::ignore::IgnoreSettings;
use chrono::{Datelike, Local};
use directories::ProjectDirs;
use std::env;
use std::path::PathBuf;

pub mod config;
pub mod ignore;

pub const QUALIFIER: &str = "dev";
pub const ORGANIZATION: &str = "xairaven";
pub const APPLICATION: &str = PROJECT_TITLE;

pub struct AppFiles {
    pub config: Config,
    pub ignore: IgnoreSettings,
}

impl AppFiles {
    pub fn load() -> Result<Self, ProjectError> {
        let config = Config::from_file()?;
        let ignore = IgnoreSettings::from_file()?;

        Ok(Self { config, ignore })
    }
}

pub enum FileType {
    Config,
    IgnoredDirectories,
    AcceptedExtensions,
    Logs,
}

impl FileType {
    pub fn path(&self) -> Result<PathBuf, ProjectError> {
        match self {
            Self::Config | Self::IgnoredDirectories | Self::AcceptedExtensions => {
                self.configuration_file()
            },
            Self::Logs => self.log_file(),
        }
    }

    fn path_id(&self) -> &'static str {
        match self {
            Self::Config => "config.toml",
            Self::IgnoredDirectories => "ignored_directories.txt",
            Self::AcceptedExtensions => "accepted_extensions.txt",
            Self::Logs => "logs",
        }
    }

    fn configuration_file(&self) -> Result<PathBuf, ProjectError> {
        let dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION);
        match dirs {
            Some(project_dirs) => Ok(project_dirs.config_dir().join(self.path_id())),
            None => {
                let mut current_dir =
                    env::current_exe().map_err(IoError::CurrentDirectory)?;
                current_dir.pop(); // Remove executable name
                current_dir.push(self.path_id());
                Ok(current_dir)
            },
        }
    }

    fn log_file(&self) -> Result<PathBuf, ProjectError> {
        let file_name = {
            let now = Local::now();
            let date = format!(
                "{year:04}-{day:02}-{month:02}",
                year = now.year(),
                day = now.day(),
                month = now.month(),
            );

            format!("{date}.log")
        };

        let mut current_dir = env::current_exe().map_err(IoError::CurrentDirectory)?;
        current_dir.pop(); // Remove executable name
        current_dir.push(self.path_id());

        std::fs::create_dir_all(&current_dir)
            .map_err(IoError::CreateParentDirectories)?;

        Ok(current_dir.join(file_name))
    }
}
