use super::types::Message;
use crate::{
    llm::LLMProvider,
    tool::{ClosureTool, ParallelExecutor, ToolArgs, ToolExecutor},
    types::ToolDefinition,
};

/// Agent loop that coordinates LLM calls and tool execution.
/// Uses [`ParallelExecutor`] by default.
pub struct Agent {
    pub messages: Vec<Message>,
    llm: Box<dyn LLMProvider>,
    executor: Box<dyn ToolExecutor>,
    tools: Vec<ToolDefinition>,
}

impl Agent {
    /// Create a new agent loop
    pub fn new(llm: impl LLMProvider + 'static) -> Self {
        Self {
            llm: Box::new(llm),
            messages: Vec::new(),
            executor: Box::new(ParallelExecutor::new()),
            tools: Vec::new(),
        }
    }

    /// Set messages
    ///
    /// # Example
    /// ```
    /// use tiny_loop::{Agent, types::Message, llm::OpenAIProvider};
    ///
    /// let messages = vec![Message::User { content: "Hello".into() }];
    /// let agent = Agent::new(OpenAIProvider::new())
    ///     .messages(messages);
    /// ```
    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = messages;
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
        self.messages.push(Message::System {
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
