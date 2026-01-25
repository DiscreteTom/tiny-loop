mod closure;
mod r#fn;
mod web;

use crate::types::{Message, ToolCall};
use async_trait::async_trait;
use futures::future::join_all;

pub use closure::*;
pub use r#fn::*;
pub use web::*;

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
                .map(async |call| Message::Tool {
                    tool_call_id: call.id,
                    content: self.call(call.function.arguments).await,
                })
                .collect::<Vec<_>>(),
        )
        .await
    }
}
