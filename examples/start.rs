use anyhow::{Context, Result};
use async_trait::async_trait;
use dotenvy::dotenv;
use grammers_client::{Update, types::update::Message};
use log::LevelFilter;
use logforth::append;
use std::{env, sync::Arc};
use tg_kit::{
    Client,
    dispatcher::EventDispatcher,
    handlers::new_message_handler::{MessageHandler, NewMessageHandler},
    rules::MessageRule,
    types::Payload,
};
use tg_kit_rules::message::{CommandRule, OrRule, TextRule};

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
    logforth::builder()
        .dispatch(|d| {
            d.filter(LevelFilter::Info)
                .append(append::Stdout::default())
        })
        .apply();

    dotenv().ok();

    let api_hash = env::var("API_HASH").context("API_HASH not set")?;
    let api_id = env::var("API_ID")
        .context("API_ID not set")?
        .parse()
        .context("API_ID invalid")?;
    let token = env::var("TOKEN").context("TOKEN not set")?;

    let dispatcher = get_dispatcher().await?;

    let client = Client::builder()
        .api_hash(api_hash)
        .api_id(api_id)
        .client_name(SESSION_FILE.to_string())
        .dispatcher(dispatcher)
        .build()
        .await?;

    client.run_bot(token).await
}

#[derive(Debug)]
pub struct StartHandler;

#[async_trait]
impl MessageHandler for StartHandler {
    async fn rules(&self) -> Vec<Box<dyn MessageRule>> {
        let text_rule = TextRule::new("/start");
        let command_rule = CommandRule::new("starting");

        vec![Box::new(
            OrRule::new().add_rule(text_rule).add_rule(command_rule),
        )]
    }

    async fn handle(
        &self,
        client: Arc<Client>,
        message: &Message,
        _payload: Payload,
    ) -> Result<()> {
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
        vec![Box::new(CommandRule::new("reg"))]
    }

    async fn handle(
        &self,
        client: Arc<Client>,
        message: &Message,
        _payload: Payload,
    ) -> Result<()> {
        let mut conv = client.conversation(message.chat());
        conv.send_message("Как вас зовут?").await?;

        let response = conv.get_response().await?;
        if let Update::NewMessage(message) = response {
            message
                .reply(format!("Привет, {}!", message.text()))
                .await?;
        }

        Ok(())
    }
}
