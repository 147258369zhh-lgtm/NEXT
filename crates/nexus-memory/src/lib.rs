use chrono::{DateTime, Utc};
use nexus_brain::BrainDecision;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryCard {
    pub id: Uuid,
    pub task_id: Uuid,
    pub card_type: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

pub struct MemoryService;

impl MemoryService {
    pub fn from_turn(task_id: Uuid, prompt: &str, reply: &str, decision: &BrainDecision) -> MemoryCard {
        let now = Utc::now();
        let title = summarize(prompt, 46);
        let content = format!(
            "Route: {}\nConfidence: {:.2}\nReason: {}\nPrompt: {}\nReply: {}",
            decision.route.as_str(),
            decision.confidence,
            decision.reason,
            prompt.trim(),
            summarize(reply, 240)
        );
        MemoryCard {
            id: Uuid::new_v4(),
            task_id,
            card_type: "conversation_summary".to_owned(),
            title,
            content,
            tags: vec![
                "chat".to_owned(),
                decision.route.as_str().to_owned(),
                "auto".to_owned(),
            ],
            created_at: now,
        }
    }
}

fn summarize(text: &str, cap: usize) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return "Empty".to_owned();
    }
    let mut result = trimmed.chars().take(cap).collect::<String>();
    if trimmed.chars().count() > cap {
        result.push_str("...");
    }
    result
}
