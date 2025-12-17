use crate::config::Config;
use crate::ui::context::GuiContext;
use crate::ui::pages::PageId;

#[derive(Debug)]
pub struct Context {
    pub gui: GuiContext,

    // Used for saving into config file
    pub config: Config,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            gui: Default::default(),
            config,
        }
    }

    pub fn active_page(&self) -> PageId {
        self.gui.active_page
    }
}
