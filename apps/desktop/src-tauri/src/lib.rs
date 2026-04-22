use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use nexus_brain::{BrainDecision, BrainKernel, BrainRoute};
use nexus_memory::{MemoryCard, MemoryService};
use nexus_protocol::{
    ApprovalRecord, ApprovalStatus, ChatResponse, TaskRecord, TaskStatus, TaskStepRecord,
};
use nexus_provider::{ChatProvider, ProviderConfig, build_provider};
use nexus_store::NexusStore;
use nexus_task::{
    RiskPolicy, TaskService, default_risk_policy, load_risk_policy_from_file, requires_approval,
};
use tauri::{Manager, State};
use uuid::Uuid;

struct AppState {
    store: Arc<Mutex<NexusStore>>,
    provider: Arc<Mutex<Arc<dyn ChatProvider>>>,
    provider_source: Arc<Mutex<String>>,
    risk_policy: Arc<Mutex<Box<dyn RiskPolicy>>>,
    risk_policy_source: Arc<Mutex<String>>,
    last_brain_route: Arc<Mutex<String>>,
    brain_enabled: Arc<Mutex<bool>>,
    memory_enabled: Arc<Mutex<bool>>,
}

#[derive(serde::Serialize)]
struct ModuleStatus {
    provider_source: String,
    risk_policy_source: String,
    pending_approvals: usize,
    memory_cards: usize,
    last_brain_route: String,
    brain_enabled: bool,
    memory_enabled: bool,
}

#[derive(serde::Serialize)]
struct ModuleDescriptor {
    id: String,
    title: String,
    hot_swappable: bool,
    enabled: bool,
}

#[derive(serde::Serialize)]
struct TaskWorkspace {
    task: TaskRecord,
    steps: Vec<TaskStepRecord>,
}

struct ExecutionContext<'a> {
    state: &'a State<'a, AppState>,
    task: TaskRecord,
    prompt: String,
    approval: Option<ApprovalRecord>,
    decision: BrainDecision,
}

struct ExecutionResult {
    task: TaskRecord,
    reply: String,
    approval: Option<ApprovalRecord>,
    plan: Vec<TaskStepRecord>,
    audits: Vec<nexus_protocol::AuditRecord>,
}

#[tauri::command]
fn submit_chat(
    message: String,
    locale: Option<String>,
    state: State<'_, AppState>,
) -> Result<ChatResponse, String> {
    let locale = normalize_locale(locale.as_deref());
    let decision = build_decision(&state, &message)?;
    let task = create_task(&state, &message)?;
    let plan = TaskService::build_plan(&task);

    persist_new_task(&state, &task, &decision, &plan)?;
    update_last_brain_route(&state, decision.route.as_str())?;

    if requires_approval(&task.risk_level) {
        return queue_approval(&state, task, locale, plan);
    }

    let result = dispatch_execution(ExecutionContext {
        state: &state,
        task,
        prompt: message,
        approval: None,
        decision,
    })?;

    Ok(ChatResponse {
        task: result.task,
        reply: result.reply,
        approval: result.approval,
        plan: result.plan,
        audits: result.audits,
    })
}

#[tauri::command]
fn list_pending_approvals(state: State<'_, AppState>) -> Result<Vec<ApprovalRecord>, String> {
    let store = lock_store(&state)?;
    store
        .list_pending_approvals()
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn list_recent_tasks(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<Vec<TaskRecord>, String> {
    let store = lock_store(&state)?;
    store
        .list_recent_tasks(limit.unwrap_or(20))
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn list_recent_approvals(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<Vec<ApprovalRecord>, String> {
    let store = lock_store(&state)?;
    store
        .list_recent_approvals(limit.unwrap_or(20))
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn list_recent_memory_cards(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<Vec<MemoryCard>, String> {
    let store = lock_store(&state)?;
    store
        .list_recent_memory_cards(limit.unwrap_or(20))
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn get_latest_workspace(state: State<'_, AppState>) -> Result<Option<TaskWorkspace>, String> {
    let store = lock_store(&state)?;
    let task = store
        .list_recent_tasks(1)
        .map_err(|err| err.to_string())?
        .into_iter()
        .next();

    match task {
        Some(task) => {
            let steps = store.list_task_steps(task.id).map_err(|err| err.to_string())?;
            Ok(Some(TaskWorkspace { task, steps }))
        }
        None => Ok(None),
    }
}

#[tauri::command]
fn resolve_approval(
    approval_id: String,
    approved: bool,
    locale: Option<String>,
    state: State<'_, AppState>,
) -> Result<ChatResponse, String> {
    let locale = normalize_locale(locale.as_deref());
    let approval_uuid = Uuid::parse_str(&approval_id).map_err(|err| err.to_string())?;

    let (task, approval) = {
        let store = lock_store(&state)?;
        let approval = store
            .find_approval(approval_uuid)
            .map_err(|err| err.to_string())?
            .ok_or_else(|| "approval not found".to_owned())?;
        if !matches!(approval.status, ApprovalStatus::Pending) {
            return Err("approval is already resolved".to_owned());
        }
        let task = store
            .find_task(approval.task_id)
            .map_err(|err| err.to_string())?
            .ok_or_else(|| "task for approval not found".to_owned())?;
        (task, approval)
    };

    {
        let store = lock_store(&state)?;
        let status = if approved {
            ApprovalStatus::Approved
        } else {
            ApprovalStatus::Rejected
        };
        store
            .update_approval_status(approval.id, status)
            .map_err(|err| err.to_string())?;
        store
            .insert_audit(&nexus_audit::approval_resolved(task.id, approved))
            .map_err(|err| err.to_string())?;
    }

    if approved {
        let decision = BrainDecision {
            route: BrainRoute::ApprovalDecision,
            confidence: 0.92,
            reason: "execution resumed after explicit approval".to_owned(),
        };
        update_last_brain_route(&state, decision.route.as_str())?;
        let result = dispatch_execution(ExecutionContext {
            state: &state,
            task,
            prompt: approval.payload.clone(),
            approval: Some(approval),
            decision,
        })?;

        return Ok(ChatResponse {
            task: result.task,
            reply: result.reply,
            approval: result.approval,
            plan: result.plan,
            audits: result.audits,
        });
    }

    let task_id = task.id;
    let mut rejected_task = task;
    let summary = "Task execution was rejected during approval review";
    rejected_task.status = TaskStatus::Cancelled;
    rejected_task.result_summary = Some(summary.to_owned());

    {
        let store = lock_store(&state)?;
        store
            .update_task_result(rejected_task.id, summary, TaskStatus::Cancelled)
            .map_err(|err| err.to_string())?;
    }

    Ok(ChatResponse {
        task: rejected_task,
        reply: localize(locale, "approval_rejected").to_owned(),
        approval: Some(ApprovalRecord {
            status: ApprovalStatus::Rejected,
            ..approval
        }),
        plan: {
            let store = lock_store(&state)?;
            store.list_task_steps(task_id).map_err(|err| err.to_string())?
        },
        audits: vec![nexus_audit::approval_resolved(task_id, approved)],
    })
}

#[tauri::command]
fn reload_risk_policy(path: Option<String>, state: State<'_, AppState>) -> Result<String, String> {
    let candidate = resolve_risk_policy_path(path);
    let (new_policy, source) = if let Some(file_path) = candidate {
        if file_path.exists() {
            let policy = load_risk_policy_from_file(&file_path)?;
            (
                policy,
                format!("file:{}", file_path.to_string_lossy()),
            )
        } else {
            (default_risk_policy(), "default:keyword".to_owned())
        }
    } else {
        (default_risk_policy(), "default:keyword".to_owned())
    };

    {
        let mut guard = state
            .risk_policy
            .lock()
            .map_err(|_| "risk policy lock poisoned".to_owned())?;
        *guard = new_policy;
    }
    {
        let mut source_guard = state
            .risk_policy_source
            .lock()
            .map_err(|_| "risk policy source lock poisoned".to_owned())?;
        *source_guard = source.clone();
    }

    Ok(source)
}

#[tauri::command]
fn get_risk_policy_source(state: State<'_, AppState>) -> Result<String, String> {
    let source = state
        .risk_policy_source
        .lock()
        .map_err(|_| "risk policy source lock poisoned".to_owned())?;
    Ok(source.clone())
}

#[tauri::command]
fn reload_provider(mode: Option<String>, state: State<'_, AppState>) -> Result<String, String> {
    let mut config = ProviderConfig::from_env();
    if let Some(explicit_mode) = mode {
        config.mode = explicit_mode;
    }
    let provider = build_provider(&config).map_err(|err| err.to_string())?;
    let source = format!("{} ({})", config.mode, provider.name());

    {
        let mut guard = state
            .provider
            .lock()
            .map_err(|_| "provider lock poisoned".to_owned())?;
        *guard = provider;
    }
    {
        let mut source_guard = state
            .provider_source
            .lock()
            .map_err(|_| "provider source lock poisoned".to_owned())?;
        *source_guard = source.clone();
    }

    Ok(source)
}

#[tauri::command]
fn get_provider_source(state: State<'_, AppState>) -> Result<String, String> {
    let source = state
        .provider_source
        .lock()
        .map_err(|_| "provider source lock poisoned".to_owned())?;
    Ok(source.clone())
}

#[tauri::command]
fn set_module_enabled(
    module: String,
    enabled: bool,
    state: State<'_, AppState>,
) -> Result<ModuleStatus, String> {
    match module.as_str() {
        "brain" => {
            let mut guard = state
                .brain_enabled
                .lock()
                .map_err(|_| "brain module lock poisoned".to_owned())?;
            *guard = enabled;
            if !enabled {
                update_last_brain_route(&state, "brain_disabled")?;
            }
        }
        "memory" => {
            let mut guard = state
                .memory_enabled
                .lock()
                .map_err(|_| "memory module lock poisoned".to_owned())?;
            *guard = enabled;
        }
        _ => return Err(format!("unsupported module: {module}")),
    }

    {
        let store = lock_store(&state)?;
        let audit = nexus_audit::module_toggled(&module, enabled);
        store.insert_audit(&audit).map_err(|err| err.to_string())?;
    }

    get_module_status(state)
}

#[tauri::command]
fn list_modules(state: State<'_, AppState>) -> Result<Vec<ModuleDescriptor>, String> {
    let status = get_module_status(state)?;
    Ok(vec![
        ModuleDescriptor {
            id: "provider".to_owned(),
            title: "Provider".to_owned(),
            hot_swappable: true,
            enabled: true,
        },
        ModuleDescriptor {
            id: "risk_policy".to_owned(),
            title: "Risk Policy".to_owned(),
            hot_swappable: true,
            enabled: true,
        },
        ModuleDescriptor {
            id: "brain".to_owned(),
            title: "Brain Kernel".to_owned(),
            hot_swappable: true,
            enabled: status.brain_enabled,
        },
        ModuleDescriptor {
            id: "memory".to_owned(),
            title: "Memory".to_owned(),
            hot_swappable: true,
            enabled: status.memory_enabled,
        },
    ])
}

#[tauri::command]
fn get_module_status(state: State<'_, AppState>) -> Result<ModuleStatus, String> {
    let provider_source = state
        .provider_source
        .lock()
        .map_err(|_| "provider source lock poisoned".to_owned())?
        .clone();
    let risk_policy_source = state
        .risk_policy_source
        .lock()
        .map_err(|_| "risk policy source lock poisoned".to_owned())?
        .clone();
    let brain_enabled = is_brain_enabled(&state)?;
    let memory_enabled = is_memory_enabled(&state)?;
    let (pending_approvals, memory_cards) = {
        let store = lock_store(&state)?;
        let pending_approvals = store
            .list_pending_approvals()
            .map_err(|err| err.to_string())?
            .len();
        let memory_cards = store.count_memory_cards().map_err(|err| err.to_string())?;
        (pending_approvals, memory_cards)
    };
    let last_brain_route = state
        .last_brain_route
        .lock()
        .map_err(|_| "brain route lock poisoned".to_owned())?
        .clone();

    Ok(ModuleStatus {
        provider_source,
        risk_policy_source,
        pending_approvals,
        memory_cards,
        last_brain_route,
        brain_enabled,
        memory_enabled,
    })
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let store_path = resolve_store_path(app.handle())?;
            if let Some(parent) = store_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let store = NexusStore::open(store_path)?;
            let config = ProviderConfig::from_env();
            let provider: Arc<dyn ChatProvider> = build_provider(&config)?;
            let provider_source = format!("{} ({})", config.mode, provider.name());
            let risk_policy_path = resolve_risk_policy_path(None);
            let (risk_policy, source) = match risk_policy_path {
                Some(path) if path.exists() => match load_risk_policy_from_file(&path) {
                    Ok(policy) => (
                        policy,
                        format!("file:{}", path.to_string_lossy()),
                    ),
                    Err(_) => (default_risk_policy(), "default:keyword".to_owned()),
                },
                _ => (default_risk_policy(), "default:keyword".to_owned()),
            };

            app.manage(AppState {
                store: Arc::new(Mutex::new(store)),
                provider: Arc::new(Mutex::new(provider)),
                provider_source: Arc::new(Mutex::new(provider_source)),
                risk_policy: Arc::new(Mutex::new(risk_policy)),
                risk_policy_source: Arc::new(Mutex::new(source)),
                last_brain_route: Arc::new(Mutex::new("boot".to_owned())),
                brain_enabled: Arc::new(Mutex::new(true)),
                memory_enabled: Arc::new(Mutex::new(true)),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            submit_chat,
            list_pending_approvals,
            list_recent_tasks,
            list_recent_approvals,
            list_recent_memory_cards,
            get_latest_workspace,
            resolve_approval,
            reload_risk_policy,
            get_risk_policy_source,
            reload_provider,
            get_provider_source,
            set_module_enabled,
            list_modules,
            get_module_status
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Nexus desktop");
}

fn create_task(state: &State<'_, AppState>, message: &str) -> Result<TaskRecord, String> {
    let risk_policy_guard = state
        .risk_policy
        .lock()
        .map_err(|_| "risk policy lock poisoned".to_owned())?;
    Ok(TaskService::create_from_prompt_with_policy(
        message,
        risk_policy_guard.as_ref(),
    ))
}

fn build_decision(state: &State<'_, AppState>, message: &str) -> Result<BrainDecision, String> {
    if is_brain_enabled(state)? {
        Ok(BrainKernel::decide(message))
    } else {
        Ok(BrainDecision {
            route: BrainRoute::Chat,
            confidence: 1.0,
            reason: "brain module disabled, fallback to chat route".to_owned(),
        })
    }
}

fn dispatch_execution(ctx: ExecutionContext<'_>) -> Result<ExecutionResult, String> {
    match select_handler(&ctx.decision.route) {
        "chat" => run_chat_handler(ctx),
        "task_execution" => run_task_handler(ctx),
        "approval_decision" => run_approval_handler(ctx),
        _ => run_chat_handler(ctx),
    }
}

fn select_handler(route: &BrainRoute) -> &'static str {
    match route {
        BrainRoute::Chat => "chat",
        BrainRoute::TaskExecution => "task_execution",
        BrainRoute::ApprovalDecision => "approval_decision",
        BrainRoute::Unknown => "chat",
    }
}

fn run_chat_handler(ctx: ExecutionContext<'_>) -> Result<ExecutionResult, String> {
    run_provider_turn(ctx)
}

fn run_task_handler(ctx: ExecutionContext<'_>) -> Result<ExecutionResult, String> {
    run_provider_turn(ctx)
}

fn run_approval_handler(ctx: ExecutionContext<'_>) -> Result<ExecutionResult, String> {
    run_provider_turn(ctx)
}

fn run_provider_turn(mut ctx: ExecutionContext<'_>) -> Result<ExecutionResult, String> {
    ctx.task.status = TaskStatus::Executing;
    {
        let store = lock_store(ctx.state)?;
        store
            .update_task_status(ctx.task.id, TaskStatus::Executing)
            .map_err(|err| err.to_string())?;
        store
            .mark_task_steps_started(ctx.task.id)
            .map_err(|err| err.to_string())?;
    }

    let provider_prompt = build_provider_prompt(ctx.state, ctx.task.id, &ctx.prompt)?;
    let provider = {
        let guard = ctx
            .state
            .provider
            .lock()
            .map_err(|_| "provider lock poisoned".to_owned())?;
        guard.clone()
    };
    let reply = provider
        .reply(&provider_prompt)
        .map_err(|err| err.to_string())?;

    ctx.task.status = TaskStatus::Completed;
    ctx.task.result_summary = Some(reply.clone());

    let completed = nexus_audit::provider_completed(ctx.task.id, provider.name());
    let mut audits = vec![completed.clone()];

    {
        let store = lock_store(ctx.state)?;
        store
            .update_task_result(ctx.task.id, &reply, TaskStatus::Completed)
            .map_err(|err| err.to_string())?;
        store
            .mark_task_steps_completed(ctx.task.id)
            .map_err(|err| err.to_string())?;
        store
            .insert_audit(&completed)
            .map_err(|err| err.to_string())?;
    }

    if is_memory_enabled(ctx.state)? {
        let memory_card = MemoryService::from_turn(ctx.task.id, &ctx.prompt, &reply, &ctx.decision);
        let memory_audit = nexus_audit::memory_saved(ctx.task.id, &memory_card.card_type);
        {
            let store = lock_store(ctx.state)?;
            store
                .insert_memory_card(&memory_card)
                .map_err(|err| err.to_string())?;
            store
                .insert_audit(&memory_audit)
                .map_err(|err| err.to_string())?;
        }
        audits.push(memory_audit);
    }

    let task_id = ctx.task.id;
    let plan = {
        let store = lock_store(ctx.state)?;
        store
            .list_task_steps(task_id)
            .map_err(|err| err.to_string())?
    };

    Ok(ExecutionResult {
        task: ctx.task,
        reply,
        approval: ctx.approval,
        plan,
        audits,
    })
}

fn build_provider_prompt(
    state: &State<'_, AppState>,
    task_id: Uuid,
    prompt: &str,
) -> Result<String, String> {
    if !is_memory_enabled(state)? {
        return Ok(prompt.to_owned());
    }

    let cards = {
        let store = lock_store(state)?;
        store
            .list_recent_memory_cards(3)
            .map_err(|err| err.to_string())?
    };

    if cards.is_empty() {
        return Ok(prompt.to_owned());
    }

    let memory_context = cards
        .iter()
        .map(|card| format!("- {}: {}", card.title, clip_text(&card.content, 120)))
        .collect::<Vec<_>>()
        .join("\n");

    {
        let store = lock_store(state)?;
        let audit = nexus_audit::memory_context_loaded(task_id, cards.len());
        store.insert_audit(&audit).map_err(|err| err.to_string())?;
    }

    Ok(format!(
        "Relevant memory context:\n{}\n\nCurrent user request:\n{}",
        memory_context, prompt
    ))
}

fn persist_new_task(
    state: &State<'_, AppState>,
    task: &TaskRecord,
    decision: &BrainDecision,
    plan: &[TaskStepRecord],
) -> Result<(), String> {
    let received = nexus_audit::task_received(task.id, "desktop prompt accepted");
    let routed = nexus_audit::brain_routed(task.id, decision.route.as_str(), &decision.reason);
    let store = lock_store(state)?;
    store.insert_task(task).map_err(|err| err.to_string())?;
    store.insert_task_steps(plan).map_err(|err| err.to_string())?;
    store
        .insert_audit(&received)
        .map_err(|err| err.to_string())?;
    store.insert_audit(&routed).map_err(|err| err.to_string())?;
    Ok(())
}

fn queue_approval(
    state: &State<'_, AppState>,
    mut task: TaskRecord,
    locale: &str,
    plan: Vec<TaskStepRecord>,
) -> Result<ChatResponse, String> {
    let approval = TaskService::create_approval(&task);
    let approval_audit =
        nexus_audit::approval_requested(task.id, "Risk level is L4/L5, waiting for approval");

    {
        let store = lock_store(state)?;
        store
            .insert_approval(&approval)
            .map_err(|err| err.to_string())?;
        store
            .update_task_status(task.id, TaskStatus::AwaitingApproval)
            .map_err(|err| err.to_string())?;
        store
            .insert_audit(&approval_audit)
            .map_err(|err| err.to_string())?;
    }

    task.status = TaskStatus::AwaitingApproval;
    Ok(ChatResponse {
        task,
        reply: localize(locale, "queued_for_approval").to_owned(),
        approval: Some(approval),
        plan,
        audits: vec![approval_audit],
    })
}

fn lock_store<'a>(
    state: &'a State<'a, AppState>,
) -> Result<std::sync::MutexGuard<'a, NexusStore>, String> {
    state
        .store
        .lock()
        .map_err(|_| "store lock poisoned".to_owned())
}

fn update_last_brain_route(state: &State<'_, AppState>, route: &str) -> Result<(), String> {
    let mut guard = state
        .last_brain_route
        .lock()
        .map_err(|_| "brain route lock poisoned".to_owned())?;
    *guard = route.to_owned();
    Ok(())
}

fn is_brain_enabled(state: &State<'_, AppState>) -> Result<bool, String> {
    let guard = state
        .brain_enabled
        .lock()
        .map_err(|_| "brain module lock poisoned".to_owned())?;
    Ok(*guard)
}

fn is_memory_enabled(state: &State<'_, AppState>) -> Result<bool, String> {
    let guard = state
        .memory_enabled
        .lock()
        .map_err(|_| "memory module lock poisoned".to_owned())?;
    Ok(*guard)
}

fn clip_text(raw: &str, cap: usize) -> String {
    let trimmed = raw.trim();
    let mut text = trimmed.chars().take(cap).collect::<String>();
    if trimmed.chars().count() > cap {
        text.push_str("...");
    }
    text
}

fn resolve_store_path(app: &tauri::AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let base = app.path().app_data_dir()?;
    let profile = std::env::var("NEXUS_PROFILE").unwrap_or_else(|_| "default".to_owned());
    Ok(base.join(profile).join("nexus.db"))
}

fn resolve_risk_policy_path(explicit: Option<String>) -> Option<PathBuf> {
    if let Some(path) = explicit {
        return Some(PathBuf::from(path));
    }
    if let Ok(env_path) = env::var("NEXUS_RISK_POLICY_FILE") {
        return Some(PathBuf::from(env_path));
    }
    Some(Path::new("infra").join("configs").join("risk-policy.json"))
}

fn normalize_locale(raw: Option<&str>) -> &str {
    match raw {
        Some(value) if value.to_lowercase().starts_with("zh") => "zh-CN",
        _ => "en-US",
    }
}

fn localize(locale: &str, key: &str) -> &'static str {
    match (locale, key) {
        ("zh-CN", "approval_rejected") => "审批未通过，任务已取消。",
        ("zh-CN", "queued_for_approval") => "该任务风险较高，已进入审批队列。",
        _ if key == "approval_rejected" => "Approval rejected. Task has been cancelled.",
        _ if key == "queued_for_approval" => {
            "Task is queued for approval because the request is high risk."
        }
        _ => "",
    }
}
