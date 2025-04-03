use anyhow::Result;
use async_trait::async_trait;
use dotenvy::dotenv;
use grammers_client::{Client as GClient, types::Message};

use std::{env, sync::Arc};
use tg_kit::{
    Client,
    dispatcher::EventDispatcher,
    handlers::new_message_handler::{MessageHandler, NewMessageHandler},
    rules::{MessageRule, RegexRule, TextRule},
    types::Payload,
};

const SESSION_FILE: &str = "example.session";

async fn get_dispatcher() -> Result<EventDispatcher> {
    let message_handler = NewMessageHandler::builder()
        .with_handler(StartHandler {})
        .build();

    EventDispatcher::builder()
        .with_handler(Arc::new(message_handler))
        .build()
        .await
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_hash = env::var("API_HASH").expect("API_HASH not set");
    let api_id = env::var("API_ID")
        .expect("API_ID not set")
        .parse()
        .expect("API_ID invalid");
    let token = env::var("TOKEN").expect("TOKEN not set");
    let dispatcher = get_dispatcher().await?;

    let client = Client::new(
        api_hash,
        api_id,
        SESSION_FILE.to_string(),
        token,
        dispatcher,
    )
    .await
    .unwrap();

    client.run().await
}

// Create a new message handler
#[derive(Debug)]
pub struct StartHandler;

#[async_trait]
impl MessageHandler for StartHandler {
    async fn rules(&self) -> Vec<Box<dyn MessageRule>> {
        vec![
            Box::new(TextRule::new("/start".to_string())),
            Box::new(RegexRule::new(r"/start").unwrap()),
        ]
    }

    async fn handle(&self, client: &GClient, message: &Message, _payload: Payload) -> Result<()> {
        client.send_message(message.chat(), "Тест").await?;

        Ok(())
    }
}
