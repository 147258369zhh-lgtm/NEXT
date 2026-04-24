import { Icon } from "./Icon";
import type {
  AuditView,
  BrowserRuntimeDescriptor,
  CopyBundle,
  ExecutorDescriptor,
  ModuleDescriptor,
  ModuleStatus,
  PatchRunnerDescriptor,
  ProviderDescriptor
} from "../types";

export function ControlCenter({
  t,
  moduleStatus,
  modules,
  executors,
  providers,
  browserRuntimes,
  patchRunners,
  audits
}: {
  t: CopyBundle;
  moduleStatus: ModuleStatus;
  modules: ModuleDescriptor[];
  executors: ExecutorDescriptor[];
  providers: ProviderDescriptor[];
  browserRuntimes: BrowserRuntimeDescriptor[];
  patchRunners: PatchRunnerDescriptor[];
  audits: AuditView[];
}) {
  const browserAudits = audits.filter((audit) =>
    audit.event_type.startsWith("browser.")
  );
  const devAudits = audits.filter((audit) => audit.event_type.startsWith("dev."));
  const patchRunnerAudits = audits.filter((audit) => audit.event_type === "dev.runner");
  const latestPatchRunnerLogAudit = audits.find(
    (audit) => audit.event_type === "dev.runner_log"
  );
  const patchRunnerStatus = extractPatchRunnerStatus(latestPatchRunnerLogAudit?.result);

  return (
    <div className="chat-stage">
      <div className="chat-stage-header">
        <div className="chat-stage-copy">
          <span className="eyebrow">
            <Icon name="modules" /> {t.controlTab}
          </span>
          <h1>{t.controlTitle}</h1>
          <p>{t.controlDesc}</p>
        </div>
      </div>

      <section className="control-board">
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlRuntime}</h2>
          </div>
          <div className="control-grid">
            <div className="control-stat">
              <span>{t.providerModule}</span>
              <strong>{moduleStatus.provider_source}</strong>
            </div>
            <div className="control-stat">
              <span>{t.riskPolicy}</span>
              <strong>{moduleStatus.risk_policy_source}</strong>
            </div>
            <div className="control-stat">
              <span>{t.brainKernel}</span>
              <strong>{moduleStatus.brain_enabled ? t.enabled : t.disabled}</strong>
            </div>
            <div className="control-stat">
              <span>{t.memoryModule}</span>
              <strong>{moduleStatus.memory_enabled ? t.enabled : t.disabled}</strong>
            </div>
          </div>
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlModuleInventory}</h2>
          </div>
          <div className="module-inventory">
            {modules.map((module) => (
              <div className="inventory-item" key={module.id}>
                <div className="inventory-copy">
                  <strong>{module.title}</strong>
                  <span>{module.hot_swappable ? t.hotSwappable : t.nativeSettings}</span>
                </div>
                <span className={`module-state ${module.enabled ? "on" : "off"}`}>
                  {module.enabled ? t.enabled : t.disabled}
                </span>
              </div>
            ))}
          </div>
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlExecutors}</h2>
          </div>
          <div className="module-inventory">
            {executors.map((executor) => (
              <div className="inventory-item" key={executor.id}>
                <div className="inventory-copy">
                  <strong>{executor.title}</strong>
                  <span>{executor.route_scope.join(" / ")}</span>
                </div>
                <span className={`module-state ${executor.enabled ? "on" : "off"}`}>
                  {executor.enabled ? t.enabled : t.disabled}
                </span>
              </div>
            ))}
          </div>
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlProviders}</h2>
          </div>
          <div className="module-inventory">
            {providers.map((provider) => (
              <div className="inventory-item" key={provider.id}>
                <div className="inventory-copy">
                  <strong>{provider.title}</strong>
                  <span>
                    {provider.vendor} / {provider.family} /{" "}
                    {provider.local_first ? "local-first" : "cloud-ready"}
                  </span>
                </div>
                <span className={`module-state ${provider.enabled ? "on" : "off"}`}>
                  {provider.enabled ? t.enabled : t.disabled}
                </span>
              </div>
            ))}
          </div>
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlBrowserRuntimes}</h2>
          </div>
          <div className="module-inventory">
            {browserRuntimes.map((runtime) => (
              <div className="inventory-item" key={runtime.id}>
                <div className="inventory-copy">
                  <strong>{runtime.title}</strong>
                  <span>
                    {runtime.engine} /{" "}
                    {runtime.headless_default ? "headless-default" : "interactive-default"} /{" "}
                    {runtime.supports_live_control ? "live-control" : "no-live-control"}
                  </span>
                </div>
                <span className={`module-state ${runtime.enabled ? "on" : "off"}`}>
                  {runtime.enabled ? t.enabled : t.disabled}
                </span>
              </div>
            ))}
          </div>
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlPatchRunners}</h2>
          </div>
          <div className="module-inventory">
            {patchRunners.map((runner) => (
              <div className="inventory-item" key={runner.id}>
                <div className="inventory-copy">
                  <strong>{runner.title}</strong>
                  <span>
                    {runner.mode} / {runner.mutates_files ? "mutates-files" : "dry-run"}
                  </span>
                </div>
                <span className={`module-state ${runner.enabled ? "on" : "off"}`}>
                  {runner.enabled ? t.enabled : t.disabled}
                </span>
              </div>
            ))}
          </div>
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlPatchRunnerStatus}</h2>
          </div>
          {patchRunnerStatus.length > 0 ? (
            <div className="workspace-actions">
              {patchRunnerStatus.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <div className="control-placeholder">
              <strong>{t.noDataTitle}</strong>
              <p>{t.controlPlaceholder}</p>
            </div>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlPatchRunnerActivity}</h2>
          </div>
          <div className="audit-list">
            {patchRunnerAudits.length === 0 ? (
              <div className="control-placeholder">
                <strong>{t.noDataTitle}</strong>
                <p>{t.controlPlaceholder}</p>
              </div>
            ) : (
              patchRunnerAudits.map((audit) => (
                <div className="audit-item" key={audit.id}>
                  <div className="audit-copy">
                    <strong>{audit.event_type}</strong>
                    <span>{audit.tool_name ?? "patch-runner"}</span>
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

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlConnectors}</h2>
          </div>
          <div className="control-placeholder">
            <strong>WeChat Connector</strong>
            <p>{t.controlPlaceholder}</p>
          </div>
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlVoice}</h2>
          </div>
          <div className="control-placeholder">
            <strong>Push-to-talk runtime</strong>
            <p>{t.controlPlaceholder}</p>
          </div>
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlBrowserActivity}</h2>
          </div>
          <div className="audit-list">
            {browserAudits.length === 0 ? (
              <div className="control-placeholder">
                <strong>{t.noDataTitle}</strong>
                <p>{t.controlPlaceholder}</p>
              </div>
            ) : (
              browserAudits.map((audit) => (
                <div className="audit-item browser-audit" key={audit.id}>
                  <div className="audit-copy">
                    <strong>{audit.event_type}</strong>
                    <span>{audit.tool_name ?? "browser-executor"}</span>
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

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.controlDevActivity}</h2>
          </div>
          <div className="audit-list">
            {devAudits.length === 0 ? (
              <div className="control-placeholder">
                <strong>{t.noDataTitle}</strong>
                <p>{t.controlPlaceholder}</p>
              </div>
            ) : (
              devAudits.map((audit) => (
                <div className="audit-item" key={audit.id}>
                  <div className="audit-copy">
                    <strong>{audit.event_type}</strong>
                    <span>{audit.tool_name ?? "dev-executor"}</span>
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

function extractPatchRunnerStatus(raw?: string): string[] {
  if (!raw) {
    return [];
  }

  try {
    const parsed = JSON.parse(raw) as {
      runner_id?: string;
      mode?: string;
      log_entries?: string[];
    };

    const items = [
      parsed.runner_id ? `runner: ${parsed.runner_id}` : null,
      parsed.mode ? `mode: ${parsed.mode}` : null,
      Array.isArray(parsed.log_entries)
        ? `log entries: ${parsed.log_entries.length}`
        : null,
      Array.isArray(parsed.log_entries) && parsed.log_entries.length > 0
        ? `latest: ${parsed.log_entries[parsed.log_entries.length - 1]}`
        : null
    ];

    return items.filter((item): item is string => Boolean(item));
  } catch {
    return [];
  }
}
