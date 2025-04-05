mod builder;
use std::sync::Arc;

use anyhow::Result;
use builder::EventDispatcherBuilder;
use grammers_client::Update;

use crate::{
    Client,
    handlers::EventHandler,
    middleware::{Middleware, MiddlewareContainer},
};

#[derive(Clone)]
pub struct EventDispatcher {
    handlers: Vec<Arc<dyn EventHandler>>,
    middlewares: MiddlewareContainer,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> EventDispatcherBuilder {
        EventDispatcherBuilder::new()
    }

    pub fn register_handler(&mut self, handler: Arc<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    pub async fn register_middleware(&self, middleware: Box<dyn Middleware>) {
        self.middlewares.add(middleware).await;
    }

    pub async fn dispatch(&self, client: &Client, update: &Update) -> Result<()> {
        if !client.conversations.is_empty().await {
            if let Update::NewMessage(message) = update {
                if !message.outgoing() {
                    let chat = &message.chat();
                    if client.conversations.has_conversation(chat.id()).await {
                        client
                            .conversations
                            .update_message(chat.id(), message.clone())
                            .await;
                        return Ok(());
                    };
                }
            }
        };

        self.middlewares.execute_before(client, update).await?;

        for handler in self.handlers.iter() {
            let handler_middlewares = handler.middlewares().await;

            handler_middlewares.execute_before(client, update).await?;
            handler.handle(client, update).await?;
            handler_middlewares.execute_after(client, update).await?;
        }

        self.middlewares.execute_after(client, update).await?;

        Ok(())
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self {
            handlers: Vec::new(),
            middlewares: MiddlewareContainer::new(),
        }
    }
}
