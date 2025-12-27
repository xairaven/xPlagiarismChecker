use crossbeam::channel::{Receiver, Sender, TryRecvError};

#[derive(Debug)]
pub struct Channel<T> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        let (tx, rx) = crossbeam::channel::unbounded();
        Self { tx, rx }
    }
}

impl<T> Channel<T> {
    pub fn try_send(&self, item: T) {
        if let Err(error) = self.tx.try_send(item) {
            log::error!("Failed to send command through channel: {}", error);
        }
    }

    pub fn try_recv(&self) -> Option<T> {
        match self.rx.try_recv() {
            Ok(item) => Some(item),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                log::error!("Channel disconnected");
                None
            },
        }
    }
}
