import { FormEvent, useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ChatWorkspace } from "./components/ChatWorkspace";
import { ControlCenter } from "./components/ControlCenter";
import { SideRail } from "./components/SideRail";
import { Topbar } from "./components/Topbar";
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
  Skill,
  ConnectorStatus,
  McpServerDescriptor,
  MemoryCard,
  TaskView,
  TaskWorkspace
} from "./types";

export default function App() {
  const [locale, setLocale] = useState<Locale>(getInitialLocale);
  const [mainView, setMainView] = useState<MainView>("workspace");
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
  const [skills, setSkills] = useState<Skill[]>([]);
  const [connectors, setConnectors] = useState<ConnectorStatus[]>([]);
  const [mcpServers, setMcpServers] = useState<McpServerDescriptor[]>([]);
  const [error, setError] = useState<string | null>(null);
  const t = useMemo(() => I18N[locale], [locale]);

  useEffect(() => {
    localStorage.setItem("nexus.locale", locale);
  }, [locale]);

  useEffect(() => {
    void refreshAll();
  }, []);

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
      refreshConnectors(),
      refreshMcpServers()
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
      const [tasks, approvals, memoryCardsList, audits, nextWorkspace] = await Promise.all([
        invoke<TaskView[]>("list_recent_tasks", { limit: 12 }),
        invoke<ApprovalView[]>("list_recent_approvals", { limit: 12 }),
        invoke<MemoryView[]>("list_recent_memory_cards", { limit: 12 }),
        invoke<AuditView[]>("list_recent_audits", { limit: 18 }),
        invoke<TaskWorkspace | null>("get_latest_workspace")
      ]);
      setRecentTasks(tasks);
      setRecentApprovals(approvals);
      setRecentMemory(memoryCardsList);
      setRecentAudits(audits);
      setWorkspace(nextWorkspace);
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
    } catch {
      setError(t.failedModuleStatus);
    }
  }

  async function refreshConnectors() {
    try {
      // 模拟数据，待后端指令补齐
      setConnectors([
        { id: "webhook-1", name: "Standard Webhook", port: 3333, status: "online" }
      ]);
    } catch {
      setError(t.failedModuleStatus);
    }
  }

  async function refreshMcpServers() {
    try {
      setMcpServers(await invoke<McpServerDescriptor[]>("list_mcp_servers"));
    } catch {
      setError(t.failedModuleStatus);
    }
  }

  async function reloadRiskPolicy(level?: string) {
    setLoading(true);
    setError(null);
    try {
      setRiskPolicySource(await invoke<string>("reload_risk_policy", { path: level || null }));
      await refreshModuleStatus();
    } catch {
      setError(t.riskReloadFailed);
    } finally {
      setLoading(false);
    }
  }

  async function reloadProvider(mode: "mock" | "openai") {
    setLoading(true);
    setError(null);
    try {
      setProviderSource(await invoke<string>("reload_provider", { mode }));
      await refreshModuleStatus();
    } catch {
      setError(t.providerReloadFailed);
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

  return (
    <main className="app-shell">
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
            loading={loading}
            pendingApprovals={pendingApprovals}
            recentTasks={recentTasks}
            recentApprovals={recentApprovals}
            recentMemory={recentMemory}
            onRefreshApprovals={() => void refreshApprovals()}
            onRefreshHistory={() => void refreshHistory()}
            onApproval={(approvalId, approved) =>
              void handleApproval(approvalId, approved)
            }
          />

          <section className="chat-column">
            {mainView === "workspace" ? (
              <ChatWorkspace
                t={t}
                workspace={workspace}
                messages={messages}
                audits={recentAudits}
              />
            ) : (
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
                skills={skills}
                connectors={connectors}
                mcpServers={mcpServers}
                memoryCards={recentMemory as any}
                audits={recentAudits}
                onReloadRiskPolicy={(level) => void reloadRiskPolicy(level)}
                onReloadProvider={(mode) => void reloadProvider(mode)}
                onToggleModule={(mod, en) => void toggleModule(mod, en)}
                loading={loading}
              />
            )}

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
              {error ? <p className="error-text">{error}</p> : null}
            </form>
          </section>
        </section>
      </section>
    </main>
  );
}
