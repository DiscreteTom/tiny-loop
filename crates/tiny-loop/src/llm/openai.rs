use crate::types::{Message, ToolDefinition};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

/// Request payload for OpenAI chat completions API
#[derive(Serialize)]
struct ChatRequest {
    /// Model ID
    model: String,
    /// Conversation messages
    messages: Vec<Message>,
    /// Available tools for the model
    tools: Vec<ToolDefinition>,
    /// Sampling temperature (0-2)
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    /// Maximum tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

/// Response from OpenAI chat completions API
#[derive(Deserialize)]
struct ChatResponse {
    /// List of completion choices
    choices: Vec<Choice>,
}

/// Single completion choice from the API response
#[derive(Deserialize)]
struct Choice {
    /// Assistant's response message
    message: Message,
}

/// OpenAI-compatible LLM provider
///
/// # Examples
///
/// ```
/// use tiny_loop::llm::OpenAIProvider;
///
/// let provider = OpenAIProvider::new()
///     .api_key("sk-...")
///     .model("gpt-4o")
///     .temperature(0.7);
/// ```
pub struct OpenAIProvider {
    /// HTTP client for API requests
    client: reqwest::Client,
    /// API base URL
    base_url: String,
    /// API authentication key
    api_key: String,
    /// Model identifier
    model: String,
    /// Sampling temperature
    temperature: Option<f32>,
    /// Maximum tokens to generate
    max_tokens: Option<u32>,
    /// Additional HTTP headers
    custom_headers: HeaderMap,
}

impl Default for OpenAIProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenAIProvider {
    /// Create a new OpenAI provider with default settings
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new();
    /// ```
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

    /// Set the base URL for the API endpoint (default: `https://api.openai.com/v1`)
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new()
    ///     .base_url("https://api.custom.com/v1");
    /// ```
    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = value.into();
        self
    }

    /// Set the API key for authentication (default: empty string)
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new()
    ///     .api_key("sk-...");
    /// ```
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = value.into();
        self
    }

    /// Set the model name to use (default: `gpt-4o`)
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new()
    ///     .model("gpt-4o-mini");
    /// ```
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = value.into();
        self
    }

    /// Set the temperature for response randomness (default: `None`)
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new()
    ///     .temperature(0.7);
    /// ```
    pub fn temperature(mut self, value: impl Into<Option<f32>>) -> Self {
        self.temperature = value.into();
        self
    }

    /// Set the maximum number of tokens to generate (default: `None`)
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new()
    ///     .max_tokens(1000);
    /// ```
    pub fn max_tokens(mut self, value: impl Into<Option<u32>>) -> Self {
        self.max_tokens = value.into();
        self
    }

    /// Add a custom HTTP header to requests
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new()
    ///     .header("x-custom-header", "value");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the header name or value contains invalid characters.
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_headers.insert(
            HeaderName::try_from(key.into()).unwrap(),
            HeaderValue::try_from(value.into()).unwrap(),
        );
        self
    }
}

#[async_trait]
impl super::LLMProvider for OpenAIProvider {
    async fn call(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> anyhow::Result<Message> {
        tracing::debug!(
            model = %self.model,
            messages = messages.len(),
            tools = tools.len(),
            "Calling LLM API"
        );

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

        let status = response.status();
        tracing::trace!("LLM API response status: {}", status);
        let body = response.text().await?;

        if !status.is_success() {
            tracing::debug!("LLM API error: status={}, body={}", status, body);
            anyhow::bail!("API error ({}): {}", status, body);
        }

        let chat_response: ChatResponse = serde_json::from_str(&body)
            .map_err(|e| anyhow::anyhow!("Failed to parse response: {}. Body: {}", e, body))?;

        tracing::debug!("LLM API call completed successfully");
        Ok(chat_response.choices[0].message.clone())
    }
}
