mod closure;
mod web;

use crate::types::{Message, ToolCall, ToolDefinition, ToolFunction};
use async_trait::async_trait;
use futures::future::join_all;
use schemars::{JsonSchema, schema_for};
use serde::Deserialize;
use serde_json::Value;

pub use closure::*;
pub use web::*;

/// Remove `$schema` and `title` fields from JSON schema
pub fn strip_schema_metadata(mut value: Value) -> Value {
    if let Some(obj) = value.as_object_mut() {
        obj.remove("$schema");
        obj.remove("title");
    }
    value
}

pub trait FnToolArgs: JsonSchema + for<'a> Deserialize<'a> {
    const TOOL_NAME: &'static str;
    const TOOL_DESCRIPTION: &'static str;

    fn definition() -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: Self::TOOL_NAME.to_string(),
                description: Self::TOOL_DESCRIPTION.to_string(),
                parameters: strip_schema_metadata(schema_for!(Self).to_value()),
            },
        }
    }
}

/// A trait for tools that can be called with JSON string arguments.
///
/// Implementors must provide the `call` method.
/// The framework only uses `call_batch` and never calls `call` directly.
///
/// The default `call_batch` implementation executes tools in parallel.
/// Implementors may override it to customize this behavior.
#[async_trait]
pub trait Tool {
    /// Calls the tool with JSON arguments and returns the result.
    /// Used by the default `call_batch` implementation.
    async fn call(&self, args: String) -> String;

    /// Executes multiple tool calls in parallel. Override to customize execution behavior.
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
