use std::io::{self, Write};
use tiny_loop::{TinyLoop, llm::OpenAIProvider};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("LLM_API_KEY").expect("LLM_API_KEY not set");

    let mut agent = TinyLoop::new(
        OpenAIProvider::new()
            .api_key(api_key)
            .base_url("https://openrouter.ai/api/v1")
            .model("google/gemini-3-flash-preview"),
    )
    .system("You are a helpful assistant");

    println!("Chatbot started. Type 'quit' to exit.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        }

        match agent.chat(input).await {
            Ok(response) => println!("\n{}\n", response),
            Err(e) => eprintln!("Error: {}\n", e),
        }
    }
}
