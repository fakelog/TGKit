use anyhow::Result;
use async_trait::async_trait;
use grammers_client::{Client, types::Message};
use std::{any::Any, fmt::Debug};

use super::Rule;

#[async_trait]
pub trait MessageHandler: Sync + Send + Debug {
    async fn rules(&self) -> Vec<Box<dyn Rule>>;
    async fn handle(
        &self,
        client: &Client,
        message: &Message,
        payload: Vec<Box<dyn Any + Send>>,
    ) -> Result<()>;
}
