mod command;
mod or;
mod regex;
mod text;

use async_trait::async_trait;
use grammers_client::types::Message;

use crate::types::PayloadItem;

pub use command::CommandRule;
pub use or::OrRule;
pub use regex::*;
pub use text::TextRule;

#[async_trait]
pub trait MessageRule: Send + Sync {
    async fn matches(&self, message: &Message) -> PayloadItem;
}
