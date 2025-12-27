use crate::config::Config;
use crate::settings::Settings;
use crate::ui::context::GuiContext;
use crate::ui::pages::PageId;

#[derive(Debug)]
pub struct Context {
    pub gui: GuiContext,
    pub settings: Settings,

    // Used for saving into config file
    pub config: Config,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            gui: GuiContext::new(&config),
            settings: Settings::from(&config),
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
