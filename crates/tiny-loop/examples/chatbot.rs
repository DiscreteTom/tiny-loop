mod common;

use common::run_cli_loop;
use tiny_loop::{Agent, llm::OpenAIProvider};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("LLM_API_KEY").expect("LLM_API_KEY not set");

    let llm = OpenAIProvider::new()
        .api_key(api_key)
        .base_url("https://openrouter.ai/api/v1")
        .model("google/gemini-3-flash-preview");

    let agent = Agent::new(llm).system("You are a helpful assistant");

    run_cli_loop(agent).await
}
