use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::tool::{CallableTool, Definition};

#[derive(Deserialize, JsonSchema)]
pub struct FetchArgs {
    /// URL to fetch
    pub url: String,
}

pub struct Fetch;

impl Definition for Fetch {
    const NAME: &'static str = "fetch";
    const DESCRIPTION: &'static str = "Fetch a webpage and convert HTML to Markdown";
    type Args = FetchArgs;
}

#[async_trait]
impl CallableTool for Fetch {
    async fn call(&self, args: String) -> String {
        let args = match serde_json::from_str::<FetchArgs>(&args) {
            Ok(args) => args,
            Err(e) => return e.to_string(),
        };
        let response = match reqwest::get(&args.url).await {
            Ok(r) => r,
            Err(e) => return format!("Error fetching URL: {}", e),
        };

        let html = match response.text().await {
            Ok(h) => h,
            Err(e) => return format!("Error reading response: {}", e),
        };

        let markdown = html2md::parse_html(&html);

        markdown
    }
}
