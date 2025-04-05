use anyhow::Result;
use async_trait::async_trait;
use grammers_client::types::CallbackQuery;
use std::fmt::Debug;

use crate::{Client, rules::CallbackRule, types::Payload};

#[async_trait]
pub trait CallbackHandler: Sync + Send + Debug {
    async fn rules(&self) -> Vec<Box<dyn CallbackRule>>;
    async fn handle(&self, client: &Client, query: &CallbackQuery, payload: Payload) -> Result<()>;
}
