import { EmptyState } from "./EmptyState";
import type {
  ApprovalView,
  CopyBundle,
  Locale,
  MemoryView,
  TaskView
} from "../types";

/**
 * SideRail — Minimal context sidebar.
 * 
 * Shows only:
 * 1. Pending approvals (actionable items)
 * 2. Recent task history (collapsed list)
 * 
 * Memory cards and other noise moved to Control Center.
 */
export function SideRail({
  t,
  locale,
  loading,
  pendingApprovals,
  recentTasks,
  recentApprovals: _recentApprovals,
  recentMemory: _recentMemory,
  onRefreshApprovals,
  onRefreshHistory,
  onApproval
}: {
  t: CopyBundle;
  locale: Locale;
  loading: boolean;
  pendingApprovals: ApprovalView[];
  recentTasks: TaskView[];
  recentApprovals: ApprovalView[];
  recentMemory: MemoryView[];
  onRefreshApprovals: () => void;
  onRefreshHistory: () => void;
  onApproval: (approvalId: string, approved: boolean) => void;
}) {
  return (
    <aside className="side-rail">
      <section className="rail-panel">
        {/* Pending Approvals — primary actionable content */}
        {pendingApprovals.length > 0 && (
          <div className="rail-block rail-emphasis">
            <div className="panel-header">
              <h2>{t.pendingItems}</h2>
              <button
                type="button"
                onClick={onRefreshApprovals}
                disabled={loading}
                className="lang-btn"
              >
                {t.refresh}
              </button>
            </div>
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
          </div>
        )}

        {/* Recent Tasks — compact history */}
        <div className="rail-block">
          <div className="panel-header">
            <h2>{t.recentTasks}</h2>
            <button
              type="button"
              onClick={onRefreshHistory}
              disabled={loading}
              className="lang-btn"
            >
              {t.refresh}
            </button>
          </div>
          {recentTasks.length === 0 ? (
            <EmptyState title={t.noDataTitle} desc={t.noTaskHistory} />
          ) : (
            <div className="approval-list">
              {recentTasks.slice(0, 8).map((task) => (
                <article key={task.id} className="approval-item">
                  <strong>{task.title}</strong>
                  <span>
                    {task.status} · {task.risk_level}
                  </span>
                </article>
              ))}
            </div>
          )}
        </div>
      </section>
    </aside>
  );
}
