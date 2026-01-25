use tiny_loop_macros::tool_internal;

use super::utils::truncate_text;

/// Fetch a webpage and convert HTML to Markdown
#[tool_internal]
pub async fn fetch(
    /// URL to fetch
    url: String,
    /// Optional start character index (default: 0)
    start: Option<usize>,
    /// Optional end character index (default: 5000)
    end: Option<usize>,
) -> String {
    let response = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => return format!("Error fetching URL: {}", e),
    };

    let html = match response.text().await {
        Ok(h) => h,
        Err(e) => return format!("Error reading response: {}", e),
    };

    let markdown = html2md::parse_html(&html);
    truncate_text(markdown, start.unwrap_or(0), end.unwrap_or(5000))
}
