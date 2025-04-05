use anyhow::{Context, Result};
use grammers_client::{Client as TGClient, Config, InitParams, types::Chat};
use grammers_session::Session;
use log::{error, info};
use tokio::select;

use crate::{
    conversation::{Conversation, ConversationContainer},
    dispatcher::EventDispatcher,
    utils::{BotReconnectionPolicy, save_session},
};

#[derive(Clone)]
pub struct Client {
    pub tg_client: TGClient,
    pub dispatcher: EventDispatcher,
    session_file: String,
    pub(crate) conversations: ConversationContainer,
}

impl Client {
    pub async fn new(
        api_hash: String,
        api_id: i32,
        session_file: String,
        token: String,
        dispatcher: EventDispatcher,
    ) -> Result<Self> {
        let session = Self::load_session(&session_file)?;
        let config = Self::build_config(api_id, api_hash, session, None);

        let tg_client = Self::connect(config).await?;
        Self::authenticate(&tg_client, &token, &session_file).await?;

        Ok(Self {
            tg_client,
            dispatcher,
            session_file,
            conversations: ConversationContainer::new(),
        })
    }

    async fn authenticate(tg_client: &TGClient, token: &str, session_file: &str) -> Result<()> {
        if !tg_client.is_authorized().await? {
            info!("Signing in...");
            tg_client.bot_sign_in(token).await?;
            let _ = save_session(tg_client, session_file);
            info!("Signed in!");
        }

        Ok(())
    }

    fn build_config(
        api_id: i32,
        api_hash: String,
        session: Session,
        params: Option<InitParams>,
    ) -> Config {
        let params = if let Some(params) = params {
            params
        } else {
            InitParams {
                catch_up: false,
                reconnection_policy: &BotReconnectionPolicy,
                ..Default::default()
            }
        };

        Config {
            session,
            api_id,
            api_hash,
            params,
        }
    }

    async fn connect(config: Config) -> Result<TGClient> {
        info!("Connecting to Telegram...");
        TGClient::connect(config)
            .await
            .context("Failed to connect to Telegram API")
    }

    async fn handle_update(&self) -> Result<()> {
        loop {
            let exit = tokio::signal::ctrl_c();
            let upd = self.tg_client.next_update();

            select! {
                _ = exit => {
                    info!("Received Ctrl+C, exiting...");
                    break;
                }
                update = upd => {
                    match update {
                        Ok(update) => {
                            let client = self.clone();
                            let dispatcher = self.dispatcher.clone();
                            tokio::task::spawn(async move {
                                if let Err(e) = dispatcher.dispatch(&client, &update).await {
                                    error!("Error handling update: {}", e);
                                }
                            });

                        }
                        Err(e) => {
                            error!("Error receiving update: {}", e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn load_session(session_file: &str) -> Result<Session> {
        Session::load_file_or_create(session_file).context("Failed to load or create session")
    }

    pub fn conversation(&self, chat: Chat) -> Conversation {
        Conversation::new(self, chat)
    }

    pub async fn run(&self) -> Result<()> {
        info!("Bot is running...");
        if let Err(e) = self.handle_update().await {
            error!("Update handling error: {e}");
        }
        info!("Bot disconnected gracefully.");

        Ok(())
    }

    fn shutdown(&self) {
        info!("Shutting down bot...");
        let _ = save_session(&self.tg_client, &self.session_file);
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.shutdown();
    }
}
