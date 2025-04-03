use crate::types::PayloadItem;

use super::MessageRule;
use async_trait::async_trait;

pub struct TextRule {
    pattern: String,
    lower: bool,
}

impl TextRule {
    pub fn new(pattern: String) -> Self {
        TextRule {
            pattern,
            lower: true,
        }
    }

    pub fn lower(&mut self, value: bool) {
        self.lower = value;
    }
}

#[async_trait]
impl MessageRule for TextRule {
    async fn matches(&self, message: &str) -> PayloadItem {
        let message_text = if self.lower {
            message.to_lowercase()
        } else {
            message.to_string()
        };

        Box::new(message_text == self.pattern) as PayloadItem
    }
}
