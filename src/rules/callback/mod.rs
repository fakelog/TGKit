mod data;

use async_trait::async_trait;
use grammers_client::types::CallbackQuery;

use crate::types::PayloadItem;

pub use data::DataRule;

#[async_trait]
pub trait CallbackRule: Send + Sync {
    async fn matches(&self, query: &CallbackQuery) -> PayloadItem;
}
