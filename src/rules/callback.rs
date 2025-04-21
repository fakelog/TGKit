use async_trait::async_trait;
use grammers_client::types::CallbackQuery;

use crate::types::PayloadItem;

#[async_trait]
pub trait CallbackRule: Send + Sync {
    async fn matches(&self, query: &CallbackQuery) -> PayloadItem;
}
