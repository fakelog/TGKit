use async_trait::async_trait;
use grammers_client::types::Message;

use crate::types::PayloadItem;

use super::MessageRule;

pub struct OrRule {
    rules: Vec<Box<dyn MessageRule + Send + Sync>>,
}

impl OrRule {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule<R>(mut self, rule: R) -> Self
    where
        R: MessageRule + Send + Sync + 'static,
    {
        self.rules.push(Box::new(rule));
        self
    }
}

impl Default for OrRule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MessageRule for OrRule {
    async fn matches(&self, message: &Message) -> PayloadItem {
        for rule in &self.rules {
            let result = rule.matches(message).await;

            if result.downcast_ref::<bool>().copied().unwrap_or(true) {
                return result;
            }
        }

        Box::new(false)
    }
}
