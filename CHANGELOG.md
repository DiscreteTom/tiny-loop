# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Re-exported `serde` and `schemars` to ensure version compatibility for users
- The `#[tool]` macro now uses re-exported dependencies (`tiny_loop::serde`, `tiny_loop::schemars`)

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

[Unreleased]: https://github.com/DiscreteTom/tiny-loop/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.0
