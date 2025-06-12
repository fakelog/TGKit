use anyhow::Result;
use async_trait::async_trait;
use grammers_client::types::update::CallbackQuery;
use std::{fmt::Debug, sync::Arc};

use crate::{Client, rules::CallbackRule, types::Payload};

#[async_trait]
pub trait CallbackHandler: Sync + Send + Debug {
    async fn rules(&self) -> Vec<Box<dyn CallbackRule>>;
    async fn handle(
        &self,
        client: Arc<Client>,
        query: &CallbackQuery,
        payload: Payload,
    ) -> Result<()>;
}
