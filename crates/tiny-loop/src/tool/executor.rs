mod parallel;
mod sequential;

use super::Tool;
use crate::types::{ToolCall, ToolMessage};
use async_trait::async_trait;

pub use parallel::*;
pub use sequential::*;

/// Executes tool calls with different strategies (parallel, sequential, etc.)
#[async_trait]
pub trait ToolExecutor {
    /// Adds a tool to the executor. Returns the previous tool with the same name if it exists.
    fn add(&mut self, name: String, tool: Box<dyn Tool + Sync>) -> Option<Box<dyn Tool + Sync>>;

    /// Executes the given tool calls and returns the results as messages.
    async fn execute(&self, calls: Vec<ToolCall>) -> Vec<ToolMessage>;
}
