use grammers_client::types::Message;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConversationState {
    pub last_message: Option<Message>,
}

impl ConversationState {
    pub fn new() -> Self {
        Self { last_message: None }
    }

    pub fn update_last_message(&mut self, message: Message) {
        self.last_message = Some(message);
    }
}
