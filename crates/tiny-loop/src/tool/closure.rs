use crate::tool::Tool;
use async_trait::async_trait;
use std::pin::Pin;

/// A tool that wraps an async closure for dynamic tool execution.
pub struct ClosureTool {
    func: Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = String> + Send>> + Sync>,
}

impl ClosureTool {
    /// Creates a new ClosureTool with the given async closure.
    pub fn new(
        func: Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = String> + Send>> + Sync>,
    ) -> Self {
        Self { func }
    }

    /// Creates a new ClosureTool from a closure, automatically boxing it.
    pub fn boxed(
        func: impl Fn(String) -> Pin<Box<dyn Future<Output = String> + Send>> + Sync + 'static,
    ) -> Self {
        Self::new(Box::new(func))
    }
}

#[async_trait]
impl Tool for ClosureTool {
    async fn call(&self, args: String) -> String {
        (self.func)(args).await
    }
}
