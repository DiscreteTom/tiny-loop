use tiny_loop_macros::tool_internal;

/// Fetch a webpage and convert HTML to Markdown
#[tool_internal]
pub async fn fetch(
    /// URL to fetch
    url: String,
) -> String {
    let response = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => return format!("Error fetching URL: {}", e),
    };

    let html = match response.text().await {
        Ok(h) => h,
        Err(e) => return format!("Error reading response: {}", e),
    };

    html2md::parse_html(&html)
}
