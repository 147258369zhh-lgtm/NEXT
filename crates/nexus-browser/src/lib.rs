use std::{env, process::Command};

use anyhow::Result;
use nexus_protocol::RiskLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct BrowserRuntimeDescriptor {
    pub id: String,
    pub title: String,
    pub engine: String,
    pub headless_default: bool,
    pub supports_live_control: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct BrowserRuntimeConfig {
    pub mode: String,
    pub cli_command: Option<String>,
    pub cli_args: Vec<String>,
}

impl BrowserRuntimeConfig {
    pub fn from_env() -> Self {
        let cli_args = env::var("NEXUS_BROWSER_CLI_ARGS")
            .ok()
            .and_then(|raw| serde_json::from_str::<Vec<String>>(&raw).ok())
            .unwrap_or_default();
        Self {
            mode: env::var("NEXUS_BROWSER_RUNTIME").unwrap_or_else(|_| "scaffold".to_owned()),
            cli_command: env::var("NEXUS_BROWSER_CLI_COMMAND").ok(),
            cli_args,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum BrowserRunMode {
    Silent,
    Observe,
    Takeover,
}

impl BrowserRunMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Silent => "silent",
            Self::Observe => "observe",
            Self::Takeover => "takeover",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum BrowserIntent {
    OpenPage,
    Login,
    ExtractInformation,
    FillForm,
    Unknown,
}

impl BrowserIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenPage => "open_page",
            Self::Login => "login",
            Self::ExtractInformation => "extract_information",
            Self::FillForm => "fill_form",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum BrowserActionPhase {
    InspectOnly,
    FillOnly,
    SubmitBlocked,
}

impl BrowserActionPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::FillOnly => "fill_only",
            Self::SubmitBlocked => "submit_blocked",
        }
    }
}

#[derive(Debug, Clone)]
pub struct BrowserTaskSpec {
    pub prompt: String,
    pub risk_level: RiskLevel,
    pub mode: BrowserRunMode,
    pub target_url: Option<String>,
    pub intent: BrowserIntent,
    pub action_phase: BrowserActionPhase,
}

#[derive(Debug, Clone, Serialize)]
pub struct BrowserExecutionOutput {
    pub summary: String,
    pub transcript: Vec<String>,
    pub target_url: Option<String>,
    pub intent: String,
    pub mode: String,
    pub action_phase: String,
    pub boundary: String,
    pub text_snippet: Option<String>,
    pub link_sample: Vec<String>,
    pub form_count: Option<u32>,
    pub input_sample: Vec<String>,
    pub field_plan: Vec<String>,
    pub missing_fields: Vec<String>,
    pub sensitive_fields: Vec<String>,
    pub recommended_next_actions: Vec<String>,
}

pub trait BrowserRuntime: Send + Sync {
    fn descriptor(&self) -> BrowserRuntimeDescriptor;
    fn execute(&self, spec: &BrowserTaskSpec) -> Result<BrowserExecutionOutput>;
}

pub struct ScaffoldBrowserRuntime;
pub struct PlaywrightCliBrowserRuntime;

#[derive(Debug, Serialize)]
struct BrowserCliSpec<'a> {
    prompt: &'a str,
    risk_level: &'a str,
    mode: &'a str,
    intent: &'a str,
    action_phase: &'a str,
    target_url: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct BrowserCliOutput {
    summary: Option<String>,
    transcript: Option<Vec<String>>,
    target_url: Option<String>,
    intent: Option<String>,
    mode: Option<String>,
    action_phase: Option<String>,
    boundary: Option<String>,
    text_snippet: Option<String>,
    link_sample: Option<Vec<String>>,
    form_count: Option<u32>,
    input_sample: Option<Vec<String>>,
    field_plan: Option<Vec<String>>,
    missing_fields: Option<Vec<String>>,
    sensitive_fields: Option<Vec<String>>,
    recommended_next_actions: Option<Vec<String>>,
}

impl BrowserRuntime for ScaffoldBrowserRuntime {
    fn descriptor(&self) -> BrowserRuntimeDescriptor {
        BrowserRuntimeDescriptor {
            id: "browser-runtime-scaffold".to_owned(),
            title: "Scaffold Browser Runtime".to_owned(),
            engine: "playwright-placeholder".to_owned(),
            headless_default: true,
            supports_live_control: false,
            enabled: true,
        }
    }

    fn execute(&self, spec: &BrowserTaskSpec) -> Result<BrowserExecutionOutput> {
        let boundary = browser_guardrail_note(&spec.intent).to_owned();
        let recommended_next_actions = suggest_next_actions(spec);
        let transcript = vec![
            format!("browser runtime selected in {} mode", spec.mode.as_str()),
            format!("intent classified as {}", spec.intent.as_str()),
            format!("action_phase={}", spec.action_phase.as_str()),
            match &spec.target_url {
                Some(url) => format!("target url parsed: {url}"),
                None => "no explicit url parsed from prompt".to_owned(),
            },
            format!("boundary={boundary}"),
            "playwright-backed runtime not enabled yet".to_owned(),
            "returned scaffold browser summary".to_owned(),
        ];

        Ok(BrowserExecutionOutput {
            summary: format!(
                "Browser runtime scaffold accepted this task.\n\nTarget request:\n{}\n\nRisk level: {}\nMode: {}\nIntent: {}\nAction phase: {}\nTarget URL: {}\n\nBehavior boundary: {}\n\nRecommended next actions:\n- {}\n\nCurrent behavior: BrowserExecutor selected the browser runtime boundary successfully. The next implementation step is replacing this scaffold runtime with a Playwright-backed engine that can open pages, keep session state, and return structured browser traces.",
                spec.prompt.trim(),
                risk_level_to_str(&spec.risk_level),
                spec.mode.as_str(),
                spec.intent.as_str(),
                spec.action_phase.as_str(),
                spec.target_url.as_deref().unwrap_or("not detected"),
                boundary,
                recommended_next_actions.join("\n- "),
            ),
            transcript,
            target_url: spec.target_url.clone(),
            intent: spec.intent.as_str().to_owned(),
            mode: spec.mode.as_str().to_owned(),
            action_phase: spec.action_phase.as_str().to_owned(),
            boundary,
            text_snippet: None,
            link_sample: Vec::new(),
            form_count: None,
            input_sample: Vec::new(),
            field_plan: Vec::new(),
            missing_fields: Vec::new(),
            sensitive_fields: Vec::new(),
            recommended_next_actions,
        })
    }
}

impl BrowserRuntime for PlaywrightCliBrowserRuntime {
    fn descriptor(&self) -> BrowserRuntimeDescriptor {
        BrowserRuntimeDescriptor {
            id: "browser-runtime-playwright-cli".to_owned(),
            title: "Playwright CLI Runtime".to_owned(),
            engine: "playwright-cli".to_owned(),
            headless_default: true,
            supports_live_control: true,
            enabled: true,
        }
    }

    fn execute(&self, spec: &BrowserTaskSpec) -> Result<BrowserExecutionOutput> {
        let config = BrowserRuntimeConfig::from_env();
        let boundary = browser_guardrail_note(&spec.intent).to_owned();
        let fallback_actions = suggest_next_actions(spec);

        let Some(cli_command) = config.cli_command.clone() else {
            return Ok(BrowserExecutionOutput {
                summary: format!(
                    "Playwright CLI runtime path selected.\n\nTarget request:\n{}\n\nMode: {}\nIntent: {}\nAction phase: {}\nTarget URL: {}\n\nBehavior boundary: {}\n\nRecommended next actions:\n- {}\n\nCurrent behavior: the runtime selection is now pointing at the Playwright-capable slot, but no browser bridge command is configured yet. Set `NEXUS_BROWSER_CLI_COMMAND` to connect a real Playwright runner while preserving the same BrowserTaskSpec and BrowserExecutionOutput contracts.",
                    spec.prompt.trim(),
                    spec.mode.as_str(),
                    spec.intent.as_str(),
                    spec.action_phase.as_str(),
                    spec.target_url.as_deref().unwrap_or("not detected"),
                    boundary,
                    fallback_actions.join("\n- "),
                ),
                transcript: vec![
                    "browser runtime selected in playwright-cli slot".to_owned(),
                    format!("mode={}", spec.mode.as_str()),
                    format!("intent={}", spec.intent.as_str()),
                    format!("action_phase={}", spec.action_phase.as_str()),
                    format!("boundary={boundary}"),
                    "playwright bridge command is not configured".to_owned(),
                ],
                target_url: spec.target_url.clone(),
                intent: spec.intent.as_str().to_owned(),
                mode: spec.mode.as_str().to_owned(),
                action_phase: spec.action_phase.as_str().to_owned(),
                boundary,
                text_snippet: None,
                link_sample: Vec::new(),
                form_count: None,
                input_sample: Vec::new(),
                field_plan: Vec::new(),
                missing_fields: Vec::new(),
                sensitive_fields: Vec::new(),
                recommended_next_actions: fallback_actions,
            });
        };

        let cli_spec = BrowserCliSpec {
            prompt: &spec.prompt,
            risk_level: risk_level_to_str(&spec.risk_level),
            mode: spec.mode.as_str(),
            intent: spec.intent.as_str(),
            action_phase: spec.action_phase.as_str(),
            target_url: spec.target_url.as_deref(),
        };
        let spec_json = serde_json::to_string(&cli_spec)?;

        let mut command = Command::new(cli_command);
        command.args(&config.cli_args);
        command.arg("--spec-json").arg(spec_json);

        let output = command.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
            return Ok(BrowserExecutionOutput {
                summary: format!(
                    "Playwright CLI runtime failed to execute.\n\nMode: {}\nIntent: {}\nAction phase: {}\nTarget URL: {}\n\nCommand bridge returned a non-zero exit status.",
                    spec.mode.as_str(),
                    spec.intent.as_str(),
                    spec.action_phase.as_str(),
                    spec.target_url.as_deref().unwrap_or("not detected"),
                ),
                transcript: vec![
                    "browser runtime selected in playwright-cli slot".to_owned(),
                    "external command launched".to_owned(),
                    format!("action_phase={}", spec.action_phase.as_str()),
                    format!("boundary={boundary}"),
                    format!(
                        "command failed: {}",
                        if stderr.is_empty() {
                            "no stderr output"
                        } else {
                            &stderr
                        }
                    ),
                ],
                target_url: spec.target_url.clone(),
                intent: spec.intent.as_str().to_owned(),
                mode: spec.mode.as_str().to_owned(),
                action_phase: spec.action_phase.as_str().to_owned(),
                boundary,
                text_snippet: None,
                link_sample: Vec::new(),
                form_count: None,
                input_sample: Vec::new(),
                field_plan: Vec::new(),
                missing_fields: Vec::new(),
                sensitive_fields: Vec::new(),
                recommended_next_actions: fallback_actions,
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        if let Ok(parsed) = serde_json::from_str::<BrowserCliOutput>(&stdout) {
            return Ok(BrowserExecutionOutput {
                summary: parsed
                    .summary
                    .unwrap_or_else(|| "Playwright CLI bridge completed.".to_owned()),
                transcript: parsed.transcript.unwrap_or_else(|| {
                    vec![
                        "browser runtime selected in playwright-cli slot".to_owned(),
                        "external command launched".to_owned(),
                        "json result parsed".to_owned(),
                    ]
                }),
                target_url: parsed.target_url.or_else(|| spec.target_url.clone()),
                intent: parsed.intent.unwrap_or_else(|| spec.intent.as_str().to_owned()),
                mode: parsed.mode.unwrap_or_else(|| spec.mode.as_str().to_owned()),
                action_phase: parsed
                    .action_phase
                    .unwrap_or_else(|| spec.action_phase.as_str().to_owned()),
                boundary: parsed.boundary.unwrap_or_else(|| boundary.clone()),
                text_snippet: parsed.text_snippet,
                link_sample: parsed.link_sample.unwrap_or_default(),
                form_count: parsed.form_count,
                input_sample: parsed.input_sample.unwrap_or_default(),
                field_plan: parsed.field_plan.unwrap_or_default(),
                missing_fields: parsed.missing_fields.unwrap_or_default(),
                sensitive_fields: parsed.sensitive_fields.unwrap_or_default(),
                recommended_next_actions: parsed
                    .recommended_next_actions
                    .unwrap_or_else(|| fallback_actions.clone()),
            });
        }

        Ok(BrowserExecutionOutput {
            summary: format!(
                "Playwright CLI runtime executed through external command bridge.\n\nTarget request:\n{}\n\nMode: {}\nIntent: {}\nAction phase: {}\nTarget URL: {}\n\nCommand output:\n{}",
                spec.prompt.trim(),
                spec.mode.as_str(),
                spec.intent.as_str(),
                spec.action_phase.as_str(),
                spec.target_url.as_deref().unwrap_or("not detected"),
                if stdout.is_empty() { "[no stdout]" } else { &stdout },
            ),
            transcript: vec![
                "browser runtime selected in playwright-cli slot".to_owned(),
                format!("mode={}", spec.mode.as_str()),
                format!("intent={}", spec.intent.as_str()),
                format!("action_phase={}", spec.action_phase.as_str()),
                format!("boundary={boundary}"),
                "external command launched".to_owned(),
                "stdout captured as plain text".to_owned(),
            ],
            target_url: spec.target_url.clone(),
            intent: spec.intent.as_str().to_owned(),
            mode: spec.mode.as_str().to_owned(),
            action_phase: spec.action_phase.as_str().to_owned(),
            boundary,
            text_snippet: None,
            link_sample: Vec::new(),
            form_count: None,
            input_sample: Vec::new(),
            field_plan: Vec::new(),
            missing_fields: Vec::new(),
            sensitive_fields: Vec::new(),
            recommended_next_actions: fallback_actions,
        })
    }
}

pub fn build_browser_runtime(config: &BrowserRuntimeConfig) -> Box<dyn BrowserRuntime> {
    match config.mode.to_lowercase().as_str() {
        "playwright" | "playwright-cli" => Box::new(PlaywrightCliBrowserRuntime),
        _ => Box::new(ScaffoldBrowserRuntime),
    }
}

pub fn list_browser_runtime_catalog(config: &BrowserRuntimeConfig) -> Vec<BrowserRuntimeDescriptor> {
    let selected = config.mode.to_lowercase();
    let mut catalog = vec![
        ScaffoldBrowserRuntime.descriptor(),
        PlaywrightCliBrowserRuntime.descriptor(),
    ];

    for runtime in &mut catalog {
        runtime.enabled = match runtime.id.as_str() {
            "browser-runtime-playwright-cli" => {
                matches!(selected.as_str(), "playwright" | "playwright-cli")
            }
            _ => selected == "scaffold",
        };
    }

    catalog
}

pub fn parse_browser_task(prompt: &str, risk_level: RiskLevel) -> BrowserTaskSpec {
    let lower = prompt.to_lowercase();
    let target_url = extract_target_url(prompt);
    let intent = if contains_any(&lower, &["login", "sign in"])
        || contains_any_raw(prompt, &["登录"])
    {
        BrowserIntent::Login
    } else if contains_any(&lower, &["form", "submit", "fill"])
        || contains_any_raw(prompt, &["填写", "表单"])
    {
        BrowserIntent::FillForm
    } else if contains_any(&lower, &["extract", "scrape"])
        || contains_any_raw(prompt, &["抓取", "提取"])
    {
        BrowserIntent::ExtractInformation
    } else if target_url.is_some()
        || contains_any(&lower, &["open", "visit"])
        || contains_any_raw(prompt, &["打开", "访问"])
    {
        BrowserIntent::OpenPage
    } else {
        BrowserIntent::Unknown
    };
    let action_phase = infer_action_phase(prompt, &intent);
    let mode = if matches!(intent, BrowserIntent::Login | BrowserIntent::FillForm)
        || matches!(risk_level, RiskLevel::L4 | RiskLevel::L5)
    {
        BrowserRunMode::Observe
    } else {
        BrowserRunMode::Silent
    };

    BrowserTaskSpec {
        prompt: prompt.to_owned(),
        risk_level,
        mode,
        target_url,
        intent,
        action_phase,
    }
}

fn extract_target_url(prompt: &str) -> Option<String> {
    prompt
        .split_whitespace()
        .find(|token| token.starts_with("http://") || token.starts_with("https://"))
        .map(|token| token.trim_matches(|c: char| c == ',' || c == ';' || c == ')').to_owned())
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn contains_any_raw(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn risk_level_to_str(level: &RiskLevel) -> &'static str {
    match level {
        RiskLevel::L1 => "L1",
        RiskLevel::L2 => "L2",
        RiskLevel::L3 => "L3",
        RiskLevel::L4 => "L4",
        RiskLevel::L5 => "L5",
    }
}

fn browser_guardrail_note(intent: &BrowserIntent) -> &'static str {
    match intent {
        BrowserIntent::Login => {
            "login intent stays in inspect-first mode and should not submit credentials automatically"
        }
        BrowserIntent::FillForm => {
            "form intent stays in inspect-first mode and should not submit until the next controlled step"
        }
        _ => "standard browser observation mode applies",
    }
}

fn infer_action_phase(prompt: &str, intent: &BrowserIntent) -> BrowserActionPhase {
    let lower = prompt.to_lowercase();
    let asks_for_submit = contains_any(&lower, &["submit", "confirm", "finish"])
        || contains_any_raw(prompt, &["提交", "确认"]);

    match intent {
        BrowserIntent::Login | BrowserIntent::FillForm if asks_for_submit => {
            BrowserActionPhase::SubmitBlocked
        }
        BrowserIntent::FillForm => BrowserActionPhase::FillOnly,
        _ => BrowserActionPhase::InspectOnly,
    }
}

fn suggest_next_actions(spec: &BrowserTaskSpec) -> Vec<String> {
    match spec.action_phase {
        BrowserActionPhase::SubmitBlocked => vec![
            "Inspect the current page and confirm the exact submission trigger.".to_owned(),
            "Prepare the final field values and submission target for approval.".to_owned(),
            "Do not submit until an explicit approval step unlocks the final action.".to_owned(),
        ],
        BrowserActionPhase::FillOnly => vec![
            "Map required fields and detect which inputs are missing.".to_owned(),
            "Prepare a structured field/value plan before any submission attempt.".to_owned(),
            "Wait for the next controlled execution step before submitting the form.".to_owned(),
        ],
        BrowserActionPhase::InspectOnly => match spec.intent {
            BrowserIntent::Login => vec![
                "Inspect visible login fields and session requirements.".to_owned(),
                "Confirm whether credentials should be provided through a controlled approval step.".to_owned(),
                "Only attempt credential entry after the next explicit authorization boundary.".to_owned(),
            ],
            BrowserIntent::ExtractInformation => vec![
                "Review the extracted snippet and sampled links.".to_owned(),
                "Decide whether another page or deeper extraction is needed.".to_owned(),
            ],
            BrowserIntent::OpenPage => vec![
                "Confirm the target page and current runtime mode.".to_owned(),
                "Choose whether to keep observing or start a more specific browser task.".to_owned(),
            ],
            BrowserIntent::Unknown => vec![
                "Clarify the intended browser action.".to_owned(),
                "Provide a target URL or name the page operation more explicitly.".to_owned(),
            ],
            BrowserIntent::FillForm => vec![
                "Inspect the target form before modifying any values.".to_owned(),
                "Decide whether the next step should stay in fill-only mode or request approval for submission.".to_owned(),
            ],
        },
    }
}
