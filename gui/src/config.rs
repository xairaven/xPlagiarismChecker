use crate::localization::Language;
use crate::logs::LogLevel;
use crate::ui::themes::Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub language: Language,
    pub log_format: String,
    pub log_level: LogLevel,
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: Language::default(),
            log_format: "".to_string(),
            log_level: LogLevel::default(),
            theme: Theme::default(),
        }
    }
}
