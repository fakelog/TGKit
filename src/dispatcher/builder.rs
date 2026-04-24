use std::sync::Arc;

use crate::{handlers::EventHandler, middleware::Middleware};
use anyhow::Result;

use super::EventDispatcher;

pub struct EventDispatcherBuilder {
    handlers: Vec<Arc<dyn EventHandler>>,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl EventDispatcherBuilder {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn with_handler(mut self, handler: Arc<dyn EventHandler>) -> Self {
        self.handlers.push(handler);
        self
    }

    pub fn with_middleware(mut self, middleware: Box<dyn Middleware>) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub fn with_handlers(mut self, handlers: Vec<Arc<dyn EventHandler>>) -> Self {
        self.handlers.extend(handlers);
        self
    }

    pub fn with_middlewares(mut self, middlewares: Vec<Box<dyn Middleware>>) -> Self {
        self.middlewares.extend(middlewares);
        self
    }

    pub fn build(self) -> Result<EventDispatcher> {
        Ok(EventDispatcher::with_parts(self.handlers, self.middlewares))
    }
}

impl Default for EventDispatcherBuilder {
    fn default() -> Self {
        Self::new()
    }
}
