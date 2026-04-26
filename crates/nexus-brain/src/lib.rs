use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrainRoute {
    Chat,
    TaskExecution,
    ApprovalDecision,
    Unknown,
}

impl BrainRoute {
    pub fn as_str(&self) -> &'static str {
        match self {
            BrainRoute::Chat => "chat",
            BrainRoute::TaskExecution => "task_execution",
            BrainRoute::ApprovalDecision => "approval_decision",
            BrainRoute::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainStep {
    pub id: String,
    pub title: String,
    pub detail: String,
    pub route: BrainRoute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainPlan {
    pub steps: Vec<BrainStep>,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainDecision {
    pub route: BrainRoute,
    pub confidence: f32,
    pub reason: String,
    pub plan: Option<BrainPlan>,
}

pub struct BrainKernel;

impl BrainKernel {
    pub fn decide(message: &str) -> BrainDecision {
        let lower = message.to_lowercase();

        let approval_tokens = ["approve", "approved", "reject", "rejected", "approval"];
        if approval_tokens.iter().any(|token| lower.contains(token)) {
            return BrainDecision {
                route: BrainRoute::ApprovalDecision,
                confidence: 0.86,
                reason: "matched approval intent keywords".to_owned(),
                plan: None,
            };
        }

        let task_tokens = [
            "create",
            "build",
            "implement",
            "modify",
            "update",
            "run",
            "deploy",
            "execute",
            "fix",
            "任务",
            "实现",
            "修改",
            "执行",
            "部署",
        ];
        if task_tokens.iter().any(|token| lower.contains(token)) {
            return BrainDecision {
                route: BrainRoute::TaskExecution,
                confidence: 0.8,
                reason: "matched task execution keywords".to_owned(),
                plan: None,
            };
        }

        if lower.trim().is_empty() {
            return BrainDecision {
                route: BrainRoute::Unknown,
                confidence: 0.2,
                reason: "empty input".to_owned(),
                plan: None,
            };
        }

        BrainDecision {
            route: BrainRoute::Chat,
            confidence: 0.62,
            reason: "default conversational route".to_owned(),
            plan: None,
        }
    }

    /// 借鉴 Roo Code：将 LLM 的文本规划解析为结构化的 BrainPlan
    pub fn parse_plan(raw_response: &str) -> Option<BrainPlan> {
        // 实际实现应使用正则表达式提取特定标记的 JSON 块
        // 这里提供一个符合架构的占位实现
        if raw_response.contains("PLAN:") {
             Some(BrainPlan {
                 steps: vec![], // 实际解析逻辑
                 rationale: "Parsed from LLM response".to_owned(),
             })
        } else {
            None
        }
    }
}
