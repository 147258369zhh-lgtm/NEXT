# Architecture Notes

This scaffold follows the project document's first-stage priorities:

- Windows-first desktop shell with Tauri + React + TypeScript
- Local-first state with SQLite
- Core task, audit, provider, and protocol crates
- Brain routing and memory persistence as isolated modules
- Dedicated execution runtime crate for orchestration
- Minimal text conversation loop before voice, gateway, and connector work

Cross-session target document:

- see `docs/development-targets.md` for durable product direction and architectural constraints that should survive machine changes and UI rewrites

## Current flow

`UI input -> Tauri command -> nexus-exec runtime -> brain route -> task service -> risk policy -> (approval or provider) -> memory context -> memory save -> audit -> SQLite -> UI response`

## Module boundaries

- `nexus-task`: task creation, risk classification policy, approval generation
- risk policy supports runtime replacement via config reload
- `nexus-store`: persistence-only API (task/audit/approval CRUD), no orchestration
- `nexus-provider`: pluggable provider interface (`mock` or `openai-compatible`)
- `nexus-audit`: structured audit event builders
- `nexus-brain`: input intent routing and fallback routing decisions
- `nexus-memory`: memory card extraction from each completed turn
- `nexus-exec`: runtime orchestration, execution dispatch, approval continuation, module toggles, provider prompt assembly
- `nexus-exec` now owns executor registration and dispatch inventory
- `nexus-browser`: browser runtime boundary; current scaffold will later be replaced by a Playwright-backed implementation
- `nexus-dev`: dedicated development-task runtime boundary; this is the landing zone for patch-first coding workflows and later repo-task loops
- `apps/desktop/src-tauri`: shell adapter only (`submit_chat`, `resolve_approval`, module toggle/reload commands)

This keeps module boundaries explicit and supports hot-swapping provider, risk strategy, brain routing, and memory persistence with minimal impact. It also makes future UI rewrites much cheaper because the desktop shell no longer owns the core task execution path.

Additional hard requirement:

- anything with likely multiple future implementations should move toward pluggable interfaces early, especially provider families, executors, connectors, and strategies

Browser runtime direction:

- browser tasks should be parsed into structured specs before execution
- runtime should carry execution mode (`silent` / `observe` / later `takeover`)
- browser runtime itself should be selectable (`scaffold`, later `playwright-cli` and beyond)
- playwright-capable runtimes should enter through an external command bridge first, then evolve toward a richer native integration later if needed
- Playwright integration should plug into `nexus-browser`, not into Tauri or the executor registry directly
- the repository-local bridge worker is now the default place to validate browser automation before deeper runtime changes

Dev runtime direction:

- code-oriented tasks should route into a dedicated development runtime, not stay embedded in provider prompts
- `nexus-dev` should move toward Aider-style patch-first output and OpenHands-style repo task loops
- IDE integration should remain an adapter concern and should not collapse the task/runtime boundary

## Next recommended steps

- Replace the mock provider with OpenAI-compatible streaming support
- Add memory retrieval ranking beyond the current recent-card injection
- Register the first non-provider executor inside `nexus-exec` so browser/dev/voice/gateway runtimes can plug in without touching Tauri
- Introduce the control center views described in the design document
