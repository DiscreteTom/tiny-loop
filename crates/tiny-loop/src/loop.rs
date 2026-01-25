use std::collections::HashMap;

use super::types::{Message, ToolCall};
use crate::{
    llm::LLMProvider,
    tool::{CallableTool, Definition},
    types::ToolDefinition,
};

/// Agent loop that coordinates LLM calls and tool execution
pub struct AgentLoop<P: LLMProvider> {
    provider: P,
    tools: HashMap<String, Box<dyn CallableTool>>,
    pub messages: Vec<Message>,
    definitions: Vec<ToolDefinition>,
}

impl<P: LLMProvider> AgentLoop<P> {
    /// Create a new agent loop
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            messages: Vec::new(),
            tools: HashMap::new(),
            definitions: Vec::new(),
        }
    }

    pub fn tool<T: Definition + CallableTool + 'static>(mut self, tool: T) -> Self {
        self.definitions.push(T::definition());
        self.tools.insert(T::NAME.into(), Box::new(tool));
        self
    }

    /// Run the agent loop until completion
    pub async fn run(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        loop {
            let message = self
                .provider
                .call(&self.messages, &self.definitions)
                .await?;

            self.messages.push(message.clone());

            if let Some(tool_calls) = message.tool_calls {
                self.execute_tools(tool_calls).await;
            } else {
                return Ok(message.content.unwrap_or_default());
            }
        }
    }

    async fn execute_tools(&mut self, calls: Vec<ToolCall>) {
        let mut grouped: HashMap<String, Vec<ToolCall>> = HashMap::new();
        for call in calls {
            grouped
                .entry(call.function.name.clone())
                .or_default()
                .push(call);
        }

        let results = futures::future::join_all(grouped.into_iter().map(|(name, calls)| {
            match self.tools.get(&name) {
                Some(tool) => futures::future::Either::Left(tool.call_batch(calls)),
                None => futures::future::Either::Right(futures::future::ready(
                    calls
                        .into_iter()
                        .map(|call| Message {
                            role: "tool".into(),
                            tool_call_id: Some(call.id),
                            tool_calls: None,
                            content: Some(format!("Tool '{}' not found", name)),
                        })
                        .collect(),
                )),
            }
        }))
        .await;

        for messages in results {
            self.messages.extend(messages);
        }
    }
}
