use crate::types::{Message, ToolDefinition};

pub mod openai;

/// LLM provider trait for making API calls
pub trait LLMProvider {
    async fn call(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
