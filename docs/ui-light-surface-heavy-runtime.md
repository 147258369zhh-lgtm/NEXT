# UI Principle: Light Surface, Heavy Runtime

Last updated: 2026-05-21

This document defines the long-term UI direction for Nexus.

Nexus should have a clean, calm, Codex-like main surface while keeping configuration, module management, audits, approvals, memory, and third-party capability controls one click deeper.

The main interface should feel simple. The runtime behind it can be powerful, modular, and deeply configurable.

## 1. Core principle

Nexus adopts a light-surface, heavy-runtime interface.

The main window should stay:

- clean
- task-focused
- input-first
- low-noise
- professional
- calm during long-running tasks

Secondary windows and panels should contain:

- model configuration
- executor configuration
- approvals
- audit timeline
- memory management
- browser runtime state
- development runtime state
- plugin/source management
- self-evolution controls
- security settings

The user should feel calm on the surface and fully in control one level deeper.

## 2. Main window role

The main window is not a settings dashboard.

It is the place where the user says what they want Nexus to do.

Primary objects:

- current workspace
- task input
- current task status
- recent tasks
- pending approval chips
- concise result summaries
- lightweight context indicators

Avoid placing these on the main surface:

- provider configuration forms
- executor tuning parameters
- raw audit logs
- long memory lists
- plugin source details
- risk policy editors
- dense module tables

## 3. Codex-like surface

The main surface may borrow these ideas from Codex-style tooling:

- large task input as the primary focus
- visible workspace context
- minimal top bar
- concise execution updates
- natural approval prompts
- summary-first completion messages
- changed files and tests as compact facts
- optional detail expansion instead of default log walls

Nexus should not imitate a chat app too closely. Chat is only the entry point. The deeper product object is the task.

## 4. Task-first interface

The core UI object should be `Task`, not `Message`.

A task should contain:

- user intent
- plan
- execution steps
- selected executor
- risk level
- approvals
- artifacts
- result summary
- audit references
- memory candidates

Messages can exist inside the task, but the user should be able to understand what happened through task state and artifacts.

## 5. Recommended main layout

A simple default layout:

```text
Topbar
- app identity
- workspace indicator
- provider indicator
- status indicator
- settings entry

Main area
- welcome or current task
- large input box
- recent task capsules
- pending approval chips

Optional detail drawer
- plan
- steps
- logs
- diff
- artifacts
- approvals
- memory candidates
```

The detail drawer should be closed by default unless the task requires attention.

## 6. Navigation model

Primary navigation should remain small:

```text
Chat
Tasks
Approvals
Memory
More
```

The deeper control center can expose:

```text
Overview
Models
Executors
Browser
Code
Memory
Audit
Security
Plugins
Self-Evolution
Settings
```

This keeps the surface light while preserving full control.

## 7. Secondary windows

### 7.1 Control Center

Shows runtime health and module status:

- brain
- memory
- provider
- browser runtime
- development runtime
- audit
- store
- executor registry

### 7.2 Settings

Contains configuration:

- language
- theme
- shortcuts
- provider defaults
- local model settings
- API keys and secret storage
- data directory

### 7.3 Executor Manager

Manages capabilities:

- dev executor
- browser executor
- terminal executor
- file executor
- voice executor
- self-evolution executor

Each executor should show:

- enabled state
- risk profile
- configuration
- health check
- recent failures

### 7.4 Approval Inbox

Shows pending high-risk actions:

- action summary
- affected files or systems
- risk level
- requested permissions
- allow once
- allow for session
- deny
- edit plan

### 7.5 Memory Center

Manages memory:

- user memories
- project memories
- system memories
- self memories
- search
- edit
- disable
- delete

### 7.6 Audit Timeline

Shows traceability:

- model calls
- tool calls
- file reads and writes
- commands
- browser actions
- approvals
- errors and retries

### 7.7 Source Lab

Tracks embedded and borrowed open-source capability sources:

- project name
- repository
- license
- imported modules
- local modifications
- update status
- risk notes

### 7.8 Self-Evolution Lab

Supports Nexus improving itself:

- self-diagnosis
- improvement proposals
- roadmap
- planned patches
- verification results
- self-memory updates

## 8. UI patterns

### 8.1 Task capsule

A compact completed or running task card.

Example:

```text
Refactor provider routing
Completed · 3 files changed · tests passed
[Diff] [Details] [Continue]
```

### 8.2 Approval chip

A compact prompt on the main surface.

Example:

```text
Approval needed: run `cargo check`
[Allow] [Deny] [Details]
```

### 8.3 Context pill

Small environment indicators.

Examples:

```text
Workspace: D:\AI\NEXT\repo
Model: OpenAI-compatible
Memory: On
Risk: Ask before L3+
```

### 8.4 Mode switch

A lightweight hint for routing, not a heavy configuration panel.

Suggested modes:

- Auto
- Ask
- Code
- Browser
- Self

### 8.5 Detail drawer

A right-side drawer for task internals.

Tabs:

- Plan
- Steps
- Logs
- Diff
- Artifacts
- Approvals
- Memory

## 9. Visual direction

Recommended visual tone:

- dark theme first
- high readability
- restrained color
- thin borders
- generous spacing
- compact but not crowded
- professional terminal-adjacent feel
- status color used sparingly

Status colors:

- blue for running
- green for completed
- yellow for approval or waiting
- red for failed or high-risk
- gray for disabled or inactive

The desired feel is:

> Codex professional focus, Raycast-like simplicity, Linear-like order, and desktop control-center reliability.

## 10. Implementation guidance

Near-term UI work should:

- keep `apps/desktop/src-tauri` as a shell adapter
- keep core task logic outside React
- reduce main-surface density
- move configuration into secondary views
- make `ControlCenter` a deeper destination, not the default surface
- show task status before raw logs
- expose detail only when requested
- keep approval visible and immediate

The UI can be redesigned later, but this principle should survive UI rewrites.

## 11. Success criteria

The UI succeeds when:

- a new user can submit a task without understanding modules
- an advanced user can inspect and configure every subsystem
- high-risk actions are visible and controlled
- long-running tasks remain understandable
- results are summarized before logs
- task artifacts are easy to find
- the main window does not feel like an admin dashboard
- Nexus feels simple on the surface and powerful underneath
