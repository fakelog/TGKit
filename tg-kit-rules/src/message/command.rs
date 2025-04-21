use async_trait::async_trait;
use grammers_client::types::Message;
use std::borrow::Cow;
use tg_kit::{rules::MessageRule, types::PayloadItem};

pub struct CommandRule {
    command: Cow<'static, str>,
    lower: bool,
    prefixes: Vec<Cow<'static, str>>,
}

impl CommandRule {
    pub fn new(command: impl Into<Cow<'static, str>>) -> Self {
        let command = command.into();

        Self {
            command,
            lower: true,
            prefixes: vec!["/".into()],
        }
    }

    pub fn lower(mut self, value: bool) -> Self {
        self.lower = value;
        self
    }

    pub fn prefixes<I, P>(mut self, prefixes: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<Cow<'static, str>>,
    {
        self.prefixes = prefixes.into_iter().map(|p| p.into()).collect();
        self
    }

    pub fn add_prefix<P>(mut self, prefix: P) -> Self
    where
        P: Into<Cow<'static, str>>,
    {
        self.prefixes.push(prefix.into());
        self
    }
}

#[async_trait]
impl MessageRule for CommandRule {
    async fn matches(&self, message: &Message) -> PayloadItem {
        let message = message.text();

        let message = if self.lower {
            message.to_lowercase()
        } else {
            message.to_string()
        };

        let is_match = self.prefixes.iter().any(|prefix| {
            let full_command = format!("{prefix}{}", self.command);

            message == full_command || message.starts_with(&format!("{full_command}@"))
        });

        Box::new(is_match) as PayloadItem
    }
}
