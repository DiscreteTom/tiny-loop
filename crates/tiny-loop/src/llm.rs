mod openai;

use crate::types::{LLMResponse, Message, ToolDefinition};
use async_trait::async_trait;

pub use openai::*;

/// LLM provider trait for making API calls
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Call the LLM with messages and available tools, returning the assistant's response
    async fn call(
        &mut self,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> crate::Result<LLMResponse>;
}
