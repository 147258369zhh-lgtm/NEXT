import type {
  AuditView,
  ChatMessage,
  CopyBundle,
  TaskWorkspace
} from "../types";
import { EmptyState } from "./EmptyState";
import { Icon } from "./Icon";

export function ChatWorkspace({
  t,
  workspace,
  messages,
  audits
}: {
  t: CopyBundle;
  workspace: TaskWorkspace | null;
  messages: ChatMessage[];
  audits: AuditView[];
}) {
  const activityAudits = audits
    .filter(
      (audit) =>
        audit.event_type.startsWith("browser.") ||
        audit.event_type.startsWith("dev.")
    )
    .slice(0, 6);
  const latestExtractionAudit = audits.find(
    (audit) => audit.event_type === "browser.extracted"
  );
  const latestDevPlannedAudit = audits.find(
    (audit) => audit.event_type === "dev.planned"
  );
  const latestDevSchemaAudit = audits.find(
    (audit) => audit.event_type === "dev.patch_schema"
  );
  const latestDevVerifiedAudit = audits.find(
    (audit) => audit.event_type === "dev.verified"
  );
  const actionPhase = extractActionPhase(latestExtractionAudit?.result);
  const fieldPlan = extractTaggedList(
    latestExtractionAudit?.result,
    "fields",
    "no field plan"
  );
  const missingFields = extractTaggedList(
    latestExtractionAudit?.result,
    "missing",
    "no missing fields"
  );
  const sensitiveFields = extractTaggedList(
    latestExtractionAudit?.result,
    "sensitive",
    "no sensitive fields"
  );
  const nextActions = extractTaggedList(
    latestExtractionAudit?.result,
    "next",
    "no recommended next actions"
  );
  const fileTargets = extractTaggedList(latestDevPlannedAudit?.result, "file_targets");
  const moduleTargets = extractTaggedList(
    latestDevPlannedAudit?.result,
    "module_targets"
  );
  const executionMode = extractTaggedValue(
    latestDevPlannedAudit?.result,
    "execution_mode"
  );
  const patchSchema = extractTaggedValue(
    latestDevPlannedAudit?.result,
    "patch_schema"
  );
  const patchSchemaPreview = extractPatchSchemaPreview(latestDevSchemaAudit?.result);
  const repoScope = extractTaggedValue(latestDevPlannedAudit?.result, "repo_scope");
  const patchStrategy = extractTaggedValue(
    latestDevPlannedAudit?.result,
    "patch_strategy"
  );
  const operationSteps = extractTaggedList(
    latestDevPlannedAudit?.result,
    "operation_steps"
  );
  const patchTargets = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_targets"
  );
  const changePlan = extractTaggedList(
    latestDevPlannedAudit?.result,
    "change_plan"
  );
  const patchOutline = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_outline"
  );
  const patchProposal = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_proposal"
  );
  const patchFiles = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_files"
  );
  const patchApplyPlan = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_apply_plan"
  );
  const patchExecutionContract = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_execution_contract"
  );
  const patchExecutionRequest = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_execution_request"
  );
  const patchItems = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_items"
  );
  const patchHunks = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_hunks"
  );
  const patchSets = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_sets"
  );
  const patchContract = extractTaggedList(
    latestDevPlannedAudit?.result,
    "patch_contract"
  );
  const artifacts = extractTaggedList(latestDevPlannedAudit?.result, "artifacts");
  const verificationTargets = extractTaggedList(
    latestDevVerifiedAudit?.result,
    "verification_targets"
  );

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
              {workspace.task.result_summary ? (
                <p className="workspace-summary">{workspace.task.result_summary}</p>
              ) : null}
            </div>
          ) : (
            <span className="workspace-empty">{t.noTaskWorkspace}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.taskOutcome}</h2>
          </div>
          {workspace?.task.result_summary ? (
            <div className="workspace-result">
              <p>{workspace.task.result_summary}</p>
            </div>
          ) : (
            <span className="workspace-empty">{t.noTaskOutcome}</span>
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

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.actionPhase}</h2>
          </div>
          {actionPhase ? (
            <div className="workspace-actions">
              <div className="action-item">
                <strong>{actionPhase}</strong>
              </div>
            </div>
          ) : (
            <span className="workspace-empty">{t.noActionPhase}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.fieldPlan}</h2>
          </div>
          {fieldPlan.length > 0 ? (
            <div className="workspace-actions">
              {fieldPlan.map((field) => (
                <div key={field} className="action-item">
                  <strong>{field}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noFieldPlan}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.missingFields}</h2>
          </div>
          {missingFields.length > 0 ? (
            <div className="workspace-actions">
              {missingFields.map((field) => (
                <div key={field} className="action-item">
                  <strong>{field}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noMissingFields}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.sensitiveFields}</h2>
          </div>
          {sensitiveFields.length > 0 ? (
            <div className="workspace-actions">
              {sensitiveFields.map((field) => (
                <div key={field} className="action-item">
                  <strong>{field}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noSensitiveFields}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.nextActions}</h2>
          </div>
          {nextActions.length > 0 ? (
            <div className="workspace-actions">
              {nextActions.map((action) => (
                <div key={action} className="action-item">
                  <strong>{action}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noNextActions}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.fileTargets}</h2>
          </div>
          {fileTargets.length > 0 ? (
            <div className="workspace-actions">
              {fileTargets.map((target) => (
                <div key={target} className="action-item">
                  <strong>{target}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noFileTargets}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.moduleTargets}</h2>
          </div>
          {moduleTargets.length > 0 ? (
            <div className="workspace-actions">
              {moduleTargets.map((target) => (
                <div key={target} className="action-item">
                  <strong>{target}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noModuleTargets}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.executionMode}</h2>
          </div>
          {executionMode ? (
            <div className="workspace-actions">
              <div className="action-item">
                <strong>{executionMode}</strong>
              </div>
            </div>
          ) : (
            <span className="workspace-empty">{t.noExecutionMode}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchSchema}</h2>
          </div>
          {patchSchema ? (
            <div className="workspace-actions">
              <div className="action-item">
                <strong>{patchSchema}</strong>
              </div>
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchSchema}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchSchemaPreview}</h2>
          </div>
          {patchSchemaPreview.length > 0 ? (
            <div className="workspace-actions">
              {patchSchemaPreview.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchSchemaPreview}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.repoScope}</h2>
          </div>
          {repoScope ? (
            <div className="workspace-actions">
              <div className="action-item">
                <strong>{repoScope}</strong>
              </div>
            </div>
          ) : (
            <span className="workspace-empty">{t.noRepoScope}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchStrategy}</h2>
          </div>
          {patchStrategy ? (
            <div className="workspace-actions">
              <div className="action-item">
                <strong>{patchStrategy}</strong>
              </div>
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchStrategy}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.operationSteps}</h2>
          </div>
          {operationSteps.length > 0 ? (
            <div className="workspace-actions">
              {operationSteps.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noOperationSteps}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchTargets}</h2>
          </div>
          {patchTargets.length > 0 ? (
            <div className="workspace-actions">
              {patchTargets.map((target) => (
                <div key={target} className="action-item">
                  <strong>{target}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchTargets}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.changePlan}</h2>
          </div>
          {changePlan.length > 0 ? (
            <div className="workspace-actions">
              {changePlan.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noChangePlan}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchOutline}</h2>
          </div>
          {patchOutline.length > 0 ? (
            <div className="workspace-actions">
              {patchOutline.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchOutline}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchProposal}</h2>
          </div>
          {patchProposal.length > 0 ? (
            <div className="workspace-actions">
              {patchProposal.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchProposal}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchFiles}</h2>
          </div>
          {patchFiles.length > 0 ? (
            <div className="workspace-actions">
              {patchFiles.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchFiles}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchApplyPlan}</h2>
          </div>
          {patchApplyPlan.length > 0 ? (
            <div className="workspace-actions">
              {patchApplyPlan.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchApplyPlan}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchExecutionContract}</h2>
          </div>
          {patchExecutionContract.length > 0 ? (
            <div className="workspace-actions">
              {patchExecutionContract.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchExecutionContract}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchExecutionRequest}</h2>
          </div>
          {patchExecutionRequest.length > 0 ? (
            <div className="workspace-actions">
              {patchExecutionRequest.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchExecutionRequest}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchItems}</h2>
          </div>
          {patchItems.length > 0 ? (
            <div className="workspace-actions">
              {patchItems.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchItems}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchHunks}</h2>
          </div>
          {patchHunks.length > 0 ? (
            <div className="workspace-actions">
              {patchHunks.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchHunks}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchSets}</h2>
          </div>
          {patchSets.length > 0 ? (
            <div className="workspace-actions">
              {patchSets.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchSets}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.patchContract}</h2>
          </div>
          {patchContract.length > 0 ? (
            <div className="workspace-actions">
              {patchContract.map((item) => (
                <div key={item} className="action-item">
                  <strong>{item}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noPatchContract}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.artifacts}</h2>
          </div>
          {artifacts.length > 0 ? (
            <div className="workspace-actions">
              {artifacts.map((artifact) => (
                <div key={artifact} className="action-item">
                  <strong>{artifact}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noArtifacts}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.verificationTargets}</h2>
          </div>
          {verificationTargets.length > 0 ? (
            <div className="workspace-actions">
              {verificationTargets.map((target) => (
                <div key={target} className="action-item">
                  <strong>{target}</strong>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noVerificationTargets}</span>
          )}
        </article>

        <article className="workspace-card">
          <div className="panel-header">
            <h2>{t.latestActivity}</h2>
          </div>
          {activityAudits.length > 0 ? (
            <div className="workspace-activity">
              {activityAudits.map((audit) => (
                <div key={audit.id} className="activity-item">
                  <strong>{audit.event_type}</strong>
                  <span>{new Date(audit.timestamp).toLocaleString()}</span>
                  <p>{audit.result}</p>
                </div>
              ))}
            </div>
          ) : (
            <span className="workspace-empty">{t.noActivity}</span>
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

function extractActionPhase(raw?: string): string | null {
  if (!raw) {
    return null;
  }

  const match = raw.match(/(?:^|;\s*)phase=([^;]+)/);
  return match?.[1]?.trim() || null;
}

function extractTaggedList(
  raw: string | undefined,
  tag: string,
  emptyValue?: string
): string[] {
  if (!raw) {
    return [];
  }

  const match = raw.match(new RegExp(`(?:^|;\\s*)${tag}=([^;]+)`));
  if (!match?.[1]) {
    return [];
  }

  const value = match[1].trim();
  if (!value || (emptyValue && value === emptyValue)) {
    return [];
  }

  return value
    .split(" | ")
    .map((item) => item.trim())
    .filter(Boolean);
}

function extractTaggedValue(raw: string | undefined, tag: string): string | null {
  if (!raw) {
    return null;
  }

  const match = raw.match(new RegExp(`(?:^|;\\s*)${tag}=([^;]+)`));
  return match?.[1]?.trim() || null;
}

function extractPatchSchemaPreview(raw?: string): string[] {
  if (!raw) {
    return [];
  }

  try {
    const parsed = JSON.parse(raw) as {
      schema_version?: string;
      execution_mode?: string;
      repo_scope?: string;
      patch_strategy?: string;
      file_targets?: string[];
      module_targets?: string[];
      patch_files?: unknown[];
      patch_apply_plan?: unknown[];
      execution_contract?: {
        write_scope?: string;
        dry_run_first?: boolean;
        approval_required?: boolean;
        rollback_scope?: string;
      };
      execution_request?: {
        mode?: string;
        selected_batches?: string[];
        target_paths?: string[];
        verification_scope?: string;
      };
      patch_items?: unknown[];
      patch_hunks?: unknown[];
      patch_sets?: unknown[];
    };

    const preview = [
      parsed.schema_version ? `version: ${parsed.schema_version}` : null,
      parsed.execution_mode ? `mode: ${parsed.execution_mode}` : null,
      parsed.repo_scope ? `scope: ${parsed.repo_scope}` : null,
      parsed.patch_strategy ? `strategy: ${parsed.patch_strategy}` : null,
      parsed.file_targets?.length
        ? `files: ${parsed.file_targets.join(", ")}`
        : null,
      parsed.module_targets?.length
        ? `modules: ${parsed.module_targets.join(", ")}`
        : null,
      Array.isArray(parsed.patch_files)
        ? `patch files: ${parsed.patch_files.length}`
        : null,
      Array.isArray(parsed.patch_apply_plan)
        ? `apply steps: ${parsed.patch_apply_plan.length}`
        : null,
      parsed.execution_contract?.write_scope
        ? `write scope: ${parsed.execution_contract.write_scope}`
        : null,
      typeof parsed.execution_contract?.approval_required === "boolean"
        ? `approval required: ${parsed.execution_contract.approval_required}`
        : null,
      parsed.execution_request?.mode
        ? `request mode: ${parsed.execution_request.mode}`
        : null,
      parsed.execution_request?.selected_batches?.length
        ? `selected batches: ${parsed.execution_request.selected_batches.length}`
        : null,
      Array.isArray(parsed.patch_items)
        ? `patch items: ${parsed.patch_items.length}`
        : null,
      Array.isArray(parsed.patch_hunks)
        ? `patch hunks: ${parsed.patch_hunks.length}`
        : null,
      Array.isArray(parsed.patch_sets)
        ? `patch sets: ${parsed.patch_sets.length}`
        : null
    ];

    return preview.filter((item): item is string => Boolean(item));
  } catch {
    return [];
  }
}
