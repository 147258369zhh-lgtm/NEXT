# Design Progress Backup

Last updated: 2026-04-23

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
- brain and memory as independently toggled modules
- UI beginning to split into smaller components instead of a single large file

Status judgment:

- `Done for current phase`
- this is the strongest part of the current implementation
- the repo is already moving in the hot-pluggable direction requested in the design document

## 3. What is only partially implemented

These areas have started, but only in a narrow or simplified form.

### 3.1 Control center

Current state:

- the current desktop UI already shows module status, approval queue, history, recent memory, current task, and task plan

Missing:

- dedicated settings center
- provider management panel
- connector management
- memory inspection tools
- skill inspection tools
- richer audit center
- voice settings

Status judgment:

- `Started, but far from spec`

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
- browser automation tasks
- auditable browser execution records

Design status:

- `Not started`

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
