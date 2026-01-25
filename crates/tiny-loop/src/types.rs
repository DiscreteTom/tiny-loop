use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
    #[serde(untagged)]
    Custom(String),
}

/// LLM message
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
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
    fn test_role_serde() {
        assert_eq!(serde_json::to_string(&Role::System).unwrap(), "\"system\"");
        assert_eq!(serde_json::to_string(&Role::User).unwrap(), "\"user\"");
        assert_eq!(
            serde_json::to_string(&Role::Assistant).unwrap(),
            "\"assistant\""
        );
        assert_eq!(serde_json::to_string(&Role::Tool).unwrap(), "\"tool\"");
        assert_eq!(
            serde_json::to_string(&Role::Custom("custom".into())).unwrap(),
            "\"custom\""
        );

        assert!(matches!(
            serde_json::from_str::<Role>("\"system\"").unwrap(),
            Role::System
        ));
        assert!(matches!(
            serde_json::from_str::<Role>("\"user\"").unwrap(),
            Role::User
        ));
        assert!(matches!(
            serde_json::from_str::<Role>("\"assistant\"").unwrap(),
            Role::Assistant
        ));
        assert!(matches!(
            serde_json::from_str::<Role>("\"tool\"").unwrap(),
            Role::Tool
        ));
        assert!(matches!(
            serde_json::from_str::<Role>("\"custom\"").unwrap(),
            Role::Custom(_)
        ));
    }

    #[test]
    fn test_message_with_role() {
        let msg = Message {
            role: Role::User,
            content: Some("test".into()),
            tool_calls: None,
            tool_call_id: None,
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"user\""));

        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed.role, Role::User));
    }
}
