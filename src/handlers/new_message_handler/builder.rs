use crate::middleware::{Middleware, MiddlewareContainer};

use super::{NewMessageHandler, message_handler::MessageHandler};

pub struct NewMessageHandlerBuilder {
    handlers: Vec<Box<dyn MessageHandler>>,
    middlewares: MiddlewareContainer,
}

impl NewMessageHandlerBuilder {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            middlewares: MiddlewareContainer::new(),
        }
    }
    pub fn with_handler<H: MessageHandler + 'static>(mut self, handler: H) -> Self {
        self.handlers.push(Box::new(handler));
        self
    }

    pub async fn with_middleware<M: Middleware + 'static>(self, middleware: M) -> Self {
        self.middlewares.add(Box::new(middleware)).await;
        self
    }

    pub fn build(self) -> NewMessageHandler {
        NewMessageHandler {
            handlers: self.handlers,
            middlewares: self.middlewares,
        }
    }
}

impl Default for NewMessageHandlerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
