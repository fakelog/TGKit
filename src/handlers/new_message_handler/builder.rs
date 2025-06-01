use super::{NewMessageHandler, message_handler::MessageHandler, rule_checker::RuleChecker};

pub struct NewMessageHandlerBuilder {
    handlers: Vec<Box<dyn MessageHandler>>,
    rule_checker: RuleChecker,
}

impl NewMessageHandlerBuilder {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            rule_checker: RuleChecker::new().strict(true),
        }
    }
    pub fn with_handler<H: MessageHandler + 'static>(mut self, handler: H) -> Self {
        self.handlers.push(Box::new(handler));
        self
    }

    pub fn build(self) -> NewMessageHandler {
        NewMessageHandler {
            handlers: self.handlers,
            rule_checker: self.rule_checker,
        }
    }
}

impl Default for NewMessageHandlerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
