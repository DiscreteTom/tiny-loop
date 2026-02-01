use serde::{Deserialize, Serialize};
use serde_json::Value;

/// LLM message with role-specific fields
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {
    /// System message with instructions
    System {
        /// Message content
        content: String,
    },
    /// User message with input
    User {
        /// Message content
        content: String,
    },
    /// Assistant message with response and optional tool calls
    Assistant {
        /// Message content
        content: String,
        /// Tool calls requested by the assistant
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },
    /// Tool execution result
    Tool {
        /// Tool execution result content
        content: String,
        /// ID of the tool call this responds to
        tool_call_id: String,
    },
    /// Custom role with arbitrary fields
    #[serde(untagged)]
    Custom {
        /// Custom role name
        role: String,
        /// Custom message body
        #[serde(flatten)]
        body: Value,
    },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_roundtrip() {
        let msg = Message::System {
            content: "test".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, Message::System { content } if content == "test"));
    }

    #[test]
    fn test_user_roundtrip() {
        let msg = Message::User {
            content: "test".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, Message::User { content } if content == "test"));
    }

    #[test]
    fn test_assistant_no_tools_roundtrip() {
        let msg = Message::Assistant {
            content: "test".into(),
            tool_calls: None,
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(!json.contains("tool_calls"));
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(
            matches!(parsed, Message::Assistant { content, tool_calls: None } if content == "test")
        );
    }

    #[test]
    fn test_assistant_with_tools_roundtrip() {
        let msg = Message::Assistant {
            content: "test".into(),
            tool_calls: Some(vec![ToolCall {
                id: "call_1".into(),
                call_type: "function".into(),
                function: FunctionCall {
                    name: "fn".into(),
                    arguments: "{}".into(),
                },
            }]),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(
            matches!(parsed, Message::Assistant { tool_calls: Some(calls), .. } if calls.len() == 1)
        );
    }

    #[test]
    fn test_tool_roundtrip() {
        let msg = Message::Tool {
            content: "result".into(),
            tool_call_id: "call_123".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, Message::Tool { content, tool_call_id }
            if content == "result" && tool_call_id == "call_123"));
    }

    #[test]
    fn test_custom_roundtrip() {
        let msg = Message::Custom {
            role: "custom".into(),
            body: serde_json::json!({"content": "test", "extra": "field"}),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, Message::Custom { role, .. } if role == "custom"));
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
