use crate::config::Config;
use crate::ui::channel::Channel;
use crate::ui::commands::UiCommand;
use crate::ui::modals::error::ErrorModal;
use crate::ui::pages::PageId;

#[derive(Debug)]
pub struct GuiContext {
    pub active_page: PageId,

    pub ui_channel: Channel<UiCommand>,
    pub errors_channel: Channel<ErrorModal>,
}

impl GuiContext {
    pub fn new(_config: &Config) -> Self {
        Self {
            active_page: Default::default(),

            ui_channel: Default::default(),
            errors_channel: Default::default(),
        }
    }
}
