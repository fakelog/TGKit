mod builder;
use std::sync::Arc;

use anyhow::Result;
use builder::EventDispatcherBuilder;
use futures::future::try_join_all;
use grammers_client::update::Update;

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
        Self::with_parts(Vec::new(), Vec::new())
    }

    pub(crate) fn with_parts(
        handlers: Vec<Arc<dyn EventHandler>>,
        middlewares: Vec<Box<dyn Middleware>>,
    ) -> Self {
        Self {
            inner: Arc::new(EventDispatcherInner {
                handlers,
                middlewares: MiddlewareContainer::new(middlewares),
            }),
        }
    }

    pub fn builder() -> EventDispatcherBuilder {
        EventDispatcherBuilder::new()
    }

    pub async fn dispatch(&self, client: Arc<Client>, update: Arc<Update>) -> Result<()> {
        if !client.conversations.is_empty() {
            let chat_id = match update.as_ref() {
                Update::NewMessage(message) => Some(message.peer_id()),
                Update::CallbackQuery(data) => Some(data.peer_id()),
                _ => None,
            };

            if let Some(chat_id) = chat_id
                && client.conversations.has_conversation(chat_id)
            {
                client
                    .conversations
                    .handle_incoming_update(chat_id, Arc::clone(&update))?;
                return Ok(());
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
