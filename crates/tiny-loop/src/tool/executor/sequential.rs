use std::collections::HashMap;

use crate::{
    tool::{Tool, executor::ToolExecutor},
    types::ToolCall,
};
use async_trait::async_trait;

/// Executes tools sequentially one by one by using [`Tool::call`]
///
/// # How it works
///
/// 1. Iterates through tool calls in order
/// 2. Executes each call one at a time using [`Tool::call`]
/// 3. Waits for each call to complete before starting the next
///
/// # Example
///
/// Given tool calls:
/// ```text
/// [
///   ToolCall { id: "1", function: { name: "weather", ... } },
///   ToolCall { id: "2", function: { name: "search", ... } },
///   ToolCall { id: "3", function: { name: "weather", ... } },
/// ]
/// ```
///
/// The executor will:
/// 1. Execute `weather_tool.call(call1)` and wait for completion
/// 2. Execute `search_tool.call(call2)` and wait for completion
/// 3. Execute `weather_tool.call(call3)` and wait for completion
/// 4. Return results in order: `[result1, result2, result3]`
pub struct SequentialExecutor {
    tools: HashMap<String, Box<dyn Tool + Sync>>,
}

impl SequentialExecutor {
    /// Create a new sequential executor
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
}

#[async_trait]
impl ToolExecutor for SequentialExecutor {
    fn add(&mut self, name: String, tool: Box<dyn Tool + Sync>) -> Option<Box<dyn Tool + Sync>> {
        tracing::trace!("Registering tool: {}", name);
        self.tools.insert(name, tool)
    }

    async fn execute(&self, calls: Vec<ToolCall>) -> Vec<crate::types::ToolMessage> {
        tracing::debug!("Executing {} tool calls sequentially", calls.len());
        let mut results = Vec::new();
        for call in calls {
            tracing::debug!("Executing tool '{}'", call.function.name);
            let message = if let Some(tool) = self.tools.get(&call.function.name) {
                crate::types::ToolMessage {
                    tool_call_id: call.id.clone(),
                    content: tool.call(call.function.arguments).await,
                }
            } else {
                tracing::debug!("Tool '{}' not found", call.function.name);
                crate::types::ToolMessage {
                    tool_call_id: call.id,
                    content: format!("Tool '{}' not found", call.function.name),
                }
            };
            results.push(message);
        }
        tracing::debug!("Sequential execution completed");
        results
    }
}
