import type {
  AuditView,
  BrowserRuntimeDescriptor,
  CopyBundle,
  ExecutorDescriptor,
  ModuleDescriptor,
  ModuleStatus,
  PatchRunnerDescriptor,
  ProviderDescriptor,
  Skill,
  ConnectorStatus,
  McpServerDescriptor,
  MemoryCard
} from "../types";

/**
 * ControlCenter — Backend settings dashboard.
 * 
 * This is where ALL configuration and monitoring lives.
 * Organized into clear sections:
 * 1. Runtime Controls (provider, risk, brain, memory toggles)
 * 2. System Inventory (modules, executors, providers, runtimes, runners)
 * 3. Activity Logs (audits)
 */
export function ControlCenter({
  t,
  moduleStatus,
  modules,
  executors,
  providers,
  browserRuntimes,
  patchRunners,
  skills,
  connectors,
  mcpServers,
  memoryCards,
  audits,
  onReloadRiskPolicy,
  onReloadProvider,
  onToggleModule,
  loading
}: {
  t: CopyBundle;
  moduleStatus: ModuleStatus;
  modules: ModuleDescriptor[];
  executors: ExecutorDescriptor[];
  providers: ProviderDescriptor[];
  browserRuntimes: BrowserRuntimeDescriptor[];
  patchRunners: PatchRunnerDescriptor[];
  skills: Skill[];
  connectors: ConnectorStatus[];
  mcpServers: McpServerDescriptor[];
  memoryCards: MemoryCard[];
  audits: AuditView[];
  onReloadRiskPolicy?: (level?: string) => void;
  onReloadProvider?: (mode: "mock" | "openai") => void;
  onToggleModule?: (module: "brain" | "memory", enabled: boolean) => void;
  loading?: boolean;
}) {
  return (
    <div className="chat-stage">
      <section className="control-board">
        {/* ---- Runtime Controls ---- */}
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlRuntime}</h2>
          </div>
          <div className="control-grid">
            <div className="control-stat">
              <span>{t.providerModule}</span>
              <strong>{moduleStatus.provider_source}</strong>
              <div className="approval-actions" style={{ marginTop: 6 }}>
                <button type="button" disabled={loading} onClick={() => onReloadProvider?.("mock")}>{t.useMock}</button>
                <button type="button" disabled={loading} onClick={() => onReloadProvider?.("openai")}>{t.useOpenai}</button>
              </div>
            </div>
            <div className="control-stat">
              <span>{t.riskPolicy}</span>
              <strong>{moduleStatus.risk_policy_source}</strong>
              <div className="approval-actions" style={{ marginTop: 6 }}>
                <button type="button" disabled={loading} onClick={() => onReloadRiskPolicy?.("low")}>Low</button>
                <button type="button" disabled={loading} onClick={() => onReloadRiskPolicy?.("medium")}>Mid</button>
                <button type="button" disabled={loading} onClick={() => onReloadRiskPolicy?.("high")}>High</button>
              </div>
            </div>
            <div className="control-stat">
              <span>{t.brainKernel}</span>
              <strong>{moduleStatus.brain_enabled ? t.enabled : t.disabled}</strong>
              <div className="approval-actions" style={{ marginTop: 6 }}>
                <button type="button" disabled={loading} onClick={() => onToggleModule?.("brain", !moduleStatus.brain_enabled)}>
                  {moduleStatus.brain_enabled ? t.disable : t.enable}
                </button>
              </div>
            </div>
            <div className="control-stat">
              <span>{t.memoryModule}</span>
              <strong>{moduleStatus.memory_enabled ? t.enabled : t.disabled}</strong>
              <div className="approval-actions" style={{ marginTop: 6 }}>
                <button type="button" disabled={loading} onClick={() => onToggleModule?.("memory", !moduleStatus.memory_enabled)}>
                  {moduleStatus.memory_enabled ? t.disable : t.enable}
                </button>
              </div>
            </div>
          </div>
        </article>

        {/* ---- Modules ---- */}
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlModuleInventory}</h2>
          </div>
          <div className="module-inventory">
            {modules.map((mod) => (
              <div className="inventory-item" key={mod.id}>
                <div className="inventory-copy">
                  <strong>{mod.title}</strong>
                  <span>{mod.hot_swappable ? t.hotSwappable : t.nativeSettings}</span>
                </div>
                <span className={`module-state ${mod.enabled ? "on" : "off"}`}>
                  {mod.enabled ? t.enabled : t.disabled}
                </span>
              </div>
            ))}
          </div>
        </article>

        {/* ---- Executors ---- */}
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlExecutors}</h2>
          </div>
          <div className="module-inventory">
            {executors.map((exec) => (
              <div className="inventory-item" key={exec.id}>
                <div className="inventory-copy">
                  <strong>{exec.title}</strong>
                  <span>{exec.route_scope.join(" / ")}</span>
                </div>
                <span className={`module-state ${exec.enabled ? "on" : "off"}`}>
                  {exec.enabled ? t.enabled : t.disabled}
                </span>
              </div>
            ))}
          </div>
        </article>

        {/* ---- Providers ---- */}
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlProviders}</h2>
          </div>
          <div className="module-inventory">
            {providers.map((p) => (
              <div className="inventory-item" key={p.id}>
                <div className="inventory-copy">
                  <strong>{p.title}</strong>
                  <span>{p.vendor} / {p.family}</span>
                </div>
                <span className={`module-state ${p.enabled ? "on" : "off"}`}>
                  {p.enabled ? t.enabled : t.disabled}
                </span>
              </div>
            ))}
          </div>
        </article>

        {/* ---- Browser Runtimes ---- */}
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlBrowserRuntimes}</h2>
          </div>
          <div className="module-inventory">
            {browserRuntimes.map((rt) => (
              <div className="inventory-item" key={rt.id}>
                <div className="inventory-copy">
                  <strong>{rt.title}</strong>
                  <span>{rt.engine}</span>
                </div>
                <span className={`module-state ${rt.enabled ? "on" : "off"}`}>
                  {rt.enabled ? t.enabled : t.disabled}
                </span>
              </div>
            ))}
          </div>
        </article>

        {/* ---- Patch Runners ---- */}
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlPatchRunners}</h2>
          </div>
          <div className="module-inventory">
            {patchRunners.map((runner) => (
              <div className="inventory-item" key={runner.id}>
                <div className="inventory-copy">
                  <strong>{runner.title}</strong>
                  <span>{runner.mode}</span>
                </div>
                <span className={`module-state ${runner.enabled ? "on" : "off"}`}>
                  {runner.enabled ? t.enabled : t.disabled}
                </span>
              </div>
            ))}
          </div>
        </article>

        {/* ---- Skills ---- */}
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlSkills}</h2>
          </div>
          <div className="module-inventory">
            {skills.length === 0 ? (
              <div className="control-placeholder">
                <strong>No Skills Found</strong>
                <p>Add .skill files to the /skills directory</p>
              </div>
            ) : (
              skills.map((skill) => (
                <div className="inventory-item" key={skill.id}>
                  <div className="inventory-copy">
                    <strong>{skill.metadata.name}</strong>
                    <span>v{skill.metadata.version} {skill.metadata.author ? `by ${skill.metadata.author}` : ""}</span>
                  </div>
                  <span className="module-state on" style={{ background: 'var(--accent-soft)', color: 'var(--accent)' }}>
                    READY
                  </span>
                </div>
              ))
            )}
          </div>
        </article>

        {/* ---- External Connectors (借鉴 OpenClaw) ---- */}
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlConnectors}</h2>
          </div>
          <div className="module-inventory">
            {connectors.length === 0 ? (
              <div className="control-placeholder">
                <strong>No Connectors Active</strong>
                <p>Register external webhooks or chat bridges</p>
              </div>
            ) : (
              connectors.map((c) => (
                <div className="inventory-item" key={c.id}>
                  <div className="inventory-copy">
                    <strong>{c.name}</strong>
                    <span>Port: {c.port} {c.last_activity ? `· active ${c.last_activity}` : ""}</span>
                  </div>
                  <span className={`module-state ${c.status === 'online' ? 'on' : 'off'}`}>
                    {c.status.toUpperCase()}
                  </span>
                </div>
              ))
            )}
          </div>
        </article>

        {/* ---- MCP Tool Catalog (对齐 MCP Spec) ---- */}
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlMcpTools}</h2>
          </div>
          <div className="module-inventory">
            {mcpServers.length === 0 ? (
              <div className="control-placeholder">
                <strong>No MCP Servers</strong>
                <p>Link standard model context servers</p>
              </div>
            ) : (
              mcpServers.map((s) => (
                <div className="inventory-item" key={s.id} style={{ display: 'block' }}>
                   <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '10px' }}>
                    <strong>{s.name}</strong>
                    <span className="module-state on">LINKED</span>
                   </div>
                   <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
                      <span className="pill-tag" title={s.command}>{s.command} {s.args.join(" ")}</span>
                   </div>
                </div>
              ))
            )}
          </div>
        </article>

        {/* ---- Memory Shards (借鉴 MemGPT) ---- */}
        <article className="workspace-card control-span-full">
          <div className="panel-header">
            <h2>{t.controlMemory}</h2>
          </div>
          <div className="memory-grid">
            {memoryCards.length === 0 ? (
              <div className="control-placeholder">
                <strong>Memory Void</strong>
                <p>Nexus will start forming long-term insights here</p>
              </div>
            ) : (
              memoryCards.map((card) => (
                <div className="memory-shard" key={card.id}>
                  <div className="shard-header">
                    <strong>{card.title}</strong>
                    <span className="shard-importance">IMP {card.importance}</span>
                  </div>
                  <p className="shard-content">{card.content}</p>
                  <div className="shard-footer">
                    <div style={{ display: 'flex', gap: '4px' }}>
                      {card.tags.map(tag => <span key={tag} className="tag">#{tag}</span>)}
                    </div>
                    <span className="shard-date">{new Date(card.created_at).toLocaleDateString()}</span>
                  </div>
                </div>
              ))
            )}
          </div>
        </article>

        {/* ---- Audit Log ---- */}
        <article className="workspace-card control-span-full">
          <div className="panel-header">
            <h2>{t.controlAudit}</h2>
          </div>
          <div className="audit-list">
            {audits.length === 0 ? (
              <div className="control-placeholder">
                <strong>{t.noDataTitle}</strong>
                <p>{t.controlPlaceholder}</p>
              </div>
            ) : (
              audits.map((audit) => (
                <div className="audit-item" key={audit.id}>
                  <div className="audit-copy">
                    <strong>{audit.event_type}</strong>
                    <span>
                      {audit.actor} / {audit.channel}
                      {audit.tool_name ? ` / ${audit.tool_name}` : ""}
                    </span>
                  </div>
                  <div className="audit-meta">
                    <span>{audit.risk_level}</span>
                    <span>{new Date(audit.timestamp).toLocaleString()}</span>
                  </div>
                  <p>{audit.result}</p>
                </div>
              ))
            )}
          </div>
        </article>
      </section>
    </div>
  );
}
