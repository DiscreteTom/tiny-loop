use super::History;
use crate::types::TimedMessage;

/// Infinite history - never cleans history
pub struct InfiniteHistory {
    messages: Vec<TimedMessage>,
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
    fn add(&mut self, message: TimedMessage) {
        self.messages.push(message);
    }

    fn get_all(&self) -> &[TimedMessage] {
        &self.messages
    }
}
