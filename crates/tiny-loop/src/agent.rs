use crate::{
    history::{History, InfiniteHistory},
    llm::LLMProvider,
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
}

impl Agent {
    /// Create a new agent loop
    pub fn new(llm: impl LLMProvider + 'static) -> Self {
        Self {
            llm: Box::new(llm),
            history: Box::new(InfiniteHistory::new()),
            executor: Box::new(ParallelExecutor::new()),
            tools: Vec::new(),
        }
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
        self.history.add(crate::types::TimedMessage {
            message: crate::types::SystemMessage {
                content: content.into(),
            }
            .into(),
            timestamp: std::time::SystemTime::now(),
            elapsed: std::time::Duration::ZERO,
        });
        self
    }

    /// Get reference to registered tool definitions
    pub fn tools(&self) -> &[ToolDefinition] {
        &self.tools
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
    /// To register a tool method with an instance, use [`Self::bind`].
    /// To register external tools (e.g. from MCP servers) use [`Self::external`]
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
    /// To register a standalone tool function, use [`Self::tool`].
    /// To register external tools (e.g. from MCP servers) use [`Self::external`]
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

    /// Register external tools (e.g. from MCP servers)
    ///
    /// To register a standalone tool function, use [`tool`](Self::tool).
    /// To register a tool method with an instance, use [`bind`](Self::bind).
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{Agent, llm::OpenAIProvider, types::{Parameters, ToolDefinition, ToolFunction}};
    /// use serde_json::{json, Value};
    ///
    /// let defs = vec![ToolDefinition {
    ///     tool_type: "function".into(),
    ///     function: ToolFunction {
    ///         name: "get_weather".into(),
    ///         description: "Get weather information".into(),
    ///         parameters: Parameters::from_object(
    ///             json!({
    ///                 "type": "object",
    ///                 "properties": {
    ///                     "city": {
    ///                         "type": "string",
    ///                         "description": "City name"
    ///                     }
    ///                 },
    ///                 "required": ["city"]
    ///             }).as_object().unwrap().clone()
    ///         ),
    ///     },
    /// }];
    ///
    /// let external_executor = move |name: String, args: String| {
    ///     async move {
    ///         let _args = serde_json::from_str::<Value>(&args).unwrap();
    ///         "result".into()
    ///     }
    /// };
    ///
    /// let agent = Agent::new(OpenAIProvider::new())
    ///     .external(defs, external_executor);
    /// ```
    pub fn external<Fut>(
        mut self,
        defs: Vec<ToolDefinition>,
        exec: impl Fn(String, String) -> Fut + Clone + Send + Sync + 'static,
    ) -> Self
    where
        Fut: Future<Output = String> + Send + 'static,
    {
        for d in &defs {
            let name = d.function.name.clone();
            let exec = exec.clone();
            self.executor.add(
                name.clone(),
                Box::new(ClosureTool::boxed(move |s: String| {
                    let name = name.clone();
                    let exec = exec.clone();
                    Box::pin(async move { exec(name.clone(), s).await })
                })),
            );
        }
        self.tools.extend(defs);
        self
    }

    /// Execute one iteration of the agent loop.
    /// Returns `Ok(Some(content))` if loop should terminate, `Ok(None)` to continue
    ///
    /// This is usually used to customize the agent loop.
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{Agent, llm::OpenAIProvider};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let mut agent = Agent::new(OpenAIProvider::new())
    ///     .system("You are a helpful assistant");
    ///
    /// // Custom loop with early break
    /// let mut iterations = 0;
    /// loop {
    ///     println!("Before step {}", iterations);
    ///     
    ///     if let Some(content) = agent.step().await? {
    ///         println!("Completed: {}", content);
    ///         break;
    ///     }
    ///     
    ///     iterations += 1;
    ///     if iterations > 10 {
    ///         println!("Max iterations reached");
    ///         break;
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn step(&mut self) -> anyhow::Result<Option<String>> {
        tracing::trace!("Calling LLM with {} messages", self.history.get_all().len());

        let messages: Vec<_> = self
            .history
            .get_all()
            .iter()
            .map(|tm| tm.message.clone())
            .collect();
        let start = std::time::SystemTime::now();
        let response = self.llm.call(&messages, &self.tools).await?;
        let elapsed = start.elapsed().unwrap();

        self.history.add(crate::types::TimedMessage {
            message: response.message.clone().into(),
            timestamp: start,
            elapsed,
        });

        // Execute tool calls if any
        if let Some(calls) = &response.message.tool_calls {
            tracing::debug!("Executing {} tool calls", calls.len());
            let results = self.executor.execute(calls.clone()).await;
            self.history.add_batch(
                results
                    .into_iter()
                    .map(|r| crate::types::TimedMessage {
                        message: r.tool_message.into(),
                        timestamp: r.timestamp,
                        elapsed: r.elapsed,
                    })
                    .collect(),
            );
        }

        // Break loop if finish reason is not tool_calls
        if !matches!(
            response.finish_reason,
            crate::types::FinishReason::ToolCalls
        ) {
            tracing::debug!(
                "Agent loop completed, finish_reason: {:?}",
                response.finish_reason
            );
            return Ok(Some(response.message.content));
        }

        Ok(None)
    }

    /// Run the agent loop until completion.
    /// Return the last AI's response
    pub async fn run(&mut self) -> anyhow::Result<String> {
        tracing::debug!("Starting agent loop");
        loop {
            if let Some(content) = self.step().await? {
                return Ok(content);
            }
        }
    }

    /// Run the agent loop with a new user input appended.
    /// Return the last AI's response
    pub async fn chat(&mut self, prompt: impl Into<String>) -> anyhow::Result<String> {
        let prompt = prompt.into();
        tracing::debug!("Chat request, prompt length: {}", prompt.len());
        self.history.add(crate::types::TimedMessage {
            message: crate::types::UserMessage { content: prompt }.into(),
            timestamp: std::time::SystemTime::now(),
            elapsed: std::time::Duration::ZERO,
        });
        self.run().await
    }
}
