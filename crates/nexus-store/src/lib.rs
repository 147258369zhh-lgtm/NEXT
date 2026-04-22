use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use nexus_memory::MemoryCard;
use nexus_protocol::{
    ApprovalRecord, ApprovalStatus, AuditRecord, ExecutionMode, RiskLevel, TaskRecord, TaskStatus,
    TaskStepRecord, TaskStepStatus,
};
use rusqlite::{Connection, OptionalExtension, params};
use uuid::Uuid;

pub struct NexusStore {
    conn: Connection,
}

impl NexusStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<()> {
        self.conn
            .execute_batch(include_str!("../../../infra/sql/001_init.sql"))?;
        Ok(())
    }

    pub fn insert_task(&self, task: &TaskRecord) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (id, title, goal, source, status, priority, risk_level, execution_mode, result_summary, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                task.id.to_string(),
                task.title,
                task.goal,
                task.source,
                stringify_task_status(&task.status),
                task.priority,
                stringify_risk_level(&task.risk_level),
                stringify_execution_mode(&task.execution_mode),
                task.result_summary,
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn update_task_result(
        &self,
        task_id: Uuid,
        summary: &str,
        status: TaskStatus,
    ) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE tasks SET result_summary = ?1, status = ?2, updated_at = ?3 WHERE id = ?4",
            params![
                summary,
                stringify_task_status(&status),
                now,
                task_id.to_string()
            ],
        )?;
        Ok(())
    }

    pub fn update_task_status(&self, task_id: Uuid, status: TaskStatus) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE tasks SET status = ?1, updated_at = ?2 WHERE id = ?3",
            params![stringify_task_status(&status), now, task_id.to_string()],
        )?;
        Ok(())
    }

    pub fn insert_audit(&self, audit: &AuditRecord) -> Result<()> {
        self.conn.execute(
            "INSERT INTO audit_records (id, task_id, event_type, actor, channel, tool_name, risk_level, result, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                audit.id.to_string(),
                audit.task_id.to_string(),
                audit.event_type,
                audit.actor,
                audit.channel,
                audit.tool_name,
                stringify_risk_level(&audit.risk_level),
                audit.result,
                audit.timestamp.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn insert_approval(&self, approval: &ApprovalRecord) -> Result<()> {
        self.conn.execute(
            "INSERT INTO approvals (id, task_id, action_type, risk_level, reason, payload, status, created_at, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                approval.id.to_string(),
                approval.task_id.to_string(),
                approval.action_type,
                stringify_risk_level(&approval.risk_level),
                approval.reason,
                approval.payload,
                stringify_approval_status(&approval.status),
                approval.created_at.to_rfc3339(),
                approval.expires_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn list_pending_approvals(&self) -> Result<Vec<ApprovalRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task_id, action_type, risk_level, reason, payload, status, created_at, expires_at
             FROM approvals WHERE status = 'pending' ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map([], map_approval_row)?;
        let mut approvals = Vec::new();
        for row in rows {
            approvals.push(row?);
        }
        Ok(approvals)
    }

    pub fn list_recent_approvals(&self, limit: usize) -> Result<Vec<ApprovalRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task_id, action_type, risk_level, reason, payload, status, created_at, expires_at
             FROM approvals ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit as i64], map_approval_row)?;
        let mut approvals = Vec::new();
        for row in rows {
            approvals.push(row?);
        }
        Ok(approvals)
    }

    pub fn find_approval(&self, approval_id: Uuid) -> Result<Option<ApprovalRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task_id, action_type, risk_level, reason, payload, status, created_at, expires_at
             FROM approvals WHERE id = ?1",
        )?;
        let approval = stmt
            .query_row([approval_id.to_string()], map_approval_row)
            .optional()?;
        Ok(approval)
    }

    pub fn update_approval_status(&self, approval_id: Uuid, status: ApprovalStatus) -> Result<()> {
        self.conn.execute(
            "UPDATE approvals SET status = ?1 WHERE id = ?2",
            params![stringify_approval_status(&status), approval_id.to_string()],
        )?;
        Ok(())
    }

    pub fn find_task(&self, task_id: Uuid) -> Result<Option<TaskRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, goal, source, status, priority, risk_level, execution_mode, result_summary, created_at, updated_at
             FROM tasks WHERE id = ?1",
        )?;
        let task = stmt
            .query_row([task_id.to_string()], map_task_row)
            .optional()?;
        Ok(task)
    }

    pub fn list_recent_tasks(&self, limit: usize) -> Result<Vec<TaskRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, goal, source, status, priority, risk_level, execution_mode, result_summary, created_at, updated_at
             FROM tasks ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit as i64], map_task_row)?;
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row?);
        }
        Ok(tasks)
    }

    pub fn insert_memory_card(&self, card: &MemoryCard) -> Result<()> {
        let tags = card.tags.join(",");
        self.conn.execute(
            "INSERT INTO memory_cards (id, task_id, card_type, title, content, tags, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                card.id.to_string(),
                card.task_id.to_string(),
                card.card_type,
                card.title,
                card.content,
                tags,
                card.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn list_recent_memory_cards(&self, limit: usize) -> Result<Vec<MemoryCard>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task_id, card_type, title, content, tags, created_at
             FROM memory_cards ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit as i64], map_memory_row)?;
        let mut cards = Vec::new();
        for row in rows {
            cards.push(row?);
        }
        Ok(cards)
    }

    pub fn count_memory_cards(&self) -> Result<usize> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(1) FROM memory_cards")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count as usize)
    }

    pub fn insert_task_steps(&self, steps: &[TaskStepRecord]) -> Result<()> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO task_steps (id, task_id, title, detail, status, position, created_at, completed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )?;
        for step in steps {
            stmt.execute(params![
                step.id.to_string(),
                step.task_id.to_string(),
                step.title,
                step.detail,
                stringify_task_step_status(&step.status),
                step.position,
                step.created_at.to_rfc3339(),
                step.completed_at.map(|value| value.to_rfc3339()),
            ])?;
        }
        Ok(())
    }

    pub fn list_task_steps(&self, task_id: Uuid) -> Result<Vec<TaskStepRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task_id, title, detail, status, position, created_at, completed_at
             FROM task_steps WHERE task_id = ?1 ORDER BY position ASC",
        )?;
        let rows = stmt.query_map([task_id.to_string()], map_task_step_row)?;
        let mut steps = Vec::new();
        for row in rows {
            steps.push(row?);
        }
        Ok(steps)
    }

    pub fn mark_task_steps_started(&self, task_id: Uuid) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE task_steps
             SET status = CASE WHEN position = 0 THEN 'in_progress' ELSE status END,
                 completed_at = CASE WHEN position = 0 THEN NULL ELSE completed_at END
             WHERE task_id = ?1",
            params![task_id.to_string()],
        )?;
        self.conn.execute(
            "UPDATE tasks SET updated_at = ?1 WHERE id = ?2",
            params![now, task_id.to_string()],
        )?;
        Ok(())
    }

    pub fn mark_task_steps_completed(&self, task_id: Uuid) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE task_steps SET status = 'completed', completed_at = ?1 WHERE task_id = ?2",
            params![now, task_id.to_string()],
        )?;
        Ok(())
    }
}

fn stringify_task_status(status: &TaskStatus) -> &'static str {
    match status {
        TaskStatus::Draft => "draft",
        TaskStatus::Queued => "queued",
        TaskStatus::Planning => "planning",
        TaskStatus::AwaitingApproval => "awaiting_approval",
        TaskStatus::Executing => "executing",
        TaskStatus::Blocked => "blocked",
        TaskStatus::Paused => "paused",
        TaskStatus::Completed => "completed",
        TaskStatus::Failed => "failed",
        TaskStatus::Cancelled => "cancelled",
    }
}

fn stringify_risk_level(level: &RiskLevel) -> &'static str {
    match level {
        RiskLevel::L1 => "L1",
        RiskLevel::L2 => "L2",
        RiskLevel::L3 => "L3",
        RiskLevel::L4 => "L4",
        RiskLevel::L5 => "L5",
    }
}

fn stringify_execution_mode(mode: &ExecutionMode) -> &'static str {
    match mode {
        ExecutionMode::Cautious => "cautious",
        ExecutionMode::Collaborative => "collaborative",
        ExecutionMode::Agent => "agent",
    }
}

fn stringify_approval_status(status: &ApprovalStatus) -> &'static str {
    match status {
        ApprovalStatus::Pending => "pending",
        ApprovalStatus::Approved => "approved",
        ApprovalStatus::Rejected => "rejected",
        ApprovalStatus::Expired => "expired",
    }
}

fn stringify_task_step_status(status: &TaskStepStatus) -> &'static str {
    match status {
        TaskStepStatus::Pending => "pending",
        TaskStepStatus::InProgress => "in_progress",
        TaskStepStatus::Completed => "completed",
        TaskStepStatus::Blocked => "blocked",
    }
}

fn map_approval_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ApprovalRecord> {
    use rusqlite::types::Type;

    let id_raw: String = row.get(0)?;
    let task_id_raw: String = row.get(1)?;
    let risk_raw: String = row.get(3)?;
    let status_raw: String = row.get(6)?;
    let created_raw: String = row.get(7)?;
    let expires_raw: String = row.get(8)?;

    let id = Uuid::parse_str(&id_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(err)))?;
    let task_id = Uuid::parse_str(&task_id_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(1, Type::Text, Box::new(err)))?;
    let risk_level = parse_risk_level(&risk_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(3, Type::Text, Box::new(err)))?;
    let status = parse_approval_status(&status_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(6, Type::Text, Box::new(err)))?;
    let created_at = chrono::DateTime::parse_from_rfc3339(&created_raw)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(7, Type::Text, Box::new(err)))?;
    let expires_at = chrono::DateTime::parse_from_rfc3339(&expires_raw)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(8, Type::Text, Box::new(err)))?;

    Ok(ApprovalRecord {
        id,
        task_id,
        action_type: row.get(2)?,
        risk_level,
        reason: row.get(4)?,
        payload: row.get(5)?,
        status,
        created_at,
        expires_at,
    })
}

fn map_task_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TaskRecord> {
    use rusqlite::types::Type;

    let id_raw: String = row.get(0)?;
    let status_raw: String = row.get(4)?;
    let risk_raw: String = row.get(6)?;
    let mode_raw: String = row.get(7)?;
    let created_raw: String = row.get(9)?;
    let updated_raw: String = row.get(10)?;

    let id = Uuid::parse_str(&id_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(err)))?;
    let status = parse_task_status(&status_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(4, Type::Text, Box::new(err)))?;
    let risk_level = parse_risk_level(&risk_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(6, Type::Text, Box::new(err)))?;
    let execution_mode = parse_execution_mode(&mode_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(7, Type::Text, Box::new(err)))?;
    let created_at = chrono::DateTime::parse_from_rfc3339(&created_raw)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(9, Type::Text, Box::new(err)))?;
    let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_raw)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(10, Type::Text, Box::new(err)))?;

    Ok(TaskRecord {
        id,
        title: row.get(1)?,
        goal: row.get(2)?,
        source: row.get(3)?,
        status,
        priority: row.get(5)?,
        risk_level,
        execution_mode,
        result_summary: row.get(8)?,
        created_at,
        updated_at,
    })
}

fn map_memory_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<MemoryCard> {
    use rusqlite::types::Type;

    let id_raw: String = row.get(0)?;
    let task_id_raw: String = row.get(1)?;
    let tags_raw: String = row.get(5)?;
    let created_raw: String = row.get(6)?;

    let id = Uuid::parse_str(&id_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(err)))?;
    let task_id = Uuid::parse_str(&task_id_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(1, Type::Text, Box::new(err)))?;
    let created_at = chrono::DateTime::parse_from_rfc3339(&created_raw)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(6, Type::Text, Box::new(err)))?;

    let tags = tags_raw
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| item.to_owned())
        .collect::<Vec<_>>();

    Ok(MemoryCard {
        id,
        task_id,
        card_type: row.get(2)?,
        title: row.get(3)?,
        content: row.get(4)?,
        tags,
        created_at,
    })
}

fn map_task_step_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TaskStepRecord> {
    use rusqlite::types::Type;

    let id_raw: String = row.get(0)?;
    let task_id_raw: String = row.get(1)?;
    let status_raw: String = row.get(4)?;
    let created_raw: String = row.get(6)?;
    let completed_raw: Option<String> = row.get(7)?;

    let id = Uuid::parse_str(&id_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(err)))?;
    let task_id = Uuid::parse_str(&task_id_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(1, Type::Text, Box::new(err)))?;
    let status = parse_task_step_status(&status_raw)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(4, Type::Text, Box::new(err)))?;
    let created_at = chrono::DateTime::parse_from_rfc3339(&created_raw)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(6, Type::Text, Box::new(err)))?;
    let completed_at = completed_raw
        .map(|value| {
            chrono::DateTime::parse_from_rfc3339(&value)
                .map(|dt| dt.with_timezone(&chrono::Utc))
        })
        .transpose()
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(7, Type::Text, Box::new(err)))?;

    Ok(TaskStepRecord {
        id,
        task_id,
        title: row.get(2)?,
        detail: row.get(3)?,
        status,
        position: row.get(5)?,
        created_at,
        completed_at,
    })
}

fn parse_task_status(raw: &str) -> Result<TaskStatus, std::io::Error> {
    match raw {
        "draft" => Ok(TaskStatus::Draft),
        "queued" => Ok(TaskStatus::Queued),
        "planning" => Ok(TaskStatus::Planning),
        "awaiting_approval" => Ok(TaskStatus::AwaitingApproval),
        "executing" => Ok(TaskStatus::Executing),
        "blocked" => Ok(TaskStatus::Blocked),
        "paused" => Ok(TaskStatus::Paused),
        "completed" => Ok(TaskStatus::Completed),
        "failed" => Ok(TaskStatus::Failed),
        "cancelled" => Ok(TaskStatus::Cancelled),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("unknown task status: {raw}"),
        )),
    }
}

fn parse_risk_level(raw: &str) -> Result<RiskLevel, std::io::Error> {
    match raw {
        "L1" => Ok(RiskLevel::L1),
        "L2" => Ok(RiskLevel::L2),
        "L3" => Ok(RiskLevel::L3),
        "L4" => Ok(RiskLevel::L4),
        "L5" => Ok(RiskLevel::L5),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("unknown risk level: {raw}"),
        )),
    }
}

fn parse_execution_mode(raw: &str) -> Result<ExecutionMode, std::io::Error> {
    match raw {
        "cautious" => Ok(ExecutionMode::Cautious),
        "collaborative" => Ok(ExecutionMode::Collaborative),
        "agent" => Ok(ExecutionMode::Agent),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("unknown execution mode: {raw}"),
        )),
    }
}

fn parse_approval_status(raw: &str) -> Result<ApprovalStatus, std::io::Error> {
    match raw {
        "pending" => Ok(ApprovalStatus::Pending),
        "approved" => Ok(ApprovalStatus::Approved),
        "rejected" => Ok(ApprovalStatus::Rejected),
        "expired" => Ok(ApprovalStatus::Expired),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("unknown approval status: {raw}"),
        )),
    }
}

fn parse_task_step_status(raw: &str) -> Result<TaskStepStatus, std::io::Error> {
    match raw {
        "pending" => Ok(TaskStepStatus::Pending),
        "in_progress" => Ok(TaskStepStatus::InProgress),
        "completed" => Ok(TaskStepStatus::Completed),
        "blocked" => Ok(TaskStepStatus::Blocked),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("unknown task step status: {raw}"),
        )),
    }
}
