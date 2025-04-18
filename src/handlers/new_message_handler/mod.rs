mod builder;
mod message_handler;

use std::sync::Arc;

pub use message_handler::MessageHandler;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use builder::NewMessageHandlerBuilder;
use grammers_client::{Update, types::Message};

use super::EventHandler;
use crate::{Client, middleware::MiddlewareContainer, rules::MessageRule, types::Payload};

pub struct NewMessageHandler {
    handlers: Vec<Box<dyn MessageHandler>>,
    middlewares: MiddlewareContainer,
}

impl NewMessageHandler {
    pub fn builder() -> NewMessageHandlerBuilder {
        NewMessageHandlerBuilder::new()
    }
}

#[async_trait]
impl EventHandler for NewMessageHandler {
    async fn middlewares(&self) -> MiddlewareContainer {
        self.middlewares.clone()
    }

    async fn handle(&self, client: Arc<Client>, update: &Update) -> Result<()> {
        // Handle only new message updates
        if let Update::NewMessage(message) = update {
            for handler in &self.handlers {
                let rules: Vec<Box<dyn MessageRule>> = handler.rules().await;
                let payload = check_rules(&rules, message).await;

                if !payload.is_empty() {
                    let client = Arc::clone(&client);
                    handler.handle(client, message, payload).await?;
                }
            }
        }

        Ok(())
    }
}

async fn check_rules(rules: &[Box<dyn MessageRule>], message_text: &Message) -> Payload {
    let mut payload: Payload = Vec::new();

    for rule in rules {
        let result = rule.matches(message_text).await;
        if let Some(&bool_result) = result.downcast_ref::<bool>() {
            if bool_result {
                payload.push(result);
            }
        } else {
            //  Если результат не является bool, добавляем его в payload
            payload.push(result);
        }
    }

    payload
}
