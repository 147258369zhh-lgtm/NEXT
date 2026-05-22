import { Icon } from "./Icon";
import type { AuditView, CopyBundle, StructuredExecutionResult, TaskWorkspace } from "../types";

export function StatusRail({
  t,
  workspace,
  latestExecution,
  audits,
  pendingCount,
  providerSource,
  riskPolicySource,
  lastBrainRoute,
  brainEnabled,
  memoryEnabled
}: {
  t: CopyBundle;
  workspace: TaskWorkspace | null;
  latestExecution: StructuredExecutionResult | null;
  audits: AuditView[];
  pendingCount: number;
  providerSource: string;
  riskPolicySource: string;
  lastBrainRoute: string;
  brainEnabled: boolean;
  memoryEnabled: boolean;
}) {
  const latestAudit = audits[0];

  return (
    <aside className="status-rail">
      <section className="status-card progress-card">
        <div className="status-card-title">
          <span>{t.statusProgress}</span>
          <strong>{workspace?.task.status ?? latestExecution?.status ?? t.readyStatus}</strong>
        </div>
        <div className="progress-line"><i /></div>
        <p>{workspace?.task.title ?? latestExecution?.summary ?? t.cleanSurfaceHint}</p>
      </section>

      <section className="status-card">
        <h3>{t.statusOutput}</h3>
        <a href="http://127.0.0.1:1450/">127.0.0.1:1450</a>
        <span>{t.localPreview}</span>
      </section>

      <section className="status-card">
        <h3>{t.statusBrowser}</h3>
        <p>Nexus · 127.0.0.1:1450</p>
        <span>{t.browserPreview}</span>
      </section>

      <section className="status-card compact-list">
        <h3>{t.statusRuntime}</h3>
        <div><Icon name="provider" /><span>{t.providerModule}</span><strong>{providerSource}</strong></div>
        <div><Icon name="risk" /><span>{t.riskPolicy}</span><strong>{riskPolicySource}</strong></div>
        <div><Icon name="approval" /><span>{t.pendingItems}</span><strong>{pendingCount}</strong></div>
      </section>

      <section className="status-card compact-list">
        <h3>{t.statusSource}</h3>
        <div><Icon name="brain" /><span>{t.brainKernel}</span><strong>{brainEnabled ? t.enabled : t.disabled}</strong></div>
        <div><Icon name="memory" /><span>{t.memoryModule}</span><strong>{memoryEnabled ? t.enabled : t.disabled}</strong></div>
        <div><Icon name="spark" /><span>{t.lastRoutePrefix}</span><strong>{lastBrainRoute}</strong></div>
      </section>

      <section className="status-card">
        <h3>{t.latestActivity}</h3>
        <p>{latestAudit?.event_type ?? latestExecution?.executor_id ?? t.noActivity}</p>
        <span>{latestAudit?.result ?? latestExecution?.status ?? t.localRuntime}</span>
      </section>
    </aside>
  );
}
