use crate::config::Config;
use chrono::{Datelike, Local, Timelike};
use log::{LevelFilter, Record};
use serde::{Deserialize, Serialize};
use std::fmt::Arguments;
use thiserror::Error;
use utils::enum_from_mirror;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

enum_from_mirror!(LogLevel, LevelFilter, {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
});

pub mod defaults {
    use super::LogLevel;

    pub const LOG_LEVEL: LogLevel = LogLevel::Off;
    pub const LOG_FORMAT: &str = "[$Y-$m-$D $H:$M $LEVEL] $MESSAGE";
}

pub struct Logger {
    format: String,
    log_level: LevelFilter,
}

impl Logger {
    pub fn from_config(config: &Config) -> Self {
        Self {
            format: config.log_format.clone(),
            log_level: config.log_level.into(),
        }
    }

    pub fn setup(self) -> Result<(), LogError> {
        if self.log_level.eq(&LevelFilter::Off) {
            return Ok(());
        }

        let file_name = Self::generate_file_name();
        let path = Self::path(file_name)?;

        let file = fern::log_file(path).map_err(LogError::IO)?;

        fern::Dispatch::new()
            .level(self.log_level)
            .format(move |out, message, record| {
                let formatted = self.format_message(message, record);

                out.finish(format_args!("{formatted}"))
            })
            .chain(file)
            .apply()
            .map_err(LogError::SetLoggerError)
    }

    fn format_message(&self, message: &Arguments, record: &Record) -> String {
        let log_message = self.format.clone();

        // Time
        let time = Local::now();

        log_message
            // Time
            .replacen("$Y", &format!("{:0>2}", time.year()), 1)
            .replacen("$m", &format!("{:0>2}", time.month()), 1)
            .replacen("$D", &format!("{:0>2}", time.day()), 1)
            .replacen("$H", &format!("{:0>2}", time.hour()), 1)
            .replacen("$M", &format!("{:0>2}", time.minute()), 1)
            .replacen("$S", &format!("{:0>2}", time.second()), 1)
            // Level
            .replacen("$LEVEL", record.level().as_str(), 1)
            // Target
            .replacen("$TARGET", record.target(), 1)
            // Message
            .replacen("$MESSAGE", &message.to_string(), 1)
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
