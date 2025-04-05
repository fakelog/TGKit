use grammers_client::types::{Chat, Message};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use super::ConversationState;

#[derive(Debug, Clone)]
pub struct ConversationContainer {
    pub conversations: Arc<Mutex<HashMap<i64, ConversationState>>>,
}

impl ConversationContainer {
    pub fn new() -> Self {
        Self {
            conversations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_conversation(&self, chat: Chat) {
        let mut conversations = self.conversations.lock().await;
        conversations.insert(chat.id(), ConversationState::new(chat));
    }

    pub async fn remove_conversation(&self, chat_id: i64) {
        let mut conversations = self.conversations.lock().await;
        conversations.remove(&chat_id);
    }

    pub async fn update_message(&self, chat_id: i64, message: Message) -> bool {
        let mut conversations = self.conversations.lock().await;
        if let Some(state) = conversations.get_mut(&chat_id) {
            state.update_last_message(message);
            true
        } else {
            false
        }
    }

    pub async fn get_message(&self, chat_id: &i64) -> Option<Message> {
        let conversations = self.conversations.lock().await;
        conversations
            .get(chat_id)
            .and_then(|s| s.last_message.clone())
    }

    pub async fn has_conversation(&self, chat_id: i64) -> bool {
        let conversations = self.conversations.lock().await;
        conversations.contains_key(&chat_id)
    }

    pub async fn is_empty(&self) -> bool {
        let conversations = self.conversations.lock().await;
        conversations.is_empty()
    }
}
