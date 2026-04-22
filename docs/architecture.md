# Architecture Notes

This scaffold follows the project document's first-stage priorities:

- Windows-first desktop shell with Tauri + React + TypeScript
- Local-first state with SQLite
- Core task, audit, provider, and protocol crates
- Brain routing and memory persistence as isolated modules
- Minimal text conversation loop before voice, gateway, and connector work

## Current flow

`UI input -> Tauri command -> brain route -> executor dispatch -> task service -> risk policy -> (approval or provider) -> memory context -> memory save -> audit -> SQLite -> UI response`

## Module boundaries

- `nexus-task`: task creation, risk classification policy, approval generation
- risk policy supports runtime replacement via config reload
- `nexus-store`: persistence-only API (task/audit/approval CRUD), no orchestration
- `nexus-provider`: pluggable provider interface (`mock` or `openai-compatible`)
- `nexus-audit`: structured audit event builders
- `nexus-brain`: input intent routing and fallback routing decisions
- `nexus-memory`: memory card extraction from each completed turn
- `apps/desktop/src-tauri`: orchestration and executor dispatch only (`submit_chat`, `resolve_approval`, module toggle/reload commands)

This keeps module boundaries explicit and supports hot-swapping provider, risk strategy, brain routing, and memory persistence with minimal impact.

## Next recommended steps

- Replace the mock provider with OpenAI-compatible streaming support
- Add memory retrieval ranking beyond the current recent-card injection
- Move executor dispatch into a dedicated runtime crate for cleaner plugin registration
- Introduce the control center views described in the design document
