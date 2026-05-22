export type Locale = "zh-CN" | "en-US";
export type SideView = "modules" | "history";
export type MainView = "workspace" | "search" | "skills" | "plugins" | "automation" | "projects" | "control";

export type IconName =
  | "modules"
  | "history"
  | "provider"
  | "risk"
  | "approval"
  | "brain"
  | "memory"
  | "spark"
  | "empty";

export type TaskView = {
  id: string;
  title: string;
  status: string;
  risk_level: string;
  result_summary?: string | null;
  created_at?: string;
};

export type ApprovalView = {
  id: string;
  task_id: string;
  reason: string;
  status: string;
  payload: string;
  expires_at: string;
};

export type MemoryView = {
  id: string;
  task_id: string;
  card_type: string;
  title: string;
  tags: string[];
  created_at: string;
};

export type AuditView = {
  id: string;
  task_id: string;
  event_type: string;
  actor: string;
  channel: string;
  tool_name?: string | null;
  risk_level: string;
  result: string;
  timestamp: string;
};

export type TaskStepView = {
  id: string;
  title: string;
  detail: string;
  status: string;
  position: number;
};

export type TaskWorkspace = {
  task: TaskView;
  steps: TaskStepView[];
};

export type ExecutionRequestView = {
  task_id: string;
  executor_id: string;
  route: string;
  task_kind: string;
  prompt: string;
  risk_level: string;
  approval_id?: string | null;
  memory_enabled: boolean;
};

export type ExecutionArtifact = {
  kind: string;
  title: string;
  summary: string;
  payload: string;
};

export type StructuredExecutionResult = {
  executor_id: string;
  status: string;
  summary: string;
  risk_level: string;
  steps: string[];
  artifacts: ExecutionArtifact[];
  audit_refs: string[];
  memory_candidates: string[];
  follow_up_suggestions: string[];
};
export type ExecutionSnapshot = {
  execution_request: ExecutionRequestView;
  execution_result: StructuredExecutionResult;
};
export type ChatPayload = {
  task: TaskView;
  reply: string;
  approval?: ApprovalView | null;
  plan: TaskStepView[];
};

export type ModuleStatus = {
  provider_source: string;
  risk_policy_source: string;
  pending_approvals: number;
  memory_cards: number;
  last_brain_route: string;
  brain_enabled: boolean;
  memory_enabled: boolean;
};

export type ModuleDescriptor = {
  id: string;
  title: string;
  hot_swappable: boolean;
  enabled: boolean;
};

export type ExecutorDescriptor = {
  id: string;
  title: string;
  family: string;
  summary: string;
  route_scope: string[];
  task_kinds: string[];
  risk_ceiling: string;
  integration_level: string;
  input_schema: string;
  output_schema: string;
  supports_dry_run: boolean;
  supports_rollback: boolean;
  requires_approval: boolean;
  enabled: boolean;
};

export type ProviderDescriptor = {
  id: string;
  family: "Chat" | "Stt" | "Tts" | "Realtime" | "Embedding" | string;
  vendor: string;
  title: string;
  local_first: boolean;
  enabled: boolean;
};

export type BrowserRuntimeDescriptor = {
  id: string;
  title: string;
  engine: string;
  headless_default: boolean;
  supports_live_control: boolean;
  enabled: boolean;
};

export type DevModeDescriptor = {
  slug: string;
  title: string;
  intent: string;
  task_kinds: string[];
  allowed_tool_groups: string[];
  allowed_path_patterns: string[];
  mutates_files: boolean;
  requires_approval: boolean;
  default_runner: string;
  borrowed_from: string;
};
export type PatchRunnerDescriptor = {
  id: string;
  title: string;
  mode: string;
  family: string;
  source: string;
  repository: string;
  license: string;
  review_status: string;
  integration_level: string;
  mutates_files: boolean;
  requires_approval: boolean;
  supports_dry_run: boolean;
  enabled: boolean;
};

export type AutomationView = {
  id: string;
  title: string;
  description: string;
  enabled: boolean;
  created_at: string;
  updated_at: string;
};

export type ModuleAction = {
  label: string;
  kind?: "danger";
  onClick: () => void;
};

export type ModuleCardData = {
  id: string;
  title: string;
  subtitle: string;
  detail: string;
  icon: IconName;
  enabled?: boolean;
  actions: ModuleAction[];
};

export type ChatMessage = {
  id: string;
  role: "user" | "assistant" | "system";
  text: string;
  meta?: string;
};

export type CopyBundle = {
  appTitle: string;
  appSubtitle: string;
  workspaceLabel: string;
  workspaceTab: string;
  controlTab: string;
  heroTitle: string;
  heroDesc: string;
  controlTitle: string;
  controlDesc: string;
  controlModuleInventory: string;
  controlExecutors: string;
  controlProviders: string;
  controlBrowserRuntimes: string;
  controlPatchRunners: string;
  controlDevModes: string;
  controlPatchRunnerActivity: string;
  controlPatchRunnerStatus: string;
  controlBrowserActivity: string;
  controlDevActivity: string;
  controlRuntime: string;
  controlConnectors: string;
  controlVoice: string;
  controlAudit: string;
  controlPlaceholder: string;
  hotSwappable: string;
  nativeSettings: string;
  providerLive: string;
  riskActive: string;
  approvalsCount: string;
  modules: string;
  history: string;
  inputPlaceholder: string;
  send: string;
  sending: string;
  noMessagesTitle: string;
  noMessagesDesc: string;
  userRole: string;
  assistantRole: string;
  systemRole: string;
  pendingItems: string;
  noDataTitle: string;
  noPending: string;
  approve: string;
  reject: string;
  refresh: string;
  useMock: string;
  useOpenai: string;
  reload: string;
  enable: string;
  disable: string;
  enabled: string;
  disabled: string;
  riskPolicy: string;
  providerModule: string;
  approvalQueue: string;
  brainKernel: string;
  memoryModule: string;
  riskPolicySub: string;
  providerSub: string;
  approvalSubAttention: string;
  approvalSubHealthy: string;
  brainSub: string;
  memorySub: string;
  pendingCountText: string;
  memoryCardsCount: string;
  lastRoutePrefix: string;
  recentTasks: string;
  recentApprovals: string;
  recentMemory: string;
  currentTask: string;
  taskOutcome: string;
  actionPhase: string;
  fieldPlan: string;
  missingFields: string;
  sensitiveFields: string;
  nextActions: string;
  fileTargets: string;
  moduleTargets: string;
  executionMode: string;
  patchSchema: string;
  patchSchemaPreview: string;
  repoScope: string;
  patchStrategy: string;
  operationSteps: string;
  patchTargets: string;
  changePlan: string;
  patchOutline: string;
  patchProposal: string;
  patchFiles: string;
  patchApplyPlan: string;
  patchExecutionContract: string;
  patchExecutionRequest: string;
  patchItems: string;
  patchHunks: string;
  patchSets: string;
  patchContract: string;
  executionResult: string;
  artifacts: string;
  followUpSuggestions: string;
  verificationTargets: string;
  latestActivity: string;
  taskPlan: string;
  noTaskWorkspace: string;
  noTaskOutcome: string;
  noActionPhase: string;
  noFieldPlan: string;
  noMissingFields: string;
  noSensitiveFields: string;
  noNextActions: string;
  noFileTargets: string;
  noModuleTargets: string;
  noExecutionMode: string;
  noPatchSchema: string;
  noPatchSchemaPreview: string;
  noRepoScope: string;
  noPatchStrategy: string;
  noOperationSteps: string;
  noPatchTargets: string;
  noChangePlan: string;
  noPatchOutline: string;
  noPatchProposal: string;
  noPatchFiles: string;
  noPatchApplyPlan: string;
  noPatchExecutionContract: string;
  noPatchExecutionRequest: string;
  noPatchItems: string;
  noPatchHunks: string;
  noPatchSets: string;
  noPatchContract: string;
  noExecutionResult: string;
  noArtifacts: string;
  noFollowUpSuggestions: string;
  noVerificationTargets: string;
  noActivity: string;
  noTaskHistory: string;
  noApprovalHistory: string;
  noMemoryHistory: string;
  status: string;
  riskLevel: string;
  createdAt: string;
  expiresAt: string;
  failedLoadApprovals: string;
  failedLoadHistory: string;
  failedRiskSource: string;
  riskReloadFailed: string;
  failedProviderSource: string;
  providerReloadFailed: string;
  moduleToggleFailed: string;
  submissionFailed: string;
  approvalUpdateFailed: string;
  failedModuleStatus: string;
  approvedMsg: string;
  rejectedMsg: string;
  queuedMsg: string;
  langZh: string;
  langEn: string;
  navChat: string;
  navSearch: string;
  navSkills: string;
  navPlugins: string;
  navAutomation: string;
  navProjects: string;
  navSettings: string;
  projectSection: string;
  currentThread: string;
  attachAction: string;
  fullAccess: string;
  autoCode: string;
  statusProgress: string;
  readyStatus: string;
  cleanSurfaceHint: string;
  statusOutput: string;
  localPreview: string;
  statusBrowser: string;
  browserPreview: string;
  statusRuntime: string;
  statusSource: string;
  localRuntime: string;
  searchTitle: string;
  searchDesc: string;
  searchPlaceholder: string;
  skillsTitle: string;
  skillsDesc: string;
  pluginsTitle: string;
  pluginsDesc: string;
  automationTitle: string;
  automationDesc: string;
  automationQueue: string;
  automationEmpty: string;
  projectsTitle: string;
  projectsDesc: string;
  projectNextDesc: string;
  searchResults: string;
  searchResultCount: string;
  noSearchResults: string;
  browserHeadless: string;
  automationDraft: string;
  automationDraftDefault: string;
  automationDraftDesc: string;
  automationVerifyDraft: string;
  automationMemoryDraft: string;
  createVerificationAutomation: string;
  createMemoryAutomation: string;
  savedAutomations: string;
  noSavedAutomations: string;
  deleteAction: string;
  allFilter: string;
  openProjectSummary: string;
  inspectProjectTargets: string;
};

export type SkillMetadata = {
  name: string;
  version: string;
  author?: string;
  description?: string;
};

export type Skill = {
  id: string;
  metadata: SkillMetadata;
  triggers: string[];
  actions: string[];
  risk_level: string;
  execution_mode: string;
  path: string;
  enabled?: boolean;
};

export type McpTool = {
  name: string;
  description: string;
  input_schema: any;
};

export type McpServer = {
  id: string;
  name: string;
  status: string;
  tools: McpTool[];
};

export type McpServerDescriptor = {
  id: string;
  name: string;
  command: string;
  args: string[];
  enabled?: boolean;
};

export type ConnectorStatus = {
  id: string;
  name: string;
  port: number;
  status: "online" | "offline";
  last_activity?: string;
};
