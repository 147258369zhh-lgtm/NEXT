# Design Progress Backup

Last updated: 2026-04-24

Source design document:

- `C:/Users/29136/Documents/xwechat_files/wxid_uu7e90c55kkd22_37cb/msg/file/2026-04/Nexus-最终项目设计文档.md`

This file is the engineering backup for the design document. It records what has already been implemented in the current repository, what is only partially implemented, and what has not started yet.

## 1. Current project judgment

The current repository is no longer an empty scaffold, but it is still far from the `Nexus 1.0` target described in the design document.

Current maturity can be described as:

- `Stage`: early engineering prototype
- `State`: desktop shell + core task loop + basic modular runtime
- `Not yet`: voice, gateway, wechat connector, browser executor, dev executor, skill system, growth system, full control center

In plain terms:

- the project already has a real desktop shell, local storage, task records, approval flow, memory cards, provider switching, and a modular backend split
- but it still does not yet have the main 1.0 execution loops that make Nexus a true long-running personal agent system

## 2. What has been implemented

### 2.1 Monorepo and project skeleton

Implemented:

- `apps/desktop`
- `crates/nexus-audit`
- `crates/nexus-brain`
- `crates/nexus-exec`
- `crates/nexus-memory`
- `crates/nexus-protocol`
- `crates/nexus-provider`
- `crates/nexus-store`
- `crates/nexus-task`
- `packages/shared-types`
- `packages/skill-schema`
- `infra/sql`
- `infra/configs`
- `docs`

Status judgment:

- `Done for current phase`
- the repository layout already follows the design document's modular direction

### 2.2 Windows desktop shell

Implemented:

- Tauri desktop app
- React + TypeScript front end
- local desktop window
- main conversation surface
- left-side module and history rail
- current task workspace card
- task-step plan card
- multilingual UI support for Chinese and English

Status judgment:

- `Partially done`
- shell exists and is usable
- UI quality is still prototype level and will need another major redesign
- tray, shortcut keys, floating surfaces, and true background control center are not implemented

### 2.3 Task system

Implemented:

- task creation from prompt
- task status persistence
- approval-required branching
- basic step-plan generation
- current task and plan display in UI
- approval resolution and cancellation path

Status judgment:

- `Partially done`
- this is now a real task pipeline, not just a chat box
- but it is still a simplified linear task flow, not the full lifecycle/state-machine system described in the design document

### 2.4 Audit and approval chain

Implemented:

- audit record creation
- approval record storage
- pending approval queue
- approve/reject operations from UI
- audit writes around task receipt, routing, provider completion, memory save, module toggles, and approval resolution

Status judgment:

- `Partially done`
- basic chain exists
- still missing richer approval metadata, expiration handling depth, rollback controls, and full irreversible-action guardrails

### 2.5 Brain routing

Implemented:

- `nexus-brain` crate
- simple route decision model
- routing between `chat`, `task_execution`, and `approval_decision`
- last route visible in UI
- runtime enable/disable switch

Status judgment:

- `Partially done`
- routing kernel exists
- not yet a true main-brain / side-brain collaboration system
- no deep escalation logic, no self-check, no guard submodules yet

### 2.6 Memory system

Implemented:

- `nexus-memory` crate
- memory card extraction from completed turns
- memory card persistence in SQLite
- recent memory listing
- recent memory prompt injection before provider call
- runtime enable/disable switch

Status judgment:

- `Partially done`
- memory exists as a useful working feature
- but only the shallow recent-card layer is implemented
- semantic retrieval, weighted ranking, editable memory layers, sensitivity tags, and failure cards are not done

### 2.7 Provider abstraction

Implemented:

- `nexus-provider` crate
- mock provider
- OpenAI-compatible provider path
- runtime provider switching
- provider source status in UI

Status judgment:

- `Partially done`
- provider abstraction is in place
- but it only covers text chat right now
- reasoning provider split, STT, TTS, realtime, local model families, and more vendors are not yet implemented

### 2.8 Risk policy hot swap

Implemented:

- configurable risk policy file
- runtime reload
- high-risk tasks move into approval queue

Status judgment:

- `Partially done`
- useful foundation exists
- but this is still much simpler than the full layered control system in the design document

### 2.9 Modular architecture

Implemented:

- clear crate split on backend
- persistence isolated in `nexus-store`
- audit builders isolated in `nexus-audit`
- provider abstraction isolated in `nexus-provider`
- task logic isolated in `nexus-task`
- runtime orchestration isolated in `nexus-exec`
- brain and memory as independently toggled modules
- UI beginning to split into smaller components instead of a single large file

Status judgment:

- `Done for current phase`
- this is the strongest part of the current implementation
- the repo is already moving in the hot-pluggable direction requested in the design document

### 2.10 Runtime boundary extraction

Implemented:

- new `nexus-exec` crate
- Tauri shell reduced to command adapters and runtime bootstrapping
- task submission, approval continuation, module toggles, provider prompt assembly, and execution dispatch moved out of `src-tauri`
- desktop shell now depends on a stable runtime API instead of owning orchestration directly

Status judgment:

- `Done for current phase`
- this is an important structural step because future UI rewrites should now touch much less backend logic

### 2.11 Executor registration foundation

Implemented:

- executor registry now lives in `nexus-exec`
- runtime dispatch no longer assumes only one execution path
- browser executor skeleton is registered as a real non-provider executor
- executor inventory is visible in the control center
- browser runtime boundary now exists in `nexus-browser`
- browser task parsing and runtime mode scaffolding now exist (`silent` / `observe`)
- browser runtime selection slot now exists (`scaffold` / `playwright-cli`)

Status judgment:

- `Done for current phase`
- this is an enabling step for browser, dev, voice, and connector executors

## 3. What is only partially implemented

These areas have started, but only in a narrow or simplified form.

### 3.1 Control center

Current state:

- the current desktop UI already shows module status, approval queue, history, recent memory, current task, and task plan
- a dedicated control-center view now exists as a separate surface from the main workspace
- control center now reads runtime status, module inventory, connector/voice placeholders, and recent audit events from backend commands

Missing:

- dedicated settings center
- provider management panel
- connector management
- memory inspection tools
- skill inspection tools
- richer audit center
- voice settings

Status judgment:

- `Started, but now structurally aligned`

### 3.2 Main conversation workspace

Current state:

- conversation is already the main surface
- the left/right structure was adjusted to match the requested direction

Missing:

- richer session model
- conversation tabs
- context panel
- result attachments
- better visual hierarchy
- macOS-grade refinement

Status judgment:

- `Started, but still rough`

### 3.3 Task planning and execution workspace

Current state:

- plan generation exists
- steps are visible in UI
- state transitions are persisted

Missing:

- blocked/paused/resume flow
- replay entry
- detailed step execution logs
- executor-specific progress
- task-level operator controls

Status judgment:

- `Foundation only`

## 4. What has not been implemented yet

These are still missing from the current repository.

### 4.1 Voice runtime

Not implemented:

- microphone capture flow
- push-to-talk workflow
- VAD
- STT
- TTS
- streaming voice session
- interruption handling

Design status:

- `Not started`

### 4.2 Gateway layer

Not implemented:

- embedded gateway runtime
- external channel routing
- session mapping
- connector lifecycle management

Design status:

- `Not started`

### 4.3 WeChat connector

Not implemented:

- wechat input path
- image input path from wechat
- result summary pushback
- approval actions from wechat
- alert push

Design status:

- `Not started`

### 4.4 Browser execution loop

Not implemented:

- Playwright-based executor
- login/session handling
- full browser automation tasks

Design status:

- `Skeleton started`

Additional current note:

- repository-local browser bridge worker now exists
- Playwright package is installed in the workspace
- real Playwright navigation path has been validated for a simple open-page flow
- structured extraction path now returns title, resolved url, content snippet, and link sample for simple information tasks

### 4.5 Light development loop

Not implemented:

- repo reading workflow inside product runtime
- patch generation workflow inside product runtime
- test execution loop inside product runtime
- result delivery as product feature

Design status:

- `Not started`

### 4.6 Skill system

Not implemented:

- skill registry
- skill loading
- skill matching
- compatibility import
- skill execution boundaries

Design status:

- `Not started`

### 4.7 Growth system

Not implemented:

- failure cards
- candidate capability expansion records
- lifecycle display
- growth review surfaces

Design status:

- `Not started`

### 4.8 Full provider matrix

Not implemented:

- Anthropic
- Gemini
- Ollama
- llama.cpp
- STT/TTS/realtime provider families

Design status:

- `Not started`

### 4.9 Shell-level native features

Not implemented:

- tray integration workflow
- global shortcuts
- wake key
- background service feeling

Design status:

- `Not started`

## 5. Gap versus the original design

The design document describes Nexus as:

- a Windows-native long-running personal agent host
- with text, voice, gateway, wechat, tasking, memory, control, audit, and extensibility

The current codebase is best described as:

- a modular desktop agent prototype with text conversation, task records, approval flow, memory cards, and runtime-swappable backend modules

That means the project currently covers:

- some of the `Core Layer`
- a small part of the `Shell Layer`
- a small part of the `Provider Layer`

It does **not** yet cover the main parts of the intended `Exec Layer`, `Gateway`, and `Voice` systems.

## 6. Practical completion estimate by module

This is a practical engineering estimate, not a product marketing estimate.

| Area | Progress |
| --- | --- |
| Monorepo structure | 85% |
| Desktop shell foundation | 40% |
| Main conversation UI | 35% |
| Task system foundation | 45% |
| Audit and approval foundation | 45% |
| Brain routing foundation | 30% |
| Memory foundation | 35% |
| Provider abstraction foundation | 35% |
| Control center | 20% |
| Voice runtime | 0% |
| Gateway | 0% |
| WeChat connector | 0% |
| Browser executor | 0% |
| Dev executor | 0% |
| Skill system | 0% |
| Growth system | 0% |

## 7. Recommended next build order

To stay aligned with the design document, the next implementation order should be:

1. finish the task runtime boundary so execution no longer lives mainly inside the Tauri app
2. build the control-center architecture so the UI has a real system-management backbone
3. add `nexus-exec` with at least one real executor path
4. add local voice runtime foundation
5. add embedded gateway
6. add WeChat connector
7. add skill loading and compatibility layer
8. add growth/failure-card recording

## 8. Current conclusion

The current repository has already solved the following hard early-stage problems:

- the codebase is modular instead of being a single pile
- there is already a real local desktop app
- there is already a task + approval + memory + audit loop
- the architecture direction is still compatible with future UI rewrites

But the repository has **not yet** reached the product scope promised by the original design document.

The most accurate summary today is:

> Nexus currently has a usable modular desktop prototype and core orchestration foundation, but most of the defining 1.0 capability modules are still waiting to be built.

## 9. Latest progress update

Recent implementation progress after the earlier browser-runtime milestone:

- `nexus-task` now produces more task-shaped plans for browser work, including dedicated step suggestions for login, form-fill, extraction, and generic browser tasks.
- The desktop workspace now surfaces the current task result summary instead of only task title and plan steps.
- The desktop workspace now includes a lightweight "latest activity" panel fed by recent browser audit events so the UI shows execution traces instead of looking like a plain chat shell.
- The browser extraction path is now easier to inspect from both the control center and the workspace because structured browser audits are shown in two places.
- The Chinese desktop copy bundle was rewritten with clean text instead of inherited garbled strings.
- Browser prompt detection in `nexus-exec` was extended with direct Chinese keyword matching so Chinese browser requests route correctly even before later intent upgrades.
- `nexus-browser` was rewritten into a clean module so browser task parsing is no longer polluted by legacy encoding issues.
- Login and form-fill browser intents now default to `observe` mode instead of silent mode, so sensitive flows stay inspect-first by default.
- The repository-local Playwright bridge now returns explicit boundary text for login and form tasks, making it clear that the current runtime inspects structure first and does not auto-submit sensitive actions.
- Browser execution output now includes structured `boundary` and `recommended_next_actions` fields instead of only a free-text summary.
- Browser extraction audits now persist the control boundary and next recommended actions, so the desktop workspace and future channel connectors can reuse them without reparsing summaries.
- Browser task parsing now also produces an explicit `action_phase` (`inspect_only`, `fill_only`, `submit_blocked`) so sensitive flows can be staged instead of treated as one undifferentiated browser action.
- The desktop workspace now surfaces the latest browser action phase directly, so the prototype already shows whether a browser task is still observing, can fill fields, or is blocked from submission.
- Browser execution output now carries a `field_plan` list for form-oriented tasks, and the workspace renders it in a dedicated panel.
- The Playwright bridge now emits `field_plan` data when a page exposes actual form controls, which gives the system a concrete pre-submit action draft instead of only sampled inputs.
- Browser form inspection now also distinguishes `missing_fields` and `sensitive_fields`, so the runtime can separate ordinary empty inputs from fields that deserve stronger control and approval.
- The desktop workspace now exposes missing-field and sensitive-field panels directly from browser audit data, which makes the form-risk shape visible without reading raw audit text.
- The repository now has a dedicated `docs/reuse-plan.md` that records which external projects should be directly reused, which should only influence architecture, and which should remain inspiration only.
- `nexus-dev` now exists as a dedicated development-task runtime boundary instead of leaving all code-oriented work inside the provider/default execution path.
- `nexus-exec` now registers a real `dev-executor`, so code tasks have their own dispatch path and audit events.
- The current development runtime is still scaffold-level, but it is already aligned to the intended borrow strategy: Aider-style patch-first execution and later OpenHands-style repo loops should land in `nexus-dev`, not in the UI shell or generic provider path.
- `nexus-dev` now returns structured `change_plan` and `verification_plan` output instead of only a generic text summary.
- `nexus-exec` now persists development planning and verification audits separately, so coding work can evolve toward a richer task loop without collapsing back into the default provider path.
- `nexus-dev` now also returns structured `patch_targets`, `verification_targets`, and `artifacts`, which moves the coding path closer to a real patch-first execution contract.
- Development audits now preserve those planning targets, so future UI surfaces, connectors, or task replay flows can inspect code-task intent without reparsing a long summary.
- `nexus-dev` now extracts `file_targets` and `module_targets` directly from the task text, which makes the code-task path closer to a real repository-aware execution contract.
- Development planning audits now preserve those targets too, so later replay, connector, or IDE-bridge work can understand which files or modules a coding task was aimed at.
- The desktop workspace now surfaces `file_targets`, `module_targets`, `artifacts`, and `verification_targets` directly from development-task audits, so code work is no longer hidden inside the control center only.
- The desktop workspace now also surfaces `patch_targets` and `change_plan`, so development tasks already read like a structured patch-first workbench instead of a plain chat transcript.
- `nexus-dev` now emits explicit `execution_mode`, `repo_scope`, and `patch_strategy` fields, so later Aider/OpenHands integration can plug into a stable execution contract instead of reshaping the runtime model again.
- The desktop workspace now shows those three strategy fields directly, so code tasks already expose whether the system is in read-only analysis, patch-ready mode, verification-only mode, or incremental refactor mode.
- `nexus-dev` now also emits `operation_steps` and `patch_outline`, which means the code-task path is no longer only describing intent and targets; it is starting to describe the actual ordered execution contract a future patch engine can follow.
- The desktop workspace now surfaces those execution steps and patch outlines, so development tasks read more like a real patch-first workbench and less like a generic result summary.
- `nexus-dev` now emits a structured `patch_proposal` field, which is the first explicit bridge between high-level task planning and a future real diff generator.
- The desktop workspace now surfaces that patch proposal directly, so the prototype can show a concrete patch-first recommendation before a real code-editing engine is attached.
- `nexus-dev` now also emits file- or module-oriented `patch_items`, which makes the patch-first path closer to a real executor contract that can later hand work to a diff generator or bounded worker.
- The desktop workspace now surfaces those patch items directly, so development tasks can already show concrete edit actions instead of only high-level strategy text.
- `nexus-dev` now also emits `patch_hunks`, which pushes the patch-first contract one step closer to real diff generation by describing the intended edit block shape inside a target file or module.
- The desktop workspace now surfaces those patch hunks directly, so the prototype can already show which code block is expected to change before an actual patch engine is attached.
- `nexus-dev` now also emits `patch_sets`, which groups patch hunks into minimal execution batches and moves the runtime contract closer to a real bounded patch engine.
- The desktop workspace now surfaces those patch sets directly, so development tasks can already show how future patch execution would be chunked into auditable batches.
- `nexus-dev` now also emits `patch_contract`, which captures preconditions, apply boundaries, and verification gates for a patch set instead of only describing the patch contents.
- The desktop workspace now surfaces that patch contract directly, so the prototype already exposes when a future patch engine is allowed to mutate files and what must be verified before completion.
- `nexus-dev` now emits an explicit `patch_schema_version`, which gives the patch-first contract a stable version boundary before the string-based fields are later upgraded into richer typed objects.
- The desktop workspace now surfaces that schema version too, so future migrations of the patch plan model can stay auditable across machines and build phases.
- `nexus-dev` now also exports a typed `PatchPlanSchema` as JSON, so the patch-first path no longer depends only on semicolon-tagged audit strings.
- Development execution now writes a dedicated `dev.patch_schema` audit event that preserves the structured schema body separately from the lighter `dev.planned` summary.
- The desktop workspace now reads that dedicated schema audit and shows a compact schema preview card with version, mode, scope, strategy, and patch batch counts.
- The patch schema has now moved to `dev-patch-schema/v2` and includes explicit `patch_files`, so file-level mutation boundaries can evolve without reshaping the whole contract again.
- Development planning audits now persist a `patch_files` field alongside patch items, hunks, and sets, and the workspace renders those file-level patch boundaries directly.
- The development runtime now also emits a batch-oriented `patch_apply_plan`, so patch planning is no longer only about what should change; it also describes preflight, apply, and verify stages for each batch.
- The workspace now renders that apply plan directly, and the schema preview was corrected to read `schema_version` from the structured audit instead of a stale placeholder key.
- The patch schema now also carries an explicit `execution_contract`, covering write scope, dry-run preference, approval requirement, and rollback scope before a real patch engine is attached.
- The desktop workspace now renders that execution contract directly from `dev.planned`, so patch execution boundaries are visible before any file mutation runtime is introduced.
- The patch schema now also carries an explicit `execution_request`, which turns the patch plan into a closer patch-engine input by defining mode, selected batches, target paths, and verification scope.
- The desktop workspace now renders that execution request directly, so dry-run patch mode, selected batch IDs, and target path scope are visible before a real runner is attached.
- `nexus-dev` now also defines a separate patch-runner boundary and ships a first `DryRunPatchRunner`, so patch execution no longer has to be invented inside the runtime itself.
- The current dry-run runner does not mutate files, but it already consumes the structured execution request and apply plan and emits a runner log, which establishes the place where later real patch engines should attach.
- The runtime now exposes patch-runner catalog data through `nexus-exec` and Tauri commands instead of forcing the desktop shell to depend on `nexus-dev` directly.
- The control center now shows a dedicated patch-runner inventory panel, so the current dry-run runner is visible as a first-class modular runtime component rather than an invisible internal detail.
- Dev execution now also writes a dedicated `dev.runner` audit event, so patch-runner work is tracked separately from generic dev planning and schema storage.
- The control center now includes a patch-runner activity panel, which makes recent dry-run execution traces visible without digging through the generic audit stream.
- Dev execution now also writes a structured `dev.runner_log` audit payload, so runner traces are preserved as JSON rather than only as flattened strings.
- The control center now derives a patch-runner status card from that structured audit, which shows the latest runner id, mode, log count, and most recent log entry.
- The workspace activity feed now mixes browser and development audit events instead of only showing browser traces, which makes the prototype feel more like one execution desk.
- The desktop copy bundle was cleaned again and rewritten with stable Chinese and English text so later UI iterations are not building on garbled locale data.
- Browser and development intent parsing now use clean Chinese keywords again, which restores reliable routing for Chinese prompts after earlier encoding pollution.

Verification after these changes:

- `cargo check`
- `npm run build --workspace @nexus/desktop`
