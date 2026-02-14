# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Breaking**: Replaced `anyhow` with `thiserror` for structured error handling
- **Breaking**: All public APIs now return `tiny_loop::Result<T>` instead of `anyhow::Result<T>`
- Added `tiny_loop::Error` enum with specific error variants
- Moved error types to dedicated `error` module

## [0.4.1] - 2026-02-07

### Added

- `Deserialize` trait to all types in `types` module for better serialization support

## [0.4.0] - 2026-02-07

### Added

- `TimedMessage` struct wrapping `Message` with `timestamp: SystemTime` and `elapsed: Duration`
- `ToolResult` struct wrapping `ToolMessage` with `timestamp: SystemTime` and `elapsed: Duration`
- `Tool::call_timed()` method with default implementation for timing measurement

### Changed

- **Breaking**: `History` trait now stores and returns `TimedMessage` instead of `Message`
- **Breaking**: `ToolExecutor::execute()` now returns `Vec<ToolResult>` instead of `Vec<ToolMessage>`
- **Breaking**: `Tool::call_batch()` now returns `Vec<ToolResult>` instead of `Vec<ToolMessage>`
- `Agent::step()` now records timing for assistant messages and tool executions
- `InfiniteHistory` now stores `TimedMessage`

## [0.3.1] - 2026-02-03

### Fixed

- `#[tool]` macro now correctly includes all lines of multi-line doc comments in tool and parameter descriptions

## [0.3.0] - 2026-02-01

### Added

- `FinishReason` enum in `types` module
- `LLMResponse` struct containing `message` and `finish_reason`
- Message body structs: `SystemMessage`, `UserMessage`, `AssistantMessage`, `ToolMessage`, `CustomMessage`
- `Agent::step()` method to execute one iteration of the agent loop for custom loop control
- `OpenAIStreamCallback` type in `llm::openai` module
- `OpenAIProvider::stream_callback()` method to set streaming callback

### Changed

- **Breaking**: `LLMProvider::call` now returns `LLMResponse` instead of `Message`
- **Breaking**: `LLMProvider::call` signature changed to `&mut self` and removed `stream_callback` parameter
- **Breaking**: `LLMResponse.message` is now `AssistantMessage` instead of `Message`
- **Breaking**: `Tool::call_batch` now returns `Vec<ToolMessage>` instead of `Vec<Message>`
- **Breaking**: `ToolExecutor::execute` now returns `Vec<ToolMessage>` instead of `Vec<Message>`
- **Breaking**: `Message` enum variants now use tuple structs instead of inline fields
- **Breaking**: Stream callback moved from `Agent` to provider-specific implementations (e.g., `OpenAIProvider`)
- Agent loop now respects `finish_reason` and stops when it's not `ToolCalls`
- Split `types.rs` into submodules: `message.rs`, `tool.rs`, `llm.rs`
- `Agent::run()` now uses `Agent::step()` internally

### Removed

- **Breaking**: `OpenAIProvider::temperature()` and `OpenAIProvider::max_tokens()` methods (use `body()` instead)
- **Breaking**: `Agent::stream_callback()` method (use provider-specific method instead)
- **Breaking**: `StreamCallback` type from `types` module

## [0.2.1] - 2026-02-01

### Removed

- Unused `html2md` dependency
- Unused `tokio::fs` feature

## [0.2.0] - 2026-02-01

### Added

- `OpenAIProvider::body()` method to set custom request body fields

### Changed

- **Breaking**: `OpenAIProvider::header()` now returns `Result<Self>` instead of panicking
- `Parameters::from_object` now removes `description` field from schema objects

### Removed

- **Breaking**: Built-in `read` and `fetch` tools
- **Breaking**: `truncate_text` utility function

## [0.1.4] - 2026-02-01

### Added

- `Agent::tools()` method to get reference to registered tool definitions
- `#[tool(name = "...")]` attribute to override tool name for functions
- `#[name = "..."]` attribute to override tool name for methods in `#[tool]` impl blocks
- `Debug` trait implementation for `ToolDefinition` and `ToolFunction`

### Changed

- **Breaking**: `read`, `fetch`, and `truncate_text` functions now use `len` parameter instead of `end` parameter (default: 5000)

### Fixed

- Tool schemas now inline subschemas instead of using `$ref` references

## [0.1.3] - 2026-01-31

### Changed

- Reverted re-export changes due to Rust macro hygiene issues
- The `#[tool]` macro now uses `serde` and `schemars` directly (users must add these dependencies)

## [0.1.2] - 2026-01-31

### Changed

- ~~Re-exported `serde` and `schemars` to ensure version compatibility for users~~ (reverted)
- ~~The `#[tool]` macro now uses re-exported dependencies (`tiny_loop::serde`, `tiny_loop::schemars`)~~ (reverted)
- Use more flexible dependency versions (major/minor instead of patch-specific)
- Reduced tokio features to only `fs` and `time` (moved `macros` and `rt-multi-thread` to dev-dependencies)
- Moved `rmcp` to dev-dependencies as it's only used in examples

## [0.1.1] - 2025-01-28

### Changed

- The `#[tool]` macro will panic if functions/methods' return type is not `String`

### Fixed

- The built-in `read` tool now returns `String`

## [0.1.0] - 2026-01-26

### Added

- Initial release
- `#[tool]` macro to create custom tools
- `Agent::bind` and `Agent::tool` to load custom tools
- MCP tools support via `Agent::external`
- Streaming support for LLM responses
- History management system
- Parallel tool execution
- Retry strategy for OpenAI provider
- Tracing instrumentation
- Built-in tools: `fetch` and `read`

[Unreleased]: https://github.com/DiscreteTom/tiny-loop/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.4.0...v0.4.1
[0.4.0]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.3.1...v0.4.0
[0.3.1]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.3.0...v0.3.1
[0.3.0]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.2.1...v0.3.0
[0.2.1]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.2.0...v0.2.1
[0.2.0]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.4...v0.2.0
[0.1.4]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.3...v0.1.4
[0.1.3]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.2...v0.1.3
[0.1.2]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.1...v0.1.2
[0.1.1]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.0
