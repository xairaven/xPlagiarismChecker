use crate::config::Config;
use crate::errors::ProjectError;
use chrono::{Datelike, Local, Timelike};
use log::LevelFilter;
use rust_i18n_derive::Localized;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use thiserror::Error;
use utils::enum_from_mirror;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Localized,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum LogLevel {
    #[default]
    #[tag("Entity.LogLevel.Off")]
    Off,
    #[tag("Entity.LogLevel.Error")]
    Error,
    #[tag("Entity.LogLevel.Warn")]
    Warn,
    #[tag("Entity.LogLevel.Info")]
    Info,
    #[tag("Entity.LogLevel.Debug")]
    Debug,
    #[tag("Entity.LogLevel.Trace")]
    Trace,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.localize())
    }
}

enum_from_mirror!(LogLevel, LevelFilter, {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
});

pub struct Logger {
    log_level: LevelFilter,
}

impl Logger {
    pub fn from_config(config: &Config) -> Self {
        Self {
            log_level: config.log_level.into(),
        }
    }

    pub fn setup(self) -> Result<(), ProjectError> {
        if self.log_level.eq(&LevelFilter::Off) {
            return Ok(());
        }

        let file_name = Self::generate_file_name();
        let path = Self::path(file_name)?;

        let file = fern::log_file(path).map_err(LogError::IO)?;

        fern::Dispatch::new()
            .level(self.log_level)
            .format(move |out, message, record| {
                let time = Local::now();
                out.finish(format_args!(
                    "[{:0>2}-{:0>2}-{:0>2} {:0>2}:{:0>2} {}] {}",
                    time.year(),
                    time.month(),
                    time.day(),
                    time.hour(),
                    time.minute(),
                    record.level(),
                    message
                ))
            })
            .chain(file)
            .apply()
            .map_err(LogError::SetLoggerError)
            .map_err(ProjectError::LogError)
    }

    fn generate_file_name() -> String {
        let now = Local::now();
        let date = format!(
            "{year:04}-{day:02}-{month:02}",
            year = now.year(),
            day = now.day(),
            month = now.month(),
        );

        format!("{date}.log")
    }
}

#[derive(Debug, Error)]
pub enum LogError {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Set Logger: {0}")]
    SetLoggerError(#[from] log::SetLoggerError),

    #[error("Failed to get current directory: {0}")]
    CurrentDirectory(std::io::Error),

    #[error("Failed to create log directory: {0}")]
    LogDirectory(std::io::Error),
}
