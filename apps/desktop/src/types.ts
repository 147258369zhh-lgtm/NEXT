export type Locale = "zh-CN" | "en-US";
export type SideView = "modules" | "history";
export type MainView = "workspace" | "control";

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
  route_scope: string[];
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

export type PatchRunnerDescriptor = {
  id: string;
  title: string;
  mode: string;
  mutates_files: boolean;
  enabled: boolean;
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
  artifacts: string;
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
  noArtifacts: string;
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
};
