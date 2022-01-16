use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

#[derive(Debug)]
pub enum CtrlCResponse {
    Quit,
    Continue,
    Error,
}

#[derive(Debug)]
pub struct CtrlCHandler {
    rx: Receiver<CtrlCResponse>,
    tx: Sender<CtrlCResponse>,
}

impl CtrlCHandler {
    pub fn new() -> Self {
        let (tx, rx) = channel::<CtrlCResponse>();
        let tx_clone = tx.clone();
        ctrlc::set_handler(move || tx_clone.send(CtrlCResponse::Quit).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
        Self {
            rx: rx,
            tx: tx.clone(),
        }
    }
    pub fn respond(&self) -> CtrlCResponse {
        match self.rx.try_recv() {
            Ok(rx) => rx,
            Err(TryRecvError::Empty) => CtrlCResponse::Continue,
            Err(TryRecvError::Disconnected) => CtrlCResponse::Error, 
        }
    }
    pub fn send_quit(&self) {
        self.tx.clone().send(CtrlCResponse::Quit).unwrap();
    }
    pub fn should_continue(&self) -> bool {
        if let CtrlCResponse::Continue = self.respond() {
            true
        } else {
            false
        }
    }
    pub fn get_tx(&self) -> Sender<CtrlCResponse> {
        self.tx.clone()
    }
}