pub mod callback_query_handler;
pub mod new_message_handler;

use crate::middleware::MiddlewareContainer;
use anyhow::Result;
use async_trait::async_trait;
use grammers_client::{Client, Update};

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn middlewares(&self) -> MiddlewareContainer;
    async fn handle(&self, client: &Client, update: &Update) -> Result<()>;
}
