import { useMemo, useState } from "react";
import type {
  AuditView,
  AutomationView,
  BrowserRuntimeDescriptor,
  CopyBundle,
  DevModeDescriptor,
  ExecutorDescriptor,
  MemoryView,
  ModuleDescriptor,
  PatchRunnerDescriptor,
  ProviderDescriptor,
  TaskView,
  Skill,
  ConnectorStatus,
  McpServerDescriptor
} from "../types";

type SecondaryView = "search" | "skills" | "plugins" | "automation" | "projects";
type InfoItem = { title: string; meta: string; detail?: string; tags?: string[] };

export function SecondaryWorkspace({
  t,
  view,
  recentTasks,
  recentMemory,
  audits,
  modules,
  executors,
  providers,
  browserRuntimes,
  patchRunners,
  devModes,
  automations,
  skills = [],
  connectors = [],
  mcpServers = [],
  onCreateAutomation,
  onToggleAutomation,
  onDeleteAutomation
}: {
  t: CopyBundle;
  view: SecondaryView;
  recentTasks: TaskView[];
  recentMemory: MemoryView[];
  audits: AuditView[];
  modules: ModuleDescriptor[];
  executors: ExecutorDescriptor[];
  providers: ProviderDescriptor[];
  browserRuntimes: BrowserRuntimeDescriptor[];
  patchRunners: PatchRunnerDescriptor[];
  devModes: DevModeDescriptor[];
  automations: AutomationView[];
  skills?: Skill[];
  connectors?: ConnectorStatus[];
  mcpServers?: McpServerDescriptor[];
  onCreateAutomation: (title: string, description: string) => void;
  onToggleAutomation: (automationId: string, enabled: boolean) => void;
  onDeleteAutomation: (automationId: string) => void;
}) {
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState<InfoItem | null>(null);
  const [automationDraft, setAutomationDraft] = useState(t.automationDraftDefault);

  function saveAutomationDraft(title: string) {
    setAutomationDraft(title);
    onCreateAutomation(title, t.automationDraftDesc);
  }

  const searchItems = useMemo(() => {
    const normalizedQuery = query.trim().toLowerCase();
    const items: InfoItem[] = [
      ...recentTasks.map((task) => ({
        title: task.title,
        meta: `${task.status} · ${task.risk_level}`,
        detail: task.result_summary ?? task.created_at ?? t.noTaskOutcome,
        tags: [t.recentTasks, task.status, task.risk_level]
      })),
      ...recentMemory.map((card) => ({
        title: card.title,
        meta: card.tags.join(" · ") || card.card_type,
        detail: `${card.card_type} · ${card.created_at}`,
        tags: [t.recentMemory, card.card_type, ...card.tags]
      })),
      ...audits.map((audit) => ({
        title: audit.event_type,
        meta: `${audit.actor} · ${audit.result}`,
        detail: `${audit.channel} · ${audit.risk_level} · ${audit.timestamp}`,
        tags: [t.latestActivity, audit.actor, audit.channel, audit.risk_level]
      }))
    ];

    if (!normalizedQuery) return items;
    return items.filter((item) => `${item.title} ${item.meta} ${item.detail ?? ""} ${item.tags?.join(" ") ?? ""}`.toLowerCase().includes(normalizedQuery));
  }, [audits, query, recentMemory, recentTasks, t]);

  if (view === "search") {
    return (
      <Shell title={t.searchTitle} desc={t.searchDesc} selected={selected} onClearSelected={() => setSelected(null)}>
        <div className="secondary-searchbox">
          <span>{t.navSearch}</span>
          <input placeholder={t.searchPlaceholder} value={query} onChange={(event) => setQuery(event.target.value)} />
        </div>
        <section className="quick-filter-row">
          <button type="button" onClick={() => setQuery("")}>{t.allFilter}</button>
          <button type="button" onClick={() => setQuery(t.recentTasks)}>{t.recentTasks}</button>
          <button type="button" onClick={() => setQuery(t.recentMemory)}>{t.recentMemory}</button>
          <button type="button" onClick={() => setQuery(t.latestActivity)}>{t.latestActivity}</button>
        </section>
        <section className="secondary-summary-row">
          <SummaryPill label={t.searchResultCount} value={searchItems.length} />
          <SummaryPill label={t.recentTasks} value={recentTasks.length} />
          <SummaryPill label={t.recentMemory} value={recentMemory.length} />
          <SummaryPill label={t.latestActivity} value={audits.length} />
        </section>
        <CardGrid>
          <InfoCard title={t.searchResults} empty={t.noSearchResults} items={searchItems} onSelect={setSelected} />
          <InfoCard title={t.recentTasks} empty={t.noTaskHistory} items={recentTasks.map((task) => ({ title: task.title, meta: `${task.status} · ${task.risk_level}`, detail: task.result_summary ?? t.noTaskOutcome }))} onSelect={setSelected} />
          <InfoCard title={t.recentMemory} empty={t.noMemoryHistory} items={recentMemory.map((card) => ({ title: card.title, meta: card.tags.join(" · ") || card.card_type, detail: card.created_at }))} onSelect={setSelected} />
        </CardGrid>
      </Shell>
    );
  }

  if (view === "skills") {
    const modeItems = devModes.map((mode) => ({ title: mode.title, meta: `${mode.intent} · ${mode.default_runner}`, detail: `${t.operationSteps}: ${mode.allowed_tool_groups.join(" · ") || t.noOperationSteps}` }));
    const executorItems = executors.map((executor) => ({ title: executor.title, meta: `${executor.family} · ${executor.integration_level}`, detail: `${t.riskLevel}: ${executor.risk_ceiling} · ${executor.summary}` }));
    const skillItems = skills.map((skill) => ({
      title: skill.metadata.name,
      meta: `${skill.metadata.version || "1.0.0"} · ${skill.risk_level}`,
      detail: `${skill.metadata.description || "No description"} \n\nTriggers: ${(skill.triggers || []).join(", ")} \nActions: ${(skill.actions || []).join(", ")}`,
      tags: [skill.risk_level, skill.execution_mode, ...(skill.triggers || [])]
    }));
    return (
      <Shell title={t.skillsTitle} desc={t.skillsDesc} selected={selected} onClearSelected={() => setSelected(null)}>
        <section className="secondary-summary-row">
          <SummaryPill label={t.navSkills} value={skills.length} />
          <SummaryPill label={t.controlDevModes} value={devModes.length} />
          <SummaryPill label={t.controlExecutors} value={executors.length} />
          <SummaryPill label={t.modules} value={modules.length} />
        </section>
        <CardGrid>
          <InfoCard title={t.navSkills} empty={t.noDataTitle} items={skillItems} onSelect={setSelected} />
          <InfoCard title={t.controlDevModes} empty={t.noDataTitle} items={modeItems} onSelect={setSelected} />
          <InfoCard title={t.controlExecutors} empty={t.noDataTitle} items={executorItems} onSelect={setSelected} />
          <InfoCard title={t.brainKernel} empty={t.noDataTitle} items={modules.map((module) => ({ title: module.title, meta: module.enabled ? t.enabled : t.disabled, detail: module.hot_swappable ? t.hotSwappable : t.nativeSettings }))} onSelect={setSelected} />
        </CardGrid>
      </Shell>
    );
  }

  if (view === "plugins") {
    const connectorItems = connectors.map((connector) => ({
      title: connector.name,
      meta: `${connector.status.toUpperCase()} · Port ${connector.port}`,
      detail: `Connector ID: ${connector.id}\nStatus: ${connector.status}\nPort: ${connector.port}\nLast Activity: ${connector.last_activity || "None"}`,
      tags: [connector.status, `Port: ${connector.port}`]
    }));

    const mcpItems = mcpServers.map((server) => ({
      title: server.name,
      meta: server.enabled !== false ? t.enabled : t.disabled,
      detail: `Server ID: ${server.id}\nCommand: ${server.command} ${server.args.join(" ")}`,
      tags: [server.enabled !== false ? "enabled" : "disabled", "MCP"]
    }));

    return (
      <Shell title={t.pluginsTitle} desc={t.pluginsDesc} selected={selected} onClearSelected={() => setSelected(null)}>
        <section className="secondary-summary-row">
          <SummaryPill label={t.controlConnectors} value={connectors.length} />
          <SummaryPill label="MCP 服务" value={mcpServers.length} />
          <SummaryPill label={t.controlProviders} value={providers.length} />
          <SummaryPill label={t.controlBrowserRuntimes} value={browserRuntimes.length} />
          <SummaryPill label={t.controlPatchRunners} value={patchRunners.length} />
        </section>
        <CardGrid>
          <InfoCard title={t.controlConnectors} empty={t.noDataTitle} items={connectorItems} onSelect={setSelected} />
          <InfoCard title="MCP 插件服务 (MCP Servers)" empty={t.noDataTitle} items={mcpItems} onSelect={setSelected} />
          <InfoCard title={t.controlProviders} empty={t.noDataTitle} items={providers.map((provider) => ({ title: provider.title, meta: `${provider.vendor} · ${provider.family}`, detail: provider.local_first ? t.localRuntime : t.providerLive }))} onSelect={setSelected} />
          <InfoCard title={t.controlBrowserRuntimes} empty={t.noDataTitle} items={browserRuntimes.map((runtime) => ({ title: runtime.title, meta: `${runtime.engine} · ${runtime.supports_live_control ? t.enabled : t.disabled}`, detail: runtime.headless_default ? t.browserHeadless : t.browserPreview }))} onSelect={setSelected} />
          <InfoCard title={t.controlPatchRunners} empty={t.noDataTitle} items={patchRunners.map((runner) => ({ title: runner.title, meta: `${runner.source} · ${runner.review_status}`, detail: `${runner.repository} · ${runner.license}` }))} onSelect={setSelected} />
        </CardGrid>
      </Shell>
    );
  }

  if (view === "automation") {
    const dryRunItems = patchRunners.filter((runner) => runner.supports_dry_run).map((runner) => ({ title: runner.title, meta: t.autoCode, detail: runner.repository }));
    return (
      <Shell title={t.automationTitle} desc={t.automationDesc} selected={selected} onClearSelected={() => setSelected(null)}>
        <section className="automation-builder">
          <div>
            <span>{t.automationDraft}</span>
            <strong>{automationDraft}</strong>
            <p>{t.automationDraftDesc}</p>
          </div>
          <div className="automation-actions">
            <button type="button" onClick={() => saveAutomationDraft(t.automationVerifyDraft)}>{t.createVerificationAutomation}</button>
            <button type="button" onClick={() => saveAutomationDraft(t.automationMemoryDraft)}>{t.createMemoryAutomation}</button>
          </div>
        </section>
        <AutomationList t={t} items={automations} onToggle={onToggleAutomation} onDelete={onDeleteAutomation} />
        <CardGrid>
          <InfoCard title={t.automationQueue} empty={t.automationEmpty} items={audits.filter((audit) => audit.channel === "automation").map((audit) => ({ title: audit.event_type, meta: audit.result, detail: audit.timestamp }))} onSelect={setSelected} />
          <InfoCard title={t.verificationTargets} empty={t.noVerificationTargets} items={dryRunItems} onSelect={setSelected} />
          <InfoCard title={t.approvalQueue} empty={t.noPending} items={recentTasks.filter((task) => task.status === "pending_approval").map((task) => ({ title: task.title, meta: task.risk_level, detail: task.result_summary ?? t.noTaskOutcome }))} onSelect={setSelected} />
        </CardGrid>
      </Shell>
    );
  }

  const projectStats = [
    { label: t.recentTasks, value: recentTasks.length },
    { label: t.navSkills, value: skills.length },
    { label: t.controlConnectors, value: connectors.length },
    { label: "MCP 服务", value: mcpServers.length }
  ];

  return (
    <Shell title={t.projectsTitle} desc={t.projectsDesc} selected={selected} onClearSelected={() => setSelected(null)}>
      <section className="project-hero-card">
        <span>{t.projectSection}</span>
        <strong>NEXT</strong>
        <p>{t.projectNextDesc}</p>
        <div className="project-stat-row">
          {projectStats.map((stat) => <SummaryPill key={stat.label} label={stat.label} value={stat.value} />)}
        </div>
        <div className="project-action-row">
          <button type="button" onClick={() => setSelected({ title: "NEXT", meta: t.projectsTitle, detail: t.projectNextDesc, tags: [t.controlExecutors, t.controlProviders, t.controlAudit] })}>{t.openProjectSummary}</button>
          <button type="button" onClick={() => setSelected({ title: t.fileTargets, meta: t.projectsTitle, detail: executors.flatMap((executor) => executor.task_kinds).join(" ? ") || t.noFileTargets })}>{t.inspectProjectTargets}</button>
        </div>
      </section>
      <CardGrid>
        <InfoCard title={t.recentTasks} empty={t.noTaskHistory} items={recentTasks.map((task) => ({ title: task.title, meta: `${task.status} · ${task.created_at ?? ""}`, detail: task.result_summary ?? t.noTaskOutcome }))} onSelect={setSelected} />
        <InfoCard title={t.fileTargets} empty={t.noFileTargets} items={executors.flatMap((executor) => executor.task_kinds.slice(0, 2).map((kind) => ({ title: kind, meta: executor.title, detail: executor.summary })))} onSelect={setSelected} />
        <InfoCard title={t.controlAudit} empty={t.noActivity} items={audits.map((audit) => ({ title: audit.event_type, meta: audit.timestamp, detail: audit.result }))} onSelect={setSelected} />
      </CardGrid>
    </Shell>
  );
}

function Shell({
  title,
  desc,
  selected,
  onClearSelected,
  children
}: {
  title: string;
  desc: string;
  selected: InfoItem | null;
  onClearSelected: () => void;
  children: React.ReactNode;
}) {
  return (
    <div className="secondary-workspace">
      <header className="secondary-header">
        <span>Nexus</span>
        <h1>{title}</h1>
        <p>{desc}</p>
      </header>
      {selected ? <DetailPanel item={selected} onClose={onClearSelected} /> : null}
      {children}
    </div>
  );
}

function CardGrid({ children }: { children: React.ReactNode }) {
  return <section className="secondary-grid">{children}</section>;
}

function SummaryPill({ label, value }: { label: string; value: number | string }) {
  return (
    <div className="summary-pill">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function DetailPanel({ item, onClose }: { item: InfoItem; onClose: () => void }) {
  return (
    <section className="detail-panel">
      <div>
        <span>{item.meta}</span>
        <strong>{item.title}</strong>
        <p>{item.detail}</p>
        {item.tags?.length ? <div className="detail-tags">{item.tags.slice(0, 6).map((tag) => <em key={tag}>{tag}</em>)}</div> : null}
      </div>
      <button type="button" onClick={onClose}>×</button>
    </section>
  );
}

function AutomationList({
  t,
  items,
  onToggle,
  onDelete
}: {
  t: CopyBundle;
  items: AutomationView[];
  onToggle: (id: string, enabled: boolean) => void;
  onDelete: (id: string) => void;
}) {
  return (
    <section className="automation-list">
      <div className="automation-list-head">
        <span>{t.savedAutomations}</span>
        <strong>{items.length}</strong>
      </div>
      {items.length === 0 ? (
        <p>{t.noSavedAutomations}</p>
      ) : (
        <div>
          {items.map((item) => (
            <article key={item.id}>
              <div>
                <strong>{item.title}</strong>
                <span>{item.created_at} · {item.enabled ? t.enabled : t.disabled}</span>
              </div>
              <button type="button" onClick={() => onToggle(item.id, !item.enabled)}>{item.enabled ? t.disable : t.enable}</button>
              <button type="button" onClick={() => onDelete(item.id)}>{t.deleteAction}</button>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}

function InfoCard({
  title,
  empty,
  items,
  onSelect
}: {
  title: string;
  empty: string;
  items: InfoItem[];
  onSelect: (item: InfoItem) => void;
}) {
  const visibleItems = items.slice(0, 8);
  return (
    <article className="secondary-card">
      <h2>{title}</h2>
      {visibleItems.length === 0 ? (
        <p className="secondary-empty">{empty}</p>
      ) : (
        <div className="secondary-list">
          {visibleItems.map((item) => (
            <button type="button" key={`${item.title}-${item.meta}`} onClick={() => onSelect(item)}>
              <strong>{item.title}</strong>
              <span>{item.meta}</span>
            </button>
          ))}
        </div>
      )}
    </article>
  );
}
