mod infinite;

use crate::types::Message;

pub use infinite::*;

/// Manages conversation history
pub trait History {
    /// Add a message to history
    fn add(&mut self, message: Message);

    /// Add multiple messages to history
    fn add_batch(&mut self, messages: Vec<Message>) {
        for msg in messages {
            self.add(msg);
        }
    }

    /// Get all messages in history
    fn get_all(&self) -> &[Message];
}
