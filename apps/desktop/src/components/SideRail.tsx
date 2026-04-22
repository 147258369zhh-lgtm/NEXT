import { EmptyState } from "./EmptyState";
import { Icon } from "./Icon";
import type {
  ApprovalView,
  CopyBundle,
  Locale,
  MemoryView,
  ModuleCardData,
  SideView,
  TaskView
} from "../types";

export function SideRail({
  t,
  locale,
  sideView,
  loading,
  moduleCards,
  pendingApprovals,
  recentTasks,
  recentApprovals,
  recentMemory,
  onSideViewChange,
  onRefreshApprovals,
  onRefreshHistory,
  onApproval
}: {
  t: CopyBundle;
  locale: Locale;
  sideView: SideView;
  loading: boolean;
  moduleCards: ModuleCardData[];
  pendingApprovals: ApprovalView[];
  recentTasks: TaskView[];
  recentApprovals: ApprovalView[];
  recentMemory: MemoryView[];
  onSideViewChange: (view: SideView) => void;
  onRefreshApprovals: () => void;
  onRefreshHistory: () => void;
  onApproval: (approvalId: string, approved: boolean) => void;
}) {
  return (
    <aside className="side-rail">
      <div className="side-tabs">
        <button
          className={sideView === "modules" ? "active" : ""}
          onClick={() => onSideViewChange("modules")}
          type="button"
        >
          <Icon name="modules" /> {t.modules}
        </button>
        <button
          className={sideView === "history" ? "active" : ""}
          onClick={() => onSideViewChange("history")}
          type="button"
        >
          <Icon name="history" /> {t.history}
        </button>
      </div>

      {sideView === "modules" ? (
        <section className="rail-panel">
          <div className="module-grid">
            {moduleCards.map((module) => (
              <section className="module-card" key={module.id}>
                <div className="module-head">
                  <h3>
                    <Icon name={module.icon} /> {module.title}
                  </h3>
                  {typeof module.enabled === "boolean" ? (
                    <span className={`module-state ${module.enabled ? "on" : "off"}`}>
                      {module.enabled ? t.enabled : t.disabled}
                    </span>
                  ) : null}
                </div>
                <p className="module-subtitle">{module.subtitle}</p>
                <p className="module-detail">{module.detail}</p>
                <div className="approval-actions">
                  {module.actions.map((action) => (
                    <button
                      key={`${module.id}-${action.label}`}
                      type="button"
                      onClick={action.onClick}
                      className={action.kind === "danger" ? "reject" : ""}
                      disabled={loading}
                    >
                      {action.label}
                    </button>
                  ))}
                </div>
              </section>
            ))}
          </div>

          <div className="rail-block rail-emphasis">
            <div className="panel-header">
              <h2>{t.pendingItems}</h2>
              <button
                type="button"
                onClick={onRefreshApprovals}
                disabled={loading}
              >
                {t.refresh}
              </button>
            </div>
            {pendingApprovals.length === 0 ? (
              <EmptyState title={t.noDataTitle} desc={t.noPending} />
            ) : (
              <div className="approval-list">
                {pendingApprovals.map((approval) => (
                  <article key={approval.id} className="approval-item">
                    <strong>{approval.reason}</strong>
                    <p>{approval.payload}</p>
                    <span>
                      {t.expiresAt}:{" "}
                      {new Date(approval.expires_at).toLocaleString(locale)}
                    </span>
                    <div className="approval-actions">
                      <button
                        type="button"
                        onClick={() => onApproval(approval.id, true)}
                        disabled={loading}
                      >
                        {t.approve}
                      </button>
                      <button
                        type="button"
                        className="reject"
                        onClick={() => onApproval(approval.id, false)}
                        disabled={loading}
                      >
                        {t.reject}
                      </button>
                    </div>
                  </article>
                ))}
              </div>
            )}
          </div>
        </section>
      ) : (
        <section className="rail-panel">
          <div className="rail-block">
            <div className="panel-header">
              <h2>{t.recentTasks}</h2>
              <button
                type="button"
                onClick={onRefreshHistory}
                disabled={loading}
              >
                {t.refresh}
              </button>
            </div>
            {recentTasks.length === 0 ? (
              <EmptyState title={t.noDataTitle} desc={t.noTaskHistory} />
            ) : (
              <div className="approval-list">
                {recentTasks.map((task) => (
                  <article key={task.id} className="approval-item">
                    <strong>{task.title}</strong>
                    <span>
                      {t.status}: {task.status}
                    </span>
                    <span>
                      {t.riskLevel}: {task.risk_level}
                    </span>
                    <span>
                      {t.createdAt}:{" "}
                      {task.created_at
                        ? new Date(task.created_at).toLocaleString(locale)
                        : "N/A"}
                    </span>
                  </article>
                ))}
              </div>
            )}
          </div>

          <div className="rail-block">
            <div className="panel-header">
              <h2>{t.recentApprovals}</h2>
            </div>
            {recentApprovals.length === 0 ? (
              <EmptyState title={t.noDataTitle} desc={t.noApprovalHistory} />
            ) : (
              <div className="approval-list">
                {recentApprovals.map((approval) => (
                  <article key={approval.id} className="approval-item">
                    <strong>{approval.reason}</strong>
                    <span>
                      {t.status}: {approval.status}
                    </span>
                    <span>
                      {t.expiresAt}:{" "}
                      {new Date(approval.expires_at).toLocaleString(locale)}
                    </span>
                  </article>
                ))}
              </div>
            )}
          </div>

          <div className="rail-block">
            <div className="panel-header">
              <h2>{t.recentMemory}</h2>
            </div>
            {recentMemory.length === 0 ? (
              <EmptyState title={t.noDataTitle} desc={t.noMemoryHistory} />
            ) : (
              <div className="approval-list">
                {recentMemory.map((card) => (
                  <article key={card.id} className="approval-item">
                    <strong>{card.title}</strong>
                    <span>{card.card_type}</span>
                    <span>{card.tags.join(" / ")}</span>
                    <span>
                      {t.createdAt}:{" "}
                      {new Date(card.created_at).toLocaleString(locale)}
                    </span>
                  </article>
                ))}
              </div>
            )}
          </div>
        </section>
      )}
    </aside>
  );
}
