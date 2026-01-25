use crate::files::AppFiles;
use crate::files::config::Config;
use crate::files::ignore::IgnoreSettings;
use crate::session::Session;
use crate::ui::context::GuiContext;
use crate::ui::pages::PageId;
use crate::ui::themes::Theme;
use egui_aesthetix::Aesthetix;
use std::sync::Arc;

#[derive(Debug)]
pub struct Context {
    pub gui: GuiContext,
    pub session: Session,
    pub settings: RuntimeSettings,

    // Used for saving into config file
    pub config: Config,
}

impl Context {
    pub fn new(app_files: AppFiles) -> Self {
        Self {
            gui: GuiContext::new(&app_files.config),
            session: Session::default(),
            settings: RuntimeSettings::from(&app_files),
            config: app_files.config,
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
    pub ignore_settings: IgnoreSettings,
}

impl From<&AppFiles> for RuntimeSettings {
    fn from(app_files: &AppFiles) -> Self {
        Self {
            theme: ThemeSettings::new(app_files.config.theme),
            ignore_settings: app_files.ignore.clone(),
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
