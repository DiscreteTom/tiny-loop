use serde::{Deserialize, Serialize};
use serde_json::Value;

/// LLM message
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {
    System {
        content: String,
    },
    User {
        content: String,
    },
    Assistant {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
    #[serde(untagged)]
    Custom {
        role: String,
        #[serde(flatten)]
        body: Value,
    },
}

/// Tool call from LLM
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: FunctionCall,
}

/// Function call details
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// Tool definition for LLM
#[derive(Serialize, Clone)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolFunction,
}

/// Tool function definition
#[derive(Serialize, Clone)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
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
