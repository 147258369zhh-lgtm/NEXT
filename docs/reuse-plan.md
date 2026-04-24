# Reuse And Borrow Plan

Last updated: 2026-04-23

This document records which external projects Nexus should directly reuse, which ones should only influence architecture, and which ones should not be pulled in as-is.

The purpose is to reduce wheel reinvention while keeping Nexus in control of its own runtime boundaries.

## 1. Reuse policy

Every external project should be placed into one of these buckets:

- direct code or protocol reuse
- architecture and workflow borrowing
- inspiration only

The rule is:

- prefer direct reuse for protocols, bridges, runtimes, and observability
- prefer architecture borrowing for full products
- avoid importing another product shell into the Nexus core

## 2. Highest-priority direct reuse targets

These are the best immediate reuse candidates for the current roadmap.

### 2.1 Browser stack

- `playwright-mcp`
  - reuse target: browser tool schema and MCP-facing browser capability model
  - Nexus landing zone: `nexus-browser`, later connector/MCP bridge layer
- `browser-use`
  - reuse target: high-level browser-agent abstraction and page/task modeling ideas
  - Nexus landing zone: `nexus-browser` upper layer and future browser task planner

### 2.2 Dev stack

- `Aider`
  - reuse target: patch-first and diff-first code editing workflow
  - Nexus landing zone: `nexus-dev`
- `OpenHands`
  - reuse target: repo-task loop, verification loop, artifact organization
  - Nexus landing zone: future `nexus-dev` runtime and dev task orchestration
- `Continue`
  - reuse target: IDE bridge direction
  - Nexus landing zone: future IDE integration adapter, not the main UI

### 2.3 Desktop automation and system control

- `Windows-Use`
  - reuse target: Windows UI Automation first strategy
  - Nexus landing zone: future desktop executor / native desktop control layer

### 2.4 Observability and execution safety

- `Langfuse`
  - reuse target: tracing, prompt/run observability
  - Nexus landing zone: cross-runtime observability layer
- `E2B`
  - reuse target: secure remote execution substrate when needed
  - Nexus landing zone: future code or tool sandbox layer

## 3. Strong architecture references, but not direct product imports

These projects should guide architecture, not be copied wholesale.

- `BrowserOS`
  - borrow: product organization, browser workspace model, scheduled workflows
- `Skyvern`
  - borrow: hard-mode browser flows like login, form handling, OTP, downloads
- `LangGraph`
  - borrow: stateful orchestration and HITL-friendly execution flow
- `OpenAI Agents SDK`
  - borrow: agent lifecycle, tracing, handoff concepts
- `Mem0`, `Letta`, `Graphiti`, `LangMem`
  - borrow: layered memory schema and retrieval concepts
- `UI-TARS`, `Agent-S`
  - borrow: GUI fallback layering when accessibility or UI automation is insufficient

## 4. Inspiration-only references for now

These are useful to study, but should not be directly imported into the current 1.0 architecture.

- `Cherry Studio`
- `5ire`
- `Jan`
- `Witsy`
- `OpenAkita`
- `QwenPaw`
- `Cronicle`
- `Healthchecks`
- `Voyager`
- `Reflexion`

They mainly provide product, workflow, marketplace, or long-term growth ideas.

## 5. Current implementation mapping

### 5.1 Already aligned

- `nexus-browser`
  - already moving toward `playwright-mcp` and `browser-use` style separation
- browser bridge
  - already serves as a bridgeable runtime boundary instead of hardwiring Playwright into the app shell
- task and approval model
  - already moving in a LangGraph-compatible direction

### 5.2 Being implemented now

- `nexus-dev`
  - should become the landing zone for `Aider`-style patch-first workflow
  - should later absorb `OpenHands`-style repo-task loops

### 5.3 Next likely direct integrations

- `Langfuse`
  - should be connected once executor/runtime traces are slightly richer
- `Windows-Use`
  - should inform the first real desktop executor instead of building a visual-only desktop layer from scratch

## 6. Practical rules for future contributors

- Do not build a custom browser protocol before checking whether `playwright-mcp` compatibility is enough.
- Do not build a code-editing workflow that ignores patch-first output.
- Do not turn Nexus into another chat-shell product by importing full desktop AI clients.
- Do not let borrowed code collapse module boundaries. Reuse should strengthen boundaries, not erase them.
