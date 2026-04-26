use std::{fs, path::Path};

use chrono::{Duration, Utc};
use nexus_protocol::{
    ApprovalRecord, ApprovalStatus, ExecutionMode, RiskLevel, TaskRecord, TaskStatus,
    TaskStepRecord, TaskStepStatus,
};
use serde::Deserialize;
use uuid::Uuid;

pub struct TaskService;

pub trait RiskPolicy: Send + Sync {
    fn classify(&self, message: &str) -> RiskLevel;
}

pub struct KeywordRiskPolicy;

pub struct RuleBasedRiskPolicy {
    default_level: RiskLevel,
    rules: Vec<RiskRule>,
}

struct RiskRule {
    level: RiskLevel,
    keywords: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RiskPolicyConfig {
    default_level: String,
    rules: Vec<RiskRuleConfig>,
}

#[derive(Debug, Deserialize)]
struct RiskRuleConfig {
    level: String,
    keywords: Vec<String>,
}

impl RiskPolicy for KeywordRiskPolicy {
    fn classify(&self, message: &str) -> RiskLevel {
        classify_risk(message)
    }
}

impl RiskPolicy for RuleBasedRiskPolicy {
    fn classify(&self, message: &str) -> RiskLevel {
        let lower = message.to_lowercase();
        for rule in &self.rules {
            if rule.keywords.iter().any(|token| lower.contains(token)) {
                return rule.level.clone();
            }
        }
        self.default_level.clone()
    }
}

impl RuleBasedRiskPolicy {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let path_ref = path.as_ref();
        let content = fs::read_to_string(path_ref).map_err(|err| {
            format!(
                "failed to read risk policy file {}: {err}",
                path_ref.display()
            )
        })?;
        let config: RiskPolicyConfig = serde_json::from_str(&content)
            .map_err(|err| format!("invalid risk policy JSON: {err}"))?;
        Self::from_config(config)
    }

    fn from_config(config: RiskPolicyConfig) -> Result<Self, String> {
        let default_level = parse_level(&config.default_level)?;
        let mut rules = Vec::new();

        for rule in config.rules {
            let level = parse_level(&rule.level)?;
            let keywords = rule
                .keywords
                .into_iter()
                .map(|item| item.to_lowercase())
                .collect::<Vec<_>>();
            if keywords.is_empty() {
                continue;
            }
            rules.push(RiskRule { level, keywords });
        }

        Ok(Self {
            default_level,
            rules,
        })
    }
}

impl TaskService {
    pub fn create_from_prompt(message: &str) -> TaskRecord {
        let policy = KeywordRiskPolicy;
        Self::create_from_prompt_with_policy(message, &policy)
    }

    pub fn create_from_prompt_with_policy(
        message: &str,
        risk_policy: &dyn RiskPolicy,
    ) -> TaskRecord {
        let now = Utc::now();
        let risk_level = risk_policy.classify(message);
        let status = if requires_approval(&risk_level) {
            TaskStatus::AwaitingApproval
        } else {
            TaskStatus::Executing
        };

        TaskRecord {
            id: Uuid::new_v4(),
            title: summarize_title(message),
            goal: message.to_owned(),
            source: "desktop.chat".to_owned(),
            status,
            priority: 2,
            risk_level,
            execution_mode: ExecutionMode::Collaborative,
            result_summary: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn create_approval(task: &TaskRecord) -> ApprovalRecord {
        let now = Utc::now();
        ApprovalRecord {
            id: Uuid::new_v4(),
            task_id: task.id,
            action_type: "task.execute".to_owned(),
            risk_level: task.risk_level.clone(),
            reason: "Task risk level requires explicit approval before execution".to_owned(),
            payload: task.goal.clone(),
            status: ApprovalStatus::Pending,
            created_at: now,
            expires_at: now + Duration::hours(24),
        }
    }

    pub fn build_plan(task: &TaskRecord) -> Vec<TaskStepRecord> {
        let now = Utc::now();
        let steps = suggest_steps(&task.goal);
        steps
            .into_iter()
            .enumerate()
            .map(|(index, (title, detail, route))| TaskStepRecord {
                id: Uuid::new_v4(),
                task_id: task.id,
                title,
                detail,
                status: TaskStepStatus::Pending,
                position: index as u32,
                route,
                created_at: now,
                completed_at: None,
            })
            .collect()
    }
}

pub fn requires_approval(level: &RiskLevel) -> bool {
    matches!(level, RiskLevel::L4 | RiskLevel::L5)
}

pub fn load_risk_policy_from_file(path: impl AsRef<Path>) -> Result<Box<dyn RiskPolicy>, String> {
    let policy = RuleBasedRiskPolicy::from_file(path)?;
    Ok(Box::new(policy))
}

pub fn default_risk_policy() -> Box<dyn RiskPolicy> {
    Box::new(KeywordRiskPolicy)
}

fn summarize_title(message: &str) -> String {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        return "Untitled Task".to_owned();
    }

    let mut title = trimmed.chars().take(32).collect::<String>();
    if trimmed.chars().count() > 32 {
        title.push_str("...");
    }
    title
}

fn classify_risk(message: &str) -> RiskLevel {
    let lower = message.to_lowercase();
    let high_risk_tokens = [
        "delete",
        "drop",
        "shutdown",
        "transfer",
        "wire",
        "send money",
        "production",
        "deploy",
        "reboot",
        "format",
        "remove",
    ];
    let medium_risk_tokens = [
        "edit",
        "modify",
        "write",
        "update",
        "change",
        "execute",
        "login",
        "submit",
        "fill form",
    ];

    if high_risk_tokens.iter().any(|token| lower.contains(token)) {
        return RiskLevel::L4;
    }
    if contains_raw(message, &["登录", "提交", "填写", "表单"])
        || medium_risk_tokens.iter().any(|token| lower.contains(token))
    {
        return RiskLevel::L3;
    }
    RiskLevel::L2
}

fn parse_level(raw: &str) -> Result<RiskLevel, String> {
    match raw {
        "L1" => Ok(RiskLevel::L1),
        "L2" => Ok(RiskLevel::L2),
        "L3" => Ok(RiskLevel::L3),
        "L4" => Ok(RiskLevel::L4),
        "L5" => Ok(RiskLevel::L5),
        _ => Err(format!("unsupported risk level: {raw}")),
    }
}

fn suggest_steps(message: &str) -> Vec<(String, String, String)> {
    let lower = message.to_lowercase();

    if contains_raw(message, &["开发", "修复", "实现"])
        || ["code", "bug", "fix", "implement", "refactor"]
            .iter()
            .any(|token| lower.contains(token))
    {
        return vec![
            (
                "Analyze workspace".to_owned(),
                "Inspect the current codebase, relevant files, and constraints.".to_owned(),
                "dev".to_owned(),
            ),
            (
                "Implement changes".to_owned(),
                "Apply the required code updates in a limited and auditable scope.".to_owned(),
                "dev".to_owned(),
            ),
            (
                "Verify results".to_owned(),
                "Run checks or builds and summarize the outcome.".to_owned(),
                "dev".to_owned(),
            ),
        ];
    }

    if contains_raw(message, &["登录"])
        || ["login", "sign in"]
            .iter()
            .any(|token| lower.contains(token))
    {
        return vec![
            (
                "Inspect target page".to_owned(),
                "Open the page, inspect login-related structure, and confirm whether credentials or session state are required.".to_owned(),
                "browser".to_owned(),
            ),
            (
                "Detect form fields".to_owned(),
                "Identify input fields, submit buttons, and any visible authentication flow before taking action.".to_owned(),
                "browser".to_owned(),
            ),
            (
                "Prepare controlled execution".to_owned(),
                "Return a structured browser result and hold for the next approved action if the flow becomes sensitive.".to_owned(),
                "browser".to_owned(),
            ),
        ];
    }

    if contains_raw(message, &["表单", "填写"])
        || ["form", "submit", "fill"]
            .iter()
            .any(|token| lower.contains(token))
    {
        return vec![
            (
                "Inspect form structure".to_owned(),
                "Determine how many forms and input controls are present on the target page.".to_owned(),
                "browser".to_owned(),
            ),
            (
                "Map required inputs".to_owned(),
                "Extract visible field hints, placeholder values, and likely submission actions.".to_owned(),
                "browser".to_owned(),
            ),
            (
                "Hold for controlled action".to_owned(),
                "Return the detected structure first so the next step can be executed under the right approval policy.".to_owned(),
                "browser".to_owned(),
            ),
        ];
    }

    if contains_raw(message, &["抓取", "提取"])
        || ["extract", "scrape"]
            .iter()
            .any(|token| lower.contains(token))
    {
        return vec![
            (
                "Open and classify page".to_owned(),
                "Open the target page and detect its basic structure, title, and entry points.".to_owned(),
                "browser".to_owned(),
            ),
            (
                "Extract structured content".to_owned(),
                "Capture a concise text snippet and representative links from the page.".to_owned(),
                "browser".to_owned(),
            ),
            (
                "Return structured result".to_owned(),
                "Package the extracted information into a task result that can be audited and reused.".to_owned(),
                "browser".to_owned(),
            ),
        ];
    }

    if contains_raw(message, &["页面", "网页", "浏览器", "网站"])
        || ["browser", "web", "site", "page"]
            .iter()
            .any(|token| lower.contains(token))
    {
        return vec![
            (
                "Interpret target".to_owned(),
                "Determine the target site or page and the intended browser operation.".to_owned(),
                "browser".to_owned(),
            ),
            (
                "Inspect live structure".to_owned(),
                "Use the browser runtime to collect page shape, form hints, or content targets before doing sensitive actions.".to_owned(),
                "browser".to_owned(),
            ),
            (
                "Return controlled result".to_owned(),
                "Summarize the browser findings and keep the next action within approval boundaries.".to_owned(),
                "browser".to_owned(),
            ),
        ];
    }

    vec![
        (
            "Clarify goal".to_owned(),
            "Interpret the user request and identify the intended deliverable.".to_owned(),
            "chat".to_owned(),
        ),
        (
            "Plan execution".to_owned(),
            "Select the right module path, constraints, and execution strategy.".to_owned(),
            "chat".to_owned(),
        ),
        (
            "Produce result".to_owned(),
            "Generate the answer or action result and prepare follow-up context.".to_owned(),
            "chat".to_owned(),
        ),
    ]
}

fn contains_raw(message: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| message.contains(needle))
}
