use std::borrow::Cow;

use crate::types::PayloadItem;

use super::MessageRule;
use async_trait::async_trait;
use grammers_client::types::Message;

pub struct TextRule {
    text: Cow<'static, str>,
    lower: bool,
}

impl TextRule {
    pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
        let text = text.into();
        Self { text, lower: true }
    }

    pub fn lower(mut self, value: bool) -> Self {
        self.lower = value;
        self
    }
}

#[async_trait]
impl MessageRule for TextRule {
    async fn matches(&self, message: &Message) -> PayloadItem {
        let message = message.text();

        let message_text = if self.lower {
            message.to_lowercase()
        } else {
            message.to_string()
        };

        Box::new(message_text == self.text) as PayloadItem
    }
}
