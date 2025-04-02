use async_trait::async_trait;
use std::any::Any;

#[async_trait]
pub trait Rule: Send + Sync {
    async fn matches(&self, message: &str) -> Box<dyn Any + Send>;
}
