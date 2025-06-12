use grammers_client::{
    Update,
    types::{update::CallbackQuery, update::Message},
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConversationState {
    pub last_data: Option<CallbackQuery>,
    pub last_message: Option<Message>,
}

impl ConversationState {
    pub fn new() -> Self {
        Self {
            last_data: None,
            last_message: None,
        }
    }

    pub fn update_last_update(&mut self, update: &Update) {
        match update {
            Update::NewMessage(message) => {
                self.last_message = Some(message.clone());
            }
            Update::CallbackQuery(data) => {
                self.last_data = Some(data.clone());
            }
            _ => {}
        }
    }
}

impl Default for ConversationState {
    fn default() -> Self {
        Self::new()
    }
}
