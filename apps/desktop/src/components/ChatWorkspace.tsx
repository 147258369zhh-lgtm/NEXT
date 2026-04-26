import type {
  AuditView,
  ChatMessage,
  CopyBundle,
  TaskWorkspace
} from "../types";
import { EmptyState } from "./EmptyState";
import { Icon } from "./Icon";

/**
 * ChatWorkspace — Codex-style clean conversation view.
 * 
 * Front stage shows ONLY:
 * 1. Welcome header (when no messages)
 * 2. Active task summary (collapsed, one line)
 * 3. Chat messages
 * 
 * All detailed patch/field/schema info is intentionally hidden
 * from the front stage — it belongs in the Control Center.
 */
export function ChatWorkspace({
  t,
  workspace,
  messages,
  audits: _audits
}: {
  t: CopyBundle;
  workspace: TaskWorkspace | null;
  messages: ChatMessage[];
  audits: AuditView[];
}) {
  const hasMessages = messages.length > 0;

  return (
    <div className="chat-stage">
      {/* Welcome header — only when conversation is empty */}
      {!hasMessages && (
        <div className="chat-stage-header">
          <div className="chat-stage-copy">
            <span className="eyebrow">
              <Icon name="spark" /> {t.workspaceLabel}
            </span>
            <h1>{t.heroTitle}</h1>
            <p>{t.heroDesc}</p>
          </div>
        </div>
      )}

      {/* Active task — compact single-line summary */}
      {workspace && (
        <div className="workspace-board">
          <article className="workspace-card">
            <div className="workspace-task">
              <strong>{workspace.task.title}</strong>
              <span>
                {workspace.task.status} · {workspace.task.risk_level}
                {workspace.task.result_summary ? ` · ${workspace.task.result_summary}` : ""}
              </span>
            </div>
            {workspace.steps.length > 0 && (
              <div className="step-list" style={{ marginTop: 12 }}>
                {workspace.steps.map((step) => (
                  <div key={step.id} className={`step-item ${step.status}`}>
                    <span className="step-index">{step.position + 1}</span>
                    <div className="step-copy">
                      <strong>{step.title}</strong>
                      <span>{step.detail}</span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </article>
        </div>
      )}

      {/* Chat messages */}
      <div className="chat-scroll">
        {!hasMessages ? (
          <EmptyState
            title={t.noMessagesTitle}
            desc={t.noMessagesDesc}
            large
          />
        ) : (
          messages.map((item) => (
            <article key={item.id} className={`chat-bubble ${item.role}`}>
              {item.meta ? <span className="bubble-meta">{item.meta}</span> : null}
              <p>{item.text}</p>
            </article>
          ))
        )}
      </div>
    </div>
  );
}
