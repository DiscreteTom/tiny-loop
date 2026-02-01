# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

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

[Unreleased]: https://github.com/DiscreteTom/tiny-loop/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.3...v0.1.4
[0.1.3]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.2...v0.1.3
[0.1.2]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.1...v0.1.2
[0.1.1]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DiscreteTom/tiny-loop/releases/tag/v0.1.0
