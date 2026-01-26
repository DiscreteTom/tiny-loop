# tiny-loop

Minimal AI agent framework in Rust.

## Features

- `#[tool]` macro for [functions](./crates/tiny-loop/examples/fn_tools.rs) and [methods](./crates/tiny-loop/examples/bind_tools.rs) to create custom tools.
- [Register MCP tools](./crates/tiny-loop/examples/mcp.rs)
- [History management](./crates/tiny-loop/examples/history.rs)
- [Streaming](./crates/tiny-loop/examples/common/streaming_cli.rs)
- Parallel tool execution
- Observability via `tracing`

## Installation

```sh
cargo add tiny-loop
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
