use crate::config::Config;
use crate::localization::Language;

#[derive(Debug)]
pub struct Settings {
    pub language: Language,
}

impl From<&Config> for Settings {
    fn from(config: &Config) -> Self {
        Self {
            language: config.language.clone(),
        }
    }
}
