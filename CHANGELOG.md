# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/DiscreteTom/tiny-loop/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.0
