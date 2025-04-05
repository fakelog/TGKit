use std::borrow::Cow;

use async_trait::async_trait;

use crate::types::PayloadItem;

use super::MessageRule;

pub struct CommandRule {
    command: Cow<'static, str>,
    lower: bool,
}

impl CommandRule {
    pub fn new(command: impl Into<Cow<'static, str>>) -> Self {
        let command = command.into();
        Self {
            command,
            lower: true,
        }
    }

    pub fn lower(&mut self, value: bool) {
        self.lower = value;
    }
}

#[async_trait]
impl MessageRule for CommandRule {
    async fn matches(&self, message: &str) -> PayloadItem {
        let message_text = if self.lower {
            message.to_lowercase()
        } else {
            message.to_string()
        };

        Box::new(message_text.starts_with(&format!("/{}", self.command))) as PayloadItem
    }
}
