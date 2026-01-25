use tiny_loop_macros::tool_internal;

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
    let start_idx = start.unwrap_or(0);
    let end_idx = end.unwrap_or(5000).min(markdown.len());
    let total_len = markdown.len();

    let mut result: String = markdown
        .chars()
        .skip(start_idx)
        .take(end_idx.saturating_sub(start_idx))
        .collect();

    if end_idx < total_len {
        result.push_str(&format!("\n---\ntruncated [{}/{}]", end_idx, total_len));
    }

    result
}
