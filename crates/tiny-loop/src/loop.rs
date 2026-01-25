use super::types::Message;
use crate::{
    llm::LLMProvider,
    tool::{ClosureTool, FnToolArgs, ParallelExecutor, ToolExecutor},
    types::ToolDefinition,
};

/// Agent loop that coordinates LLM calls and tool execution
pub struct AgentLoop {
    llm: Box<dyn LLMProvider>,
    executor: Box<dyn ToolExecutor>,
    pub messages: Vec<Message>,
    tools: Vec<ToolDefinition>,
}

impl AgentLoop {
    /// Create a new agent loop
    pub fn new(llm: impl LLMProvider + 'static) -> Self {
        Self {
            llm: Box::new(llm),
            messages: Vec::new(),
            executor: Box::new(ParallelExecutor::new()),
            tools: Vec::new(),
        }
    }

    pub fn tool<Args, Fut>(mut self, tool: fn(Args) -> Fut) -> Self
    where
        Fut: Future<Output = String> + Send + 'static,
        Args: FnToolArgs + 'static,
    {
        self.tools.push(Args::definition());
        self.executor.add(
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
            let message = self.llm.call(&self.messages, &self.tools).await?;

            self.messages.push(message.clone());

            match message {
                Message::Assistant {
                    tool_calls: Some(calls),
                    ..
                } => {
                    let results = self.executor.execute(calls).await;
                    self.messages.extend(results);
                }
                Message::Assistant { content, .. } => {
                    return Ok(content);
                }
                _ => return Ok(String::new()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::OpenAIProvider;
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
