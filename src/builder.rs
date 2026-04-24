use std::sync::Arc;

use anyhow::{Context, Result};
use grammers_client::{
    Client as GrammersClient,
    client::ClientConfiguration,
    sender::{ConnectionParams, SenderPool},
    session::storages::SqliteSession,
};

use crate::{Client, conversation::ConversationContainer, dispatcher::EventDispatcher};

pub struct ClientBuilder {
    api_id: Option<i32>,
    api_hash: Option<String>,
    client_name: Option<String>,
    device_model: Option<String>,
    system_version: Option<String>,
    app_version: Option<String>,
    system_lang_code: Option<String>,
    lang_code: Option<String>,
    session: Option<Arc<SqliteSession>>,
    dispatcher: Option<EventDispatcher>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            api_id: None,
            api_hash: None,
            client_name: None,
            device_model: None,
            system_version: None,
            app_version: None,
            system_lang_code: None,
            lang_code: None,
            session: None,
            dispatcher: None,
        }
    }

    pub fn api_id(mut self, api_id: i32) -> Self {
        self.api_id = Some(api_id);
        self
    }

    pub fn api_hash(mut self, api_hash: String) -> Self {
        self.api_hash = Some(api_hash);
        self
    }

    pub fn client_name(mut self, client_name: String) -> Self {
        self.client_name = Some(client_name);
        self
    }

    pub fn device_model(mut self, device_model: String) -> Self {
        self.device_model = Some(device_model);
        self
    }

    pub fn system_version(mut self, system_version: String) -> Self {
        self.system_version = Some(system_version);
        self
    }

    pub fn app_version(mut self, app_version: String) -> Self {
        self.app_version = Some(app_version);
        self
    }

    pub fn system_lang_code(mut self, system_lang_code: String) -> Self {
        self.system_lang_code = Some(system_lang_code);
        self
    }

    pub fn lang_code(mut self, lang_code: String) -> Self {
        self.lang_code = Some(lang_code);
        self
    }

    pub fn session(mut self, session: Arc<SqliteSession>) -> Self {
        self.session = Some(session);
        self
    }

    pub fn dispatcher(mut self, dispatcher: EventDispatcher) -> Self {
        self.dispatcher = Some(dispatcher);
        self
    }

    pub async fn build(self) -> Result<Arc<Client>> {
        let api_id = self.api_id.context("API ID is required")?;
        let api_hash = self.api_hash.context("API hash is required")?;
        let client_name = self
            .client_name
            .unwrap_or_else(|| "tg-kit-session".to_string());
        let session = match self.session {
            Some(session) => session,
            None => Arc::new(SqliteSession::open(&client_name).await?),
        };

        let connection_params = ConnectionParams {
            device_model: self.device_model.unwrap_or_else(|| "Unknown".to_string()),
            system_version: self.system_version.unwrap_or_else(|| "1.0".to_string()),
            app_version: self.app_version.unwrap_or_else(|| "1.0".to_string()),
            system_lang_code: self.system_lang_code.unwrap_or_else(|| "en".to_string()),
            lang_code: self.lang_code.unwrap_or_else(|| "en".to_string()),
            ..Default::default()
        };

        let sender_pool = SenderPool::with_configuration(session, api_id, connection_params);
        let tg_client =
            GrammersClient::with_configuration(sender_pool.handle, ClientConfiguration::default());
        let updates = sender_pool.updates;
        tokio::spawn(sender_pool.runner.run());

        let client = Client {
            tg_client,
            dispatcher: self.dispatcher.context("EventDispatcher is required")?,
            api_hash,
            conversations: ConversationContainer::new(),
            updates: tokio::sync::Mutex::new(Some(updates)),
        };

        Ok(Arc::new(client))
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
