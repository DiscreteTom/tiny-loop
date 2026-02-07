use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{Duration, SystemTime};

/// System message body
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemMessage {
    /// Message content
    pub content: String,
}

/// User message body
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserMessage {
    /// Message content
    pub content: String,
}

/// Assistant message body
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssistantMessage {
    /// Message content
    pub content: String,
    /// Tool calls requested by the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Tool message body
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolMessage {
    /// Tool execution result content
    pub content: String,
    /// ID of the tool call this responds to
    pub tool_call_id: String,
}

/// Custom message body
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomMessage {
    /// Custom role name
    pub role: String,
    /// Custom message body
    #[serde(flatten)]
    pub body: Value,
}

/// LLM message with role-specific fields
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {
    /// System message with instructions
    System(SystemMessage),
    /// User message with input
    User(UserMessage),
    /// Assistant message with response and optional tool calls
    Assistant(AssistantMessage),
    /// Tool execution result
    Tool(ToolMessage),
    /// Custom role with arbitrary fields
    #[serde(untagged)]
    Custom(CustomMessage),
}

impl From<SystemMessage> for Message {
    fn from(msg: SystemMessage) -> Self {
        Message::System(msg)
    }
}

impl From<UserMessage> for Message {
    fn from(msg: UserMessage) -> Self {
        Message::User(msg)
    }
}

impl From<AssistantMessage> for Message {
    fn from(msg: AssistantMessage) -> Self {
        Message::Assistant(msg)
    }
}

impl From<ToolMessage> for Message {
    fn from(msg: ToolMessage) -> Self {
        Message::Tool(msg)
    }
}

impl From<CustomMessage> for Message {
    fn from(msg: CustomMessage) -> Self {
        Message::Custom(msg)
    }
}

/// Tool call from LLM
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolCall {
    /// Unique identifier for this tool call
    pub id: String,
    /// Type of the call (typically "function")
    #[serde(rename = "type")]
    pub call_type: String,
    /// Function call details
    pub function: FunctionCall,
}

/// Function call details
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionCall {
    /// Function name to call
    pub name: String,
    /// JSON-encoded function arguments
    pub arguments: String,
}

/// Message with timing metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimedMessage {
    pub message: Message,
    /// When the message was created
    pub timestamp: SystemTime,
    /// Time taken to generate this message
    pub elapsed: Duration,
}

/// Tool execution result with timing metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolResult {
    pub tool_message: ToolMessage,
    /// When the tool execution started
    pub timestamp: SystemTime,
    /// Time taken to execute the tool
    pub elapsed: Duration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_roundtrip() {
        let msg = Message::System(SystemMessage {
            content: "test".into(),
        });
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, Message::System(SystemMessage { content }) if content == "test"));
    }

    #[test]
    fn test_user_roundtrip() {
        let msg = Message::User(UserMessage {
            content: "test".into(),
        });
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, Message::User(UserMessage { content }) if content == "test"));
    }

    #[test]
    fn test_assistant_no_tools_roundtrip() {
        let msg = Message::Assistant(AssistantMessage {
            content: "test".into(),
            tool_calls: None,
        });
        let json = serde_json::to_string(&msg).unwrap();
        assert!(!json.contains("tool_calls"));
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(
            matches!(parsed, Message::Assistant(AssistantMessage { content, tool_calls: None }) if content == "test")
        );
    }

    #[test]
    fn test_assistant_with_tools_roundtrip() {
        let msg = Message::Assistant(AssistantMessage {
            content: "test".into(),
            tool_calls: Some(vec![ToolCall {
                id: "call_1".into(),
                call_type: "function".into(),
                function: FunctionCall {
                    name: "fn".into(),
                    arguments: "{}".into(),
                },
            }]),
        });
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(
            matches!(parsed, Message::Assistant(AssistantMessage { tool_calls: Some(calls), .. }) if calls.len() == 1)
        );
    }

    #[test]
    fn test_tool_roundtrip() {
        let msg = Message::Tool(ToolMessage {
            content: "result".into(),
            tool_call_id: "call_123".into(),
        });
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(
            matches!(parsed, Message::Tool(ToolMessage { content, tool_call_id })
            if content == "result" && tool_call_id == "call_123")
        );
    }

    #[test]
    fn test_custom_roundtrip() {
        let msg = Message::Custom(CustomMessage {
            role: "custom".into(),
            body: serde_json::json!({"content": "test", "extra": "field"}),
        });
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, Message::Custom(CustomMessage { role, .. }) if role == "custom"));
    }

    #[test]
    fn test_tool_call_roundtrip() {
        let tc = ToolCall {
            id: "call_1".into(),
            call_type: "function".into(),
            function: FunctionCall {
                name: "test".into(),
                arguments: r#"{"key":"value"}"#.into(),
            },
        };
        let json = serde_json::to_string(&tc).unwrap();
        let parsed: ToolCall = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, "call_1");
        assert_eq!(parsed.function.name, "test");
    }
}
