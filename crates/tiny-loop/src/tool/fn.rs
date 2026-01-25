use crate::types::{Parameters, ToolDefinition, ToolFunction};
use schemars::JsonSchema;
use serde::Deserialize;

pub trait FnToolArgs: JsonSchema + for<'a> Deserialize<'a> {
    const TOOL_NAME: &'static str;
    const TOOL_DESCRIPTION: &'static str;

    fn definition() -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: Self::TOOL_NAME.to_string(),
                description: Self::TOOL_DESCRIPTION.to_string(),
                parameters: Parameters::from_type::<Self>(),
            },
        }
    }
}
