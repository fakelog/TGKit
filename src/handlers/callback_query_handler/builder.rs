use super::{CallbackQueryHandler, callback_handler::CallbackHandler};

pub struct CallbackQueryHandlerBuilder {
    handlers: Vec<Box<dyn CallbackHandler>>,
}

impl CallbackQueryHandlerBuilder {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }
    pub fn with_handler<H: CallbackHandler + 'static>(mut self, handler: H) -> Self {
        self.handlers.push(Box::new(handler));
        self
    }

    pub fn build(self) -> CallbackQueryHandler {
        CallbackQueryHandler {
            handlers: self.handlers,
        }
    }
}

impl Default for CallbackQueryHandlerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
