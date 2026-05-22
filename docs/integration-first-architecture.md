# Integration-First Architecture

Last updated: 2026-05-21

This document defines the long-term integration strategy for Nexus.

Nexus should not hand-build every low-level AI capability. The project direction is to build the personal agent host, runtime, safety layer, memory, audit, and user experience while carefully embedding, adapting, or reimplementing the best ideas from strong open-source AI projects.

The goal is not to become a thin API wrapper. The goal is to become an embedded, local-first, controllable agent operating layer that can absorb high-quality third-party implementations behind Nexus-owned boundaries.

## 1. Product stance

Nexus is an integration-first personal Agent OS.

Nexus owns:

- desktop shell and user experience
- task-first runtime orchestration
- module and executor registry
- risk policy and approval flow
- audit trail and observability
- local memory and long-term learning
- provider selection and capability routing
- self-evolution workflow
- third-party capability governance

Nexus should borrow or embed:

- code agent loops
- patch generation and application strategies
- browser automation primitives
- workflow/state-machine ideas
- memory retrieval and update algorithms
- model provider routing patterns
- tool execution UX patterns
- sandbox and checkpoint strategies

## 2. Integration principle

The preferred rule is:

> Nexus does not remake mature AI tools from scratch. It absorbs the best open-source implementations into a local, auditable, memory-aware, approval-gated runtime.

This means:

- third-party code must be wrapped behind Nexus interfaces
- imported code must not dictate Nexus architecture
- every embedded capability must have clear ownership
- all high-risk actions still pass through Nexus approval policy
- all meaningful actions still create Nexus audit records
- useful execution results should become memory candidates
- license and supply-chain risk must be tracked before embedding

## 3. Capability categories

Third-party capabilities should map into four categories.

### 3.1 Provider

A provider supplies AI inference or perception.

Examples:

- chat model
- code model
- embedding model
- speech-to-text
- text-to-speech
- vision model
- realtime model

Nexus should keep provider interfaces broad enough for multimodal and realtime use, not only text chat.

### 3.2 Executor

An executor performs actions.

Examples:

- development executor
- browser executor
- terminal executor
- file executor
- document executor
- voice executor
- self-evolution executor

Executors are the main way third-party action systems enter Nexus.

### 3.3 Connector

A connector integrates with an external system or data source.

Examples:

- GitHub
- local git repository
- browser profile
- database
- file index
- chat channel
- MCP-like tools
- WeChat gateway

Even when Nexus embeds code directly, connector boundaries should remain explicit.

### 3.4 Runtime

A runtime manages multi-step stateful execution.

Examples:

- coding-agent runtime
- browser-agent runtime
- workflow graph runtime
- voice session runtime
- self-improvement runtime

A runtime may own multiple executors internally, but it should still report progress through Nexus task state and audit events.

## 4. Embedding levels

Nexus should support four levels of reuse.

### 4.1 Level 0: design borrowing

Read source code and reimplement the idea inside Nexus.

Use when:

- license is unclear or restrictive
- language/runtime mismatch is high
- the upstream project is too large
- the idea is simple but implementation is coupled

### 4.2 Level 1: small module adaptation

Copy and adapt small, isolated pieces of MIT/Apache/BSD-compatible code.

Requirements:

- record source repository and commit
- keep license and notice text
- document local modifications
- wrap the module behind a Nexus API

### 4.3 Level 2: vendored component

Place a third-party component under a controlled `third_party/` or `vendor/` boundary.

Requirements:

- pin upstream version or commit
- keep upstream license files
- keep a patch log
- expose only a Nexus adapter to the rest of the repo
- avoid leaking upstream data types into core crates

### 4.4 Level 3: fork and deep integration

Fork a major project and evolve it as part of Nexus.

Use rarely.

This is only acceptable when:

- the component is core to Nexus differentiation
- license is safe
- maintenance cost is justified
- Nexus can keep up with upstream security updates
- the boundary remains testable and replaceable

## 5. Core Nexus interfaces

Before large embedding work, Nexus should stabilize its internal schemas.

### 5.1 Executor descriptor

Each executor should declare:

- id
- display name
- capability family
- supported task kinds
- risk profile
- input schema
- output schema
- configuration schema
- whether dry-run is supported
- whether rollback/checkpoint is supported
- whether human approval is required by default

### 5.2 Execution request

Each execution request should carry:

- task id
- user intent
- normalized task kind
- workspace context
- execution mode
- risk budget
- memory context
- provider preference
- approval policy
- artifacts available to the executor

### 5.3 Execution result

Each execution result should return:

- status
- summary
- step list
- risk level reached
- files touched
- commands run
- browser actions taken
- generated artifacts
- audit references
- memory candidates
- follow-up suggestions
- error and retry metadata

### 5.4 Artifact

Artifacts should be first-class objects.

Examples:

- patch
- diff
- screenshot
- command log
- browser trace
- extracted data
- generated document
- test report
- plan
- review

## 6. Safety boundary

Embedding code must not bypass Nexus control.

Every embedded module that can act on the local machine must go through:

- risk classification
- approval policy
- audit recording
- scoped filesystem access
- scoped command access
- secret redaction
- memory filtering
- rollback/checkpoint planning when possible

Third-party modules should never receive unrestricted access to:

- global environment variables
- all filesystem paths
- raw secrets
- browser credentials
- destructive command execution
- silent network access

## 7. License and supply-chain governance

Before embedding third-party code, Nexus should record:

- repository URL
- upstream commit or release
- license
- license compatibility notes
- copied files or adapted modules
- local modifications
- known security issues
- update policy
- whether upstream contains enterprise or source-available directories

Priority should go to:

- MIT
- Apache-2.0
- BSD-family licenses

Use caution with:

- GPL
- AGPL
- SSPL
- custom source-available licenses
- unclear dual licensing
- projects with mixed community and enterprise code

## 8. Recommended first integration targets

The first wave should focus on capabilities that unlock self-evolution.

Recommended order:

1. development executor patterns
2. browser executor patterns
3. memory schema and retrieval
4. workflow/state-machine patterns
5. provider routing patterns
6. checkpoint and rollback patterns
7. skill/plugin packaging patterns

This supports the long-term loop:

`Observe -> Diagnose -> Propose -> Approve -> Patch -> Verify -> Reflect -> Memorize`

## 9. Architectural non-goals

Nexus should not become:

- a pile of pasted upstream projects
- a UI shell around remote APIs
- an uncontrolled agent spawner
- a plugin marketplace with weak permissions
- a model-provider demo app
- a chat app with hidden side effects

The value of Nexus is the local, controlled, memory-aware runtime that makes third-party intelligence safe and useful.
