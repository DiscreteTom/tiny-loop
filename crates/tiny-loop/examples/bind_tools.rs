mod common;

use common::run_cli_loop;
use std::{
    collections::HashMap,
    io::{Write, stdout},
    sync::Arc,
};
use tiny_loop::{Agent, llm::OpenAIProvider, tool::tool};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ReadonlyTool {
    data: Arc<HashMap<String, String>>,
}

#[tool]
impl ReadonlyTool {
    /// Fetch data from database 1
    pub async fn fetch(
        self,
        /// Data key
        key: String,
    ) -> String {
        self.data
            .get(&key)
            .cloned()
            .unwrap_or_else(|| format!("Key '{}' not found", key))
    }
}

#[derive(Clone)]
pub struct WritableTool {
    data: Arc<Mutex<HashMap<String, String>>>,
}

#[tool]
impl WritableTool {
    /// Read data from database 2
    pub async fn read(
        self,
        /// Data key
        key: String,
    ) -> String {
        self.data
            .lock()
            .await
            .get(&key)
            .cloned()
            .unwrap_or_default()
    }

    /// Write data to database 2
    pub async fn write(
        self,
        /// Data key
        key: String,
        /// Data value
        value: String,
    ) -> String {
        self.data.lock().await.insert(key.clone(), value.clone());
        format!("Wrote '{}' to key '{}'", value, key)
    }
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

    let mut data = HashMap::new();
    data.insert("name".to_string(), "Alice".to_string());
    data.insert("age".to_string(), "30".to_string());

    let r = ReadonlyTool {
        data: Arc::new(data),
    };

    let w = WritableTool {
        data: Arc::new(Mutex::new(HashMap::new())),
    };

    let agent = Agent::new(llm)
        .system("You are a helpful assistant with access to tools")
        .bind(r.clone(), ReadonlyTool::fetch)
        .bind(w.clone(), WritableTool::read)
        .bind(w, WritableTool::write);

    run_cli_loop(agent).await
}
