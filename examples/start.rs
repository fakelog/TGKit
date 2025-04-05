use anyhow::Result;
use async_trait::async_trait;
use dotenvy::dotenv;
use grammers_client::types::Message;

use std::{env, sync::Arc};
use tg_kit::{
    Client,
    dispatcher::EventDispatcher,
    handlers::new_message_handler::{MessageHandler, NewMessageHandler},
    rules::{CommandRule, MessageRule, TextRule},
    types::Payload,
};

const SESSION_FILE: &str = "example.session";

async fn get_dispatcher() -> Result<EventDispatcher> {
    let message_handler = NewMessageHandler::builder()
        .with_handler(StartHandler)
        .with_handler(RegHandler)
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

#[derive(Debug)]
pub struct StartHandler;

#[async_trait]
impl MessageHandler for StartHandler {
    async fn rules(&self) -> Vec<Box<dyn MessageRule>> {
        vec![Box::new(TextRule::new("/start".to_string()))]
    }

    async fn handle(&self, client: &Client, message: &Message, _payload: Payload) -> Result<()> {
        let tg_client = &client.tg_client;
        tg_client
            .send_message(
                message.chat(),
                "Для продолжения вам нужно зарегестрироваться! /reg",
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct RegHandler;

#[async_trait]
impl MessageHandler for RegHandler {
    async fn rules(&self) -> Vec<Box<dyn MessageRule>> {
        vec![Box::new(CommandRule::new("reg".to_string()))]
    }

    async fn handle(&self, client: &Client, message: &Message, _payload: Payload) -> Result<()> {
        let conv = client.conversation(message.chat());
        conv.send_message("Как вас зовут?").await?;

        let response = conv.get_response().await?;
        response
            .reply(format!("Привет, {}!", response.text()))
            .await?;

        Ok(())
    }
}
