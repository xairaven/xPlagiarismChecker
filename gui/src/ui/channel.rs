use crate::ui::commands::UiCommand;
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
    pub fn try_recv(&self) -> Option<UiCommand> {
        self.rx.try_recv().ok()
    }
}
