use std::{fs, path::PathBuf};
use tauri::{Manager, State};
use std::sync::Arc;

pub use nexus_exec::{
    AppRuntime, ExecutorDescriptor, ModuleDescriptor, ModuleStatus, PatchRunnerDescriptor,
    TaskWorkspace,
    resolve_risk_policy_path,
};
pub use nexus_skill;
pub use nexus_connector;

use nexus_browser::BrowserRuntimeDescriptor;
use nexus_memory::MemoryCard;
use nexus_protocol::{ApprovalRecord, AuditRecord, ChatResponse, TaskRecord};
use nexus_provider::ProviderDescriptor;

#[tauri::command]
async fn submit_chat(
    message: String,
    locale: Option<String>,
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<ChatResponse, String> {
    runtime.submit_chat(message, locale).await
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
async fn resolve_approval(
    approval_id: String,
    approved: bool,
    locale: Option<String>,
    runtime: State<'_, Arc<AppRuntime>>,
) -> Result<ChatResponse, String> {
    runtime.resolve_approval(approval_id, approved, locale).await
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
fn list_skills(runtime: State<'_, Arc<AppRuntime>>) -> Result<Vec<nexus_skill::Skill>, String> {
    runtime.list_skills()
}

#[tauri::command]
fn list_mcp_servers(runtime: State<'_, Arc<AppRuntime>>) -> Result<Vec<nexus_mcp::McpServerDescriptor>, String> {
    runtime.list_mcp_servers()
}

#[tauri::command]
fn get_module_status(runtime: State<'_, Arc<AppRuntime>>) -> Result<ModuleStatus, String> {
    runtime.get_module_status()
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let store_path = resolve_store_path(app.handle())?;
            let runtime = Arc::new(AppRuntime::boot(store_path, nexus_exec::resolve_risk_policy_path(None))
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
            list_skills,
            list_mcp_servers,
            get_module_status
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Nexus desktop");
}

fn resolve_store_path(app: &tauri::AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let base = app.path().app_data_dir()?;
    let profile = std::env::var("NEXUS_PROFILE").unwrap_or_else(|_| "default".to_owned());
    Ok(base.join(profile).join("nexus.db"))
}
