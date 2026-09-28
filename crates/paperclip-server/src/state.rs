use std::sync::Arc;
use paperclip_core::Storage;
use paperclip_runner::HeartbeatDispatcher;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<Storage>,
    pub dispatcher: Arc<HeartbeatDispatcher>,
}

impl AppState {
    pub fn new(storage: Arc<Storage>, dispatcher: Arc<HeartbeatDispatcher>) -> Self {
        Self { storage, dispatcher }
    }
}
