import type {
  ChatMessage,
  CopyBundle,
  TaskWorkspace
} from "../types";
import { EmptyState } from "./EmptyState";
import { Icon } from "./Icon";

export function ChatWorkspace({
  t,
  workspace,
  messages
}: {
  t: CopyBundle;
  workspace: TaskWorkspace | null;
  messages: ChatMessage[];
}) {
  return (
    <div className="chat-stage">
      <div className="chat-stage-header">
        <div className="chat-stage-copy">
          <span className="eyebrow">
            <Icon name="spark" /> {t.workspaceLabel}
          </span>
          <h1>{t.heroTitle}</h1>
          <p>{t.heroDesc}</p>
        </div>
      </div>

      <section className="workspace-board">
        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.currentTask}</h2>
          </div>
          {workspace ? (
            <div className="workspace-task">
              <strong>{workspace.task.title}</strong>
              <span>
                {t.status}: {workspace.task.status}
              </span>
              <span>
                {t.riskLevel}: {workspace.task.risk_level}
              </span>
            </div>
          ) : (
            <span className="workspace-empty">{t.noTaskWorkspace}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.taskPlan}</h2>
          </div>
          {workspace && workspace.steps.length > 0 ? (
            <div className="step-list">
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
          ) : (
            <span className="workspace-empty">{t.noTaskWorkspace}</span>
          )}
        </article>
      </section>

      <div className="chat-scroll">
        {messages.length === 0 ? (
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
