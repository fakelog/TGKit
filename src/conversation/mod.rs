mod conversation_container;
mod conversation_state;

pub use conversation_container::ConversationContainer;
pub use conversation_state::ConversationState;

use anyhow::Result;
use grammers_client::types::{Chat, Message};
use std::time::Duration;
use tokio::time;

use crate::Client;

pub struct Conversation<'a> {
    client: &'a Client,
    chat: Chat,
    timeout: Duration,
}

impl<'a> Conversation<'a> {
    pub fn new(client: &'a Client, chat: Chat) -> Self {
        Self {
            client,
            chat,
            timeout: Duration::from_secs(60),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn send_message(&self, text: &str) -> Result<()> {
        self.client.tg_client.send_message(&self.chat, text).await?;
        Ok(())
    }

    pub async fn get_response(&self) -> Result<Message> {
        self.client
            .conversations
            .add_conversation(self.chat.clone())
            .await;

        let start = time::Instant::now();
        let chat_id = &self.chat.id();

        loop {
            if start.elapsed() > self.timeout {
                return Err(anyhow::anyhow!("Timeout waiting for response"));
            }

            match time::timeout(
                Duration::from_secs(1),
                self.client.conversations.get_message(chat_id),
            )
            .await
            {
                Ok(message) => {
                    if let Some(msg) = message {
                        return Ok(msg);
                    } else {
                        continue;
                    }
                }
                Err(_) => continue,
            }
        }
    }
}

impl Drop for Conversation<'_> {
    fn drop(&mut self) {
        let client = self.client.clone();
        let chat_id = self.chat.id();
        tokio::spawn(async move {
            client.conversations.remove_conversation(chat_id).await;
        });
    }
}
