use crate::config::Config;
use crate::ui::modals::error::ErrorModal;
use crossbeam::channel::{Receiver, Sender};

pub struct Context {
    pub errors_tx: Sender<ErrorModal>,
    pub errors_rx: Receiver<ErrorModal>,

    // Used for saving into config file
    pub config: Config,
}

impl Context {
    pub fn new(config: Config) -> Self {
        let (errors_tx, errors_rx) = crossbeam::channel::unbounded();

        Self {
            errors_tx,
            errors_rx,
            config,
        }
    }
}
