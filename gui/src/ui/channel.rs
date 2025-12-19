use crate::context::Context;
use crate::errors::ProjectError;
use crate::ui::commands::UiCommand;
use crate::ui::modals::error::ErrorModal;
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub struct UiCommandChannel {
    pub tx: Sender<UiCommand>,
    pub rx: Receiver<UiCommand>,
}

impl Default for UiCommandChannel {
    fn default() -> Self {
        let (tx, rx) = crossbeam::channel::unbounded();
        Self { tx, rx }
    }
}

impl UiCommandChannel {
    pub fn try_send(&self, command: UiCommand, ctx: &Context) {
        if self.tx.try_send(command).is_err() {
            let modal = ErrorModal::new(ProjectError::ChannelSend);
            modal.try_send_by(&ctx.gui.errors_tx);
        }
    }

    pub fn try_recv(&self) -> Option<UiCommand> {
        self.rx.try_recv().ok()
    }
}
