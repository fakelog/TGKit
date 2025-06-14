use anyhow::Result;
use grammers_client::{Client as TGClient, types::Chat};
use log::{error, info};
use std::sync::Arc;
use tokio::select;

use crate::{
    builder::ClientBuilder,
    conversation::{Conversation, ConversationContainer},
    dispatcher::EventDispatcher,
};

pub struct Client {
    pub tg_client: TGClient,
    pub dispatcher: EventDispatcher,
    pub(crate) client_name: String,
    pub(crate) conversations: ConversationContainer,
}

impl Client {
    async fn auth_bot(&self, token: &str) -> Result<()> {
        if !self.tg_client.is_authorized().await? {
            info!("Signing in...");
            self.tg_client.bot_sign_in(token).await?;
            info!("Signed in!");

            self.tg_client.session().save_to_file(&self.client_name)?;
            info!("Session saved!");
        }

        Ok(())
    }

    async fn handle_update(self: Arc<Self>) -> Result<()> {
        info!("Starting update handling loop...");

        loop {
            let exit = tokio::signal::ctrl_c();
            let upd = self.tg_client.next_update();

            select! {
                _ = exit => {
                    info!("Received Ctrl+C, exiting...");
                    break;
                }
                update = upd => {
                    let client = Arc::clone(&self);
                    let dispatcher = self.dispatcher.clone();


                    tokio::spawn(async move {
                        match update {
                            Ok(update) => {
                                let update = Arc::new(update);
                                if let Err(e) = dispatcher.dispatch(client, update).await {
                                    error!("Error handling update: {}", e);
                                }
                            }
                            Err(e) => {
                                error!("Error receiving update: {}", e);
                            }
                        }
                    });
                }
            }
        }

        Ok(())
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn conversation(self: Arc<Self>, chat: Chat) -> Conversation {
        Conversation::new(self, chat)
    }

    pub async fn run_bot(self: Arc<Self>, token: String) -> Result<()> {
        self.auth_bot(&token).await?;

        info!("Bot is running...");
        if let Err(e) = self.handle_update().await {
            error!("Update handling error: {e}");
        }
        info!("Bot disconnected gracefully.");

        Ok(())
    }

    pub async fn run_user(self: Arc<Self>) -> Result<()> {
        info!("User bot is running...");
        if let Err(e) = self.handle_update().await {
            error!("Update handling error: {e}");
        }
        info!("User bot disconnected gracefully.");

        Ok(())
    }
}
