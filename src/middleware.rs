use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use grammers_client::Update;
use tokio::sync::RwLock;

use crate::Client;

#[async_trait]
pub trait Middleware: Send + Sync {
    async fn before(&self, client: Arc<Client>, update: Arc<Update>) -> Result<bool>;
    async fn after(&self, client: Arc<Client>, update: Arc<Update>) -> Result<()>;
}

#[derive(Clone)]
pub struct MiddlewareContainer {
    middlewares: Arc<RwLock<Vec<Box<dyn Middleware>>>>,
}

impl Default for MiddlewareContainer {
    fn default() -> Self {
        Self {
            middlewares: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl MiddlewareContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add(&self, middleware: Box<dyn Middleware>) {
        self.middlewares.write().await.push(middleware);
    }

    pub async fn execute_before(&self, client: Arc<Client>, update: Arc<Update>) -> Result<bool> {
        let middlewares = self.middlewares.read().await;
        for middleware in middlewares.iter() {
            let client = Arc::clone(&client);
            let update = Arc::clone(&update);

            if !middleware.before(client, update).await? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub async fn execute_after(&self, client: Arc<Client>, update: Arc<Update>) -> Result<()> {
        let middlewares = self.middlewares.read().await;
        for middleware in middlewares.iter() {
            let client = Arc::clone(&client);
            let update = Arc::clone(&update);

            middleware.after(client, update).await?;
        }
        Ok(())
    }
}
