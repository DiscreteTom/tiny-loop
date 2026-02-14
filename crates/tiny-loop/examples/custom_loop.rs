use tiny_loop::{Agent, llm::OpenAIProvider, tool::tool};

/// Get the current weather for a location
#[tool]
async fn get_weather(
    /// City name
    city: String,
) -> String {
    format!("The weather in {} is sunny and 72°F", city)
}

#[tokio::main]
async fn main() -> tiny_loop::Result<()> {
    let api_key = std::env::var("LLM_API_KEY").expect("LLM_API_KEY not set");

    let llm = OpenAIProvider::new()
        .api_key(api_key)
        .base_url("https://openrouter.ai/api/v1")
        .model("google/gemini-3-flash-preview");

    let mut agent = Agent::new(llm)
        .system("You are a helpful assistant with access to tools")
        .tool(get_weather);

    agent.history.add(tiny_loop::types::TimedMessage {
        message: tiny_loop::types::UserMessage {
            content: "What's the weather in Tokyo?".into(),
        }
        .into(),
        timestamp: std::time::SystemTime::now(),
        elapsed: std::time::Duration::ZERO,
    });

    // Custom loop with iteration limit
    let mut iterations = 0;
    loop {
        println!("[Iteration {}]", iterations);

        if let Some(content) = agent.step().await? {
            println!("Completed: {}", content);
            break;
        }

        iterations += 1;
        if iterations > 10 {
            return Err(tiny_loop::Error::Custom("Max iterations reached".into()));
        }
    }

    Ok(())
}
