use async_trait::async_trait;
use grammers_client::types::update::Message;
use tg_kit::{rules::MessageRule, types::PayloadItem};

pub struct FromSenderRule {
    id: i64,
}

impl FromSenderRule {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

#[async_trait]
impl MessageRule for FromSenderRule {
    async fn matches(&self, message: &Message) -> PayloadItem {
        let sender = match message.sender() {
            Some(sender) => sender,
            None => return Box::new(false) as PayloadItem,
        };

        Box::new(sender.id() == self.id) as PayloadItem
    }
}
