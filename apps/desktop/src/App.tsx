import { FormEvent, useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ChatWorkspace } from "./components/ChatWorkspace";
import { ControlCenter } from "./components/ControlCenter";
import { SideRail } from "./components/SideRail";
import { Topbar } from "./components/Topbar";
import { SecondaryWorkspace } from "./components/SecondaryWorkspace";
import { I18N, getInitialLocale } from "./i18n";
import type {
  ApprovalView,
  AuditView,
  BrowserRuntimeDescriptor,
  ChatMessage,
  ChatPayload,
  ExecutorDescriptor,
  Locale,
  MainView,
  MemoryView,
  ModuleCardData,
  ModuleDescriptor,
  ModuleStatus,
  PatchRunnerDescriptor,
  ProviderDescriptor,
  SideView,
  TaskView,
  TaskWorkspace,
  Skill,
  ConnectorStatus,
  McpServerDescriptor,
  DevModeDescriptor,
  AutomationView,
  StructuredExecutionResult,
  ExecutionSnapshot
} from "./types";

export default function App() {
  const [locale, setLocale] = useState<Locale>(getInitialLocale);
  const [mainView, setMainView] = useState<MainView>("workspace");
  const [sideView, setSideView] = useState<SideView>("modules");
  const [message, setMessage] = useState("");
  const [loading, setLoading] = useState(false);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [pendingApprovals, setPendingApprovals] = useState<ApprovalView[]>([]);
  const [recentTasks, setRecentTasks] = useState<TaskView[]>([]);
  const [recentApprovals, setRecentApprovals] = useState<ApprovalView[]>([]);
  const [recentMemory, setRecentMemory] = useState<MemoryView[]>([]);
  const [recentAudits, setRecentAudits] = useState<AuditView[]>([]);
  const [workspace, setWorkspace] = useState<TaskWorkspace | null>(null);
  const [riskPolicySource, setRiskPolicySource] = useState("loading...");
  const [providerSource, setProviderSource] = useState("loading...");
  const [pendingCount, setPendingCount] = useState(0);
  const [memoryCards, setMemoryCards] = useState(0);
  const [lastBrainRoute, setLastBrainRoute] = useState("boot");
  const [brainEnabled, setBrainEnabled] = useState(true);
  const [memoryEnabled, setMemoryEnabled] = useState(true);
  const [modules, setModules] = useState<ModuleDescriptor[]>([]);
  const [executors, setExecutors] = useState<ExecutorDescriptor[]>([]);
  const [providers, setProviders] = useState<ProviderDescriptor[]>([]);
  const [browserRuntimes, setBrowserRuntimes] = useState<BrowserRuntimeDescriptor[]>([]);
  const [patchRunners, setPatchRunners] = useState<PatchRunnerDescriptor[]>([]);

  // Integrated states for capability and external modules
  const [skills, setSkills] = useState<Skill[]>([]);
  const [mcpServers, setMcpServers] = useState<McpServerDescriptor[]>([]);
  const [connectors, setConnectors] = useState<ConnectorStatus[]>([]);
  const [devModes, setDevModes] = useState<DevModeDescriptor[]>([]);
  const [automations, setAutomations] = useState<AutomationView[]>([]);
  const [latestExecution, setLatestExecution] = useState<StructuredExecutionResult | null>(null);

  const [error, setError] = useState<string | null>(null);
  const t = useMemo(() => I18N[locale], [locale]);

  useEffect(() => {
    localStorage.setItem("nexus.locale", locale);
  }, [locale]);

  useEffect(() => {
    void refreshAll();
  }, []);

  // Auto-dismiss the sleek floating error toast after 6 seconds
  useEffect(() => {
    if (error) {
      const timer = setTimeout(() => setError(null), 6000);
      return () => clearTimeout(timer);
    }
  }, [error]);

  function applyModuleStatus(status: ModuleStatus) {
    setProviderSource(status.provider_source);
    setRiskPolicySource(status.risk_policy_source);
    setPendingCount(status.pending_approvals);
    setMemoryCards(status.memory_cards);
    setLastBrainRoute(status.last_brain_route);
    setBrainEnabled(status.brain_enabled);
    setMemoryEnabled(status.memory_enabled);
  }

  async function refreshAll() {
    await Promise.all([
      refreshApprovals(),
      refreshHistory(),
      refreshRiskPolicySource(),
      refreshProviderSource(),
      refreshModuleStatus(),
      refreshModules(),
      refreshExecutors(),
      refreshProviders(),
      refreshBrowserRuntimes(),
      refreshPatchRunners(),
      refreshSkills(),
      refreshMcpServers(),
      refreshConnectors(),
      refreshDevModes(),
      refreshAutomations()
    ]);
  }

  async function refreshApprovals() {
    try {
      setPendingApprovals(await invoke<ApprovalView[]>("list_pending_approvals"));
    } catch {
      setError(t.failedLoadApprovals);
    }
  }

  async function refreshHistory() {
    try {
      const [tasks, approvals, memoryCardsList, audits, nextWorkspace, snapshot] = await Promise.all([
        invoke<TaskView[]>("list_recent_tasks", { limit: 12 }),
        invoke<ApprovalView[]>("list_recent_approvals", { limit: 12 }),
        invoke<MemoryView[]>("list_recent_memory_cards", { limit: 12 }),
        invoke<AuditView[]>("list_recent_audits", { limit: 18 }),
        invoke<TaskWorkspace | null>("get_latest_workspace"),
        invoke<ExecutionSnapshot | null>("get_latest_execution_snapshot")
      ]);
      setRecentTasks(tasks);
      setRecentApprovals(approvals);
      setRecentMemory(memoryCardsList);
      setRecentAudits(audits);
      setWorkspace(nextWorkspace);
      setLatestExecution(snapshot ? snapshot.execution_result : null);
    } catch {
      setError(t.failedLoadHistory);
    }
  }

  async function refreshRiskPolicySource() {
    try {
      setRiskPolicySource(await invoke<string>("get_risk_policy_source"));
    } catch {
      setError(t.failedRiskSource);
    }
  }

  async function refreshProviderSource() {
    try {
      setProviderSource(await invoke<string>("get_provider_source"));
    } catch {
      setError(t.failedProviderSource);
    }
  }

  async function refreshModuleStatus() {
    try {
      const status = await invoke<ModuleStatus>("get_module_status");
      applyModuleStatus(status);
    } catch {
      setError(t.failedModuleStatus);
    }
  }

  async function refreshModules() {
    try {
      setModules(await invoke<ModuleDescriptor[]>("list_modules"));
    } catch {
      setError(t.failedModuleStatus);
    }
  }

  async function refreshExecutors() {
    try {
      setExecutors(await invoke<ExecutorDescriptor[]>("list_executors"));
    } catch {
      setError(t.failedModuleStatus);
    }
  }

  async function refreshProviders() {
    try {
      setProviders(await invoke<ProviderDescriptor[]>("list_providers"));
    } catch {
      setError(t.failedProviderSource);
    }
  }

  async function refreshBrowserRuntimes() {
    try {
      setBrowserRuntimes(
        await invoke<BrowserRuntimeDescriptor[]>("list_browser_runtimes")
      );
    } catch {
      setError(t.failedModuleStatus);
    }
  }

  async function refreshPatchRunners() {
    try {
      setPatchRunners(await invoke<PatchRunnerDescriptor[]>("list_patch_runners"));
    } catch {
      setError(t.failedModuleStatus);
    }
  }

  async function refreshSkills() {
    try {
      setSkills(await invoke<Skill[]>("list_skills"));
    } catch (err) {
      console.error("Failed to load skills", err);
    }
  }

  async function refreshMcpServers() {
    try {
      setMcpServers(await invoke<McpServerDescriptor[]>("list_mcp_servers"));
    } catch (err) {
      console.error("Failed to load MCP servers", err);
    }
  }

  async function refreshConnectors() {
    try {
      setConnectors(await invoke<ConnectorStatus[]>("list_connectors"));
    } catch (err) {
      console.error("Failed to load connectors", err);
    }
  }

  async function refreshDevModes() {
    try {
      setDevModes(await invoke<DevModeDescriptor[]>("list_dev_modes"));
    } catch (err) {
      console.error("Failed to load dev modes", err);
    }
  }

  async function refreshAutomations() {
    try {
      setAutomations(await invoke<AutomationView[]>("list_automations"));
    } catch (err) {
      console.error("Failed to load automations", err);
    }
  }

  async function handleCreateAutomation(title: string, description: string) {
    try {
      await invoke("create_automation", { title, description });
      await refreshAutomations();
    } catch (err) {
      setError(t.moduleToggleFailed);
    }
  }

  async function handleToggleAutomation(automationId: string, enabled: boolean) {
    try {
      await invoke("set_automation_enabled", { automationId, enabled });
      await refreshAutomations();
    } catch (err) {
      setError(t.moduleToggleFailed);
    }
  }

  async function handleDeleteAutomation(automationId: string) {
    try {
      await invoke("delete_automation", { automationId });
      await refreshAutomations();
    } catch (err) {
      setError(t.moduleToggleFailed);
    }
  }

  async function reloadRiskPolicy() {
    setLoading(true);
    setError(null);
    try {
      setRiskPolicySource(await invoke<string>("reload_risk_policy", { path: null }));
      await refreshModuleStatus();
    } catch {
      setError(t.riskReloadFailed);
    } finally {
      setLoading(false);
    }
  }

  async function reloadProvider(mode: string) {
    setLoading(true);
    setError(null);
    try {
      setProviderSource(await invoke<string>("reload_provider", { mode }));
      await refreshModuleStatus();
    } catch (err: any) {
      setError(err?.toString() || t.providerReloadFailed);
    } finally {
      setLoading(false);
    }
  }

  async function toggleModule(module: "brain" | "memory", enabled: boolean) {
    setLoading(true);
    setError(null);
    try {
      const status = await invoke<ModuleStatus>("set_module_enabled", {
        module,
        enabled
      });
      applyModuleStatus(status);
      await refreshModules();
    } catch {
      setError(t.moduleToggleFailed);
    } finally {
      setLoading(false);
    }
  }

  async function sendMessage(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!message.trim()) {
      return;
    }

    const input = message.trim();
    setMessages((prev) => [
      ...prev,
      { id: crypto.randomUUID(), role: "user", text: input }
    ]);
    setMessage("");
    setLoading(true);
    setError(null);

    try {
      const result = await invoke<ChatPayload>("submit_chat", {
        message: input,
        locale
      });
      setMessages((prev) => [
        ...prev,
        {
          id: crypto.randomUUID(),
          role: "assistant",
          text: result.approval ? t.queuedMsg : result.reply,
          meta: `${result.task.title} / ${result.task.risk_level} / ${result.task.status}`
        }
      ]);
      await refreshApprovals();
      await refreshHistory();
      await refreshModuleStatus();
    } catch {
      setError(t.submissionFailed);
    } finally {
      setLoading(false);
    }
  }

  async function handleApproval(approvalId: string, approved: boolean) {
    setLoading(true);
    setError(null);
    try {
      await invoke<ChatPayload>("resolve_approval", {
        approvalId,
        approved,
        locale
      });
      setMessages((prev) => [
        ...prev,
        {
          id: crypto.randomUUID(),
          role: "system",
          text: approved ? t.approvedMsg : t.rejectedMsg
        }
      ]);
      await refreshApprovals();
      await refreshHistory();
      await refreshModuleStatus();
    } catch {
      setError(t.approvalUpdateFailed);
    } finally {
      setLoading(false);
    }
  }

  const moduleCards: ModuleCardData[] = [
    {
      id: "risk-policy",
      title: t.riskPolicy,
      subtitle: t.riskPolicySub,
      detail: riskPolicySource,
      icon: "risk",
      actions: [{ label: t.reload, onClick: () => void reloadRiskPolicy() }]
    },
    {
      id: "provider",
      title: t.providerModule,
      subtitle: t.providerSub,
      detail: providerSource,
      icon: "provider",
      actions: [
        { label: t.useMock, onClick: () => void reloadProvider("mock") },
        { label: t.useOpenai, onClick: () => void reloadProvider("openai") }
      ]
    },
    {
      id: "approvals",
      title: t.approvalQueue,
      subtitle: pendingCount > 0 ? t.approvalSubAttention : t.approvalSubHealthy,
      detail: `${pendingCount} ${t.pendingCountText}`,
      icon: "approval",
      actions: [{ label: t.refresh, onClick: () => void refreshApprovals() }]
    },
    {
      id: "brain",
      title: t.brainKernel,
      subtitle: t.brainSub,
      detail: `${t.lastRoutePrefix}: ${lastBrainRoute}`,
      icon: "brain",
      enabled: brainEnabled,
      actions: [
        {
          label: brainEnabled ? t.disable : t.enable,
          onClick: () => void toggleModule("brain", !brainEnabled)
        },
        { label: t.refresh, onClick: () => void refreshModuleStatus() }
      ]
    },
    {
      id: "memory",
      title: t.memoryModule,
      subtitle: t.memorySub,
      detail: `${memoryCards} ${t.memoryCardsCount}`,
      icon: "memory",
      enabled: memoryEnabled,
      actions: [
        {
          label: memoryEnabled ? t.disable : t.enable,
          onClick: () => void toggleModule("memory", !memoryEnabled)
        },
        { label: t.refresh, onClick: () => void refreshModuleStatus() }
      ]
    }
  ];

  return (
    <main className="app-shell">
      {error ? <div className="quiet-error">{error}</div> : null}
      <section className="window-shell">
        <Topbar
          t={t}
          locale={locale}
          pendingCount={pendingCount}
          mainView={mainView}
          onLocaleChange={setLocale}
          onMainViewChange={setMainView}
        />

        <section className="chat-layout">
          <SideRail
            t={t}
            locale={locale}
            mainView={mainView}
            pendingCount={pendingCount}
            memoryCards={memoryCards}
            onLocaleChange={setLocale}
            onMainViewChange={setMainView}
          />

          <section className="chat-column">
            {mainView === "workspace" ? (
              <ChatWorkspace
                t={t}
                workspace={workspace}
                messages={messages}
                audits={recentAudits}
                latestExecution={latestExecution}
              />
            ) : mainView === "control" ? (
              <ControlCenter
                t={t}
                moduleStatus={{
                  provider_source: providerSource,
                  risk_policy_source: riskPolicySource,
                  pending_approvals: pendingCount,
                  memory_cards: memoryCards,
                  last_brain_route: lastBrainRoute,
                  brain_enabled: brainEnabled,
                  memory_enabled: memoryEnabled
                }}
                modules={modules}
                executors={executors}
                providers={providers}
                browserRuntimes={browserRuntimes}
                patchRunners={patchRunners}
                devModes={devModes}
                audits={recentAudits}
                onReloadProvider={reloadProvider}
              />
            ) : (
              <SecondaryWorkspace
                t={t}
                view={mainView as any}
                recentTasks={recentTasks}
                recentMemory={recentMemory}
                audits={recentAudits}
                modules={modules}
                executors={executors}
                providers={providers}
                browserRuntimes={browserRuntimes}
                patchRunners={patchRunners}
                devModes={devModes}
                automations={automations}
                skills={skills}
                connectors={connectors}
                mcpServers={mcpServers}
                onCreateAutomation={handleCreateAutomation}
                onToggleAutomation={handleToggleAutomation}
                onDeleteAutomation={handleDeleteAutomation}
              />
            )}

            {mainView === "workspace" && (
              <form className="chat-input" onSubmit={sendMessage}>
                <textarea
                  value={message}
                  onChange={(event) => setMessage(event.target.value)}
                  placeholder={t.inputPlaceholder}
                  rows={3}
                />
                <button type="submit" disabled={loading}>
                  {loading ? t.sending : t.send}
                </button>
              </form>
            )}
          </section>
        </section>
      </section>
    </main>
  );
}

