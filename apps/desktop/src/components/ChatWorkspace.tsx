import type {
  AuditView,
  ChatMessage,
  CopyBundle,
  StructuredExecutionResult,
  TaskWorkspace
} from "../types";
import { EmptyState } from "./EmptyState";
import { Icon } from "./Icon";

export function ChatWorkspace({
  t,
  workspace,
  messages,
  audits,
  latestExecution
}: {
  t: CopyBundle;
  workspace: TaskWorkspace | null;
  messages: ChatMessage[];
  audits: AuditView[];
  latestExecution: StructuredExecutionResult | null;
}) {
  const latestAudit = audits[0];
  const hasConversation = messages.length > 0;

  return (
    <div className="codex-workspace">
      <section className="codex-hero">
        <span className="eyebrow">
          <Icon name="spark" /> {t.workspaceLabel}
        </span>
        <h1>{hasConversation ? t.workspaceTab : t.heroTitle}</h1>
        <p>{t.heroDesc}</p>
      </section>

      {workspace || latestExecution ? (
        <section className="task-capsule">
          <div>
            <span className="capsule-label">{t.currentTask}</span>
            <strong>{workspace?.task.title ?? latestExecution?.executor_id}</strong>
            <p>
              {latestExecution?.summary || workspace?.task.result_summary || t.noTaskOutcome}
            </p>
          </div>
          <div className="capsule-meta">
            {workspace ? <span>{workspace.task.status}</span> : null}
            {workspace ? <span>{workspace.task.risk_level}</span> : null}
            {latestExecution ? <span>{latestExecution.executor_id}</span> : null}
          </div>
        </section>
      ) : null}

      <section className="message-stream">
        {messages.length === 0 ? (
          <EmptyState title={t.noMessagesTitle} desc={t.noMessagesDesc} />
        ) : (
          messages.map((item) => (
            <article className={`message-row ${item.role}`} key={item.id}>
              <div className="message-role">{formatRole(item.role, t)}</div>
              <div className="message-bubble">
                <p>{item.text}</p>
                {item.meta ? <span>{item.meta}</span> : null}
              </div>
            </article>
          ))
        )}
      </section>

      <section className="quiet-details">
        {latestExecution?.artifacts.length ? (
          <details>
            <summary>{t.artifacts}</summary>
            <div className="detail-list">
              {latestExecution.artifacts.slice(0, 4).map((artifact) => (
                <div key={`${artifact.kind}-${artifact.title}`}>
                  <strong>{artifact.title}</strong>
                  <span>{artifact.kind}</span>
                </div>
              ))}
            </div>
          </details>
        ) : null}

        {latestExecution?.follow_up_suggestions.length ? (
          <details>
            <summary>{t.followUpSuggestions}</summary>
            <div className="detail-list">
              {latestExecution.follow_up_suggestions.slice(0, 4).map((suggestion) => (
                <div key={suggestion}>
                  <strong>{suggestion}</strong>
                </div>
              ))}
            </div>
          </details>
        ) : null}

        {latestAudit ? (
          <details>
            <summary>{t.latestActivity}</summary>
            <div className="detail-list">
              <div>
                <strong>{latestAudit.event_type}</strong>
                <span>{latestAudit.result}</span>
              </div>
            </div>
          </details>
        ) : null}
      </section>
    </div>
  );
}

function formatRole(role: ChatMessage["role"], t: CopyBundle): string {
  if (role === "user") return t.userRole;
  if (role === "assistant") return t.assistantRole;
  return t.systemRole;
}
