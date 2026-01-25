use std::collections::HashMap;

use super::types::{Message, ToolCall};
use crate::{
    llm::LLMProvider,
    tool::{ClosureTool, FnToolArgs, Tool},
    types::ToolDefinition,
};

/// Agent loop that coordinates LLM calls and tool execution
pub struct AgentLoop<P: LLMProvider> {
    provider: P,
    tools: HashMap<String, Box<dyn Tool + Sync>>,
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

    pub fn tool<Args, Fut>(mut self, tool: fn(Args) -> Fut) -> Self
    where
        Fut: Future<Output = String> + Send + 'static,
        Args: FnToolArgs + 'static,
    {
        self.definitions.push(Args::definition());
        self.tools.insert(
            Args::TOOL_NAME.into(),
            Box::new(ClosureTool::boxed(move |s: String| {
                Box::pin(async move {
                    let args = match serde_json::from_str::<Args>(&s) {
                        Ok(args) => args,
                        Err(e) => return e.to_string(),
                    };
                    tool(args).await
                })
            })),
        );
        self
    }

    /// Run the agent loop until completion
    pub async fn run(&mut self) -> anyhow::Result<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::openai::OpenAIProvider;
    use tiny_loop_macros::tool_internal;

    /// Fetch a URL.
    #[tool_internal]
    pub async fn fetch(
        /// URL to fetch
        url: String,
    ) -> String {
        todo!()
    }

    fn test() {
        AgentLoop::new(OpenAIProvider::new()).tool(fetch);
    }
}
