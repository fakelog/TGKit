mod builder;
mod message_handler;
mod rule_checker;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use builder::NewMessageHandlerBuilder;
use grammers_client::Update;
use rule_checker::RuleChecker;
use std::sync::Arc;

use super::EventHandler;
use crate::{Client, rules::MessageRule};

pub use message_handler::MessageHandler;

pub struct NewMessageHandler {
    handlers: Vec<Box<dyn MessageHandler>>,
    rule_checker: RuleChecker,
}

impl NewMessageHandler {
    pub fn builder() -> NewMessageHandlerBuilder {
        NewMessageHandlerBuilder::new()
    }
}

#[async_trait]
impl EventHandler for NewMessageHandler {
    async fn handle(&self, client: Arc<Client>, update: Arc<Update>) -> Result<()> {
        if let Update::NewMessage(message) = update.as_ref() {
            for handler in &self.handlers {
                let rules: Vec<Box<dyn MessageRule>> = handler.rules().await;
                let check_result = self.rule_checker.check(rules, message).await;

                if check_result.all_passed() {
                    handler
                        .handle(Arc::clone(&client), message, check_result.into_payload())
                        .await?;
                }
            }
        }

        Ok(())
    }
}
