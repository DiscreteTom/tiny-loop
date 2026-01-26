use super::types::Message;
use crate::{
    history::{History, InfiniteHistory},
    llm::{LLMProvider, StreamCallback},
    tool::{ClosureTool, ParallelExecutor, ToolArgs, ToolExecutor},
    types::ToolDefinition,
};

/// Agent loop that coordinates LLM calls and tool execution.
/// Uses [`ParallelExecutor`] by default.
pub struct Agent {
    pub history: Box<dyn History>,
    llm: Box<dyn LLMProvider>,
    executor: Box<dyn ToolExecutor>,
    tools: Vec<ToolDefinition>,
    stream_callback: Option<StreamCallback>,
}

impl Agent {
    /// Create a new agent loop
    pub fn new(llm: impl LLMProvider + 'static) -> Self {
        Self {
            llm: Box::new(llm),
            history: Box::new(InfiniteHistory::new()),
            executor: Box::new(ParallelExecutor::new()),
            tools: Vec::new(),
            stream_callback: None,
        }
    }

    /// Set stream callback for LLM responses
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{Agent, llm::OpenAIProvider};
    ///
    /// let agent = Agent::new(OpenAIProvider::new())
    ///     .stream_callback(|chunk| print!("{}", chunk));
    /// ```
    pub fn stream_callback<F>(mut self, callback: F) -> Self
    where
        F: FnMut(String) + Send + 'static,
    {
        self.stream_callback = Some(Box::new(callback));
        self
    }

    /// Set custom history manager (default: [`InfiniteHistory`])
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{Agent, history::InfiniteHistory, llm::OpenAIProvider};
    ///
    /// let agent = Agent::new(OpenAIProvider::new())
    ///     .history(InfiniteHistory::new());
    /// ```
    pub fn history(mut self, history: impl History + 'static) -> Self {
        self.history = Box::new(history);
        self
    }

    /// Append a system message
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{Agent, llm::OpenAIProvider};
    ///
    /// let agent = Agent::new(OpenAIProvider::new())
    ///     .system("You are a helpful assistant");
    /// ```
    pub fn system(mut self, content: impl Into<String>) -> Self {
        self.history.add(Message::System {
            content: content.into(),
        });
        self
    }

    /// Set a custom tool executor (default: [`ParallelExecutor`])
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{Agent, tool::SequentialExecutor, llm::OpenAIProvider};
    ///
    /// let agent = Agent::new(OpenAIProvider::new())
    ///     .executor(SequentialExecutor::new());
    /// ```
    pub fn executor(mut self, executor: impl ToolExecutor + 'static) -> Self {
        self.executor = Box::new(executor);
        self
    }

    /// Register a tool function created by [`#[tool]`](crate::tool::tool)
    ///
    /// To register a tool method with an instance, use [`bind`](Self::bind).
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{Agent, tool::tool, llm::OpenAIProvider};
    ///
    /// #[tool]
    /// async fn fetch(
    ///     /// URL to fetch
    ///     url: String,
    /// ) -> String {
    ///     todo!()
    /// }
    ///
    /// let agent = Agent::new(OpenAIProvider::new())
    ///     .tool(fetch);
    /// ```
    pub fn tool<Args, Fut>(mut self, tool: fn(Args) -> Fut) -> Self
    where
        Fut: Future<Output = String> + Send + 'static,
        Args: ToolArgs + 'static,
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

    /// Bind an instance to a tool method created by [`#[tool]`](crate::tool::tool)
    ///
    /// To register a standalone tool function, use [`tool`](Self::tool).
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{Agent, tool::tool, llm::OpenAIProvider};
    /// use std::sync::Arc;
    ///
    /// #[derive(Clone)]
    /// struct Database {
    ///     data: Arc<String>,
    /// }
    ///
    /// #[tool]
    /// impl Database {
    ///     /// Fetch data from database
    ///     async fn fetch(
    ///         self,
    ///         /// Data key
    ///         key: String,
    ///     ) -> String {
    ///         todo!()
    ///     }
    /// }
    ///
    /// let db = Database { data: Arc::new("data".into()) };
    /// let agent = Agent::new(OpenAIProvider::new())
    ///     .bind(db, Database::fetch);
    /// ```
    pub fn bind<T, Args, Fut>(mut self, ins: T, tool: fn(T, Args) -> Fut) -> Self
    where
        T: Send + Sync + Clone + 'static,
        Fut: Future<Output = String> + Send + 'static,
        Args: ToolArgs + 'static,
    {
        self.tools.push(Args::definition());
        self.executor.add(
            Args::TOOL_NAME.into(),
            Box::new(ClosureTool::boxed(move |s: String| {
                let ins = ins.clone();
                Box::pin(async move {
                    let args = match serde_json::from_str::<Args>(&s) {
                        Ok(args) => args,
                        Err(e) => return e.to_string(),
                    };
                    tool(ins, args).await
                })
            })),
        );
        self
    }

    /// Run the agent loop until completion.
    /// Return the last AI's response
    pub async fn run(&mut self) -> anyhow::Result<String> {
        tracing::debug!("Starting agent loop");
        loop {
            tracing::trace!("Calling LLM with {} messages", self.history.get_all().len());
            let message = self
                .llm
                .call(
                    self.history.get_all(),
                    &self.tools,
                    self.stream_callback.as_mut(),
                )
                .await?;

            self.history.add(message.clone());

            match message {
                Message::Assistant {
                    tool_calls: Some(calls),
                    ..
                } => {
                    tracing::debug!("Executing {} tool calls", calls.len());
                    let results = self.executor.execute(calls).await;
                    self.history.add_batch(results);
                }
                Message::Assistant { content, .. } => {
                    tracing::debug!("Agent loop completed, response length: {}", content.len());
                    return Ok(content);
                }
                _ => return Ok(String::new()),
            }
        }
    }

    /// Run the agent loop with a new user input appended.
    /// Return the last AI's response
    pub async fn chat(&mut self, prompt: impl Into<String>) -> anyhow::Result<String> {
        let prompt = prompt.into();
        tracing::debug!("Chat request, prompt length: {}", prompt.len());
        self.history.add(Message::User { content: prompt });
        self.run().await
    }
}
