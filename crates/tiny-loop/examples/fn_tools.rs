mod common;

use common::run_cli_loop;
use std::io::{Write, stdout};
use tiny_loop::{Agent, llm::OpenAIProvider, tool::tool};

/// Get the current weather for a location
#[tool]
async fn get_weather(
    /// City name
    city: String,
) -> String {
    format!("The weather in {} is sunny and 72°F", city)
}

/// Calculate the sum of two numbers
#[tool]
async fn add(
    /// First number
    a: i32,
    /// Second number
    b: i32,
) -> String {
    format!("{}", a + b)
}

#[tokio::main]
async fn main() {
    let api_key = std::env::var("LLM_API_KEY").expect("LLM_API_KEY not set");

    let llm = OpenAIProvider::new()
        .api_key(api_key)
        .base_url("https://openrouter.ai/api/v1")
        .model("google/gemini-3-flash-preview")
        .stream_callback(|chunk| {
            print!("{}", chunk);
            stdout().flush().unwrap();
        });

    let agent = Agent::new(llm)
        .system("You are a helpful assistant with access to tools")
        .tool(get_weather)
        .tool(add);

    run_cli_loop(agent).await
}
