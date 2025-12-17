use crate::ui::components::navigator::Navigator;
use crate::ui::modals::error::ErrorModal;
use crate::ui::pages::PageId;
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub struct GuiContext {
    pub active_page: PageId,
    pub navigator: Navigator,

    pub errors_tx: Sender<ErrorModal>,
    pub errors_rx: Receiver<ErrorModal>,
}

impl Default for GuiContext {
    fn default() -> Self {
        let (errors_tx, errors_rx) = crossbeam::channel::unbounded();

        Self {
            active_page: Default::default(),
            navigator: Default::default(),

            errors_tx,
            errors_rx,
        }
    }
}
