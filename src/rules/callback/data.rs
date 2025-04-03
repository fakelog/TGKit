use crate::types::PayloadItem;

use super::CallbackRule;
use async_trait::async_trait;
use grammers_client::types::CallbackQuery;

pub struct DataRule {
    data: String,
}

impl DataRule {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

#[async_trait]
impl CallbackRule for DataRule {
    async fn matches(&self, query: &CallbackQuery) -> PayloadItem {
        let data = std::str::from_utf8(query.data()).unwrap();

        Box::new(data == self.data) as PayloadItem
    }
}
