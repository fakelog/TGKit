mod command;
mod regex;
mod text;

use crate::types::PayloadItem;
use async_trait::async_trait;

pub use command::CommandRule;
use grammers_client::types::Message;
pub use regex::*;
pub use text::TextRule;

#[async_trait]
pub trait MessageRule: Send + Sync {
    async fn matches(&self, message: &Message) -> PayloadItem;
}
