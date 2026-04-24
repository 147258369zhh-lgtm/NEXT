use std::{fs, path::PathBuf};

use nexus_exec::{
    AppRuntime, ExecutorDescriptor, ModuleDescriptor, ModuleStatus, PatchRunnerDescriptor,
    TaskWorkspace,
    resolve_risk_policy_path,
};
use nexus_browser::BrowserRuntimeDescriptor;
use nexus_memory::MemoryCard;
use nexus_protocol::{ApprovalRecord, AuditRecord, ChatResponse, TaskRecord};
use nexus_provider::ProviderDescriptor;
use tauri::{Manager, State};

#[tauri::command]
fn submit_chat(
    message: String,
    locale: Option<String>,
    runtime: State<'_, AppRuntime>,
) -> Result<ChatResponse, String> {
    runtime.submit_chat(message, locale)
}

#[tauri::command]
fn list_pending_approvals(runtime: State<'_, AppRuntime>) -> Result<Vec<ApprovalRecord>, String> {
    runtime.list_pending_approvals()
}

#[tauri::command]
fn list_recent_tasks(
    runtime: State<'_, AppRuntime>,
    limit: Option<usize>,
) -> Result<Vec<TaskRecord>, String> {
    runtime.list_recent_tasks(limit)
}

#[tauri::command]
fn list_recent_approvals(
    runtime: State<'_, AppRuntime>,
    limit: Option<usize>,
) -> Result<Vec<ApprovalRecord>, String> {
    runtime.list_recent_approvals(limit)
}

#[tauri::command]
fn list_recent_memory_cards(
    runtime: State<'_, AppRuntime>,
    limit: Option<usize>,
) -> Result<Vec<MemoryCard>, String> {
    runtime.list_recent_memory_cards(limit)
}

#[tauri::command]
fn list_recent_audits(
    runtime: State<'_, AppRuntime>,
    limit: Option<usize>,
) -> Result<Vec<AuditRecord>, String> {
    runtime.list_recent_audits(limit)
}

#[tauri::command]
fn get_latest_workspace(runtime: State<'_, AppRuntime>) -> Result<Option<TaskWorkspace>, String> {
    runtime.get_latest_workspace()
}

#[tauri::command]
fn resolve_approval(
    approval_id: String,
    approved: bool,
    locale: Option<String>,
    runtime: State<'_, AppRuntime>,
) -> Result<ChatResponse, String> {
    runtime.resolve_approval(approval_id, approved, locale)
}

#[tauri::command]
fn reload_risk_policy(
    path: Option<String>,
    runtime: State<'_, AppRuntime>,
) -> Result<String, String> {
    runtime.reload_risk_policy(path)
}

#[tauri::command]
fn get_risk_policy_source(runtime: State<'_, AppRuntime>) -> Result<String, String> {
    runtime.get_risk_policy_source()
}

#[tauri::command]
fn reload_provider(
    mode: Option<String>,
    runtime: State<'_, AppRuntime>,
) -> Result<String, String> {
    runtime.reload_provider(mode)
}

#[tauri::command]
fn get_provider_source(runtime: State<'_, AppRuntime>) -> Result<String, String> {
    runtime.get_provider_source()
}

#[tauri::command]
fn set_module_enabled(
    module: String,
    enabled: bool,
    runtime: State<'_, AppRuntime>,
) -> Result<ModuleStatus, String> {
    runtime.set_module_enabled(module, enabled)
}

#[tauri::command]
fn list_modules(runtime: State<'_, AppRuntime>) -> Result<Vec<ModuleDescriptor>, String> {
    runtime.list_modules()
}

#[tauri::command]
fn list_executors(runtime: State<'_, AppRuntime>) -> Vec<ExecutorDescriptor> {
    runtime.list_executors()
}

#[tauri::command]
fn list_providers(runtime: State<'_, AppRuntime>) -> Result<Vec<ProviderDescriptor>, String> {
    runtime.list_providers()
}

#[tauri::command]
fn list_browser_runtimes(runtime: State<'_, AppRuntime>) -> Vec<BrowserRuntimeDescriptor> {
    runtime.list_browser_runtimes()
}

#[tauri::command]
fn list_patch_runners(runtime: State<'_, AppRuntime>) -> Vec<PatchRunnerDescriptor> {
    runtime.list_patch_runners()
}

#[tauri::command]
fn get_module_status(runtime: State<'_, AppRuntime>) -> Result<ModuleStatus, String> {
    runtime.get_module_status()
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let store_path = resolve_store_path(app.handle())?;
            if let Some(parent) = store_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let runtime = AppRuntime::boot(store_path, resolve_risk_policy_path(None))?;
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
