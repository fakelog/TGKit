use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use grammers_client::update::Update;

use crate::Client;

#[async_trait]
pub trait Middleware: Send + Sync {
    async fn before(&self, client: Arc<Client>, update: Arc<Update>) -> Result<bool>;
    async fn after(&self, client: Arc<Client>, update: Arc<Update>) -> Result<()>;
}

#[derive(Clone)]
pub struct MiddlewareContainer {
    middlewares: Arc<Vec<Box<dyn Middleware>>>,
}

impl Default for MiddlewareContainer {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl MiddlewareContainer {
    pub fn new(middlewares: Vec<Box<dyn Middleware>>) -> Self {
        Self {
            middlewares: Arc::new(middlewares),
        }
    }

    pub async fn execute_before(&self, client: Arc<Client>, update: Arc<Update>) -> Result<bool> {
        for middleware in self.middlewares.iter() {
            let client = Arc::clone(&client);
            let update = Arc::clone(&update);

            if !middleware.before(client, update).await? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub async fn execute_after(&self, client: Arc<Client>, update: Arc<Update>) -> Result<()> {
        for middleware in self.middlewares.iter() {
            let client = Arc::clone(&client);
            let update = Arc::clone(&update);

            middleware.after(client, update).await?;
        }
        Ok(())
    }
}
