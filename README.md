# Nexus

Nexus is a Windows-first desktop agent hub inspired by the attached design document. This scaffold implements the first-stage monorepo, core Rust crates, and a minimal text conversation loop through Tauri.

## Workspace

```text
NEXT/
  apps/desktop
  crates/
    nexus-audit
    nexus-brain
    nexus-memory
    nexus-protocol
    nexus-provider
    nexus-store
    nexus-task
  packages/
    shared-types
    skill-schema
  infra/
    sql
    configs
  docs/
```

## Run

1. `npm install`
2. `cargo check`
3. `npm run tauri --workspace @nexus/desktop -- dev`

The desktop app uses local SQLite by default with a mock provider.

Progress tracking documents:

- architecture notes: `docs/architecture.md`
- design progress backup: `docs/design-progress-backup.md`

To switch to an OpenAI-compatible provider:

1. Set `NEXUS_PROVIDER_MODE=openai`
2. Set `OPENAI_API_KEY=...`
3. Optionally set `OPENAI_BASE_URL` and `NEXUS_CHAT_MODEL`

Risk policy scaffold:

- `L4/L5` tasks do not execute immediately
- they create a pending approval record
- task status is set to `awaiting_approval`
- desktop shell exposes approval commands:
  - `list_pending_approvals`
  - `resolve_approval`

Risk policy is now hot-pluggable:

- default file: `infra/configs/risk-policy.json`
- env override: `NEXUS_RISK_POLICY_FILE`
- runtime reload command: `reload_risk_policy`

Provider is also hot-pluggable at runtime:

- inspect current provider via `get_provider_source`
- switch provider via `reload_provider` with mode (`mock` or `openai`)
- inspect overall module health via `get_module_status`

Brain and memory modules are now runtime-pluggable:

- list memory cards via `list_recent_memory_cards`
- toggle modules via `set_module_enabled` (`brain`/`memory`, true/false)
- inspect module toggles via `get_module_status`
- executor dispatch now routes `chat` / `task_execution` / `approval_decision`
- recent memory is injected into provider prompts when the memory module is enabled
