use std::sync::Arc;

use grammers_client::Update;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConversationState {
    pub last_update: Option<Arc<Update>>,
}

impl ConversationState {
    pub fn new() -> Self {
        Self { last_update: None }
    }

    pub fn update_last_update(&mut self, update: Arc<Update>) {
        self.last_update = Some(update);
    }
}

impl Default for ConversationState {
    fn default() -> Self {
        Self::new()
    }
}
