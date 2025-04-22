use anyhow::{Context, Result};
use dashmap::DashMap;
use grammers_client::Update;
use std::sync::Arc;
use tokio::sync::mpsc;

use super::ConversationState;

#[derive(Debug, Clone)]
pub struct ConversationContainer {
    pub conversations: Arc<DashMap<i64, (ConversationState, mpsc::Sender<Update>)>>,
}

impl ConversationContainer {
    pub fn new() -> Self {
        Self {
            conversations: Arc::new(DashMap::new()),
        }
    }

    pub fn register_conversation(&self, chat_id: i64) -> mpsc::Receiver<Update> {
        let (tx, rx) = mpsc::channel(32);
        self.conversations
            .insert(chat_id, (ConversationState::new(), tx));
        rx
    }

    pub fn unregister_conversation(&self, chat_id: i64) {
        self.conversations.remove(&chat_id);
    }

    pub fn handle_incoming_update(&self, chat_id: i64, update: Update) -> Result<()> {
        if let Some(mut entry) = self.conversations.get_mut(&chat_id) {
            let (state, sender) = entry.value_mut();

            state.update_last_update(&update);

            sender
                .try_send(update)
                .context("Failed to send update to conversation channel")?;
        }
        Ok(())
    }

    pub fn has_conversation(&self, chat_id: i64) -> bool {
        self.conversations.contains_key(&chat_id)
    }

    pub fn is_empty(&self) -> bool {
        self.conversations.is_empty()
    }
}

impl Default for ConversationContainer {
    fn default() -> Self {
        Self::new()
    }
}
