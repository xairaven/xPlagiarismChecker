use crate::config::Config;
use crate::ui::context::GuiContext;
use crate::ui::pages::PageId;
use crate::ui::themes::Theme;
use egui_aesthetix::Aesthetix;
use std::sync::Arc;

#[derive(Debug)]
pub struct Context {
    pub gui: GuiContext,
    pub settings: RuntimeSettings,

    // Used for saving into config file
    pub config: Config,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            gui: GuiContext::new(&config),
            settings: RuntimeSettings::from(&config),
            config,
        }
    }

    pub fn active_page(&self) -> PageId {
        self.gui.active_page
    }

    pub fn synchronize_config(&mut self) {
        self.settings.update_config(&mut self.config);
    }
}

#[derive(Debug)]
pub struct RuntimeSettings {
    pub theme: ThemeSettings,
}

impl From<&Config> for RuntimeSettings {
    fn from(config: &Config) -> Self {
        Self {
            theme: ThemeSettings::new(config.theme),
        }
    }
}

impl RuntimeSettings {
    pub fn update_config(&self, config: &mut Config) {
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
