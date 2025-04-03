mod builder;
use anyhow::Result;
use builder::EventDispatcherBuilder;
use grammers_client::{Client, Update};

use crate::{
    handlers::EventHandler,
    middleware::{Middleware, MiddlewareContainer},
};

pub struct EventDispatcher {
    handlers: Vec<Box<dyn EventHandler>>,
    middlewares: MiddlewareContainer,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> EventDispatcherBuilder {
        EventDispatcherBuilder::new()
    }

    pub fn register_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    pub async fn register_middleware(&self, middleware: Box<dyn Middleware>) {
        self.middlewares.add(middleware).await;
    }

    pub async fn dispatch(&self, client: &Client, update: &Update) -> Result<()> {
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
