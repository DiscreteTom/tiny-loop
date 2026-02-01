# tiny-loop

![Crates.io Version](https://img.shields.io/crates/v/tiny-loop)
![GitHub License](https://img.shields.io/github/license/DiscreteTom/tiny-loop)

Minimal AI agent framework in Rust.

> **⚠️ Early Development**: This project is in early phase and the API will change frequently.

## Features

- [`#[tool]`](https://docs.rs/tiny-loop/latest/tiny_loop/tool/attr.tool.html) macro for [functions](./crates/tiny-loop/examples/fn_tools.rs) and [methods](./crates/tiny-loop/examples/bind_tools.rs) to create custom tools.
- [Register MCP tools](./crates/tiny-loop/examples/mcp.rs)
- [History management](./crates/tiny-loop/examples/history.rs)
- [Streaming](./crates/tiny-loop/examples/chatbot.rs)
- [Custom loop control](./crates/tiny-loop/examples/custom_loop.rs)
- Parallel tool execution
- Observability via [`tracing`](https://docs.rs/tracing/latest/tracing/)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tiny-loop = "0.2"
serde = { version = "1", features = ["derive"] }
schemars = "1"
```

## Quick Start

```rust
use tiny_loop::{Agent, llm::OpenAIProvider, tool::tool};

#[tool]
async fn search(
    /// Search query
    query: String
) -> String {
    format!("Results for: {}", query)
}

#[tokio::main]
async fn main() {
    let mut agent = Agent::new(OpenAIProvider::new())
        .system("You are a helpful assistant")
        .tool(search);

    let response = agent.chat("Search for Rust tutorials").await.unwrap();
    println!("{}", response);
}
```

## [Examples](./crates/tiny-loop/examples/)

## [Changelog](./CHANGELOG.md)
