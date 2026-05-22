use std::{fs, path::PathBuf, sync::Arc};

use nexus_exec::{
    resolve_risk_policy_path, AppRuntime, AutomationView, ChatResponse, DevModeDescriptor,
    ExecutionSnapshot, ExecutorDescriptor, ModuleDescriptor, ModuleStatus, PatchRunnerDescriptor,
    TaskWorkspace,
};
use nexus_browser::BrowserRuntimeDescriptor;
use nexus_memory::MemoryCard;
use nexus_protocol::{ApprovalRecord, AuditRecord, TaskRecord};
use nexus_provider::ProviderDescriptor;
use tauri::{Manager, State};

#[tauri::command]
fn submit_chat(
    message: String,
    locale: Option<String>,
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<ChatResponse, String> {
    runtime.submit_chat(message, locale)
}

#[tauri::command]
fn list_pending_approvals(runtime: State<'_, Arc<AppRuntime>>) -> Result<Vec<ApprovalRecord>, String> {
    runtime.list_pending_approvals()
}

#[tauri::command]
fn list_recent_tasks(
    runtime: State<'_, Arc<AppRuntime>>,
    limit: Option<usize>,
) -> Result<Vec<TaskRecord>, String> {
    runtime.list_recent_tasks(limit)
}

#[tauri::command]
fn list_recent_approvals(
    runtime: State<'_, Arc<AppRuntime>>,
    limit: Option<usize>,
) -> Result<Vec<ApprovalRecord>, String> {
    runtime.list_recent_approvals(limit)
}

#[tauri::command]
fn list_recent_memory_cards(
    runtime: State<'_, Arc<AppRuntime>>,
    limit: Option<usize>,
) -> Result<Vec<MemoryCard>, String> {
    runtime.list_recent_memory_cards(limit)
}

#[tauri::command]
fn list_recent_audits(
    runtime: State<'_, Arc<AppRuntime>>,
    limit: Option<usize>,
) -> Result<Vec<AuditRecord>, String> {
    runtime.list_recent_audits(limit)
}

#[tauri::command]
fn get_latest_workspace(runtime: State<'_, Arc<AppRuntime>>) -> Result<Option<TaskWorkspace>, String> {
    runtime.get_latest_workspace()
}

#[tauri::command]
fn get_latest_execution_snapshot(
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<Option<ExecutionSnapshot>, String> {
    runtime.get_latest_execution_snapshot()
}

#[tauri::command]
fn list_automations(runtime: State<'_, Arc<AppRuntime>>) -> Result<Vec<AutomationView>, String> {
    runtime.list_automations()
}

#[tauri::command]
fn create_automation(
    title: String,
    description: String,
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<AutomationView, String> {
    runtime.create_automation(title, description)
}

#[tauri::command]
fn set_automation_enabled(
    automation_id: String,
    enabled: bool,
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<Vec<AutomationView>, String> {
    let automation_id = automation_id
        .parse()
        .map_err(|err| format!("invalid automation id: {err}"))?;
    runtime.set_automation_enabled(automation_id, enabled)
}

#[tauri::command]
fn delete_automation(
    automation_id: String,
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<Vec<AutomationView>, String> {
    let automation_id = automation_id
        .parse()
        .map_err(|err| format!("invalid automation id: {err}"))?;
    runtime.delete_automation(automation_id)
}

#[tauri::command]
fn resolve_approval(
    approval_id: String,
    approved: bool,
    locale: Option<String>,
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<ChatResponse, String> {
    runtime.resolve_approval(approval_id, approved, locale)
}

#[tauri::command]
fn reload_risk_policy(
    path: Option<String>,
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<String, String> {
    runtime.reload_risk_policy(path)
}

#[tauri::command]
fn get_risk_policy_source(runtime: State<'_, Arc<AppRuntime>>) -> Result<String, String> {
    runtime.get_risk_policy_source()
}

#[tauri::command]
fn reload_provider(
    mode: Option<String>,
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<String, String> {
    runtime.reload_provider(mode)
}

#[tauri::command]
fn get_provider_source(runtime: State<'_, Arc<AppRuntime>>) -> Result<String, String> {
    runtime.get_provider_source()
}

#[tauri::command]
fn set_module_enabled(
    module: String,
    enabled: bool,
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<ModuleStatus, String> {
    runtime.set_module_enabled(module, enabled)
}

#[tauri::command]
fn list_modules(runtime: State<'_, Arc<AppRuntime>>) -> Result<Vec<ModuleDescriptor>, String> {
    runtime.list_modules()
}

#[tauri::command]
fn list_executors(runtime: State<'_, Arc<AppRuntime>>) -> Vec<ExecutorDescriptor> {
    runtime.list_executors()
}

#[tauri::command]
fn list_providers(runtime: State<'_, Arc<AppRuntime>>) -> Result<Vec<ProviderDescriptor>, String> {
    runtime.list_providers()
}

#[tauri::command]
fn list_browser_runtimes(runtime: State<'_, Arc<AppRuntime>>) -> Vec<BrowserRuntimeDescriptor> {
    runtime.list_browser_runtimes()
}

#[tauri::command]
fn list_patch_runners(runtime: State<'_, Arc<AppRuntime>>) -> Vec<PatchRunnerDescriptor> {
    runtime.list_patch_runners()
}

#[tauri::command]
fn list_dev_modes(runtime: State<'_, Arc<AppRuntime>>) -> Vec<DevModeDescriptor> {
    runtime.list_dev_modes()
}

#[tauri::command]
fn get_module_status(runtime: State<'_, Arc<AppRuntime>>) -> Result<ModuleStatus, String> {
    runtime.get_module_status()
}

#[tauri::command]
fn list_skills(runtime: State<'_, Arc<AppRuntime>>) -> Result<Vec<nexus_skill::Skill>, String> {
    runtime.list_skills()
}

#[tauri::command]
fn list_mcp_servers(runtime: State<'_, Arc<AppRuntime>>) -> Result<Vec<nexus_mcp::McpServerDescriptor>, String> {
    runtime.list_mcp_servers()
}

#[tauri::command]
fn list_connectors() -> Result<Vec<nexus_connector::ConnectorStatus>, String> {
    Ok(nexus_connector::list_connectors())
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let store_path = resolve_store_path(app.handle())?;
            if let Some(parent) = store_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let runtime = Arc::new(AppRuntime::boot(store_path, resolve_risk_policy_path(None))
                .map_err(|e| e.to_string())?);

            // 启动连接器后台服务
            let connector_runtime = Arc::clone(&runtime);
            tauri::async_runtime::spawn(async move {
                let config = nexus_connector::ConnectorConfig::default();
                if let Err(e) = nexus_connector::start_connector_server(connector_runtime, config).await {
                    eprintln!("Failed to start connector server: {}", e);
                }
            });

            app.manage(runtime);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            submit_chat,
            list_pending_approvals,
            list_recent_tasks,
            list_recent_approvals,
            list_recent_memory_cards,
            list_recent_audits,
            get_latest_workspace,
            get_latest_execution_snapshot,
            list_automations,
            create_automation,
            set_automation_enabled,
            delete_automation,
            resolve_approval,
            reload_risk_policy,
            get_risk_policy_source,
            reload_provider,
            get_provider_source,
            set_module_enabled,
            list_modules,
            list_executors,
            list_providers,
            list_browser_runtimes,
            list_patch_runners,
            list_dev_modes,
            get_module_status,
            list_skills,
            list_mcp_servers,
            list_connectors
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Nexus desktop");
}

fn resolve_store_path(app: &tauri::AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let base = app.path().app_data_dir()?;
    let profile = std::env::var("NEXUS_PROFILE").unwrap_or_else(|_| "default".to_owned());
    Ok(base.join(profile).join("nexus.db"))
}
