use serde::{Deserialize, Serialize};
use utils::enum_from_mirror;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    #[default]
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

enum_from_mirror!(LogLevel, log::LevelFilter, {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
});
