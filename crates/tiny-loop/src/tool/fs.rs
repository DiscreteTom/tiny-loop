use tiny_loop_macros::tool_internal;

use super::utils::truncate_text;

/// Read file contents with optional character range.
#[tool_internal]
pub async fn read(
    /// File path
    path: String,
    /// Optional start character index (default: 0)
    start: Option<usize>,
    /// Optional end character index (default: 5000)
    end: Option<usize>,
) -> String {
    match tokio::fs::read_to_string(&path).await {
        Ok(content) => truncate_text(content, start.unwrap_or(0), end.unwrap_or(5000)),
        Err(e) => format!("Error reading file: {}", e),
    }
}
