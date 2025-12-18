use crate::config::Config;
use crate::ui::components::navigator::Navigator;
use crate::ui::modals::error::ErrorModal;
use crate::ui::pages::PageId;
use crate::ui::styles::StyleSettings;
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub struct GuiContext {
    pub style: StyleSettings,

    pub active_page: PageId,
    pub navigator: Navigator,

    pub errors_tx: Sender<ErrorModal>,
    pub errors_rx: Receiver<ErrorModal>,
}

impl GuiContext {
    pub fn new(config: &Config) -> Self {
        let (errors_tx, errors_rx) = crossbeam::channel::unbounded();

        Self {
            style: StyleSettings::new(config.theme),
            active_page: Default::default(),
            navigator: Default::default(),

            errors_tx,
            errors_rx,
        }
    }
}
