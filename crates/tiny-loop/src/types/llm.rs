use super::message::Message;
use serde::{Deserialize, Serialize};

/// Callback for streaming LLM responses
pub type StreamCallback = Box<dyn FnMut(String) + Send>;

/// Finish reason for LLM completion
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    #[serde(untagged)]
    Custom(String),
}

/// LLM response containing message and finish reason
#[derive(Debug, Clone)]
pub struct LLMResponse {
    pub message: Message,
    pub finish_reason: FinishReason,
}
