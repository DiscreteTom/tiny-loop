use crate::types::{Message, ToolDefinition};
use async_trait::async_trait;

pub mod openai;

/// LLM provider trait for making API calls
#[async_trait]
pub trait LLMProvider {
    async fn call(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
