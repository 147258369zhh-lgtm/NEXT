use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct DevRuntimeDescriptor {
    pub id: String,
    pub title: String,
    pub engine: String,
    pub patch_first: bool,
    pub supports_repo_inspection: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub enum DevIntent {
    Analyze,
    Patch,
    Verify,
    Refactor,
    Unknown,
}

impl DevIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Analyze => "analyze",
            Self::Patch => "patch",
            Self::Verify => "verify",
            Self::Refactor => "refactor",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum DevExecutionMode {
    ReadOnly,
    PatchReady,
    VerifyOnly,
    RefactorIncremental,
}

impl DevExecutionMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PatchReady => "patch_ready",
            Self::VerifyOnly => "verify_only",
            Self::RefactorIncremental => "refactor_incremental",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum RepoScope {
    Focused,
    ModuleWide,
    WorkspaceWide,
}

impl RepoScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Focused => "focused",
            Self::ModuleWide => "module_wide",
            Self::WorkspaceWide => "workspace_wide",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum PatchStrategy {
    None,
    MinimalDiff,
    BoundaryExtraction,
    VerificationPass,
}

impl PatchStrategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::MinimalDiff => "minimal_diff",
            Self::BoundaryExtraction => "boundary_extraction",
            Self::VerificationPass => "verification_pass",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DevTaskSpec {
    pub prompt: String,
    pub intent: DevIntent,
    pub execution_mode: DevExecutionMode,
    pub repo_scope: RepoScope,
    pub patch_strategy: PatchStrategy,
    pub patch_first: bool,
    pub recommended_steps: Vec<String>,
    pub file_targets: Vec<String>,
    pub module_targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchItemRecord {
    pub target: String,
    pub action: String,
    pub strategy: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchHunkRecord {
    pub target: String,
    pub area: String,
    pub action: String,
    pub diff_shape: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchSetRecord {
    pub batch_id: String,
    pub target: String,
    pub composition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hunk {
    pub search: String,
    pub replace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFileRecord {
    pub path: String,
    pub hunks: Vec<Hunk>,
    pub role: String,
    pub mutation_boundary: String,
    pub verification_target: String,
    pub patch_set_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchApplyStepRecord {
    pub batch_id: String,
    pub stage: String,
    pub action: String,
    pub target: String,
    pub check: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchExecutionContractRecord {
    pub write_scope: String,
    pub dry_run_first: bool,
    pub approval_required: bool,
    pub rollback_scope: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchExecutionRequestRecord {
    pub mode: String,
    pub selected_batches: Vec<String>,
    pub target_paths: Vec<String>,
    pub verification_scope: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchContractRecord {
    pub execution_mode: String,
    pub repo_scope: String,
    pub patch_strategy: String,
    pub precondition: String,
    pub apply_boundary: String,
    pub verification_gate: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchPlanSchema {
    pub schema_version: String,
    pub intent: String,
    pub execution_mode: String,
    pub repo_scope: String,
    pub patch_strategy: String,
    pub file_targets: Vec<String>,
    pub module_targets: Vec<String>,
    pub patch_files: Vec<PatchFileRecord>,
    pub patch_apply_plan: Vec<PatchApplyStepRecord>,
    pub execution_contract: PatchExecutionContractRecord,
    pub execution_request: PatchExecutionRequestRecord,
    pub patch_items: Vec<PatchItemRecord>,
    pub patch_hunks: Vec<PatchHunkRecord>,
    pub patch_sets: Vec<PatchSetRecord>,
    pub contract: PatchContractRecord,
}

#[derive(Debug, Clone, Serialize)]
pub struct DevExecutionOutput {
    pub summary: String,
    pub transcript: Vec<String>,
    pub patch_schema_version: String,
    pub patch_schema_json: String,
    pub patch_runner_id: String,
    pub patch_runner_mode: String,
    pub patch_runner_log: Vec<String>,
    pub patch_runner_log_json: String,
    pub intent: String,
    pub execution_mode: String,
    pub repo_scope: String,
    pub patch_strategy: String,
    pub patch_first: bool,
    pub recommended_steps: Vec<String>,
    pub operation_steps: Vec<String>,
    pub change_plan: Vec<String>,
    pub patch_outline: Vec<String>,
    pub patch_proposal: Vec<String>,
    pub patch_files: Vec<String>,
    pub patch_apply_plan: Vec<String>,
    pub patch_execution_contract: Vec<String>,
    pub patch_execution_request: Vec<String>,
    pub patch_items: Vec<String>,
    pub patch_hunks: Vec<String>,
    pub patch_sets: Vec<String>,
    pub patch_contract: Vec<String>,
    pub verification_plan: Vec<String>,
    pub patch_targets: Vec<String>,
    pub verification_targets: Vec<String>,
    pub artifacts: Vec<String>,
    pub file_targets: Vec<String>,
    pub module_targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchRunnerDescriptor {
    pub id: String,
    pub title: String,
    pub mode: String,
    pub mutates_files: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchRunnerOutput {
    pub runner_id: String,
    pub mode: String,
    pub execution_log: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchRunnerAuditPayload {
    pub runner_id: String,
    pub mode: String,
    pub log_entries: Vec<String>,
}

pub trait DevRuntime: Send + Sync {
    fn descriptor(&self) -> DevRuntimeDescriptor;
    fn execute(&self, spec: &DevTaskSpec) -> Result<DevExecutionOutput>;
}

pub trait PatchRunner: Send + Sync {
    fn descriptor(&self) -> PatchRunnerDescriptor;
    fn run(&self, schema: &PatchPlanSchema) -> Result<PatchRunnerOutput>;
}

pub struct ScaffoldDevRuntime;
pub struct DryRunPatchRunner;
pub struct NativePatchRunner;

impl PatchRunner for DryRunPatchRunner {
    fn descriptor(&self) -> PatchRunnerDescriptor {
        PatchRunnerDescriptor {
            id: "patch-runner-dry-run".to_owned(),
            title: "Dry Run Patch Runner".to_owned(),
            mode: "dry_run".to_owned(),
            mutates_files: false,
            enabled: true,
        }
    }

    fn run(&self, schema: &PatchPlanSchema) -> Result<PatchRunnerOutput> {
        let descriptor = self.descriptor();
        let mut execution_log = vec![
            format!("runner={} mode={}", descriptor.id, descriptor.mode),
            format!(
                "selected_batches={}",
                schema.execution_request.selected_batches.join(", ")
            ),
            format!("target_paths={}", schema.execution_request.target_paths.join(", ")),
        ];

        execution_log.extend(schema.patch_apply_plan.iter().map(|step| {
            format!(
                "dry-run stage={} batch={} target={} action={}",
                step.stage, step.batch_id, step.target, step.action
            )
        }));

        Ok(PatchRunnerOutput {
            runner_id: descriptor.id,
            mode: descriptor.mode,
            execution_log,
        })
    }
}

pub fn build_patch_runner() -> Box<dyn PatchRunner> {
    Box::new(NativePatchRunner)
}

pub fn list_patch_runner_catalog() -> Vec<PatchRunnerDescriptor> {
    vec![DryRunPatchRunner.descriptor(), NativePatchRunner.descriptor()]
}

impl PatchRunner for NativePatchRunner {
    fn descriptor(&self) -> PatchRunnerDescriptor {
        PatchRunnerDescriptor {
            id: "patch-runner-native".to_owned(),
            title: "Native File Patch Runner".to_owned(),
            mode: "native_fs".to_owned(),
            mutates_files: true,
            enabled: true,
        }
    }

    fn run(&self, schema: &PatchPlanSchema) -> Result<PatchRunnerOutput> {
        let descriptor = self.descriptor();
        let mut execution_log = vec![
            format!("runner={} mode={}", descriptor.id, descriptor.mode),
        ];

        for file_patch in &schema.patch_files {
            let path = Path::new(&file_patch.path);
            if !path.exists() {
                execution_log.push(format!("Error: file not found: {}", file_patch.path));
                continue;
            }

            let content = std::fs::read_to_string(path)?;
            let mut new_content = content.clone();
            let mut success_count = 0;

            for hunk in &file_patch.hunks {
                match apply_hunk(&new_content, hunk) {
                    Ok(applied) => {
                        new_content = applied;
                        success_count += 1;
                    }
                    Err(e) => {
                        execution_log.push(format!("Failed to apply hunk in {}: {}", file_patch.path, e));
                    }
                }
            }

            if success_count > 0 {
                std::fs::write(path, new_content)?;
                execution_log.push(format!("Applied {} hunks to {}", success_count, file_patch.path));
            }
        }

        Ok(PatchRunnerOutput {
            runner_id: descriptor.id,
            mode: descriptor.mode,
            execution_log,
        })
    }
}

/// 核心算法：Aider 风格的缩进感知型 Search/Replace 匹配
fn apply_hunk(content: &str, hunk: &Hunk) -> Result<String, String> {
    let content_lines: Vec<&str> = content.lines().collect();
    let search_lines: Vec<&str> = hunk.search.lines().map(|s| s.trim_end()).collect();
    let replace_lines: Vec<&str> = hunk.replace.lines().map(|s| s.trim_end()).collect();

    if search_lines.is_empty() {
        return Err("Empty search block".to_string());
    }

    let mut matches = Vec::new();

    // 在内容中滑动寻找搜索块
    for i in 0..=content_lines.len().saturating_sub(search_lines.len()) {
        let mut found = true;
        let mut detected_indent: Option<&str> = None;

        for j in 0..search_lines.len() {
            let content_line = content_lines[i + j].trim_end();
            let search_line = search_lines[j];

            // 尝试检测缩进差异（Aider 会尝试将搜索块的缩进与内容对齐）
            if content_line.ends_with(search_line) {
                let indent_len = content_line.len() - search_line.len();
                let current_indent = &content_lines[i + j][..indent_len];
                
                if let Some(prev_indent) = detected_indent {
                    if prev_indent != current_indent {
                        found = false;
                        break;
                    }
                } else {
                    detected_indent = Some(current_indent);
                }
            } else {
                found = false;
                break;
            }
        }

        if found {
            matches.push((i, detected_indent.unwrap_or("")));
        }
    }

    if matches.is_empty() {
        return Err("Search block not found (check indentation and exact content)".to_string());
    }

    if matches.len() > 1 {
        return Err(format!("Ambiguous match: found {} occurrences of the search block", matches.len()));
    }

    let (start_index, indent) = matches[0];
    let mut final_lines = Vec::new();

    // 复制搜索块之前的内容
    for i in 0..start_index {
        final_lines.push(content_lines[i].to_string());
    }

    // 插入替换内容，并应用检测到的缩进
    for line in replace_lines {
        final_lines.push(format!("{}{}", indent, line));
    }

    // 复制搜索块之后的内容
    for i in (start_index + search_lines.len())..content_lines.len() {
        final_lines.push(content_lines[i].to_string());
    }

    Ok(final_lines.join("\n"))
}

impl DevRuntime for ScaffoldDevRuntime {
    fn descriptor(&self) -> DevRuntimeDescriptor {
        DevRuntimeDescriptor {
            id: "dev-runtime-scaffold".to_owned(),
            title: "Scaffold Dev Runtime".to_owned(),
            engine: "patch-first-placeholder".to_owned(),
            patch_first: true,
            supports_repo_inspection: true,
            enabled: true,
        }
    }

    fn execute(&self, spec: &DevTaskSpec) -> Result<DevExecutionOutput> {
        let operation_steps = suggest_operation_steps(spec);
        let change_plan = suggest_change_plan(spec);
        let patch_outline = suggest_patch_outline(spec);
        let patch_proposal = suggest_patch_proposal(spec);
        let patch_files = suggest_patch_files(spec);
        let patch_apply_plan = suggest_patch_apply_plan(spec);
        let patch_execution_contract = suggest_patch_execution_contract(spec);
        let patch_execution_request = suggest_patch_execution_request(spec);
        let patch_items = suggest_patch_items(spec);
        let patch_hunks = suggest_patch_hunks(spec);
        let patch_sets = suggest_patch_sets(spec);
        let patch_contract = suggest_patch_contract(spec);
        let patch_schema = build_patch_schema(spec);
        let patch_runner = build_patch_runner();
        let patch_runner_output = patch_runner.run(&patch_schema)?;
        let patch_schema_json = serde_json::to_string_pretty(&patch_schema)?;
        let patch_runner_log_json = serde_json::to_string_pretty(&PatchRunnerAuditPayload {
            runner_id: patch_runner_output.runner_id.clone(),
            mode: patch_runner_output.mode.clone(),
            log_entries: patch_runner_output.execution_log.clone(),
        })?;
        let verification_plan = suggest_verification_plan(spec);
        let patch_targets = suggest_patch_targets(spec);
        let verification_targets = suggest_verification_targets(spec);
        let artifacts = suggest_artifacts(spec);

        Ok(DevExecutionOutput {
            summary: format!(
                "Dev runtime scaffold accepted this task.\n\nPatch schema: {}\nIntent: {}\nExecution mode: {}\nRepo scope: {}\nPatch strategy: {}\nPatch-first: {}\nPatch runner: {} ({})\n\nTarget request:\n{}\n\nFile targets:\n- {}\n\nModule targets:\n- {}\n\nRecommended steps:\n- {}\n\nOperation steps:\n- {}\n\nChange plan:\n- {}\n\nPatch outline:\n- {}\n\nPatch proposal:\n- {}\n\nPatch files:\n- {}\n\nApply plan:\n- {}\n\nExecution contract:\n- {}\n\nExecution request:\n- {}\n\nRunner log:\n- {}\n\nPatch items:\n- {}\n\nPatch hunks:\n- {}\n\nPatch sets:\n- {}\n\nPatch contract:\n- {}\n\nPatch schema export: stored separately in dev.patch_schema audit ({}, {} files / {} apply steps / {} items / {} hunks / {} sets).\n\nPatch targets:\n- {}\n\nVerification plan:\n- {}\n\nVerification targets:\n- {}\n\nArtifacts:\n- {}\n\nCurrent behavior: this runtime is now the dedicated execution boundary for coding work. The next step is wiring in Aider-style patch planning and OpenHands-style repo task loops without pushing that logic back into the desktop shell.",
                patch_schema_version(),
                spec.intent.as_str(),
                spec.execution_mode.as_str(),
                spec.repo_scope.as_str(),
                spec.patch_strategy.as_str(),
                spec.patch_first,
                patch_runner_output.runner_id,
                patch_runner_output.mode,
                spec.prompt.trim(),
                join_or_default(&spec.file_targets, "No explicit file targets detected."),
                join_or_default(&spec.module_targets, "No explicit module targets detected."),
                spec.recommended_steps.join("\n- "),
                operation_steps.join("\n- "),
                change_plan.join("\n- "),
                patch_outline.join("\n- "),
                patch_proposal.join("\n- "),
                patch_files.join("\n- "),
                patch_apply_plan.join("\n- "),
                patch_execution_contract.join("\n- "),
                patch_execution_request.join("\n- "),
                patch_runner_output.execution_log.join("\n- "),
                patch_items.join("\n- "),
                patch_hunks.join("\n- "),
                patch_sets.join("\n- "),
                patch_contract.join("\n- "),
                patch_schema_version(),
                patch_schema.patch_files.len(),
                patch_schema.patch_apply_plan.len(),
                patch_schema.patch_items.len(),
                patch_schema.patch_hunks.len(),
                patch_schema.patch_sets.len(),
                patch_targets.join("\n- "),
                verification_plan.join("\n- "),
                verification_targets.join("\n- "),
                artifacts.join("\n- "),
            ),
            transcript: vec![
                "dev runtime selected".to_owned(),
                format!("patch_schema={}", patch_schema_version()),
                format!("intent={}", spec.intent.as_str()),
                format!("execution_mode={}", spec.execution_mode.as_str()),
                format!("repo_scope={}", spec.repo_scope.as_str()),
                format!("patch_strategy={}", spec.patch_strategy.as_str()),
                format!("patch_first={}", spec.patch_first),
                format!("file_targets={}", spec.file_targets.len()),
                format!("module_targets={}", spec.module_targets.len()),
                format!("operation_steps={}", operation_steps.len()),
                format!("change_plan_items={}", change_plan.len()),
                format!("patch_outline_items={}", patch_outline.len()),
                format!("patch_proposal_items={}", patch_proposal.len()),
                format!("patch_files={}", patch_files.len()),
                format!("patch_apply_plan={}", patch_apply_plan.len()),
                format!("patch_execution_contract={}", patch_execution_contract.len()),
                format!("patch_execution_request={}", patch_execution_request.len()),
                format!("patch_runner={}", patch_runner_output.runner_id),
                format!("patch_runner_log={}", patch_runner_output.execution_log.len()),
                format!("patch_items={}", patch_items.len()),
                format!("patch_hunks={}", patch_hunks.len()),
                format!("patch_sets={}", patch_sets.len()),
                format!("patch_contract={}", patch_contract.len()),
                "patch_schema_exported=true".to_owned(),
                format!("patch_targets={}", patch_targets.len()),
                format!("verification_plan_items={}", verification_plan.len()),
                format!("verification_targets={}", verification_targets.len()),
                format!("artifacts={}", artifacts.len()),
                "returned scaffold dev summary".to_owned(),
            ],
            patch_schema_version: patch_schema_version().to_owned(),
            patch_schema_json,
            patch_runner_id: patch_runner_output.runner_id,
            patch_runner_mode: patch_runner_output.mode,
            patch_runner_log: patch_runner_output.execution_log,
            patch_runner_log_json,
            intent: spec.intent.as_str().to_owned(),
            execution_mode: spec.execution_mode.as_str().to_owned(),
            repo_scope: spec.repo_scope.as_str().to_owned(),
            patch_strategy: spec.patch_strategy.as_str().to_owned(),
            patch_first: spec.patch_first,
            recommended_steps: spec.recommended_steps.clone(),
            operation_steps,
            change_plan,
            patch_outline,
            patch_proposal,
            patch_files,
            patch_apply_plan,
            patch_execution_contract,
            patch_execution_request,
            patch_items,
            patch_hunks,
            patch_sets,
            patch_contract,
            verification_plan,
            patch_targets,
            verification_targets,
            artifacts,
            file_targets: spec.file_targets.clone(),
            module_targets: spec.module_targets.clone(),
        })
    }
}

pub fn build_dev_runtime() -> Box<dyn DevRuntime> {
    Box::new(ScaffoldDevRuntime)
}

pub fn list_dev_runtime_catalog() -> Vec<DevRuntimeDescriptor> {
    vec![ScaffoldDevRuntime.descriptor()]
}

pub fn patch_schema_version() -> &'static str {
    "dev-patch-schema/v2"
}

fn build_patch_schema(spec: &DevTaskSpec) -> PatchPlanSchema {
    PatchPlanSchema {
        schema_version: patch_schema_version().to_owned(),
        intent: spec.intent.as_str().to_owned(),
        execution_mode: spec.execution_mode.as_str().to_owned(),
        repo_scope: spec.repo_scope.as_str().to_owned(),
        patch_strategy: spec.patch_strategy.as_str().to_owned(),
        file_targets: spec.file_targets.clone(),
        module_targets: spec.module_targets.clone(),
        patch_files: build_patch_file_records(spec),
        patch_apply_plan: build_patch_apply_plan_records(spec),
        execution_contract: build_patch_execution_contract_record(spec),
        execution_request: build_patch_execution_request_record(spec),
        patch_items: build_patch_item_records(spec),
        patch_hunks: build_patch_hunk_records(spec),
        patch_sets: build_patch_set_records(spec),
        contract: build_patch_contract_record(spec),
    }
}

pub fn parse_dev_task(prompt: &str) -> DevTaskSpec {
    let lower = prompt.to_lowercase();
    let intent = if contains_any(&lower, &["refactor"]) || contains_raw(prompt, &["重构"]) {
        DevIntent::Refactor
    } else if contains_any(&lower, &["fix", "patch", "implement", "change", "edit"])
        || contains_raw(prompt, &["修改", "修复", "实现", "开发"])
    {
        DevIntent::Patch
    } else if contains_any(&lower, &["test", "verify", "build", "check"])
        || contains_raw(prompt, &["检查", "验证", "测试", "构建"])
    {
        DevIntent::Verify
    } else if contains_any(&lower, &["analyze", "read code", "inspect"])
        || contains_raw(prompt, &["分析", "看代码", "阅读代码"])
    {
        DevIntent::Analyze
    } else {
        DevIntent::Unknown
    };

    let file_targets = infer_file_targets(prompt);
    let module_targets = infer_module_targets(prompt, &file_targets);
    let repo_scope = infer_repo_scope(prompt, &file_targets, &module_targets);
    let execution_mode = infer_execution_mode(&intent);
    let patch_strategy = infer_patch_strategy(&intent, &repo_scope);

    let recommended_steps = match intent {
        DevIntent::Analyze => vec![
            "Inspect the relevant workspace files and current architecture boundary.".to_owned(),
            "Identify the smallest auditable change surface.".to_owned(),
            "Summarize constraints before editing.".to_owned(),
        ],
        DevIntent::Patch => vec![
            "Inspect the relevant files and confirm the target module boundary.".to_owned(),
            "Prepare a patch-first change plan instead of freeform rewriting.".to_owned(),
            "Run build or verification after the patch lands.".to_owned(),
        ],
        DevIntent::Verify => vec![
            "Inspect the expected verification scope.".to_owned(),
            "Run build, tests, or focused checks.".to_owned(),
            "Return a concise verification summary.".to_owned(),
        ],
        DevIntent::Refactor => vec![
            "Map the current boundary and dependency surface.".to_owned(),
            "Move code incrementally behind a cleaner runtime interface.".to_owned(),
            "Verify behavior remains stable after the refactor.".to_owned(),
        ],
        DevIntent::Unknown => vec![
            "Clarify whether this is analysis, patching, verification, or refactoring.".to_owned(),
            "Prefer patch-first execution if code changes are needed.".to_owned(),
        ],
    };

    DevTaskSpec {
        prompt: prompt.to_owned(),
        intent,
        execution_mode,
        repo_scope,
        patch_strategy,
        patch_first: true,
        recommended_steps,
        file_targets,
        module_targets,
    }
}

fn infer_execution_mode(intent: &DevIntent) -> DevExecutionMode {
    match intent {
        DevIntent::Analyze | DevIntent::Unknown => DevExecutionMode::ReadOnly,
        DevIntent::Patch => DevExecutionMode::PatchReady,
        DevIntent::Verify => DevExecutionMode::VerifyOnly,
        DevIntent::Refactor => DevExecutionMode::RefactorIncremental,
    }
}

fn infer_repo_scope(
    prompt: &str,
    file_targets: &[String],
    module_targets: &[String],
) -> RepoScope {
    let lower = prompt.to_lowercase();
    if contains_any(&lower, &["workspace", "whole repo", "entire repo", "all files"])
        || contains_raw(prompt, &["全仓库", "整个仓库", "所有文件"])
    {
        RepoScope::WorkspaceWide
    } else if module_targets.len() > 1 || file_targets.len() > 3 {
        RepoScope::ModuleWide
    } else {
        RepoScope::Focused
    }
}

fn infer_patch_strategy(intent: &DevIntent, repo_scope: &RepoScope) -> PatchStrategy {
    match intent {
        DevIntent::Analyze | DevIntent::Unknown => PatchStrategy::None,
        DevIntent::Verify => PatchStrategy::VerificationPass,
        DevIntent::Patch => PatchStrategy::MinimalDiff,
        DevIntent::Refactor => match repo_scope {
            RepoScope::Focused => PatchStrategy::MinimalDiff,
            RepoScope::ModuleWide | RepoScope::WorkspaceWide => PatchStrategy::BoundaryExtraction,
        },
    }
}

fn suggest_change_plan(spec: &DevTaskSpec) -> Vec<String> {
    let mut plan = match spec.intent {
        DevIntent::Analyze => vec![
            "Locate the primary files and modules touched by the request.".to_owned(),
            "Map interfaces, dependencies, and current runtime boundaries before editing.".to_owned(),
        ],
        DevIntent::Patch => vec![
            "Prepare a minimal diff against the relevant files instead of broad rewriting.".to_owned(),
            "Keep changes inside the current module boundary unless the task explicitly needs extraction.".to_owned(),
            "Record expected artifacts such as patch output, build result, or updated config.".to_owned(),
        ],
        DevIntent::Verify => vec![
            "Select the narrowest build, test, or lint scope that proves the requested behavior.".to_owned(),
        ],
        DevIntent::Refactor => vec![
            "Move logic behind a dedicated runtime boundary or module interface.".to_owned(),
            "Preserve existing behavior while reducing shell or UI coupling.".to_owned(),
            "Keep the refactor incremental enough to remain auditable in diff form.".to_owned(),
        ],
        DevIntent::Unknown => vec!["Clarify the expected code outcome before editing.".to_owned()],
    };

    plan.push(format!(
        "Execution mode is `{}` with `{}` repo scope.",
        spec.execution_mode.as_str(),
        spec.repo_scope.as_str()
    ));
    plan.push(format!(
        "Preferred patch strategy is `{}`.",
        spec.patch_strategy.as_str()
    ));
    plan
}

fn suggest_operation_steps(spec: &DevTaskSpec) -> Vec<String> {
    match spec.execution_mode {
        DevExecutionMode::ReadOnly => vec![
            "Inspect the target files or module boundary.".to_owned(),
            "Collect findings without writing changes.".to_owned(),
            "Return the change recommendation as structured output.".to_owned(),
        ],
        DevExecutionMode::PatchReady => vec![
            "Open the smallest target file set.".to_owned(),
            "Draft a minimal diff for the requested behavior.".to_owned(),
            "Run focused verification after the patch.".to_owned(),
        ],
        DevExecutionMode::VerifyOnly => vec![
            "Select the narrowest verification command path.".to_owned(),
            "Run verification without editing workspace files.".to_owned(),
            "Return failures, passes, and uncovered scope.".to_owned(),
        ],
        DevExecutionMode::RefactorIncremental => vec![
            "Map the current runtime or module boundary.".to_owned(),
            "Apply incremental extraction or movement steps.".to_owned(),
            "Verify behavior after each boundary change.".to_owned(),
        ],
    }
}

fn suggest_patch_outline(spec: &DevTaskSpec) -> Vec<String> {
    match spec.patch_strategy {
        PatchStrategy::None => vec![
            "No patch outline yet because the task is currently read-only.".to_owned(),
        ],
        PatchStrategy::VerificationPass => vec![
            "No code patch by default; verification path should stay read-only.".to_owned(),
        ],
        PatchStrategy::MinimalDiff => {
            if !spec.file_targets.is_empty() {
                spec.file_targets
                    .iter()
                    .map(|target| {
                        format!(
                            "Apply a focused diff in `{target}` only if the requested behavior demands it."
                        )
                    })
                    .collect()
            } else {
                vec![
                    "Prepare a minimal diff against the smallest affected source file.".to_owned(),
                    "Keep tests or config changes secondary unless required for proof.".to_owned(),
                ]
            }
        }
        PatchStrategy::BoundaryExtraction => vec![
            "Extract the boundary-facing logic behind a clearer runtime interface.".to_owned(),
            "Move only the directly coupled files in the first refactor pass.".to_owned(),
            "Defer unrelated cleanup until the new boundary is verified.".to_owned(),
        ],
    }
}

fn suggest_patch_proposal(spec: &DevTaskSpec) -> Vec<String> {
    match spec.patch_strategy {
        PatchStrategy::None => vec![
            "proposal: keep this turn read-only and return findings only.".to_owned(),
        ],
        PatchStrategy::VerificationPass => vec![
            "proposal: do not patch source files; only run focused verification commands."
                .to_owned(),
        ],
        PatchStrategy::MinimalDiff => {
            if !spec.file_targets.is_empty() {
                spec.file_targets
                    .iter()
                    .map(|target| {
                        format!(
                            "proposal: update `{target}` with the smallest auditable diff that satisfies the requested behavior."
                        )
                    })
                    .collect()
            } else {
                vec![
                    "proposal: locate the narrowest source file and prepare a minimal diff."
                        .to_owned(),
                    "proposal: keep test or config edits secondary unless they are required for proof."
                        .to_owned(),
                ]
            }
        }
        PatchStrategy::BoundaryExtraction => vec![
            "proposal: extract boundary-facing logic into a cleaner runtime or service interface."
                .to_owned(),
            "proposal: move only directly coupled files in the first patch set.".to_owned(),
            "proposal: defer opportunistic cleanup until the new boundary passes verification."
                .to_owned(),
        ],
    }
}

fn suggest_patch_files(spec: &DevTaskSpec) -> Vec<String> {
    build_patch_file_records(spec)
        .into_iter()
        .map(|file| {
            format!(
                "file: path={} / role={} / boundary={} / verification={} / sets={}",
                file.path,
                file.role,
                file.mutation_boundary,
                file.verification_target,
                file.patch_set_ids.join(", ")
            )
        })
        .collect()
}

fn suggest_patch_apply_plan(spec: &DevTaskSpec) -> Vec<String> {
    build_patch_apply_plan_records(spec)
        .into_iter()
        .map(|step| {
            format!(
                "apply: batch={} / stage={} / action={} / target={} / check={}",
                step.batch_id, step.stage, step.action, step.target, step.check
            )
        })
        .collect()
}

fn suggest_patch_execution_contract(spec: &DevTaskSpec) -> Vec<String> {
    let contract = build_patch_execution_contract_record(spec);
    vec![
        format!("write_scope={}", contract.write_scope),
        format!("dry_run_first={}", contract.dry_run_first),
        format!("approval_required={}", contract.approval_required),
        format!("rollback_scope={}", contract.rollback_scope),
    ]
}

fn suggest_patch_execution_request(spec: &DevTaskSpec) -> Vec<String> {
    let request = build_patch_execution_request_record(spec);
    vec![
        format!("mode={}", request.mode),
        format!("selected_batches={}", request.selected_batches.join(", ")),
        format!("target_paths={}", request.target_paths.join(", ")),
        format!("verification_scope={}", request.verification_scope),
    ]
}

fn suggest_patch_items(spec: &DevTaskSpec) -> Vec<String> {
    build_patch_item_records(spec)
        .into_iter()
        .map(|item| {
            format!(
                "item: target={} / action={} / strategy={} / rationale={}",
                item.target, item.action, item.strategy, item.rationale
            )
        })
        .collect()
}

fn suggest_patch_hunks(spec: &DevTaskSpec) -> Vec<String> {
    build_patch_hunk_records(spec)
        .into_iter()
        .map(|hunk| {
            format!(
                "hunk: target={} / area={} / action={} / diff_shape={}",
                hunk.target, hunk.area, hunk.action, hunk.diff_shape
            )
        })
        .collect()
}

fn suggest_patch_sets(spec: &DevTaskSpec) -> Vec<String> {
    build_patch_set_records(spec)
        .into_iter()
        .map(|set| {
            format!(
                "{}: target={} / composition={}",
                set.batch_id, set.target, set.composition
            )
        })
        .collect()
}

fn suggest_patch_contract(spec: &DevTaskSpec) -> Vec<String> {
    let contract = build_patch_contract_record(spec);
    vec![
        format!("contract: execution_mode={}", contract.execution_mode),
        format!("contract: repo_scope={}", contract.repo_scope),
        format!("contract: patch_strategy={}", contract.patch_strategy),
        format!("contract: precondition={}", contract.precondition),
        format!("contract: apply_boundary={}", contract.apply_boundary),
        format!("contract: verification_gate={}", contract.verification_gate),
    ]
}

fn suggest_verification_plan(spec: &DevTaskSpec) -> Vec<String> {
    let mut plan = match spec.intent {
        DevIntent::Analyze => vec![
            "Return the relevant findings and affected files without modifying the workspace."
                .to_owned(),
        ],
        DevIntent::Patch | DevIntent::Refactor => vec![
            "Run a focused build or type-check after the patch lands.".to_owned(),
            "Run task-specific verification if the touched module has a narrower check path."
                .to_owned(),
            "Summarize the result and any residual gaps.".to_owned(),
        ],
        DevIntent::Verify => vec![
            "Capture failing or passing checks and summarize what they prove.".to_owned(),
        ],
        DevIntent::Unknown => vec![
            "State what could not be verified and what additional scope is needed.".to_owned(),
        ],
    };

    if matches!(spec.repo_scope, RepoScope::WorkspaceWide) {
        plan.push("Prefer staged verification instead of one oversized full-repo pass.".to_owned());
    }

    plan
}

fn suggest_patch_targets(spec: &DevTaskSpec) -> Vec<String> {
    if !spec.file_targets.is_empty() {
        return spec.file_targets.clone();
    }

    match spec.intent {
        DevIntent::Analyze => vec!["Read-only inspection of the relevant files.".to_owned()],
        DevIntent::Patch => vec![
            "The smallest set of source files required by the requested behavior change."
                .to_owned(),
            "Tests or configs only if the behavior change cannot be validated otherwise."
                .to_owned(),
        ],
        DevIntent::Verify => {
            vec!["No patch target by default; verification should stay read-only.".to_owned()]
        }
        DevIntent::Refactor => vec![
            "Runtime boundary files first.".to_owned(),
            "Then only the modules directly coupled to that boundary.".to_owned(),
        ],
        DevIntent::Unknown => vec!["Target files are not clear yet.".to_owned()],
    }
}

fn suggest_verification_targets(spec: &DevTaskSpec) -> Vec<String> {
    match spec.intent {
        DevIntent::Analyze => vec!["Findings summary and affected files.".to_owned()],
        DevIntent::Patch | DevIntent::Refactor => {
            if !spec.module_targets.is_empty() {
                vec![
                    format!("Focused checks for modules: {}", spec.module_targets.join(", ")),
                    "Run the narrowest build or type-check that covers the touched boundary."
                        .to_owned(),
                ]
            } else {
                vec![
                    "Focused build or type-check for the touched workspace.".to_owned(),
                    "Narrow module-level verification when available.".to_owned(),
                ]
            }
        }
        DevIntent::Verify => vec!["Requested test or build target.".to_owned()],
        DevIntent::Unknown => vec!["Verification target needs clarification.".to_owned()],
    }
}

fn suggest_artifacts(spec: &DevTaskSpec) -> Vec<String> {
    let mut artifacts = match spec.intent {
        DevIntent::Analyze => vec!["Findings summary".to_owned(), "Impacted file list".to_owned()],
        DevIntent::Patch => vec![
            "Patch or diff summary".to_owned(),
            "Verification result".to_owned(),
            "Residual risk notes".to_owned(),
        ],
        DevIntent::Verify => vec!["Verification report".to_owned()],
        DevIntent::Refactor => vec![
            "Boundary change summary".to_owned(),
            "Patch or diff summary".to_owned(),
            "Verification result".to_owned(),
        ],
        DevIntent::Unknown => vec!["Clarified task summary".to_owned()],
    };

    artifacts.push(format!("Execution contract: {}", spec.execution_mode.as_str()));
    artifacts.push(format!("Schema export: {}", patch_schema_version()));
    artifacts
}

fn build_patch_item_records(spec: &DevTaskSpec) -> Vec<PatchItemRecord> {
    match spec.patch_strategy {
        PatchStrategy::None => vec![PatchItemRecord {
            target: "read-only".to_owned(),
            action: "inspect".to_owned(),
            strategy: "none".to_owned(),
            rationale: "collect findings before any edit".to_owned(),
        }],
        PatchStrategy::VerificationPass => vec![PatchItemRecord {
            target: "verification-only".to_owned(),
            action: "run checks".to_owned(),
            strategy: "verification_pass".to_owned(),
            rationale: "do not modify source files in this turn".to_owned(),
        }],
        PatchStrategy::MinimalDiff => {
            if !spec.file_targets.is_empty() {
                spec.file_targets
                    .iter()
                    .map(|target| PatchItemRecord {
                        target: target.clone(),
                        action: "edit".to_owned(),
                        strategy: "minimal_diff".to_owned(),
                        rationale: "smallest auditable change set".to_owned(),
                    })
                    .collect()
            } else {
                vec![PatchItemRecord {
                    target: "unknown".to_owned(),
                    action: "locate target".to_owned(),
                    strategy: "minimal_diff".to_owned(),
                    rationale: "identify narrowest source file first".to_owned(),
                }]
            }
        }
        PatchStrategy::BoundaryExtraction => {
            if !spec.module_targets.is_empty() {
                spec.module_targets
                    .iter()
                    .map(|target| PatchItemRecord {
                        target: target.clone(),
                        action: "extract boundary".to_owned(),
                        strategy: "boundary_extraction".to_owned(),
                        rationale: "reduce coupling incrementally".to_owned(),
                    })
                    .collect()
            } else {
                vec![PatchItemRecord {
                    target: "unknown".to_owned(),
                    action: "extract boundary".to_owned(),
                    strategy: "boundary_extraction".to_owned(),
                    rationale: "identify runtime seam before moving code".to_owned(),
                }]
            }
        }
    }
}

fn build_patch_file_records(spec: &DevTaskSpec) -> Vec<PatchFileRecord> {
    match spec.patch_strategy {
        PatchStrategy::None => vec![PatchFileRecord {
            path: "read-only".to_owned(),
            hunks: vec![],
            role: "inspection".to_owned(),
            mutation_boundary: "no file mutation".to_owned(),
            verification_target: "findings only".to_owned(),
            patch_set_ids: vec!["set-none".to_owned()],
        }],
        PatchStrategy::VerificationPass => vec![PatchFileRecord {
            path: "verification-only".to_owned(),
            hunks: vec![],
            role: "verification".to_owned(),
            mutation_boundary: "checks only".to_owned(),
            verification_target: "verification command path".to_owned(),
            patch_set_ids: vec!["set-verification".to_owned()],
        }],
        PatchStrategy::MinimalDiff => {
            if !spec.file_targets.is_empty() {
                spec.file_targets
                    .iter()
                    .enumerate()
                    .map(|(index, path)| PatchFileRecord {
                        path: path.clone(),
                        hunks: vec![],
                        role: "primary patch target".to_owned(),
                        mutation_boundary: "minimal in-place diff only".to_owned(),
                        verification_target: format!("focused verification for {}", path),
                        patch_set_ids: vec![format!("set-{}", index + 1)],
                    })
                    .collect()
            } else {
                vec![PatchFileRecord {
                    path: "unknown".to_owned(),
                    hunks: vec![],
                    role: "discovery target".to_owned(),
                    mutation_boundary: "locate file before edit".to_owned(),
                    verification_target: "focused verification after file selection".to_owned(),
                    patch_set_ids: vec!["set-1".to_owned()],
                }]
            }
        }
        PatchStrategy::BoundaryExtraction => {
            if !spec.file_targets.is_empty() {
                spec.file_targets
                    .iter()
                    .enumerate()
                    .map(|(index, path)| PatchFileRecord {
                        path: path.clone(),
                        hunks: vec![],
                        role: "boundary extraction target".to_owned(),
                        mutation_boundary: "incremental seam extraction only".to_owned(),
                        verification_target: format!("incremental verification for {}", path),
                        patch_set_ids: vec![format!("set-{}", index + 1)],
                    })
                    .collect()
            } else if !spec.module_targets.is_empty() {
                spec.module_targets
                    .iter()
                    .enumerate()
                    .map(|(index, module)| PatchFileRecord {
                        path: module.clone(),
                        hunks: vec![],
                        role: "module boundary target".to_owned(),
                        mutation_boundary: "extract runtime seam before cleanup".to_owned(),
                        verification_target: format!("module-level verification for {}", module),
                        patch_set_ids: vec![format!("set-{}", index + 1)],
                    })
                    .collect()
            } else {
                vec![PatchFileRecord {
                    path: "unknown".to_owned(),
                    hunks: vec![],
                    role: "boundary discovery".to_owned(),
                    mutation_boundary: "identify seam before moving code".to_owned(),
                    verification_target: "verification after seam discovery".to_owned(),
                    patch_set_ids: vec!["set-1".to_owned()],
                }]
            }
        }
    }
}

fn build_patch_apply_plan_records(spec: &DevTaskSpec) -> Vec<PatchApplyStepRecord> {
    match spec.patch_strategy {
        PatchStrategy::None => vec![PatchApplyStepRecord {
            batch_id: "set-none".to_owned(),
            stage: "inspect".to_owned(),
            action: "collect findings only".to_owned(),
            target: "read-only".to_owned(),
            check: "return findings without file mutation".to_owned(),
        }],
        PatchStrategy::VerificationPass => vec![PatchApplyStepRecord {
            batch_id: "set-verification".to_owned(),
            stage: "verify".to_owned(),
            action: "run focused verification".to_owned(),
            target: "verification-only".to_owned(),
            check: "capture passing and failing checks".to_owned(),
        }],
        PatchStrategy::MinimalDiff => {
            if !spec.file_targets.is_empty() {
                spec.file_targets
                    .iter()
                    .enumerate()
                    .flat_map(|(index, path)| {
                        let batch_id = format!("set-{}", index + 1);
                        [
                            PatchApplyStepRecord {
                                batch_id: batch_id.clone(),
                                stage: "preflight".to_owned(),
                                action: "inspect target and confirm narrow diff boundary".to_owned(),
                                target: path.clone(),
                                check: format!("verify target file `{}` matches requested behavior", path),
                            },
                            PatchApplyStepRecord {
                                batch_id: batch_id.clone(),
                                stage: "apply".to_owned(),
                                action: "apply minimal in-place patch".to_owned(),
                                target: path.clone(),
                                check: "keep change inside one auditable diff batch".to_owned(),
                            },
                            PatchApplyStepRecord {
                                batch_id,
                                stage: "verify".to_owned(),
                                action: "run focused verification".to_owned(),
                                target: path.clone(),
                                check: format!("run the narrowest check path covering `{}`", path),
                            },
                        ]
                    })
                    .collect()
            } else {
                vec![
                    PatchApplyStepRecord {
                        batch_id: "set-1".to_owned(),
                        stage: "preflight".to_owned(),
                        action: "locate the primary target file".to_owned(),
                        target: "unknown".to_owned(),
                        check: "confirm the smallest file boundary before editing".to_owned(),
                    },
                    PatchApplyStepRecord {
                        batch_id: "set-1".to_owned(),
                        stage: "apply".to_owned(),
                        action: "prepare minimal diff".to_owned(),
                        target: "unknown".to_owned(),
                        check: "avoid broad rewrites".to_owned(),
                    },
                    PatchApplyStepRecord {
                        batch_id: "set-1".to_owned(),
                        stage: "verify".to_owned(),
                        action: "run focused verification".to_owned(),
                        target: "unknown".to_owned(),
                        check: "prove the edited behavior only".to_owned(),
                    },
                ]
            }
        }
        PatchStrategy::BoundaryExtraction => {
            if !spec.module_targets.is_empty() {
                spec.module_targets
                    .iter()
                    .enumerate()
                    .flat_map(|(index, module)| {
                        let batch_id = format!("set-{}", index + 1);
                        [
                            PatchApplyStepRecord {
                                batch_id: batch_id.clone(),
                                stage: "preflight".to_owned(),
                                action: "confirm runtime seam and dependency surface".to_owned(),
                                target: module.clone(),
                                check: format!("identify boundary inside module `{}`", module),
                            },
                            PatchApplyStepRecord {
                                batch_id: batch_id.clone(),
                                stage: "apply".to_owned(),
                                action: "extract boundary incrementally".to_owned(),
                                target: module.clone(),
                                check: "defer unrelated cleanup".to_owned(),
                            },
                            PatchApplyStepRecord {
                                batch_id,
                                stage: "verify".to_owned(),
                                action: "run incremental verification".to_owned(),
                                target: module.clone(),
                                check: format!("verify extracted boundary for `{}`", module),
                            },
                        ]
                    })
                    .collect()
            } else {
                vec![
                    PatchApplyStepRecord {
                        batch_id: "set-1".to_owned(),
                        stage: "preflight".to_owned(),
                        action: "discover runtime seam".to_owned(),
                        target: "unknown".to_owned(),
                        check: "confirm a stable extraction boundary".to_owned(),
                    },
                    PatchApplyStepRecord {
                        batch_id: "set-1".to_owned(),
                        stage: "apply".to_owned(),
                        action: "extract boundary in one small pass".to_owned(),
                        target: "unknown".to_owned(),
                        check: "avoid cross-cutting cleanup".to_owned(),
                    },
                    PatchApplyStepRecord {
                        batch_id: "set-1".to_owned(),
                        stage: "verify".to_owned(),
                        action: "run incremental verification".to_owned(),
                        target: "unknown".to_owned(),
                        check: "prove behavior stayed stable after extraction".to_owned(),
                    },
                ]
            }
        }
    }
}

fn build_patch_execution_contract_record(spec: &DevTaskSpec) -> PatchExecutionContractRecord {
    let (write_scope, rollback_scope, approval_required) = match spec.patch_strategy {
        PatchStrategy::None => (
            "no file writes".to_owned(),
            "no rollback needed".to_owned(),
            false,
        ),
        PatchStrategy::VerificationPass => (
            "verification only".to_owned(),
            "no rollback needed".to_owned(),
            false,
        ),
        PatchStrategy::MinimalDiff => (
            if spec.file_targets.is_empty() {
                "single focused source file after target confirmation".to_owned()
            } else {
                format!("focused file set: {}", spec.file_targets.join(", "))
            },
            "revert the latest patch set only".to_owned(),
            false,
        ),
        PatchStrategy::BoundaryExtraction => (
            if spec.module_targets.is_empty() {
                "module boundary files after seam confirmation".to_owned()
            } else {
                format!("boundary extraction inside modules: {}", spec.module_targets.join(", "))
            },
            "revert one extraction batch at a time".to_owned(),
            true,
        ),
    };

    PatchExecutionContractRecord {
        write_scope,
        dry_run_first: !matches!(spec.patch_strategy, PatchStrategy::None),
        approval_required,
        rollback_scope,
    }
}

fn build_patch_execution_request_record(spec: &DevTaskSpec) -> PatchExecutionRequestRecord {
    let selected_batches = build_patch_set_records(spec)
        .into_iter()
        .map(|set| set.batch_id)
        .collect::<Vec<_>>();
    let target_paths = if !spec.file_targets.is_empty() {
        spec.file_targets.clone()
    } else if !spec.module_targets.is_empty() {
        spec.module_targets.clone()
    } else {
        vec!["unknown".to_owned()]
    };
    let mode = match spec.patch_strategy {
        PatchStrategy::None => "inspect_only",
        _ if matches!(spec.execution_mode, DevExecutionMode::VerifyOnly) => "verify_only",
        _ if matches!(spec.patch_strategy, PatchStrategy::BoundaryExtraction) => "dry_run_boundary",
        _ => "dry_run_patch",
    }
    .to_owned();
    let verification_scope = match spec.repo_scope {
        RepoScope::Focused => "focused target verification",
        RepoScope::ModuleWide => "module-level verification",
        RepoScope::WorkspaceWide => "staged workspace verification",
    }
    .to_owned();

    PatchExecutionRequestRecord {
        mode,
        selected_batches,
        target_paths,
        verification_scope,
    }
}

fn build_patch_hunk_records(spec: &DevTaskSpec) -> Vec<PatchHunkRecord> {
    match spec.patch_strategy {
        PatchStrategy::None => vec![PatchHunkRecord {
            target: "none".to_owned(),
            area: "analysis only".to_owned(),
            action: "none".to_owned(),
            diff_shape: "none".to_owned(),
        }],
        PatchStrategy::VerificationPass => vec![PatchHunkRecord {
            target: "none".to_owned(),
            area: "verification only".to_owned(),
            action: "none".to_owned(),
            diff_shape: "none".to_owned(),
        }],
        PatchStrategy::MinimalDiff => {
            if !spec.file_targets.is_empty() {
                spec.file_targets
                    .iter()
                    .map(|target| PatchHunkRecord {
                        target: target.clone(),
                        area: "smallest affected block".to_owned(),
                        action: "edit in place".to_owned(),
                        diff_shape: "minimal".to_owned(),
                    })
                    .collect()
            } else {
                vec![PatchHunkRecord {
                    target: "unknown".to_owned(),
                    area: "smallest affected block".to_owned(),
                    action: "locate before edit".to_owned(),
                    diff_shape: "minimal".to_owned(),
                }]
            }
        }
        PatchStrategy::BoundaryExtraction => {
            if !spec.module_targets.is_empty() {
                spec.module_targets
                    .iter()
                    .map(|target| PatchHunkRecord {
                        target: target.clone(),
                        area: "boundary-facing logic".to_owned(),
                        action: "extract or move".to_owned(),
                        diff_shape: "incremental".to_owned(),
                    })
                    .collect()
            } else {
                vec![PatchHunkRecord {
                    target: "unknown".to_owned(),
                    area: "runtime seam".to_owned(),
                    action: "extract boundary first".to_owned(),
                    diff_shape: "incremental".to_owned(),
                }]
            }
        }
    }
}

fn build_patch_set_records(spec: &DevTaskSpec) -> Vec<PatchSetRecord> {
    match spec.patch_strategy {
        PatchStrategy::None => vec![PatchSetRecord {
            batch_id: "set-none".to_owned(),
            target: "read-only".to_owned(),
            composition: "findings only".to_owned(),
        }],
        PatchStrategy::VerificationPass => vec![PatchSetRecord {
            batch_id: "set-verification".to_owned(),
            target: "verification-only".to_owned(),
            composition: "checks and reports".to_owned(),
        }],
        PatchStrategy::MinimalDiff => {
            if !spec.file_targets.is_empty() {
                spec.file_targets
                    .iter()
                    .enumerate()
                    .map(|(index, target)| PatchSetRecord {
                        batch_id: format!("set-{}", index + 1),
                        target: target.clone(),
                        composition: "minimal file diff + focused verification".to_owned(),
                    })
                    .collect()
            } else {
                vec![PatchSetRecord {
                    batch_id: "set-1".to_owned(),
                    target: "unknown".to_owned(),
                    composition: "locate file + minimal diff + focused verification".to_owned(),
                }]
            }
        }
        PatchStrategy::BoundaryExtraction => {
            if !spec.module_targets.is_empty() {
                spec.module_targets
                    .iter()
                    .enumerate()
                    .map(|(index, target)| PatchSetRecord {
                        batch_id: format!("set-{}", index + 1),
                        target: target.clone(),
                        composition:
                            "boundary extraction patch + incremental verification".to_owned(),
                    })
                    .collect()
            } else {
                vec![PatchSetRecord {
                    batch_id: "set-1".to_owned(),
                    target: "unknown".to_owned(),
                    composition:
                        "discover seam + extraction patch + incremental verification".to_owned(),
                }]
            }
        }
    }
}

fn build_patch_contract_record(spec: &DevTaskSpec) -> PatchContractRecord {
    let (precondition, apply_boundary) = match spec.patch_strategy {
        PatchStrategy::None => (
            "read-only analysis only".to_owned(),
            "no file edits allowed".to_owned(),
        ),
        PatchStrategy::VerificationPass => (
            "verification commands are available".to_owned(),
            "checks only, no source mutation".to_owned(),
        ),
        PatchStrategy::MinimalDiff => (
            "target file and requested behavior are both clear".to_owned(),
            "smallest auditable diff only".to_owned(),
        ),
        PatchStrategy::BoundaryExtraction => (
            "runtime seam or module boundary is identified".to_owned(),
            "incremental extraction, no broad cleanup in first pass".to_owned(),
        ),
    };

    PatchContractRecord {
        execution_mode: spec.execution_mode.as_str().to_owned(),
        repo_scope: spec.repo_scope.as_str().to_owned(),
        patch_strategy: spec.patch_strategy.as_str().to_owned(),
        precondition,
        apply_boundary,
        verification_gate:
            "run focused verification before marking patch set complete".to_owned(),
    }
}

fn infer_file_targets(prompt: &str) -> Vec<String> {
    let mut targets = Vec::new();

    for token in prompt.split_whitespace() {
        let cleaned = token
            .trim_matches(|c: char| matches!(c, ',' | ';' | '(' | ')' | '[' | ']' | '"' | '\''))
            .trim();
        if (cleaned.contains('/') || cleaned.contains('\\')) && looks_like_file_target(cleaned) {
            let value = cleaned.replace('\\', "/");
            if !targets.contains(&value) {
                targets.push(value);
            }
        }
    }

    targets
}

fn infer_module_targets(prompt: &str, file_targets: &[String]) -> Vec<String> {
    let mut modules = file_targets
        .iter()
        .filter_map(|path| path.split('/').nth_back(1).map(|part| part.to_owned()))
        .collect::<Vec<_>>();

    for token in prompt.split_whitespace() {
        let cleaned = token
            .trim_matches(|c: char| matches!(c, ',' | ';' | '(' | ')' | '[' | ']' | '"' | '\''))
            .trim();
        if cleaned.ends_with("module")
            || cleaned.ends_with("service")
            || cleaned.ends_with("runtime")
            || cleaned.ends_with("executor")
        {
            if !modules.iter().any(|item| item == cleaned) {
                modules.push(cleaned.to_owned());
            }
        }
    }

    modules
}

fn looks_like_file_target(token: &str) -> bool {
    [
        ".rs", ".ts", ".tsx", ".js", ".jsx", ".json", ".toml", ".md", ".css", ".yml", ".yaml",
    ]
    .iter()
    .any(|suffix| token.ends_with(suffix))
}

fn join_or_default(items: &[String], fallback: &str) -> String {
    if items.is_empty() {
        fallback.to_owned()
    } else {
        items.join("\n- ")
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn contains_raw(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}
