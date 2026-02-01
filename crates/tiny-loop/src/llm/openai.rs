use crate::types::{FinishReason, LLMResponse, Message, StreamCallback, ToolDefinition};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Request payload for OpenAI chat completions API
#[derive(Serialize)]
struct ChatRequest {
    /// Model ID
    model: String,
    /// Conversation messages
    messages: Vec<Message>,
    /// Available tools for the model
    tools: Vec<ToolDefinition>,
    /// Enable streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// Response from OpenAI chat completions API
#[derive(Deserialize)]
struct ChatResponse {
    /// List of completion choices
    choices: Vec<Choice>,
}

/// Streaming response chunk
#[derive(Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

/// Streaming choice
#[derive(Deserialize)]
struct StreamChoice {
    delta: Delta,
    #[serde(default)]
    finish_reason: Option<FinishReason>,
}

/// Delta content in streaming
#[derive(Deserialize)]
struct Delta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<crate::types::ToolCall>>,
}

/// Single completion choice from the API response
#[derive(Deserialize)]
struct Choice {
    /// Assistant's response message
    message: Message,
    /// Reason the completion finished
    finish_reason: FinishReason,
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
///     .model("gpt-4o");
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
    /// Additional HTTP headers
    custom_headers: HeaderMap,
    /// Maximum number of retries on failure
    max_retries: u32,
    /// Delay between retries in milliseconds
    retry_delay_ms: u64,
    /// Custom body fields to merge into the request
    custom_body: Map<String, Value>,
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
            custom_headers: HeaderMap::new(),
            max_retries: 3,
            retry_delay_ms: 1000,
            custom_body: Map::new(),
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

    /// Add a custom HTTP header to requests
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new()
    ///     .header("x-custom-header", "value")
    ///     .unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the header name or value contains invalid characters.
    pub fn header(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> anyhow::Result<Self> {
        self.custom_headers.insert(
            HeaderName::try_from(key.into())?,
            HeaderValue::try_from(value.into())?,
        );
        Ok(self)
    }

    /// Set maximum number of retries on failure (default: 3)
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new()
    ///     .max_retries(5);
    /// ```
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Set delay between retries in milliseconds (default: 1000)
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new()
    ///     .retry_delay(2000);
    /// ```
    pub fn retry_delay(mut self, delay_ms: u64) -> Self {
        self.retry_delay_ms = delay_ms;
        self
    }

    /// Set custom body fields to merge into the request
    ///
    /// # Examples
    ///
    /// ```
    /// use tiny_loop::llm::OpenAIProvider;
    /// use serde_json::json;
    ///
    /// let provider = OpenAIProvider::new()
    ///     .body(json!({
    ///         "top_p": 0.9,
    ///         "frequency_penalty": 0.5
    ///     }))
    ///     .unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not a JSON object
    pub fn body(mut self, body: Value) -> anyhow::Result<Self> {
        self.custom_body = body
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("body must be a JSON object"))?
            .clone();
        Ok(self)
    }
}

#[async_trait]
impl super::LLMProvider for OpenAIProvider {
    async fn call(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
        mut stream_callback: Option<&mut StreamCallback>,
    ) -> anyhow::Result<LLMResponse> {
        let mut attempt = 0;
        loop {
            attempt += 1;
            tracing::debug!(
                model = %self.model,
                messages = messages.len(),
                tools = tools.len(),
                streaming = stream_callback.is_some(),
                attempt = attempt,
                max_retries = self.max_retries,
                "Calling LLM API"
            );

            match self
                .call_once(messages, tools, stream_callback.as_deref_mut())
                .await
            {
                Ok(response) => return Ok(response),
                Err(e) if attempt > self.max_retries => {
                    tracing::debug!("Max retries exceeded");
                    return Err(e);
                }
                Err(e) => {
                    tracing::debug!("API call failed, retrying: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_millis(self.retry_delay_ms))
                        .await;
                }
            }
        }
    }
}

impl OpenAIProvider {
    async fn call_once(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
        stream_callback: Option<&mut StreamCallback>,
    ) -> anyhow::Result<LLMResponse> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: messages.to_vec(),
            tools: tools.to_vec(),
            stream: if stream_callback.is_some() {
                Some(true)
            } else {
                None
            },
        };

        let mut body = serde_json::to_value(&request)?.as_object().unwrap().clone();
        body.extend(self.custom_body.clone());

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .headers(self.custom_headers.clone())
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        tracing::trace!("LLM API response status: {}", status);

        if !status.is_success() {
            let body = response.text().await?;
            tracing::debug!("LLM API error: status={}, body={}", status, body);
            anyhow::bail!("API error ({}): {}", status, body);
        }

        if let Some(callback) = stream_callback {
            self.handle_stream(response, callback).await
        } else {
            let body = response.text().await?;
            let chat_response: ChatResponse = serde_json::from_str(&body)
                .map_err(|e| anyhow::anyhow!("Failed to parse response: {}. Body: {}", e, body))?;
            tracing::debug!("LLM API call completed successfully");
            let choice = &chat_response.choices[0];
            let Message::Assistant(msg) = &choice.message else {
                anyhow::bail!("Expected Assistant message, got: {:?}", choice.message);
            };
            Ok(LLMResponse {
                message: msg.clone(),
                finish_reason: choice.finish_reason.clone(),
            })
        }
    }

    async fn handle_stream(
        &self,
        response: reqwest::Response,
        callback: &mut StreamCallback,
    ) -> anyhow::Result<LLMResponse> {
        use futures::TryStreamExt;

        let mut stream = response.bytes_stream();
        let mut buffer = String::new();
        let mut content = String::new();
        let mut tool_calls = Vec::new();
        let mut finish_reason = FinishReason::Stop;

        while let Some(chunk) = stream.try_next().await? {
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(line_end) = buffer.find('\n') {
                let line = buffer[..line_end].trim().to_string();
                buffer.drain(..=line_end);

                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        break;
                    }

                    if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                        if let Some(choice) = chunk.choices.first() {
                            if let Some(delta_content) = &choice.delta.content {
                                content.push_str(delta_content);
                                callback(delta_content.clone());
                            }

                            if let Some(delta_tool_calls) = &choice.delta.tool_calls {
                                tool_calls.extend(delta_tool_calls.clone());
                            }

                            if let Some(reason) = &choice.finish_reason {
                                finish_reason = reason.clone();
                            }
                        }
                    }
                }
            }
        }

        tracing::debug!("Streaming completed, total length: {}", content.len());
        Ok(LLMResponse {
            message: crate::types::AssistantMessage {
                content,
                tool_calls: if tool_calls.is_empty() {
                    None
                } else {
                    Some(tool_calls)
                },
            },
            finish_reason,
        })
    }
}
