# Development Targets

Last updated: 2026-04-24

This document records long-lived engineering goals and cross-session requirements that must continue to apply even when development moves to another machine.

It is not a progress log.  
It is the durable target and constraint document for the repository.

## 1. Product direction

Nexus is a Windows-first native personal agent host.

The product direction is:

- Windows native first
- local-first state and execution
- long-running desktop host
- embedded gateway
- WeChat as the first external channel
- real-time PC voice interaction as a core capability
- strong audit, approval, and controllability
- high compatibility with useful OpenClaw / Hermes ideas

The project should be treated as:

> a Windows-native enhanced fusion of OpenClaw channel/gateway ideas and Hermes memory/growth/model ideas, with stronger local execution and stricter control boundaries.

## 2. Stable architectural principles

These principles are not optional and should guide all future implementation work.

### 2.1 UI is replaceable, core runtime is not

- The desktop UI will likely be redesigned later.
- UI and layout should not own business logic.
- Core execution, task orchestration, approvals, memory, providers, executors, and gateway logic must live outside the UI layer.
- Future UI rewrites should mostly affect `apps/desktop/src` and a thin shell adapter, not backend modules.

### 2.2 Thin shell, thick core

- `apps/desktop/src-tauri` should remain a shell adapter.
- Runtime orchestration belongs in dedicated crates, not in Tauri command files.
- The correct direction is `desktop shell -> runtime -> modules`.

### 2.3 Modular by default

- New capability work should prefer new modules or clear extension points.
- Avoid coupling task, provider, memory, connector, and UI logic together.
- Any feature likely to grow independently should get an explicit boundary early.

### 2.4 Task-first, not agent-theater

- The main runtime object is the `Task`.
- The user should see tasks and results, not uncontrolled agent spawning.
- The system may have:
  - a few resident agents
  - optional short-lived workers
  - many executors
- execution should remain traceable through task state and audit logs

## 3. Agent / worker / executor strategy

The preferred strategy is:

- few resident agents
- optional temporary workers for complex tasks
- executor-driven implementation as the default

### 3.1 Resident agents

Allowed long-lived roles:

- main brain
- side brain
- voice session manager
- gateway session manager

### 3.2 Temporary workers

Workers may exist for bounded complex tasks, but they must:

- belong to one task
- have constrained context
- have constrained tools
- disappear after completion

### 3.3 Executors are the default workhorses

Concrete work should usually be implemented through executors, for example:

- browser executor
- dev executor
- script executor
- connector executor

This keeps the system more stable and easier to audit than a fully agent-per-task design.

Executor descriptors should remain rich enough to support embedded third-party capabilities. At minimum they should expose family, route scope, task kinds, risk ceiling, integration level, input/output schemas, dry-run support, rollback support, approval requirements, and enabled state. Execution requests and results should also remain structured, carrying executor id, task kind, risk level, artifacts, audit references, memory candidates, and follow-up suggestions. The UI may present a simplified view, but the runtime descriptor and structured execution result are the long-term contracts.

Patch runners inside 
exus-dev should also stay registry-driven. The default runner may be a safe dry-run scaffold, while reserved embedded-agent runners define the adapter boundary for future Cline/Roo/OpenCode/Aider-style code agents without forcing those implementations into the runtime prematurely.

## 4. Pluggability is a hard requirement

Anything that may reasonably have multiple implementations should be designed as pluggable first, not rewritten later.

This is a core long-term requirement.

### 4.1 Provider families must be pluggable

The provider layer should support family-based pluggability:

- chat provider
- reasoning provider
- STT provider
- TTS provider
- realtime session provider
- embedding provider

The system must bind to interfaces and descriptors, not to one vendor SDK.

### 4.2 Runtime components must be pluggable

The following should be designed for registration/replacement:

- executors
- connectors
- browser runtimes
- voice engines
- wake-word engines

For browser work specifically:

- repository-local bridge workers are acceptable as an intermediate step
- browser runtime contracts should remain stable even if the underlying Playwright bridge changes
- command-bridge prototypes should not leak transport details into task orchestration

### 4.3 Strategies must be pluggable

The following should support replacement:

- risk policy
- routing strategy
- memory retrieval strategy
- approval policy
- skill match strategy

### 4.4 Import and compatibility layers should be pluggable

Compatibility work should prefer importers/adapters rather than hardcoding external ecosystems directly into the core.

### 4.5 Patch planning should converge toward file-level contracts

- Development execution should keep moving from loose patch summaries toward file-level patch contracts.
- Schema evolution is expected, but changes should be versioned instead of silently reshaping old data.
- Patch plans should clearly separate:
  - target files
  - mutation boundaries
  - verification targets
  - execution batches
- Structured patch schema audits should remain machine-readable so future patch engines, workers, or IDE bridges can reuse them directly.

## 5. Voice requirements

PC real-time voice interaction is a 1.0 core requirement.

The agreed direction is:

- push-to-talk first
- keyword wake later
- short spoken reply, fuller text response
- local-first voice stack in the first version
- cloud-capable interfaces preserved from the start

Voice architecture should be split, not hardcoded:

- capture
- VAD
- STT
- TTS
- voice runtime / session management

## 6. Channel and gateway requirements

### 6.1 Gateway

- gateway is embedded in the Windows host for 1.0
- it is not the authority source
- Windows host remains the single authority state source

### 6.2 WeChat first

- WeChat is the first formal channel in 1.0
- Feishu comes later
- channel logic should go through connector abstractions and the embedded gateway

### 6.3 Connector direction

Connector behavior should not be implemented directly in UI or task modules.

The preferred flow is:

`channel connector -> gateway/runtime boundary -> task/orchestration path`

## 7. UI direction

The UI direction should follow the Codex-like principle already discussed:

- minimal foreground workspace
- deep configuration in control center
- workspace style layout instead of a pure chatbox
- system-management surfaces in the control center

The current UI is acknowledged to be temporary and likely to be redesigned later.

That is expected.  
The engineering goal is to keep that future redesign cheap.

## 8. Compatibility direction

The project should avoid rebuilding wheels where useful ecosystem practices already exist.

Preferred compatibility goals:

- useful OpenClaw connector and gateway ideas
- useful Hermes memory and growth ideas
- `SKILL.md`-style compatibility where reasonable
- import or adapter approaches preferred over full lock-in

The rule is:

- high compatibility
- low duplication
- do not let external ecosystems dictate core architecture

### 8.1 Reuse-first rule

When a mature external project already solves one layer well, Nexus should prefer:

- direct protocol or bridge reuse where safe
- runtime boundary compatibility where useful
- architecture borrowing instead of product copying

Concrete priority references are tracked in `docs/reuse-plan.md`.

Current highest-value reuse targets are:

- `playwright-mcp`
- `browser-use`
- `Aider`
- `OpenHands`
- `Continue`
- `Windows-Use`
- `Langfuse`
- `E2B`

## 9. Current engineering priority order

The current practical order remains:

1. keep runtime boundaries clean
2. keep UI thin and replaceable
3. add executor registration and real executors
4. add local voice runtime foundation
5. add embedded gateway
6. add WeChat connector
7. add skill compatibility and growth recording

## 10. Rule for future work

Before implementing a new feature, the preferred question is:

> should this be a pluggable interface, a runtime component, a strategy, or just a UI surface?

If the answer suggests more than one likely implementation over time, it should not be hardcoded.




## Dev Mode Catalog Note

Dev modes are now a first-class catalog in `nexus-dev`. Future code-agent borrowing should map into these modes first, then into patch runners. Mode permissions should eventually be enforced by the runtime, not only displayed by the UI.
