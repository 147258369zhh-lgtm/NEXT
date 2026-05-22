use std::{env, sync::Arc, time::Duration};

use anyhow::{Context, Result, anyhow};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderFamily {
    Chat,
    Stt,
    Tts,
    Realtime,
    Embedding,
}

impl ProviderFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Chat => "chat",
            Self::Stt => "stt",
            Self::Tts => "tts",
            Self::Realtime => "realtime",
            Self::Embedding => "embedding",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDescriptor {
    pub id: String,
    pub family: ProviderFamily,
    pub vendor: String,
    pub title: String,
    pub local_first: bool,
    pub enabled: bool,
}

pub trait ProviderBase: Send + Sync {
    fn id(&self) -> &'static str;
    fn vendor(&self) -> &'static str;
    fn family(&self) -> ProviderFamily;
    fn name(&self) -> &'static str {
        self.id()
    }
    fn local_first(&self) -> bool {
        false
    }

    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.id().to_owned(),
            family: self.family(),
            vendor: self.vendor().to_owned(),
            title: self.id().to_owned(),
            local_first: self.local_first(),
            enabled: true,
        }
    }
}

pub trait ChatProvider: ProviderBase {
    fn reply(&self, prompt: &str) -> Result<String>;
}

pub trait SttProvider: ProviderBase {
    fn transcribe_bytes(&self, _audio: &[u8]) -> Result<String>;
}

pub trait TtsProvider: ProviderBase {
    fn synthesize_bytes(&self, _text: &str) -> Result<Vec<u8>>;
}

fn load_dotenv() {
    let mut current_dir = match env::current_dir() {
        Ok(d) => d,
        Err(_) => return,
    };
    loop {
        let dotenv_path = current_dir.join(".env");
        if dotenv_path.is_file() {
            if let Ok(content) = std::fs::read_to_string(&dotenv_path) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    if let Some((key, val)) = line.split_once('=') {
                        let key = key.trim();
                        let val = val.trim();
                        let val = if (val.starts_with('"') && val.ends_with('"')) || (val.starts_with('\'') && val.ends_with('\'')) {
                            if val.len() >= 2 {
                                &val[1..val.len() - 1]
                            } else {
                                val
                            }
                        } else {
                            val
                        };
                        if env::var(key).is_err() {
                            env::set_var(key, val);
                        }
                    }
                }
            }
            break;
        }
        if !current_dir.pop() {
            break;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub mode: String,
    pub openai_base_url: String,
    pub chat_model: String,
    pub stt_mode: String,
    pub tts_mode: String,
    pub stt_model: String,
    pub tts_model: String,
}

impl ProviderConfig {
    pub fn from_env() -> Self {
        load_dotenv();

        let mode = env::var("NEXUS_PROVIDER_MODE").unwrap_or_else(|_| "mock".to_owned());

        let default_base_url = match mode.to_lowercase().as_str() {
            "ollama" => "http://localhost:11434/v1",
            "deepseek" => "https://api.deepseek.com/v1",
            "qwen" | "dashscope" => "https://dashscope.aliyuncs.com/compatible-mode/v1",
            _ => "https://api.openai.com/v1",
        };

        let default_model = match mode.to_lowercase().as_str() {
            "ollama" => "qwen2.5",
            "deepseek" => "deepseek-chat",
            "qwen" | "dashscope" => "qwen-plus",
            "openai" => "gpt-4o-mini",
            _ => "gpt-4.1-mini",
        };

        Self {
            mode: mode.clone(),
            openai_base_url: env::var("OPENAI_BASE_URL")
                .or_else(|_| match mode.to_lowercase().as_str() {
                    "ollama" => env::var("OLLAMA_BASE_URL"),
                    "deepseek" => env::var("DEEPSEEK_BASE_URL"),
                    "qwen" | "dashscope" => env::var("DASHSCOPE_BASE_URL").or_else(|_| env::var("QWEN_BASE_URL")),
                    _ => Err(env::VarError::NotPresent),
                })
                .unwrap_or_else(|_| default_base_url.to_owned()),
            chat_model: env::var("NEXUS_CHAT_MODEL")
                .or_else(|_| env::var("OPENAI_MODEL"))
                .or_else(|_| match mode.to_lowercase().as_str() {
                    "deepseek" => env::var("DEEPSEEK_MODEL"),
                    "qwen" | "dashscope" => env::var("QWEN_MODEL"),
                    _ => Err(env::VarError::NotPresent),
                })
                .unwrap_or_else(|_| default_model.to_owned()),
            stt_mode: env::var("NEXUS_STT_MODE").unwrap_or_else(|_| "mock".to_owned()),
            tts_mode: env::var("NEXUS_TTS_MODE").unwrap_or_else(|_| "mock".to_owned()),
            stt_model: env::var("NEXUS_STT_MODEL").unwrap_or_else(|_| "local-placeholder".to_owned()),
            tts_model: env::var("NEXUS_TTS_MODEL").unwrap_or_else(|_| "local-placeholder".to_owned()),
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
        "openai" | "openai-compatible" | "deepseek" | "ollama" | "qwen" | "dashscope" => {
            Ok(Arc::new(OpenAiCompatibleProvider::new(config)?))
        }
        _ => Ok(Arc::new(MockChatProvider)),
    }
}

pub fn build_stt_provider(config: &ProviderConfig) -> Result<Arc<dyn SttProvider>> {
    match config.stt_mode.to_lowercase().as_str() {
        "mock" => Ok(Arc::new(MockSttProvider::new(&config.stt_model))),
        _ => Ok(Arc::new(MockSttProvider::new(&config.stt_model))),
    }
}

pub fn build_tts_provider(config: &ProviderConfig) -> Result<Arc<dyn TtsProvider>> {
    match config.tts_mode.to_lowercase().as_str() {
        "mock" => Ok(Arc::new(MockTtsProvider::new(&config.tts_model))),
        _ => Ok(Arc::new(MockTtsProvider::new(&config.tts_model))),
    }
}

pub fn list_provider_catalog(config: &ProviderConfig) -> Result<Vec<ProviderDescriptor>> {
    Ok(vec![
        build_provider(config)?.descriptor(),
        build_stt_provider(config)?.descriptor(),
        build_tts_provider(config)?.descriptor(),
    ])
}

pub struct MockChatProvider;

impl ProviderBase for MockChatProvider {
    fn id(&self) -> &'static str {
        "mock-side-brain"
    }

    fn vendor(&self) -> &'static str {
        "nexus"
    }

    fn family(&self) -> ProviderFamily {
        ProviderFamily::Chat
    }

    fn local_first(&self) -> bool {
        true
    }
}

impl ChatProvider for MockChatProvider {
    fn reply(&self, prompt: &str) -> Result<String> {
        Ok(format!(
            "Nexus accepted the task:\n{prompt}\n\nThis is a stage-one scaffold response from the local mock provider."
        ))
    }
}

pub struct MockSttProvider {
    model: String,
}

impl MockSttProvider {
    fn new(model: &str) -> Self {
        Self {
            model: model.to_owned(),
        }
    }
}

impl ProviderBase for MockSttProvider {
    fn id(&self) -> &'static str {
        "mock-stt"
    }

    fn vendor(&self) -> &'static str {
        "nexus"
    }

    fn family(&self) -> ProviderFamily {
        ProviderFamily::Stt
    }

    fn local_first(&self) -> bool {
        true
    }

    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.id().to_owned(),
            family: self.family(),
            vendor: self.vendor().to_owned(),
            title: format!("Mock STT ({})", self.model),
            local_first: true,
            enabled: true,
        }
    }
}

impl SttProvider for MockSttProvider {
    fn transcribe_bytes(&self, _audio: &[u8]) -> Result<String> {
        Ok("[mock stt transcription placeholder]".to_owned())
    }
}

pub struct MockTtsProvider {
    model: String,
}

impl MockTtsProvider {
    fn new(model: &str) -> Self {
        Self {
            model: model.to_owned(),
        }
    }
}

impl ProviderBase for MockTtsProvider {
    fn id(&self) -> &'static str {
        "mock-tts"
    }

    fn vendor(&self) -> &'static str {
        "nexus"
    }

    fn family(&self) -> ProviderFamily {
        ProviderFamily::Tts
    }

    fn local_first(&self) -> bool {
        true
    }

    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.id().to_owned(),
            family: self.family(),
            vendor: self.vendor().to_owned(),
            title: format!("Mock TTS ({})", self.model),
            local_first: true,
            enabled: true,
        }
    }
}

impl TtsProvider for MockTtsProvider {
    fn synthesize_bytes(&self, text: &str) -> Result<Vec<u8>> {
        Ok(text.as_bytes().to_vec())
    }
}

struct OpenAiCompatibleProvider {
    client: Client,
    base_url: String,
    api_key: String,
    model: String,
    mode: String,
}

impl OpenAiCompatibleProvider {
    fn new(config: &ProviderConfig) -> Result<Self> {
        let mode = config.mode.to_lowercase();

        let api_key = match mode.as_str() {
            "ollama" => {
                env::var("OLLAMA_API_KEY")
                    .or_else(|_| env::var("OPENAI_API_KEY"))
                    .unwrap_or_else(|_| "ollama".to_owned())
            }
            "deepseek" => {
                env::var("DEEPSEEK_API_KEY")
                    .or_else(|_| env::var("OPENAI_API_KEY"))
                    .context("DEEPSEEK_API_KEY (or OPENAI_API_KEY) is missing. Please define DEEPSEEK_API_KEY in your .env file or environment variables to use DeepSeek mode.")?
            }
            "qwen" | "dashscope" => {
                env::var("DASHSCOPE_API_KEY")
                    .or_else(|_| env::var("QWEN_API_KEY"))
                    .or_else(|_| env::var("OPENAI_API_KEY"))
                    .context("DASHSCOPE_API_KEY (or QWEN_API_KEY/OPENAI_API_KEY) is missing. Please define DASHSCOPE_API_KEY in your .env file or environment variables to use Qwen mode.")?
            }
            "openai" => {
                env::var("OPENAI_API_KEY")
                    .context("OPENAI_API_KEY is missing. Please define OPENAI_API_KEY in your .env file or environment variables to use OpenAI mode.")?
            }
            _ => {
                env::var("OPENAI_API_KEY")
                    .context("OPENAI_API_KEY is required for openai-compatible mode. Please configure OPENAI_API_KEY in your .env file or environment variables.")?
            }
        };

        // Determine base url dynamically if the config matches standard fallback or default
        let base_url = match mode.as_str() {
            "ollama" => {
                env::var("OLLAMA_BASE_URL")
                    .or_else(|_| env::var("OPENAI_BASE_URL"))
                    .unwrap_or_else(|_| {
                        if config.openai_base_url == "https://api.openai.com/v1" {
                            "http://localhost:11434/v1".to_owned()
                        } else {
                            config.openai_base_url.clone()
                        }
                    })
            }
            "deepseek" => {
                env::var("DEEPSEEK_BASE_URL")
                    .or_else(|_| env::var("OPENAI_BASE_URL"))
                    .unwrap_or_else(|_| {
                        if config.openai_base_url == "https://api.openai.com/v1" {
                            "https://api.deepseek.com/v1".to_owned()
                        } else {
                            config.openai_base_url.clone()
                        }
                    })
            }
            "qwen" | "dashscope" => {
                env::var("DASHSCOPE_BASE_URL")
                    .or_else(|_| env::var("QWEN_BASE_URL"))
                    .or_else(|_| env::var("OPENAI_BASE_URL"))
                    .unwrap_or_else(|_| {
                        if config.openai_base_url == "https://api.openai.com/v1" {
                            "https://dashscope.aliyuncs.com/compatible-mode/v1".to_owned()
                        } else {
                            config.openai_base_url.clone()
                        }
                    })
            }
            _ => config.openai_base_url.clone(),
        };

        let model = match mode.as_str() {
            "ollama" => {
                env::var("NEXUS_CHAT_MODEL")
                    .or_else(|_| env::var("OPENAI_MODEL"))
                    .unwrap_or_else(|_| {
                        if config.chat_model == "gpt-4.1-mini" {
                            "qwen2.5".to_owned()
                        } else {
                            config.chat_model.clone()
                        }
                    })
            }
            "deepseek" => {
                env::var("NEXUS_CHAT_MODEL")
                    .or_else(|_| env::var("OPENAI_MODEL"))
                    .or_else(|_| env::var("DEEPSEEK_MODEL"))
                    .unwrap_or_else(|_| {
                        if config.chat_model == "gpt-4.1-mini" {
                            "deepseek-chat".to_owned()
                        } else {
                            config.chat_model.clone()
                        }
                    })
            }
            "qwen" | "dashscope" => {
                env::var("NEXUS_CHAT_MODEL")
                    .or_else(|_| env::var("OPENAI_MODEL"))
                    .or_else(|_| env::var("QWEN_MODEL"))
                    .unwrap_or_else(|_| {
                        if config.chat_model == "gpt-4.1-mini" {
                            "qwen-plus".to_owned()
                        } else {
                            config.chat_model.clone()
                        }
                    })
            }
            "openai" => {
                env::var("NEXUS_CHAT_MODEL")
                    .or_else(|_| env::var("OPENAI_MODEL"))
                    .unwrap_or_else(|_| {
                        if config.chat_model == "gpt-4.1-mini" {
                            "gpt-4o-mini".to_owned()
                        } else {
                            config.chat_model.clone()
                        }
                    })
            }
            _ => config.chat_model.clone(),
        };

        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .context("failed to build reqwest client")?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_owned(),
            api_key,
            model,
            mode: config.mode.clone(),
        })
    }
}

impl ProviderBase for OpenAiCompatibleProvider {
    fn id(&self) -> &'static str {
        match self.mode.to_lowercase().as_str() {
            "ollama" => "ollama",
            "deepseek" => "deepseek",
            "qwen" => "qwen",
            "dashscope" => "dashscope",
            "openai" => "openai",
            _ => "openai-compatible",
        }
    }

    fn vendor(&self) -> &'static str {
        match self.mode.to_lowercase().as_str() {
            "ollama" => "ollama",
            "deepseek" => "deepseek",
            "qwen" | "dashscope" => "qwen",
            "openai" => "openai",
            _ => "openai-compatible",
        }
    }

    fn family(&self) -> ProviderFamily {
        ProviderFamily::Chat
    }
}

impl ChatProvider for OpenAiCompatibleProvider {
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
