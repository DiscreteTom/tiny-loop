use crate::types::{Message, ToolDefinition};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

/// Request payload for OpenAI chat completions API
#[derive(Serialize)]
pub struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    tools: Vec<ToolDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

/// Response from OpenAI chat completions API
#[derive(Deserialize)]
pub struct ChatResponse {
    choices: Vec<Choice>,
}

/// Single completion choice from the API response
#[derive(Deserialize)]
pub struct Choice {
    message: Message,
}

/// OpenAI-compatible LLM provider
pub struct OpenAIProvider {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    custom_headers: HeaderMap,
}

impl Default for OpenAIProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenAIProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.openai.com/v1".into(),
            api_key: "".into(),
            model: "gpt-4o".into(),
            temperature: None,
            max_tokens: None,
            custom_headers: HeaderMap::new(),
        }
    }

    /// Set the base URL for the API endpoint
    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = value.into();
        self
    }

    /// Set the API key for authentication
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = value.into();
        self
    }

    /// Set the model name to use
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = value.into();
        self
    }

    /// Set the temperature for response randomness
    pub fn temperature(mut self, value: impl Into<Option<f32>>) -> Self {
        self.temperature = value.into();
        self
    }

    /// Set the maximum number of tokens to generate
    pub fn max_tokens(mut self, value: impl Into<Option<u32>>) -> Self {
        self.max_tokens = value.into();
        self
    }

    /// Add a custom HTTP header to requests
    pub fn header(mut self, key: impl Into<HeaderName>, value: impl Into<HeaderValue>) -> Self {
        self.custom_headers.insert(key.into(), value.into());
        self
    }
}

impl super::LLMProvider for OpenAIProvider {
    async fn call(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: messages.to_vec(),
            tools: tools.to_vec(),
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .headers(self.custom_headers.clone())
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;

        Ok(chat_response.choices[0].message.clone())
    }
}
