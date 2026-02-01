mod args;
mod closure;
mod executor;

use crate::types::{ToolCall, ToolMessage};
use async_trait::async_trait;
use futures::future::join_all;

pub use args::*;
pub(crate) use closure::*;
pub use executor::*;
pub use tiny_loop_macros::tool;

/// A trait for tools that can be called with JSON string arguments.
///
/// Users must provide the `call` method. The framework auto-provides `call_batch` to run tools in parallel.
/// At runtime, different tool executors may call `call` or `call_batch` in different ways.
/// Users can override `call_batch` to customize this behavior.
#[async_trait]
pub trait Tool {
    /// Calls the tool with JSON arguments and returns the result.
    async fn call(&self, args: String) -> String;

    /// Executes multiple tool calls in parallel. Override to customize execution behavior.
    async fn call_batch(&self, args: Vec<ToolCall>) -> Vec<ToolMessage> {
        join_all(
            args.into_iter()
                .map(async |call| ToolMessage {
                    tool_call_id: call.id,
                    content: self.call(call.function.arguments).await,
                })
                .collect::<Vec<_>>(),
        )
        .await
    }
}
