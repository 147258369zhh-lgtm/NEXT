use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, MutexGuard},
};
use async_trait::async_trait;

use nexus_brain::{BrainDecision, BrainKernel, BrainRoute};
use nexus_browser::{
    BrowserRuntimeConfig, BrowserRuntimeDescriptor, build_browser_runtime, list_browser_runtime_catalog,
    parse_browser_task,
};
use nexus_dev::{build_dev_runtime, list_patch_runner_catalog, parse_dev_task};
use nexus_memory::{MemoryCard, MemoryService};
use nexus_protocol::{
    ApprovalRecord, ApprovalStatus, AuditRecord, ChatResponse, TaskRecord, TaskStatus,
    TaskStepRecord,
};
use nexus_provider::{ChatProvider, ProviderConfig, ProviderDescriptor, build_provider, list_provider_catalog};
use nexus_store::NexusStore;
use nexus_task::{
    RiskPolicy, TaskService, default_risk_policy, load_risk_policy_from_file, requires_approval,
};
use serde::Serialize;
use uuid::Uuid;

pub use nexus_dev::PatchRunnerDescriptor;

pub struct AppRuntime {
    store: Mutex<NexusStore>,
    provider: Mutex<Arc<dyn ChatProvider>>,
    provider_source: Mutex<String>,
    risk_policy: Mutex<Box<dyn RiskPolicy>>,
    risk_policy_source: Mutex<String>,
    last_brain_route: Mutex<String>,
    brain_enabled: Mutex<bool>,
    memory_enabled: Mutex<bool>,
    skill_manager: Mutex<nexus_skill::SkillManager>,
    executors: Vec<Arc<dyn TaskExecutor>>,
}

#[derive(Serialize)]
pub struct ModuleStatus {
    provider_source: String,
    risk_policy_source: String,
    pending_approvals: usize,
    memory_cards: usize,
    last_brain_route: String,
    brain_enabled: bool,
    memory_enabled: bool,
}

#[derive(Serialize)]
pub struct ModuleDescriptor {
    id: String,
    title: String,
    hot_swappable: bool,
    enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExecutorDescriptor {
    pub id: String,
    pub title: String,
    pub route_scope: Vec<String>,
    pub enabled: bool,
}

#[derive(Serialize)]
pub struct TaskWorkspace {
    task: TaskRecord,
    steps: Vec<TaskStepRecord>,
}

struct ExecutionContext {
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

#[async_trait::async_trait]
trait TaskExecutor: Send + Sync {
    fn descriptor(&self) -> ExecutorDescriptor;
    fn supports(&self, route: &BrainRoute, prompt: &str) -> bool;
    async fn execute(&self, runtime: &AppRuntime, ctx: ExecutionContext) -> Result<ExecutionResult, String>;
}

struct ProviderExecutor;
struct BrowserExecutor;
struct DevExecutor;
struct McpExecutor;

#[async_trait::async_trait]
impl TaskExecutor for ProviderExecutor {
    fn descriptor(&self) -> ExecutorDescriptor {
        ExecutorDescriptor {
            id: "provider-default".to_owned(),
            title: "Provider Default Executor".to_owned(),
            route_scope: vec![
                BrainRoute::Chat.as_str().to_owned(),
                BrainRoute::TaskExecution.as_str().to_owned(),
                BrainRoute::ApprovalDecision.as_str().to_owned(),
                BrainRoute::Unknown.as_str().to_owned(),
            ],
            enabled: true,
        }
    }

    fn supports(&self, route: &BrainRoute, _prompt: &str) -> bool {
        matches!(
            route,
            BrainRoute::Chat
                | BrainRoute::TaskExecution
                | BrainRoute::ApprovalDecision
                | BrainRoute::Unknown
        )
    }

    async fn execute(&self, runtime: &AppRuntime, ctx: ExecutionContext) -> Result<ExecutionResult, String> {
        runtime.run_provider_turn(ctx).await
    }
}

#[async_trait::async_trait]
impl TaskExecutor for BrowserExecutor {
    fn descriptor(&self) -> ExecutorDescriptor {
        ExecutorDescriptor {
            id: "browser-executor".to_owned(),
            title: "Browser Executor".to_owned(),
            route_scope: vec![BrainRoute::TaskExecution.as_str().to_owned()],
            enabled: true,
        }
    }

    fn supports(&self, route: &BrainRoute, prompt: &str) -> bool {
        matches!(route, BrainRoute::TaskExecution) && is_browser_prompt(prompt)
    }

    async fn execute(&self, runtime: &AppRuntime, ctx: ExecutionContext) -> Result<ExecutionResult, String> {
        runtime.run_browser_turn(ctx).await
    }
}

#[async_trait::async_trait]
impl TaskExecutor for DevExecutor {
    fn descriptor(&self) -> ExecutorDescriptor {
        ExecutorDescriptor {
            id: "dev-executor".to_owned(),
            title: "Dev Executor".to_owned(),
            route_scope: vec![BrainRoute::TaskExecution.as_str().to_owned()],
            enabled: true,
        }
    }

    fn supports(&self, route: &BrainRoute, prompt: &str) -> bool {
        matches!(route, BrainRoute::TaskExecution) && is_dev_prompt(prompt)
    }

    async fn execute(&self, runtime: &AppRuntime, ctx: ExecutionContext) -> Result<ExecutionResult, String> {
        runtime.run_dev_turn(ctx).await
    }
}

#[async_trait::async_trait]
impl TaskExecutor for McpExecutor {
    fn descriptor(&self) -> ExecutorDescriptor {
        ExecutorDescriptor {
            id: "mcp-executor".to_owned(),
            title: "MCP Ecosystem Executor".to_owned(),
            route_scope: vec![BrainRoute::TaskExecution.as_str().to_owned()],
            enabled: true,
        }
    }

    fn supports(&self, route: &BrainRoute, prompt: &str) -> bool {
        matches!(route, BrainRoute::TaskExecution) && (prompt.contains("skill") || prompt.contains("mcp") || prompt.contains("openclaw") || prompt.contains("hermes"))
    }

    async fn execute(&self, _runtime: &AppRuntime, mut ctx: ExecutionContext) -> Result<ExecutionResult, String> {
        // Mocking the MCP/Skill execution
        ctx.task.status = TaskStatus::Completed;
        let reply = format!("MCP Ecosystem Executor picked up the task: {}. \n\nConnecting to external MCP servers and resolving skills...", ctx.prompt);
        ctx.task.result_summary = Some(reply.clone());
        let plan = vec![];
        let audits = vec![];

        Ok(ExecutionResult {
            task: ctx.task,
            reply,
            approval: ctx.approval,
            plan,
            audits,
        })
    }
}

impl AppRuntime {
    pub fn boot(store_path: PathBuf, risk_policy_path: Option<PathBuf>) -> anyhow::Result<Self> {
        if let Some(parent) = store_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let store = NexusStore::open(store_path)?;
        let config = ProviderConfig::from_env();
        let provider = build_provider(&config)?;
        let provider_source = format!("{} ({})", config.mode, provider.name());
        let (risk_policy, source) = match risk_policy_path {
            Some(path) if path.exists() => match load_risk_policy_from_file(&path) {
                Ok(policy) => (policy, format!("file:{}", path.to_string_lossy())),
                Err(_) => (default_risk_policy(), "default:keyword".to_owned()),
            },
            _ => (default_risk_policy(), "default:keyword".to_owned()),
        };

        Ok(Self {
            store: Mutex::new(store),
            provider: Mutex::new(provider),
            provider_source: Mutex::new(provider_source),
            risk_policy: Mutex::new(risk_policy),
            risk_policy_source: Mutex::new(source),
            last_brain_route: Mutex::new("boot".to_owned()),
            brain_enabled: Mutex::new(true),
            memory_enabled: Mutex::new(true),
            skill_manager: Mutex::new(nexus_skill::SkillManager::new("./skills")),
            executors: vec![
                Arc::new(BrowserExecutor),
                Arc::new(DevExecutor),
                Arc::new(McpExecutor),
                Arc::new(ProviderExecutor),
            ],
        })
    }

    pub async fn submit_chat(
        &self,
        message: String,
        locale: Option<String>,
    ) -> Result<ChatResponse, String> {
        self.process_message(message, locale, "user").await
    }

    pub async fn inject_external_message(
        &self,
        message: String,
        source: &str,
    ) -> Result<ChatResponse, String> {
        // 外部注入的消息默认使用英文环境并标记来源
        self.process_message(message, Some("en-US".to_string()), source).await
    }

    async fn process_message(
        &self,
        message: String,
        locale: Option<String>,
        source: &str,
    ) -> Result<ChatResponse, String> {
        let locale = normalize_locale(locale.as_deref());
        
        // 1. Architect Phase: Decide route and build initial plan
        let mut decision = self.build_decision(&message)?;
        let mut task = self.create_task(&message)?;
        task.source = source.to_string();
        
        // 2. Initial Persist
        let plan_steps = TaskService::build_plan(&task);
        self.persist_new_task(&task, &decision, &plan_steps)?;
        self.update_last_brain_route(decision.route.as_str())?;

        // 3. Approval Check
        if requires_approval(&task.risk_level) {
            return self.queue_approval(task, locale, plan_steps);
        }

        // 4. Multi-Step Execution Loop (Orchestration Pattern)
        let mut final_reply = String::new();
        let mut all_audits = Vec::new();

        for step in &plan_steps {
            println!("🔄 Executing step: {} - {}", step.id, step.title);
            
            // 根据步骤的任务类型动态调整决策路由
            let step_decision = BrainDecision {
                route: nexus_brain::BrainRoute::Unknown, // Will be resolved by dispatcher
                confidence: 1.0,
                reason: format!("Executing planned step: {}", step.title),
                plan: None,
            };

            let result = self.dispatch_execution(ExecutionContext {
                task: task.clone(),
                prompt: format!("{} - {}", step.title, step.detail),
                approval: None,
                decision: step_decision,
            }).await?;

            final_reply.push_str(&result.reply);
            final_reply.push_str("\n\n");
            all_audits.extend(result.audits);
            
            // 这里可以增加状态检查：如果某一步失败，则中断计划并反思
        }

        Ok(ChatResponse {
            task,
            reply: final_reply.trim().to_owned(),
            approval: None,
            plan: plan_steps,
            audits: all_audits,
        })
    }

    pub fn list_pending_approvals(&self) -> Result<Vec<ApprovalRecord>, String> {
        self.lock_store()?
            .list_pending_approvals()
            .map_err(|err| err.to_string())
    }

    pub fn list_recent_tasks(&self, limit: Option<usize>) -> Result<Vec<TaskRecord>, String> {
        self.lock_store()?
            .list_recent_tasks(limit.unwrap_or(20))
            .map_err(|err| err.to_string())
    }

    pub fn list_recent_approvals(&self, limit: Option<usize>) -> Result<Vec<ApprovalRecord>, String> {
        self.lock_store()?
            .list_recent_approvals(limit.unwrap_or(20))
            .map_err(|err| err.to_string())
    }

    pub fn list_recent_memory_cards(&self, limit: Option<usize>) -> Result<Vec<MemoryCard>, String> {
        self.lock_store()?
            .list_recent_memory_cards(limit.unwrap_or(20))
            .map_err(|err| err.to_string())
    }

    pub fn list_recent_audits(&self, limit: Option<usize>) -> Result<Vec<AuditRecord>, String> {
        self.lock_store()?
            .list_recent_audits(limit.unwrap_or(30))
            .map_err(|err| err.to_string())
    }

    pub fn list_executors(&self) -> Vec<ExecutorDescriptor> {
        self.executors
            .iter()
            .map(|executor| executor.descriptor())
            .collect()
    }

    pub fn list_browser_runtimes(&self) -> Vec<BrowserRuntimeDescriptor> {
        list_browser_runtime_catalog(&BrowserRuntimeConfig::from_env())
    }

    pub fn list_patch_runners(&self) -> Vec<PatchRunnerDescriptor> {
        list_patch_runner_catalog()
    }

    pub fn list_providers(&self) -> Result<Vec<ProviderDescriptor>, String> {
        list_provider_catalog(&ProviderConfig::from_env()).map_err(|err| err.to_string())
    }

    pub fn list_skills(&self) -> Result<Vec<nexus_skill::Skill>, String> {
        let mut manager = self
            .skill_manager
            .lock()
            .map_err(|_| "skill manager lock poisoned".to_owned())?;
        manager.scan().map_err(|err| err.to_string())
    }

    pub fn list_mcp_servers(&self) -> Result<Vec<nexus_mcp::McpServerDescriptor>, String> {
        // TODO: Implement actual MCP server registry. For now returning empty list.
        Ok(vec![])
    }

    pub fn get_latest_workspace(&self) -> Result<Option<TaskWorkspace>, String> {
        let store = self.lock_store()?;
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

    pub async fn resolve_approval(
        &self,
        approval_id: String,
        approved: bool,
        locale: Option<String>,
    ) -> Result<ChatResponse, String> {
        let locale = normalize_locale(locale.as_deref());
        let approval_uuid = Uuid::parse_str(&approval_id).map_err(|err| err.to_string())?;

        let (task, approval) = {
            let store = self.lock_store()?;
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
            let store = self.lock_store()?;
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
                plan: None,
            };
            self.update_last_brain_route(decision.route.as_str())?;
            let result = self.dispatch_execution(ExecutionContext {
                task,
                prompt: approval.payload.clone(),
                approval: Some(approval),
                decision,
            }).await?;

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
            let store = self.lock_store()?;
            store
                .update_task_result(rejected_task.id, summary, TaskStatus::Cancelled)
                .map_err(|err| err.to_string())?;
        }

        Ok(ChatResponse {
            task: rejected_task,
            reply: localized_text(locale, "approval_rejected").to_owned(),
            approval: Some(ApprovalRecord {
                status: ApprovalStatus::Rejected,
                ..approval
            }),
            plan: {
                let store = self.lock_store()?;
                store.list_task_steps(task_id).map_err(|err| err.to_string())?
            },
            audits: vec![nexus_audit::approval_resolved(task_id, approved)],
        })
    }

    pub fn reload_risk_policy(&self, path: Option<String>) -> Result<String, String> {
        let candidate = resolve_risk_policy_path(path);
        let (new_policy, source) = if let Some(file_path) = candidate {
            if file_path.exists() {
                let policy = load_risk_policy_from_file(&file_path)?;
                (policy, format!("file:{}", file_path.to_string_lossy()))
            } else {
                (default_risk_policy(), "default:keyword".to_owned())
            }
        } else {
            (default_risk_policy(), "default:keyword".to_owned())
        };

        {
            let mut guard = self
                .risk_policy
                .lock()
                .map_err(|_| "risk policy lock poisoned".to_owned())?;
            *guard = new_policy;
        }
        {
            let mut source_guard = self
                .risk_policy_source
                .lock()
                .map_err(|_| "risk policy source lock poisoned".to_owned())?;
            *source_guard = source.clone();
        }

        Ok(source)
    }

    pub fn get_risk_policy_source(&self) -> Result<String, String> {
        let source = self
            .risk_policy_source
            .lock()
            .map_err(|_| "risk policy source lock poisoned".to_owned())?;
        Ok(source.clone())
    }

    pub fn reload_provider(&self, mode: Option<String>) -> Result<String, String> {
        let mut config = ProviderConfig::from_env();
        if let Some(explicit_mode) = mode {
            config.mode = explicit_mode;
        }
        let provider = build_provider(&config).map_err(|err| err.to_string())?;
        let source = format!("{} ({})", config.mode, provider.name());

        {
            let mut guard = self
                .provider
                .lock()
                .map_err(|_| "provider lock poisoned".to_owned())?;
            *guard = provider;
        }
        {
            let mut source_guard = self
                .provider_source
                .lock()
                .map_err(|_| "provider source lock poisoned".to_owned())?;
            *source_guard = source.clone();
        }

        Ok(source)
    }

    pub fn get_provider_source(&self) -> Result<String, String> {
        let source = self
            .provider_source
            .lock()
            .map_err(|_| "provider source lock poisoned".to_owned())?;
        Ok(source.clone())
    }

    pub fn set_module_enabled(&self, module: String, enabled: bool) -> Result<ModuleStatus, String> {
        match module.as_str() {
            "brain" => {
                let mut guard = self
                    .brain_enabled
                    .lock()
                    .map_err(|_| "brain module lock poisoned".to_owned())?;
                *guard = enabled;
                if !enabled {
                    self.update_last_brain_route("brain_disabled")?;
                }
            }
            "memory" => {
                let mut guard = self
                    .memory_enabled
                    .lock()
                    .map_err(|_| "memory module lock poisoned".to_owned())?;
                *guard = enabled;
            }
            _ => return Err(format!("unsupported module: {module}")),
        }

        {
            let store = self.lock_store()?;
            let audit = nexus_audit::module_toggled(&module, enabled);
            store.insert_audit(&audit).map_err(|err| err.to_string())?;
        }

        self.get_module_status()
    }

    pub fn list_modules(&self) -> Result<Vec<ModuleDescriptor>, String> {
        let status = self.get_module_status()?;
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

    pub fn get_module_status(&self) -> Result<ModuleStatus, String> {
        let provider_source = self
            .provider_source
            .lock()
            .map_err(|_| "provider source lock poisoned".to_owned())?
            .clone();
        let risk_policy_source = self
            .risk_policy_source
            .lock()
            .map_err(|_| "risk policy source lock poisoned".to_owned())?
            .clone();
        let brain_enabled = self.is_brain_enabled()?;
        let memory_enabled = self.is_memory_enabled()?;
        let (pending_approvals, memory_cards) = {
            let store = self.lock_store()?;
            let pending_approvals = store
                .list_pending_approvals()
                .map_err(|err| err.to_string())?
                .len();
            let memory_cards = store.count_memory_cards().map_err(|err| err.to_string())?;
            (pending_approvals, memory_cards)
        };
        let last_brain_route = self
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

    fn create_task(&self, message: &str) -> Result<TaskRecord, String> {
        let risk_policy_guard = self
            .risk_policy
            .lock()
            .map_err(|_| "risk policy lock poisoned".to_owned())?;
        Ok(TaskService::create_from_prompt_with_policy(
            message,
            risk_policy_guard.as_ref(),
        ))
    }

    fn build_decision(&self, message: &str) -> Result<BrainDecision, String> {
        if self.is_brain_enabled()? {
            Ok(BrainKernel::decide(message))
        } else {
            Ok(BrainDecision {
                route: BrainRoute::Chat,
                confidence: 1.0,
                reason: "brain module disabled, fallback to chat route".to_owned(),
                plan: None,
            })
        }
    }

    async fn dispatch_execution(&self, ctx: ExecutionContext) -> Result<ExecutionResult, String> {
        let executor = self
            .executors
            .iter()
            .find(|executor| executor.supports(&ctx.decision.route, &ctx.prompt))
            .cloned()
            .ok_or_else(|| format!("no executor registered for route {}", ctx.decision.route.as_str()))?;

        let descriptor = executor.descriptor();
        {
            let store = self.lock_store()?;
            let audit =
                nexus_audit::executor_dispatched(ctx.task.id, &descriptor.id, ctx.decision.route.as_str());
            store.insert_audit(&audit).map_err(|err| err.to_string())?;
        }

        executor.execute(self, ctx).await
    }

    async fn run_provider_turn(&self, mut ctx: ExecutionContext) -> Result<ExecutionResult, String> {
        ctx.task.status = TaskStatus::Executing;
        {
            let store = self.lock_store()?;
            store
                .update_task_status(ctx.task.id, TaskStatus::Executing)
                .map_err(|err| err.to_string())?;
            store
                .mark_task_steps_started(ctx.task.id)
                .map_err(|err| err.to_string())?;
        }

        let provider_prompt = self.build_provider_prompt(ctx.task.id, &ctx.prompt)?;
        let provider = {
            let guard = self
                .provider
                .lock()
                .map_err(|_| "provider lock poisoned".to_owned())?;
            guard.clone()
        };
        let reply = provider.reply(&provider_prompt).await.map_err(|err| err.to_string())?;

        ctx.task.status = TaskStatus::Completed;
        ctx.task.result_summary = Some(reply.clone());

        let completed = nexus_audit::provider_completed(ctx.task.id, provider.name());
        let mut audits = vec![completed.clone()];

        {
            let store = self.lock_store()?;
            store
                .update_task_result(ctx.task.id, &reply, TaskStatus::Completed)
                .map_err(|err| err.to_string())?;
            store
                .mark_task_steps_completed(ctx.task.id)
                .map_err(|err| err.to_string())?;
            store.insert_audit(&completed).map_err(|err| err.to_string())?;
        }

        if self.is_memory_enabled()? {
            let memory_card =
                MemoryService::from_turn(ctx.task.id, &ctx.prompt, &reply, &ctx.decision);
            let memory_audit = nexus_audit::memory_saved(ctx.task.id, &memory_card.card_type);
            {
                let store = self.lock_store()?;
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
            let store = self.lock_store()?;
            store.list_task_steps(task_id).map_err(|err| err.to_string())?
        };

        Ok(ExecutionResult {
            task: ctx.task,
            reply,
            approval: ctx.approval,
            plan,
            audits,
        })
    }

    async fn run_browser_turn(&self, mut ctx: ExecutionContext) -> Result<ExecutionResult, String> {
        let runtime = build_browser_runtime(&BrowserRuntimeConfig::from_env());
        let browser_task = parse_browser_task(&ctx.prompt, ctx.task.risk_level.clone());
        ctx.task.status = TaskStatus::Executing;
        {
            let store = self.lock_store()?;
            store
                .update_task_status(ctx.task.id, TaskStatus::Executing)
                .map_err(|err| err.to_string())?;
            store
                .mark_task_steps_started(ctx.task.id)
                .map_err(|err| err.to_string())?;
        }

        let browser_output = runtime
            .execute(&browser_task)
            .map_err(|err| err.to_string())?;
        let extraction_audit = build_browser_extraction_audit(ctx.task.id, &browser_output);
        let summary = browser_output.summary.clone();
        ctx.task.status = TaskStatus::Completed;
        ctx.task.result_summary = Some(summary.clone());

        let prepared = nexus_audit::browser_executor_prepared(
            ctx.task.id,
            &format!(
                "browser executor used runtime {} in {} mode for {}",
                runtime.descriptor().id,
                browser_output.mode,
                browser_output.intent
            ),
        );
        let mut audits = vec![prepared.clone()];

        {
            let store = self.lock_store()?;
            store
                .update_task_result(ctx.task.id, &summary, TaskStatus::Completed)
                .map_err(|err| err.to_string())?;
            store
                .mark_task_steps_completed(ctx.task.id)
                .map_err(|err| err.to_string())?;
            store.insert_audit(&prepared).map_err(|err| err.to_string())?;
            if let Some(audit) = &extraction_audit {
                store.insert_audit(audit).map_err(|err| err.to_string())?;
            }
        }
        if let Some(audit) = extraction_audit {
            audits.push(audit);
        }

        if self.is_memory_enabled()? {
            let memory_card =
                MemoryService::from_turn(ctx.task.id, &ctx.prompt, &summary, &ctx.decision);
            let memory_audit = nexus_audit::memory_saved(ctx.task.id, &memory_card.card_type);
            {
                let store = self.lock_store()?;
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
            let store = self.lock_store()?;
            store.list_task_steps(task_id).map_err(|err| err.to_string())?
        };

        Ok(ExecutionResult {
            task: ctx.task,
            reply: summary,
            approval: ctx.approval,
            plan,
            audits,
        })
    }

    async fn run_dev_turn(&self, mut ctx: ExecutionContext) -> Result<ExecutionResult, String> {
        let runtime = build_dev_runtime();
        let dev_task = parse_dev_task(&ctx.prompt);
        ctx.task.status = TaskStatus::Executing;
        {
            let store = self.lock_store()?;
            store
                .update_task_status(ctx.task.id, TaskStatus::Executing)
                .map_err(|err| err.to_string())?;
            store
                .mark_task_steps_started(ctx.task.id)
                .map_err(|err| err.to_string())?;
        }

        let dev_output = runtime.execute(&dev_task).map_err(|err| err.to_string())?;

        // --- NEW: Real Dual-Brain Native Patching Logic ---
        let mut patch_runner_log = dev_output.patch_runner_log.clone();
        if !dev_task.file_targets.is_empty() {
            let mut prompt_context = String::new();
            prompt_context.push_str("You are Nexus Main Brain. Your task is to modify the following files according to the user's request.\n");
            prompt_context.push_str("Output ONLY valid SEARCH/REPLACE blocks in this exact format:\n");
            prompt_context.push_str("<<<<\npath/to/file\n====\nEXACT lines to search for\n====\nReplacement lines\n>>>>\n\n");
            prompt_context.push_str(&format!("User request: {}\n\n", ctx.prompt));

            for target in &dev_task.file_targets {
                if let Ok(content) = std::fs::read_to_string(target) {
                    prompt_context.push_str(&format!("File: {}\n```\n{}\n```\n\n", target, content));
                }
            }

            // Execute via Main Brain
            let provider = {
                let guard = self.provider.lock().map_err(|_| "provider lock poisoned".to_owned())?;
                guard.clone()
            };
            if let Ok(reply) = provider.reply(&prompt_context).await {
                patch_runner_log.push("Received code modifications from Main Brain.".to_owned());
                // Simple SEARCH/REPLACE block parser
                let mut current_file = String::new();
                let mut search_block = String::new();
                let mut replace_block = String::new();
                let mut state = 0; // 0: outside, 1: file, 2: search, 3: replace

                for line in reply.lines() {
                    if line == "<<<<" { state = 1; current_file.clear(); search_block.clear(); replace_block.clear(); continue; }
                    if line == "====" { state += 1; continue; }
                    if line == ">>>>" {
                        state = 0;
                        if !current_file.is_empty() && !search_block.is_empty() {
                            if let Ok(content) = std::fs::read_to_string(&current_file) {
                                let new_content = content.replace(&search_block, &replace_block);
                                if new_content != content {
                                    if std::fs::write(&current_file, new_content).is_ok() {
                                        patch_runner_log.push(format!("Successfully patched {}", current_file));
                                    } else {
                                        patch_runner_log.push(format!("Failed to write {}", current_file));
                                    }
                                } else {
                                    patch_runner_log.push(format!("Search block not found in {}", current_file));
                                }
                            }
                        }
                        continue;
                    }

                    match state {
                        1 => current_file = line.trim().to_owned(),
                        2 => { search_block.push_str(line); search_block.push('\n'); },
                        3 => { replace_block.push_str(line); replace_block.push('\n'); },
                        _ => {}
                    }
                }
            } else {
                patch_runner_log.push("Main Brain failed to generate a response.".to_owned());
            }
        }
        // ---------------------------------------------------
        let plan_audit = nexus_audit::dev_plan_saved(
            ctx.task.id,
            &format!(
                "patch_schema={}; intent={}; execution_mode={}; repo_scope={}; patch_strategy={}; patch_first={}; file_targets={}; module_targets={}; steps={}; operation_steps={}; change_plan={}; patch_outline={}; patch_proposal={}; patch_files={}; patch_apply_plan={}; patch_execution_contract={}; patch_execution_request={}; patch_items={}; patch_hunks={}; patch_sets={}; patch_contract={}; patch_targets={}; artifacts={}",
                dev_output.patch_schema_version,
                dev_output.intent,
                dev_output.execution_mode,
                dev_output.repo_scope,
                dev_output.patch_strategy,
                dev_output.patch_first,
                dev_output.file_targets.join(" | "),
                dev_output.module_targets.join(" | "),
                dev_output.recommended_steps.join(" | "),
                dev_output.operation_steps.join(" | "),
                dev_output.change_plan.join(" | "),
                dev_output.patch_outline.join(" | "),
                dev_output.patch_proposal.join(" | "),
                dev_output.patch_files.join(" | "),
                dev_output.patch_apply_plan.join(" | "),
                dev_output.patch_execution_contract.join(" | "),
                dev_output.patch_execution_request.join(" | "),
                dev_output.patch_items.join(" | "),
                dev_output.patch_hunks.join(" | "),
                dev_output.patch_sets.join(" | "),
                dev_output.patch_contract.join(" | "),
                dev_output.patch_targets.join(" | "),
                dev_output.artifacts.join(" | ")
            ),
        );
        let verification_audit = nexus_audit::dev_verification_saved(
            ctx.task.id,
            &format!(
                "verification_plan={}; verification_targets={}",
                dev_output.verification_plan.join(" | "),
                dev_output.verification_targets.join(" | ")
            ),
        );
        let patch_schema_audit =
            nexus_audit::dev_patch_schema_saved(ctx.task.id, &dev_output.patch_schema_json);
        let patch_runner_audit = nexus_audit::dev_runner_saved(
            ctx.task.id,
            &format!(
                "runner_id={}; mode={}; log={}",
                dev_output.patch_runner_id,
                dev_output.patch_runner_mode,
                patch_runner_log.join(" | ")
            ),
        );
        let patch_runner_log_audit =
            nexus_audit::dev_runner_log_saved(ctx.task.id, &serde_json::to_string(&patch_runner_log).unwrap_or_default());
        let prepared = nexus_audit::dev_executor_prepared(
            ctx.task.id,
            &format!(
                "dev executor used runtime {} for {}",
                runtime.descriptor().id,
                dev_output.intent
            ),
        );
        let summary = dev_output.summary.clone();
        ctx.task.status = TaskStatus::Completed;
        ctx.task.result_summary = Some(summary.clone());

        let mut audits = vec![
            prepared.clone(),
            plan_audit.clone(),
            verification_audit.clone(),
            patch_schema_audit.clone(),
            patch_runner_audit.clone(),
            patch_runner_log_audit.clone(),
        ];
        {
            let store = self.lock_store()?;
            store
                .update_task_result(ctx.task.id, &summary, TaskStatus::Completed)
                .map_err(|err| err.to_string())?;
            store
                .mark_task_steps_completed(ctx.task.id)
                .map_err(|err| err.to_string())?;
            store.insert_audit(&prepared).map_err(|err| err.to_string())?;
            store.insert_audit(&plan_audit).map_err(|err| err.to_string())?;
            store
                .insert_audit(&verification_audit)
                .map_err(|err| err.to_string())?;
            store
                .insert_audit(&patch_schema_audit)
                .map_err(|err| err.to_string())?;
            store
                .insert_audit(&patch_runner_audit)
                .map_err(|err| err.to_string())?;
            store
                .insert_audit(&patch_runner_log_audit)
                .map_err(|err| err.to_string())?;
        }

        if self.is_memory_enabled()? {
            let memory_card =
                MemoryService::from_turn(ctx.task.id, &ctx.prompt, &summary, &ctx.decision);
            let memory_audit = nexus_audit::memory_saved(ctx.task.id, &memory_card.card_type);
            {
                let store = self.lock_store()?;
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
            let store = self.lock_store()?;
            store.list_task_steps(task_id).map_err(|err| err.to_string())?
        };

        Ok(ExecutionResult {
            task: ctx.task,
            reply: summary,
            approval: ctx.approval,
            plan,
            audits,
        })
    }

    fn build_provider_prompt(&self, task_id: Uuid, prompt: &str) -> Result<String, String> {
        if !self.is_memory_enabled()? {
            return Ok(prompt.to_owned());
        }

        let cards = {
            let store = self.lock_store()?;
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
            let store = self.lock_store()?;
            let audit = nexus_audit::memory_context_loaded(task_id, cards.len());
            store.insert_audit(&audit).map_err(|err| err.to_string())?;
        }

        Ok(format!(
            "Relevant memory context:\n{}\n\nCurrent user request:\n{}",
            memory_context, prompt
        ))
    }

    fn persist_new_task(
        &self,
        task: &TaskRecord,
        decision: &BrainDecision,
        plan: &[TaskStepRecord],
    ) -> Result<(), String> {
        let received = nexus_audit::task_received(task.id, "desktop prompt accepted");
        let routed = nexus_audit::brain_routed(task.id, decision.route.as_str(), &decision.reason);
        let store = self.lock_store()?;
        store.insert_task(task).map_err(|err| err.to_string())?;
        store.insert_task_steps(plan).map_err(|err| err.to_string())?;
        store.insert_audit(&received).map_err(|err| err.to_string())?;
        store.insert_audit(&routed).map_err(|err| err.to_string())?;
        Ok(())
    }

    fn queue_approval(
        &self,
        mut task: TaskRecord,
        locale: &str,
        plan: Vec<TaskStepRecord>,
    ) -> Result<ChatResponse, String> {
        let approval = TaskService::create_approval(&task);
        let approval_audit =
            nexus_audit::approval_requested(task.id, "Risk level is L4/L5, waiting for approval");

        {
            let store = self.lock_store()?;
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
            reply: localized_text(locale, "queued_for_approval").to_owned(),
            approval: Some(approval),
            plan,
            audits: vec![approval_audit],
        })
    }

    fn lock_store(&self) -> Result<MutexGuard<'_, NexusStore>, String> {
        self.store
            .lock()
            .map_err(|_| "store lock poisoned".to_owned())
    }

    fn update_last_brain_route(&self, route: &str) -> Result<(), String> {
        let mut guard = self
            .last_brain_route
            .lock()
            .map_err(|_| "brain route lock poisoned".to_owned())?;
        *guard = route.to_owned();
        Ok(())
    }

    fn is_brain_enabled(&self) -> Result<bool, String> {
        let guard = self
            .brain_enabled
            .lock()
            .map_err(|_| "brain module lock poisoned".to_owned())?;
        Ok(*guard)
    }

    fn is_memory_enabled(&self) -> Result<bool, String> {
        let guard = self
            .memory_enabled
            .lock()
            .map_err(|_| "memory module lock poisoned".to_owned())?;
        Ok(*guard)
    }
}

fn clip_text(raw: &str, cap: usize) -> String {
    let trimmed = raw.trim();
    let mut text = trimmed.chars().take(cap).collect::<String>();
    if trimmed.chars().count() > cap {
        text.push_str("...");
    }
    text
}

fn is_browser_prompt(prompt: &str) -> bool {
    let lower = prompt.to_lowercase();
    if ["浏览器", "网页", "网站", "页面", "登录", "表单"]
        .iter()
        .any(|token| prompt.contains(token))
    {
        return true;
    }
    [
        "browser",
        "web",
        "website",
        "site",
        "page",
        "url",
        "http://",
        "https://",
        "login",
        "form",
        "浏览器",
        "网页",
        "网站",
        "页面",
        "登录",
        "表单",
    ]
    .iter()
    .any(|token| lower.contains(token))
}

fn is_dev_prompt(prompt: &str) -> bool {
    let lower = prompt.to_lowercase();
    if [
        "代码",
        "开发",
        "修复",
        "重构",
        "实现",
        "看代码",
        "补丁",
        "测试",
        "验证",
    ]
    .iter()
    .any(|token| prompt.contains(token))
    {
        return true;
    }

    [
        "code",
        "repo",
        "patch",
        "diff",
        "fix",
        "implement",
        "refactor",
        "test",
        "build",
        "verify",
    ]
    .iter()
    .any(|token| lower.contains(token))
}

fn build_browser_extraction_audit(
    task_id: Uuid,
    output: &nexus_browser::BrowserExecutionOutput,
) -> Option<AuditRecord> {
    if output.text_snippet.is_none()
        && output.link_sample.is_empty()
        && output.recommended_next_actions.is_empty()
    {
        return None;
    }

    let snippet = output
        .text_snippet
        .as_deref()
        .map(|text| clip_text(text, 180))
        .unwrap_or_else(|| "no text snippet".to_owned());
    let links = if output.link_sample.is_empty() {
        "no link sample".to_owned()
    } else {
        output
            .link_sample
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join(" | ")
    };
    let forms = output
        .form_count
        .map(|count| count.to_string())
        .unwrap_or_else(|| "unknown".to_owned());
    let inputs = if output.input_sample.is_empty() {
        "no input sample".to_owned()
    } else {
        output
            .input_sample
            .iter()
            .take(4)
            .cloned()
            .collect::<Vec<_>>()
            .join(" | ")
    };
    let fields = if output.field_plan.is_empty() {
        "no field plan".to_owned()
    } else {
        output
            .field_plan
            .iter()
            .take(4)
            .cloned()
            .collect::<Vec<_>>()
            .join(" | ")
    };
    let missing = if output.missing_fields.is_empty() {
        "no missing fields".to_owned()
    } else {
        output
            .missing_fields
            .iter()
            .take(4)
            .cloned()
            .collect::<Vec<_>>()
            .join(" | ")
    };
    let sensitive = if output.sensitive_fields.is_empty() {
        "no sensitive fields".to_owned()
    } else {
        output
            .sensitive_fields
            .iter()
            .take(4)
            .cloned()
            .collect::<Vec<_>>()
            .join(" | ")
    };
    let next_actions = if output.recommended_next_actions.is_empty() {
        "no recommended next actions".to_owned()
    } else {
        output
            .recommended_next_actions
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join(" | ")
    };

    Some(nexus_audit::browser_extraction_saved(
        task_id,
        &format!(
            "phase={}; boundary={}; snippet={snippet}; links={links}; forms={forms}; inputs={inputs}; fields={fields}; missing={missing}; sensitive={sensitive}; next={next_actions}",
            output.action_phase,
            output.boundary
        ),
    ))
}


pub fn resolve_risk_policy_path(explicit: Option<String>) -> Option<PathBuf> {
    if let Some(path) = explicit {
        return Some(PathBuf::from(path));
    }
    if let Ok(env_path) = env::var("NEXUS_RISK_POLICY_FILE") {
        return Some(PathBuf::from(env_path));
    }
    Some(Path::new("infra").join("configs").join("risk-policy.json"))
}

pub fn normalize_locale(raw: Option<&str>) -> &str {
    match raw {
        Some(value) if value.to_lowercase().starts_with("zh") => "zh-CN",
        _ => "en-US",
    }
}

fn localized_text(locale: &str, key: &str) -> &'static str {
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

#[allow(dead_code, unreachable_patterns)]
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
