use std::collections::HashMap;

use crate::{
    tool::{Tool, executor::ToolExecutor},
    types::{Message, ToolCall},
};
use async_trait::async_trait;

/// Executes tools sequentially one by one by using [`Tool::call`]
pub struct SequentialExecutor {
    tools: HashMap<String, Box<dyn Tool + Sync>>,
}

impl SequentialExecutor {
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

    async fn execute(&self, calls: Vec<ToolCall>) -> Vec<Message> {
        tracing::debug!("Executing {} tool calls sequentially", calls.len());
        let mut results = Vec::new();
        for call in calls {
            tracing::debug!("Executing tool '{}'", call.function.name);
            let message = if let Some(tool) = self.tools.get(&call.function.name) {
                Message::Tool {
                    tool_call_id: call.id.clone(),
                    content: tool.call(call.function.arguments).await,
                }
            } else {
                tracing::debug!("Tool '{}' not found", call.function.name);
                Message::Tool {
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
