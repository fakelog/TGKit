use grammers_client::types::{Chat, Message};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConversationState {
    pub chat: Chat,
    pub last_message: Option<Message>,
}

impl ConversationState {
    pub fn new(chat: Chat) -> Self {
        Self {
            chat,
            last_message: None,
        }
    }

    pub fn update_last_message(&mut self, message: Message) {
        self.last_message = Some(message);
    }
}
