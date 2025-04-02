pub mod types;

use anyhow::Result;
use async_trait::async_trait;
use grammers_client::{Client, Update};
use linkme::distributed_slice;
use std::any::Any;
use tg_kit_core::{event_handler::EventHandler, middleware::MiddlewareContainer};
use types::{MessageHandler, Rule};

async fn check_rules(rules: &[Box<dyn Rule>], message_text: &str) -> Vec<Box<dyn Any + Send>> {
    let mut payload: Vec<Box<dyn Any + Send>> = Vec::new();

    for rule in rules {
        let result = rule.matches(message_text).await;
        if let Some(&bool_result) = result.downcast_ref::<bool>() {
            if bool_result {
                payload.push(result);
            }
        } else {
            // Если результат не является bool, добавляем его в payload
            payload.push(result);
        }
    }

    payload
}

pub struct NewMessageHandler;

#[async_trait]
impl EventHandler for NewMessageHandler {
    async fn get_middlewares(&self) -> MiddlewareContainer {
        MiddlewareContainer::new()
    }

    async fn handle(&self, client: &Client, update: &Update) -> Result<()> {
        if let Update::NewMessage(message) = update {
            if message.outgoing() {
                return Ok(());
            }

            for handler in MESSAGES_HANDLERS {
                // Получаем правила для текущего обработчика
                let rules: Vec<Box<dyn Rule>> = handler.rules().await;
                // Проверяем правила и получаем payload
                let payload = check_rules(&rules, message.text()).await;

                if !payload.is_empty() {
                    if let Err(e) = handler.handle(client, message, payload).await {
                        eprintln!("Error handling message: {:?}", e);
                    }

                    break;
                }
            }
        }

        Ok(())
    }
}

#[distributed_slice]
pub static MESSAGES_HANDLERS: [&(dyn MessageHandler)];
