use super::History;
use crate::types::Message;

/// Infinite history - never cleans history
pub struct InfiniteHistory {
    messages: Vec<Message>,
}

impl InfiniteHistory {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

impl Default for InfiniteHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl History for InfiniteHistory {
    fn add(&mut self, message: Message) {
        self.messages.push(message);
    }

    fn get_all(&self) -> &[Message] {
        &self.messages
    }
}
