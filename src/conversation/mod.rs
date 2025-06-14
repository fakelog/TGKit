mod conversation_container;
mod conversation_state;

pub use conversation_container::ConversationContainer;
pub use conversation_state::ConversationState;

use anyhow::{Context, Result};
use grammers_client::{
    InputMessage, Update,
    types::{Chat, Message},
};
use std::{sync::Arc, time::Duration};
use tokio::{sync::mpsc, time::timeout};

use crate::Client;

pub struct Conversation {
    client: Arc<Client>,
    chat: Chat,
    message_rx: mpsc::Receiver<Arc<Update>>,
    timeout: Duration,
}

impl Conversation {
    pub fn new(client: Arc<Client>, chat: Chat) -> Self {
        let message_rx = client.conversations.register_conversation(chat.id());
        Self {
            client,
            chat,
            message_rx,
            timeout: Duration::from_secs(60),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn send_message(&self, message: impl Into<InputMessage>) -> Result<Message> {
        self.client
            .tg_client
            .send_message(&self.chat, message)
            .await
            .context("Failed to send message")
    }

    pub async fn get_response(&mut self) -> Result<Arc<Update>> {
        match timeout(self.timeout, self.message_rx.recv()).await {
            Ok(Some(upd)) => Ok(upd),
            Ok(None) => Err(anyhow::anyhow!("Conversation channel closed")),
            Err(_) => Err(anyhow::anyhow!("Timeout waiting for response")),
        }
    }
}

impl Drop for Conversation {
    fn drop(&mut self) {
        self.client
            .conversations
            .unregister_conversation(self.chat.id());
    }
}
