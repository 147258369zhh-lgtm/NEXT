use std::{env, sync::Arc, time::Duration};

use anyhow::{Context, Result, anyhow};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub trait ChatProvider: Send + Sync {
    fn name(&self) -> &'static str;
    fn reply(&self, prompt: &str) -> Result<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub mode: String,
    pub openai_base_url: String,
    pub chat_model: String,
}

impl ProviderConfig {
    pub fn from_env() -> Self {
        Self {
            mode: env::var("NEXUS_PROVIDER_MODE").unwrap_or_else(|_| "mock".to_owned()),
            openai_base_url: env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_owned()),
            chat_model: env::var("NEXUS_CHAT_MODEL")
                .or_else(|_| env::var("OPENAI_MODEL"))
                .unwrap_or_else(|_| "gpt-4.1-mini".to_owned()),
        }
    }
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

pub fn build_provider(config: &ProviderConfig) -> Result<Arc<dyn ChatProvider>> {
    match config.mode.to_lowercase().as_str() {
        "openai" | "openai-compatible" => Ok(Arc::new(OpenAiCompatibleProvider::new(config)?)),
        _ => Ok(Arc::new(MockProvider)),
    }
}

pub struct MockProvider;

impl ChatProvider for MockProvider {
    fn name(&self) -> &'static str {
        "mock-side-brain"
    }

    fn reply(&self, prompt: &str) -> Result<String> {
        Ok(format!(
            "Nexus accepted the task:\n{prompt}\n\nThis is a stage-one scaffold response from the local mock provider."
        ))
    }
}

struct OpenAiCompatibleProvider {
    client: Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl OpenAiCompatibleProvider {
    fn new(config: &ProviderConfig) -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY is required when NEXUS_PROVIDER_MODE=openai")?;
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .context("failed to build reqwest client")?;

        Ok(Self {
            client,
            base_url: config.openai_base_url.trim_end_matches('/').to_owned(),
            api_key,
            model: config.chat_model.clone(),
        })
    }
}

impl ChatProvider for OpenAiCompatibleProvider {
    fn name(&self) -> &'static str {
        "openai-compatible"
    }

    fn reply(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": "You are Nexus Side Brain. Be concise and execution-oriented."},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.2
        });

        let response = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .context("failed to call openai-compatible endpoint")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .unwrap_or_else(|_| "unable to read error body".to_owned());
            return Err(anyhow!("provider request failed: {status} - {text}"));
        }

        let value: serde_json::Value = response
            .json()
            .context("failed to parse openai-compatible response")?;
        let content = value
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .map(str::trim)
            .filter(|content| !content.is_empty())
            .ok_or_else(|| anyhow!("provider response did not include message content"))?;

        Ok(content.to_owned())
    }
}
