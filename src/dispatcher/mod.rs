mod builder;
use std::sync::Arc;

use anyhow::Result;
use builder::EventDispatcherBuilder;
use futures::future::try_join_all;
use grammers_client::Update;
use log::warn;

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
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner.handlers.push(handler)
        } else {
            warn!("Cannot register handler - dispatcher is already shared");
        }
    }

    pub async fn register_middleware(&self, middleware: Box<dyn Middleware>) {
        self.inner.middlewares.add(middleware).await;
    }

    pub async fn dispatch(&self, client: Arc<Client>, update: Arc<Update>) -> Result<()> {
        if !client.conversations.is_empty() {
            let chat_id = match update.as_ref() {
                Update::NewMessage(message) => Some(message.chat().id()),
                Update::CallbackQuery(data) => Some(data.chat().id()),
                _ => None,
            };

            if let Some(chat_id) = chat_id {
                if client.conversations.has_conversation(chat_id) {
                    client
                        .conversations
                        .handle_incoming_update(chat_id, Arc::clone(&update))?;
                    return Ok(());
                }
            }
        };

        if !self
            .inner
            .middlewares
            .execute_before(Arc::clone(&client), Arc::clone(&update))
            .await?
        {
            return Ok(());
        }

        let futures = self
            .inner
            .handlers
            .iter()
            .map(|handler| {
                let client = Arc::clone(&client);
                let update = Arc::clone(&update);

                async move { handler.handle(client, update).await }
            })
            .collect::<Vec<_>>();

        try_join_all(futures).await?;

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
