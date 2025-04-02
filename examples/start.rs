use anyhow::Result;
use async_trait::async_trait;
use dotenvy::dotenv;
use grammers_client::{Client as GClient, types::Message};
use linkme::distributed_slice;
use new_message_event_handler::{
    MESSAGES_HANDLERS, NewMessageHandler,
    types::{MessageHandler, Rule},
};
use std::{any::Any, env};
use tg_kit::{Client, EventDispatcher};

const SESSION_FILE: &str = "example.session";

async fn get_dispatcher() -> EventDispatcher {
    let mut event_dispatcher = EventDispatcher::new();
    event_dispatcher.register_handler(Box::new(NewMessageHandler));

    event_dispatcher
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_hash = env::var("API_HASH").expect("API_HASH not set");
    let api_id = env::var("API_ID")
        .expect("API_ID not set")
        .parse()
        .expect("API_ID invalid");
    let token = env::var("TOKEN").expect("TOKEN not set");
    let dispatcher = get_dispatcher().await;

    let client = Client::new(
        api_hash,
        api_id,
        SESSION_FILE.to_string(),
        token,
        dispatcher,
    )
    .await
    .unwrap();

    let _ = client.run().await;
}

// Create a new message handler
#[derive(Debug)]
pub struct StartHandler;

#[async_trait]
impl MessageHandler for StartHandler {
    async fn rules(&self) -> Vec<Box<dyn Rule>> {
        vec![Box::new(TextRule::new("/start".to_string()))]
    }

    async fn handle(
        &self,
        client: &GClient,
        message: &Message,
        _payload: Vec<Box<dyn Any + Send>>,
    ) -> Result<()> {
        client.send_message(message.chat(), "Тест").await?;

        Ok(())
    }
}

// Register the handler
#[distributed_slice(MESSAGES_HANDLERS)]
static START_MESSAGE_HANDLER: &dyn MessageHandler = &StartHandler;

// Create a rule to match the message
pub struct TextRule {
    pattern: String,
}

impl TextRule {
    pub fn new(pattern: String) -> Self {
        TextRule { pattern }
    }
}

#[async_trait]
impl Rule for TextRule {
    async fn matches(&self, text: &str) -> Box<dyn Any + Send> {
        Box::new(text == self.pattern) as Box<dyn Any + Send>
    }
}
