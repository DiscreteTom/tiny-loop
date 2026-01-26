mod common;

use common::run_cli_loop;
use tiny_loop::{Agent, history::History, llm::OpenAIProvider, types::Message};

pub struct CustomHistory {
    max: usize,
    messages: Vec<Message>,
}

impl CustomHistory {
    pub fn new(max: usize) -> Self {
        Self {
            max,
            messages: Vec::new(),
        }
    }
}

impl History for CustomHistory {
    fn add(&mut self, message: Message) {
        self.messages.push(message);
        if self.messages.len() > self.max {
            self.messages.remove(0);
        }
    }

    fn get_all(&self) -> &[Message] {
        &self.messages
    }
}

#[tokio::main]
async fn main() {
    let api_key = std::env::var("LLM_API_KEY").expect("LLM_API_KEY not set");

    let llm = OpenAIProvider::new()
        .api_key(api_key)
        .base_url("https://openrouter.ai/api/v1")
        .model("google/gemini-3-flash-preview");

    let agent = Agent::new(llm)
        .system("You are a helpful assistant")
        .history(CustomHistory::new(3));

    run_cli_loop(agent).await
}
