mod builder;
mod callback_handler;

pub use callback_handler::CallbackHandler;

use anyhow::Result;
use async_trait::async_trait;
use builder::CallbackQueryHandlerBuilder;
use grammers_client::{Update, types::update::CallbackQuery};
use std::sync::Arc;

use super::EventHandler;
use crate::{Client, rules::CallbackRule, types::Payload};

pub struct CallbackQueryHandler {
    handlers: Vec<Box<dyn CallbackHandler>>,
}

impl CallbackQueryHandler {
    pub fn builder() -> CallbackQueryHandlerBuilder {
        CallbackQueryHandlerBuilder::new()
    }
}

#[async_trait]
impl EventHandler for CallbackQueryHandler {
    async fn handle(&self, client: Arc<Client>, update: Arc<Update>) -> Result<()> {
        // Handle only callback query updates
        if let Update::CallbackQuery(query) = update.as_ref() {
            for handler in &self.handlers {
                let rules: Vec<Box<dyn CallbackRule>> = handler.rules().await;
                let payload = check_rules(&rules, query).await;

                // Если payload не пустой, обрабатываем запрос
                if !payload.is_empty() {
                    let client = Arc::clone(&client);
                    handler.handle(client, query, payload).await?;
                }
            }
        }

        Ok(())
    }
}

async fn check_rules(rules: &[Box<dyn CallbackRule>], data: &CallbackQuery) -> Payload {
    let mut payload: Payload = Vec::new();

    for rule in rules {
        let result = rule.matches(data).await;
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
