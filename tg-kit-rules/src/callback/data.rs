use std::borrow::Cow;

use async_trait::async_trait;
use grammers_client::types::update::CallbackQuery;
use tg_kit::{rules::CallbackRule, types::PayloadItem};

pub struct DataRule {
    data: Cow<'static, str>,
}

impl DataRule {
    pub fn new(data: impl Into<Cow<'static, str>>) -> Self {
        let data = data.into();
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
