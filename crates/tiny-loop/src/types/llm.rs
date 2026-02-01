use super::message::AssistantMessage;
use serde::{Deserialize, Serialize};

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
    pub message: AssistantMessage,
    pub finish_reason: FinishReason,
}
