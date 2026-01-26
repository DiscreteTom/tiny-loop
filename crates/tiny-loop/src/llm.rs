mod openai;

use crate::types::{Message, ToolDefinition};
use async_trait::async_trait;

pub use openai::*;

/// Callback for streaming LLM responses
pub type StreamCallback = Box<dyn FnMut(String) + Send>;

/// LLM provider trait for making API calls
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Call the LLM with messages and available tools, returning the assistant's response
    ///
    /// If `stream_callback` is provided, the LLM will be invoked in streaming mode,
    /// calling the callback for each chunk of the response as it arrives.
    async fn call(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
        stream_callback: Option<&mut StreamCallback>,
    ) -> anyhow::Result<Message>;
}
