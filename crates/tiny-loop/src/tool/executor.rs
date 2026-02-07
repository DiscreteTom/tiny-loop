mod parallel;
mod sequential;

use super::Tool;
use crate::types::{ToolCall, ToolResult};
use async_trait::async_trait;

pub use parallel::*;
pub use sequential::*;

/// Executes tool calls with different strategies (parallel, sequential, etc.)
#[async_trait]
pub trait ToolExecutor {
    /// Adds a tool to the executor. Returns the previous tool with the same name if it exists.
    fn add(&mut self, name: String, tool: Box<dyn Tool + Sync>) -> Option<Box<dyn Tool + Sync>>;

    /// Executes the given tool calls and returns the results with timing metadata.
    async fn execute(&self, calls: Vec<ToolCall>) -> Vec<ToolResult>;
}

/// Creates a ToolResult for a tool not found error
fn tool_not_found_result(call_id: String, tool_name: &str) -> ToolResult {
    ToolResult {
        tool_message: crate::types::ToolMessage {
            tool_call_id: call_id,
            content: format!("Tool '{}' not found", tool_name),
        },
        timestamp: std::time::SystemTime::now(),
        elapsed: std::time::Duration::ZERO,
    }
}
