use crate::middleware::{Middleware, MiddlewareContainer};

use super::{CallbackQueryHandler, callback_handler::CallbackHandler};

pub struct CallbackQueryHandlerBuilder {
    handlers: Vec<Box<dyn CallbackHandler>>,
    middlewares: MiddlewareContainer,
}

impl CallbackQueryHandlerBuilder {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            middlewares: MiddlewareContainer::new(),
        }
    }
    pub fn with_handler<H: CallbackHandler + 'static>(mut self, handler: H) -> Self {
        self.handlers.push(Box::new(handler));
        self
    }

    pub async fn with_middleware<M: Middleware + 'static>(self, middleware: M) -> Self {
        self.middlewares.add(Box::new(middleware)).await;
        self
    }

    pub fn build(self) -> CallbackQueryHandler {
        CallbackQueryHandler {
            handlers: self.handlers,
            middlewares: self.middlewares,
        }
    }
}

impl Default for CallbackQueryHandlerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
