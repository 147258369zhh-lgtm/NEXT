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
- long-lived development targets: `docs/development-targets.md`
- integration-first architecture: `docs/integration-first-architecture.md`
- third-party research roadmap: `docs/third-party-research-roadmap.md`
- light-surface/heavy-runtime UI principle: `docs/ui-light-surface-heavy-runtime.md`
- reuse and borrow plan: `docs/reuse-plan.md`

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

Current structural direction:

- UI is expected to change significantly later
- core runtime should remain outside the UI layer
- executor and provider families should be pluggable by default
- provider families are being expanded beyond chat toward STT/TTS/realtime-style interfaces

Browser runtime slot:

- default env: `NEXUS_BROWSER_RUNTIME=scaffold`
- Playwright-capable slot: `NEXUS_BROWSER_RUNTIME=playwright-cli`
- external command bridge:
  - `NEXUS_BROWSER_CLI_COMMAND=...`
  - `NEXUS_BROWSER_CLI_ARGS=["..."]`
- repository-local bridge worker:
  - script: `infra/scripts/browser-bridge.mjs`
  - helper command: `npm run browser:bridge -- --spec-json "{...}"`
  - Windows-safe alternative: set `NEXUS_BROWSER_SPEC_JSON` and then run `npm run browser:bridge`
  - example configuration is already reflected in `infra/configs/app.example.json`
  - optional real browser path: set `NEXUS_BROWSER_BRIDGE_MODE=playwright`

Playwright bridge status:

- the repository now includes the `playwright` package
- the browser bridge worker can run in:
  - scaffold fallback mode
  - real Playwright mode when `NEXUS_BROWSER_BRIDGE_MODE=playwright`

