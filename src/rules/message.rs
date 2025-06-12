use async_trait::async_trait;
use grammers_client::types::update::Message;

use crate::types::PayloadItem;

#[async_trait]
pub trait MessageRule: Send + Sync {
    async fn matches(&self, message: &Message) -> PayloadItem;
}
