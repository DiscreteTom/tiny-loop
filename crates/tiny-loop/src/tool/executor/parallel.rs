use crate::{
    tool::{Tool, executor::ToolExecutor},
    types::{Message, ToolCall},
};
use async_trait::async_trait;
use futures::future::join_all;
use std::collections::HashMap;

/// Executes tools in parallel by grouping calls by tool name and using [`Tool::call_batch`]
pub struct ParallelExecutor {
    tools: HashMap<String, Box<dyn Tool + Sync>>,
}

impl ParallelExecutor {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
}

#[async_trait]
impl ToolExecutor for ParallelExecutor {
    fn add(&mut self, name: String, tool: Box<dyn Tool + Sync>) -> Option<Box<dyn Tool + Sync>> {
        self.tools.insert(name, tool)
    }

    async fn execute(&self, calls: Vec<ToolCall>) -> Vec<Message> {
        let mut grouped: HashMap<String, Vec<ToolCall>> = HashMap::new();
        for call in calls {
            grouped
                .entry(call.function.name.clone())
                .or_default()
                .push(call);
        }

        let futures = grouped.into_iter().map(|(name, calls)| async move {
            if let Some(tool) = self.tools.get(&name) {
                tool.call_batch(calls).await
            } else {
                calls
                    .into_iter()
                    .map(|call| Message::Tool {
                        tool_call_id: call.id,
                        content: format!("Tool '{}' not found", name),
                    })
                    .collect()
            }
        });

        join_all(futures).await.into_iter().flatten().collect()
    }
}
