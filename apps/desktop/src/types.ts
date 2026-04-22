export type Locale = "zh-CN" | "en-US";
export type SideView = "modules" | "history";

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
  heroTitle: string;
  heroDesc: string;
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
  taskPlan: string;
  noTaskWorkspace: string;
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
