use crate::config::Config;
use crate::localization::Language;
use crate::ui::themes::Theme;
use egui_aesthetix::Aesthetix;
use std::sync::Arc;

#[derive(Debug)]
pub struct Settings {
    pub language: Language,
    pub theme: ThemeSettings,
}

impl From<&Config> for Settings {
    fn from(config: &Config) -> Self {
        Self {
            language: config.language,
            theme: ThemeSettings::new(config.theme),
        }
    }
}

impl Settings {
    pub fn update_config(&self, config: &mut Config) {
        config.language = self.language;
        config.theme = self.theme.get_preference();
    }
}

#[derive(Debug)]
pub struct ThemeSettings {
    pub preference: Theme,
    pub aesthetix: Arc<dyn Aesthetix>,
}

impl ThemeSettings {
    pub fn new(preference: Theme) -> Self {
        Self {
            preference,
            aesthetix: preference.into_aesthetix_theme(),
        }
    }

    pub fn get_converted(&self) -> &Arc<dyn Aesthetix> {
        &self.aesthetix
    }

    pub fn get_preference(&self) -> Theme {
        self.preference
    }

    pub fn set(&mut self, new_theme: Theme) {
        if self.preference == new_theme {
            return;
        }

        self.preference = new_theme;
        self.aesthetix = new_theme.into_aesthetix_theme();
    }
}
