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
pub struct BrainDecision {
    pub route: BrainRoute,
    pub confidence: f32,
    pub reason: String,
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
            };
        }

        if lower.trim().is_empty() {
            return BrainDecision {
                route: BrainRoute::Unknown,
                confidence: 0.2,
                reason: "empty input".to_owned(),
            };
        }

        BrainDecision {
            route: BrainRoute::Chat,
            confidence: 0.62,
            reason: "default conversational route".to_owned(),
        }
    }
}
