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

pub struct EventDispatcher {
    inner: Arc<EventDispatcherInner>,
}

pub struct EventDispatcherInner {
    handlers: Vec<Arc<dyn EventHandler>>,
    middlewares: MiddlewareContainer,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(EventDispatcherInner {
                handlers: Vec::new(),
                middlewares: MiddlewareContainer::new(),
            }),
        }
    }

    pub fn builder() -> EventDispatcherBuilder {
        EventDispatcherBuilder::new()
    }

    pub fn register_handler(&mut self, handler: Arc<dyn EventHandler>) {
        Arc::get_mut(&mut self.inner)
            .expect("Cannot register handler after clone")
            .handlers
            .push(handler);
    }

    pub async fn register_middleware(&self, middleware: Box<dyn Middleware>) {
        self.inner.middlewares.add(middleware).await;
    }

    pub async fn dispatch(&self, client: Arc<Client>, update: &Update) -> Result<()> {
        if !client.conversations.is_empty() {
            match &update {
                Update::NewMessage(message) if !message.outgoing() => {
                    let chat_id = message.chat().id();

                    if client.conversations.has_conversation(chat_id) {
                        client
                            .conversations
                            .handle_incoming_update(chat_id, update.clone())?;
                        return Ok(());
                    }
                }
                Update::CallbackQuery(data) => {
                    let chat_id = data.chat().id();

                    if client.conversations.has_conversation(chat_id) {
                        client
                            .conversations
                            .handle_incoming_update(chat_id, update.clone())?;
                        return Ok(());
                    }
                }
                _ => {}
            }
        };

        self.inner
            .middlewares
            .execute_before(Arc::clone(&client), update)
            .await?;

        for handler in self.inner.handlers.iter() {
            let handler_middlewares = handler.middlewares().await;

            handler_middlewares
                .execute_before(Arc::clone(&client), update)
                .await?;
            handler.handle(Arc::clone(&client), update).await?;
            handler_middlewares
                .execute_after(Arc::clone(&client), update)
                .await?;
        }

        self.inner
            .middlewares
            .execute_after(Arc::clone(&client), update)
            .await?;

        Ok(())
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for EventDispatcher {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
