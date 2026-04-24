use chrono::Utc;
use nexus_protocol::{AuditRecord, RiskLevel};
use uuid::Uuid;

pub fn task_received(task_id: Uuid, detail: &str) -> AuditRecord {
    audit(
        task_id,
        "task.received",
        "user",
        "desktop",
        None,
        RiskLevel::L1,
        detail,
    )
}

pub fn provider_completed(task_id: Uuid, model: &str) -> AuditRecord {
    audit(
        task_id,
        "provider.completed",
        "provider",
        "desktop",
        Some(model.to_owned()),
        RiskLevel::L2,
        "provider reply generated",
    )
}

pub fn approval_requested(task_id: Uuid, reason: &str) -> AuditRecord {
    audit(
        task_id,
        "approval.requested",
        "policy",
        "desktop",
        None,
        RiskLevel::L4,
        reason,
    )
}

pub fn approval_resolved(task_id: Uuid, approved: bool) -> AuditRecord {
    let result = if approved {
        "approval was granted"
    } else {
        "approval was rejected"
    };
    audit(
        task_id,
        "approval.resolved",
        "user",
        "desktop",
        None,
        RiskLevel::L4,
        result,
    )
}

pub fn brain_routed(task_id: Uuid, route: &str, reason: &str) -> AuditRecord {
    audit(
        task_id,
        "brain.routed",
        "brain",
        "desktop",
        Some(route.to_owned()),
        RiskLevel::L2,
        reason,
    )
}

pub fn memory_saved(task_id: Uuid, card_type: &str) -> AuditRecord {
    audit(
        task_id,
        "memory.saved",
        "memory",
        "desktop",
        Some(card_type.to_owned()),
        RiskLevel::L1,
        "memory card persisted",
    )
}

pub fn module_toggled(module: &str, enabled: bool) -> AuditRecord {
    let result = if enabled {
        "module enabled"
    } else {
        "module disabled"
    };
    audit(
        Uuid::nil(),
        "module.toggled",
        "user",
        "desktop",
        Some(module.to_owned()),
        RiskLevel::L1,
        result,
    )
}

pub fn memory_context_loaded(task_id: Uuid, cards: usize) -> AuditRecord {
    audit(
        task_id,
        "memory.context_loaded",
        "memory",
        "desktop",
        Some(format!("{} cards", cards)),
        RiskLevel::L1,
        "memory context injected into provider prompt",
    )
}

pub fn executor_dispatched(task_id: Uuid, executor: &str, route: &str) -> AuditRecord {
    audit(
        task_id,
        "executor.dispatched",
        "runtime",
        "desktop",
        Some(executor.to_owned()),
        RiskLevel::L2,
        &format!("executor selected for route: {route}"),
    )
}

pub fn browser_executor_prepared(task_id: Uuid, detail: &str) -> AuditRecord {
    audit(
        task_id,
        "browser.prepared",
        "browser-executor",
        "desktop",
        Some("browser-executor".to_owned()),
        RiskLevel::L3,
        detail,
    )
}

pub fn browser_extraction_saved(task_id: Uuid, detail: &str) -> AuditRecord {
    audit(
        task_id,
        "browser.extracted",
        "browser-executor",
        "desktop",
        Some("browser-executor".to_owned()),
        RiskLevel::L2,
        detail,
    )
}

pub fn dev_executor_prepared(task_id: Uuid, detail: &str) -> AuditRecord {
    audit(
        task_id,
        "dev.prepared",
        "dev-executor",
        "desktop",
        Some("dev-executor".to_owned()),
        RiskLevel::L2,
        detail,
    )
}

pub fn dev_plan_saved(task_id: Uuid, detail: &str) -> AuditRecord {
    audit(
        task_id,
        "dev.planned",
        "dev-executor",
        "desktop",
        Some("dev-executor".to_owned()),
        RiskLevel::L2,
        detail,
    )
}

pub fn dev_verification_saved(task_id: Uuid, detail: &str) -> AuditRecord {
    audit(
        task_id,
        "dev.verified",
        "dev-executor",
        "desktop",
        Some("dev-executor".to_owned()),
        RiskLevel::L2,
        detail,
    )
}

pub fn dev_patch_schema_saved(task_id: Uuid, schema_json: &str) -> AuditRecord {
    audit(
        task_id,
        "dev.patch_schema",
        "dev-executor",
        "desktop",
        Some("dev-executor".to_owned()),
        RiskLevel::L2,
        schema_json,
    )
}

pub fn dev_runner_saved(task_id: Uuid, detail: &str) -> AuditRecord {
    audit(
        task_id,
        "dev.runner",
        "patch-runner",
        "desktop",
        Some("patch-runner".to_owned()),
        RiskLevel::L2,
        detail,
    )
}

pub fn dev_runner_log_saved(task_id: Uuid, detail: &str) -> AuditRecord {
    audit(
        task_id,
        "dev.runner_log",
        "patch-runner",
        "desktop",
        Some("patch-runner".to_owned()),
        RiskLevel::L2,
        detail,
    )
}

fn audit(
    task_id: Uuid,
    event_type: &str,
    actor: &str,
    channel: &str,
    tool_name: Option<String>,
    risk_level: RiskLevel,
    result: &str,
) -> AuditRecord {
    AuditRecord {
        id: Uuid::new_v4(),
        task_id,
        event_type: event_type.to_owned(),
        actor: actor.to_owned(),
        channel: channel.to_owned(),
        tool_name,
        risk_level,
        result: result.to_owned(),
        timestamp: Utc::now(),
    }
}
