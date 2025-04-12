pub mod callback_query_handler;
pub mod new_message_handler;

use std::sync::Arc;

use crate::{Client, middleware::MiddlewareContainer};
use anyhow::Result;
use async_trait::async_trait;
use grammers_client::Update;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn middlewares(&self) -> MiddlewareContainer;
    async fn handle(&self, client: Arc<Client>, update: &Update) -> Result<()>;
}
