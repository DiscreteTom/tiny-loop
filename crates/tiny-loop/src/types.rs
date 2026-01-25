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

/// Tool definition for LLM
#[derive(Serialize, Clone)]
pub struct ToolDefinition {
    /// Type of the tool (typically "function")
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function definition
    pub function: ToolFunction,
}

/// Tool function definition
#[derive(Serialize, Clone)]
pub struct ToolFunction {
    /// Function name
    pub name: String,
    /// Function description
    pub description: String,
    /// JSON schema for function parameters
    pub parameters: Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_system() {
        let msg = Message::System {
            content: "test".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"system\""));
        assert!(json.contains("\"content\":\"test\""));
    }

    #[test]
    fn test_message_user() {
        let msg = Message::User {
            content: "test".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"user\""));
        assert!(json.contains("\"content\":\"test\""));
    }

    #[test]
    fn test_message_assistant() {
        let msg = Message::Assistant {
            content: "test".into(),
            tool_calls: None,
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"assistant\""));
        assert!(json.contains("\"content\":\"test\""));
    }

    #[test]
    fn test_message_tool() {
        let msg = Message::Tool {
            content: "result".into(),
            tool_call_id: "call_123".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"tool\""));
        assert!(json.contains("\"content\":\"result\""));
        assert!(json.contains("\"tool_call_id\":\"call_123\""));
    }

    #[test]
    fn test_message_custom() {
        let msg = Message::Custom {
            role: "custom".into(),
            body: serde_json::json!({"content": "test", "extra": "field"}),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"custom\""));
        assert!(json.contains("\"content\":\"test\""));
        assert!(json.contains("\"extra\":\"field\""));
    }
}
