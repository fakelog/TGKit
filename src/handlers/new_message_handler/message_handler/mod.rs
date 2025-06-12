use anyhow::Result;
use async_trait::async_trait;
use grammers_client::types::update::Message;
use std::{fmt::Debug, sync::Arc};

use crate::{Client, rules::MessageRule, types::Payload};

#[async_trait]
pub trait MessageHandler: Sync + Send + Debug {
    async fn rules(&self) -> Vec<Box<dyn MessageRule>>;
    async fn handle(&self, client: Arc<Client>, message: &Message, payload: Payload) -> Result<()>;
}
