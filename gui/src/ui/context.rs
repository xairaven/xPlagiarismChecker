use crate::config::Config;
use crate::errors::ProjectError;
use crate::ui::channel::UiCommandChannel;
use crate::ui::commands::UiCommand;
use crate::ui::modals::error::ErrorModal;
use crate::ui::pages::PageId;
use crate::ui::styles::StyleSettings;
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub struct GuiContext {
    pub style: StyleSettings,

    pub active_page: PageId,

    pub ui_channel: UiCommandChannel,
    pub errors_tx: Sender<ErrorModal>,
    pub errors_rx: Receiver<ErrorModal>,
}

impl GuiContext {
    pub fn new(config: &Config) -> Self {
        let (errors_tx, errors_rx) = crossbeam::channel::unbounded();

        Self {
            style: StyleSettings::new(config.theme),
            active_page: Default::default(),

            ui_channel: Default::default(),
            errors_tx,
            errors_rx,
        }
    }

    pub fn try_send_ui_command(&self, command: UiCommand) {
        if self.ui_channel.tx.try_send(command).is_err() {
            let modal = ErrorModal::new(ProjectError::ChannelSend);
            modal.try_send_by(&self.errors_tx);
        }
    }
}
