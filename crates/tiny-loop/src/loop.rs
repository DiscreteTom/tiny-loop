use super::types::Message;
use crate::{
    llm::LLMProvider,
    tool::{ClosureTool, FnToolArgs, ParallelExecutor, ToolExecutor},
    types::ToolDefinition,
};

/// Agent loop that coordinates LLM calls and tool execution.
/// Uses [`ParallelExecutor`] by default.
pub struct TinyLoop {
    pub messages: Vec<Message>,
    llm: Box<dyn LLMProvider>,
    executor: Box<dyn ToolExecutor>,
    tools: Vec<ToolDefinition>,
}

impl TinyLoop {
    /// Create a new agent loop
    pub fn new(llm: impl LLMProvider + 'static) -> Self {
        Self {
            llm: Box::new(llm),
            messages: Vec::new(),
            executor: Box::new(ParallelExecutor::new()),
            tools: Vec::new(),
        }
    }

    /// Set a custom tool executor (default: [`ParallelExecutor`])
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{TinyLoop, tool::SequentialExecutor, llm::OpenAIProvider};
    ///
    /// let agent = TinyLoop::new(OpenAIProvider::new())
    ///     .executor(SequentialExecutor::new());
    /// ```
    pub fn executor(mut self, executor: impl ToolExecutor + 'static) -> Self {
        self.executor = Box::new(executor);
        self
    }

    /// Register a tool created by [`#[tool]`](crate::tool::tool)
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{TinyLoop, tool::tool, llm::OpenAIProvider};
    ///
    /// #[tool]
    /// async fn fetch(url: String) -> String {
    ///     todo!()
    /// }
    ///
    /// let agent = TinyLoop::new(OpenAIProvider::new())
    ///     .tool(fetch);
    /// ```
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

    /// Run the agent loop until completion.
    /// Return the last AI's response
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

    /// Run the agent loop with a new user input appended.
    /// Return the last AI's response
    pub async fn chat(&mut self, prompt: impl Into<String>) -> anyhow::Result<String> {
        self.messages.push(Message::User {
            content: prompt.into(),
        });
        self.run().await
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
        TinyLoop::new(OpenAIProvider::new()).tool(fetch);
    }
}
