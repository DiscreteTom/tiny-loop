mod args;
mod closure;
mod executor;

use crate::types::{ToolCall, ToolResult};
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

    /// Calls the tool with timing measurement
    async fn call_timed(&self, call: ToolCall) -> ToolResult {
        let start = std::time::SystemTime::now();
        let content = self.call(call.function.arguments).await;
        let elapsed = start.elapsed().unwrap();
        ToolResult {
            tool_message: crate::types::ToolMessage {
                tool_call_id: call.id,
                content,
            },
            timestamp: start,
            elapsed,
        }
    }

    /// Executes multiple tool calls in parallel. Override to customize execution behavior.
    async fn call_batch(&self, args: Vec<ToolCall>) -> Vec<ToolResult> {
        join_all(
            args.into_iter()
                .map(|call| self.call_timed(call))
                .collect::<Vec<_>>(),
        )
        .await
    }
}
