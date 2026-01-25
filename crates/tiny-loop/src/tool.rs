mod web;

use crate::types::{Message, ToolCall, ToolDefinition, ToolFunction};
use async_trait::async_trait;
use futures::future::join_all;
use schemars::{JsonSchema, schema_for};
use serde_json::Value;

/// Remove `$schema` and `title` fields from JSON schema
pub fn strip_schema_metadata(mut value: Value) -> Value {
    if let Some(obj) = value.as_object_mut() {
        obj.remove("$schema");
        obj.remove("title");
    }
    value
}

pub trait Definition {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    type Args: JsonSchema;

    fn definition() -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: Self::NAME.to_string(),
                description: Self::DESCRIPTION.to_string(),
                parameters: strip_schema_metadata(schema_for!(Self::Args).to_value()),
            },
        }
    }
}

#[async_trait]
pub trait CallableTool: Send + Sync {
    async fn call(&self, args: String) -> String;

    async fn call_batch(&self, args: Vec<ToolCall>) -> Vec<Message> {
        join_all(
            args.into_iter()
                .map(async |call| Message {
                    role: "tool".into(),
                    tool_call_id: Some(call.id),
                    tool_calls: None,
                    content: Some(self.call(call.function.arguments).await),
                })
                .collect::<Vec<_>>(),
        )
        .await
    }
}
