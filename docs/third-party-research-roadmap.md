# Third-Party Research Roadmap

Last updated: 2026-05-21

This document tracks the open-source projects Nexus should research for embedded implementation ideas.

The purpose is not to blindly copy code. The purpose is to study the best available implementations for each capability, decide which parts are safe and valuable to absorb, and map them into Nexus-owned module boundaries.

## 1. Research method

For each candidate project, record:

- project name
- repository URL
- license
- primary language and runtime
- relevant modules or files
- capabilities worth borrowing
- whether code can be embedded directly
- whether only design should be borrowed
- security and supply-chain risks
- Nexus target module
- recommended integration level
- next action

Recommended integration levels are defined in `docs/integration-first-architecture.md`.

## 2. Priority matrix

### 2.1 Code agents

These projects are the highest priority because Nexus self-evolution depends on a strong development executor.

| Project | Repository | Borrowing target | Nexus module | Initial level |
| --- | --- | --- | --- | --- |
| Cline | `github.com/cline/cline` | human-in-the-loop coding, file edits, terminal approvals | `nexus-dev`, UI approval flows | Level 0/1 |
| Roo Code | `github.com/RooCodeInc/Roo-Code` | modes, MCP-style tool use, checkpoint UX, task control | `nexus-dev`, `nexus-task` | Level 0/1 |
| OpenCode | `github.com/opencode-ai/opencode` | terminal coding agent structure, lightweight execution | `nexus-dev` | Level 0/1 |
| Aider | `github.com/Aider-AI/aider` | git-native patching, diff management, repo editing loop | `nexus-dev` | Level 0/1 |
| OpenHands | `github.com/All-Hands-AI/OpenHands` | repo task loop, event stream, sandbox, runtime model | `nexus-exec`, `nexus-dev` | Level 0 |
| Codex CLI | `github.com/openai/codex` | coding agent loop, approval boundary, patch workflow | `nexus-dev`, `nexus-self` | Level 0/1 after license review |

Research questions:

- How do they choose files to read?
- How do they generate and apply patches?
- How do they request approval before commands?
- How do they represent task progress?
- How do they recover from failed tests?
- How do they summarize completed work?
- How do they prevent destructive actions?

Expected Nexus output:

- `DevExecutionRequest` schema
- `PatchArtifact` schema
- `CommandApproval` schema
- development task timeline model
- first self-evolution workflow design

## 3. Browser agents

Browser automation should evolve from simple Playwright command execution into structured `observe`, `act`, and `extract` primitives.

| Project | Repository | Borrowing target | Nexus module | Initial level |
| --- | --- | --- | --- | --- |
| Playwright | `github.com/microsoft/playwright` | reliable browser automation substrate | `nexus-browser` | Existing dependency |
| Stagehand | `github.com/browserbase/stagehand` | AI browser primitives: observe, act, extract | `nexus-browser` | Level 0/1/2 after review |
| browser-use | `github.com/browser-use/browser-use` | autonomous browser agent loop and page observation | `nexus-browser` | Level 0/1 |
| MCP browser tools | `github.com/modelcontextprotocol/servers` | browser tool schemas and permission boundaries | `nexus-browser`, future tools | Level 0/1 |

Research questions:

- How is page state summarized for models?
- How are clickable elements identified?
- How are screenshots and DOM snapshots combined?
- How are browser actions logged?
- How are credentials and sessions protected?
- How should failure and retry be represented?

Expected Nexus output:

- `BrowserActionSpec`
- `BrowserObservation`
- `BrowserArtifact`
- browser risk policy rules
- browser execution timeline UI

## 4. Agent orchestration

Nexus should avoid premature multi-agent complexity. Research should focus on state machines, role pipelines, checkpoints, and traceability.

| Project | Repository | Borrowing target | Nexus module | Initial level |
| --- | --- | --- | --- | --- |
| LangGraph | `github.com/langchain-ai/langgraph` | graph/state/checkpoint model | `nexus-exec`, `nexus-brain` | Level 0 |
| AutoGen | `github.com/microsoft/autogen` | multi-agent messaging and roles | `nexus-brain` | Level 0 |
| CrewAI | `github.com/crewAIInc/crewAI` | role/task/crew concepts | `nexus-brain` | Level 0 |
| Agno | `github.com/agno-agi/agno` | agent platform, sessions, tracing | `nexus-exec`, `nexus-audit` | Level 0 |
| Mastra | `github.com/mastra-ai/mastra` | TypeScript workflows, memory, tool organization | UI/backend bridge ideas | Level 0/1 |
| smolagents | `github.com/huggingface/smolagents` | small code-agent design | `nexus-brain`, `nexus-dev` | Level 0/1 |

Research questions:

- Is a graph runtime needed now, or later?
- How should role-based phases be represented?
- What should be persisted for resume/retry?
- How should planner, executor, reviewer, verifier, and guardian roles communicate?
- How much multi-agent behavior should be exposed to the user?

Expected Nexus output:

- role-based execution phases
- task checkpoint schema
- retry and resume model
- executor dispatch contract

## 5. Memory systems

Nexus memory must remain local-first and user-controllable.

| Project | Repository | Borrowing target | Nexus module | Initial level |
| --- | --- | --- | --- | --- |
| Mem0 | `github.com/mem0ai/mem0` | memory extraction, update, search, user memories | `nexus-memory` | Level 0/1 |
| Letta | `github.com/letta-ai/letta` | stateful agents and long-term memory concepts | `nexus-memory`, `nexus-brain` | Level 0 |
| MCP Memory Server | `github.com/modelcontextprotocol/servers` | simple persistent knowledge graph memory | `nexus-memory` | Level 0/1 |
| OpenClaw-style memory | upstream to verify | local-first agent memory patterns | `nexus-memory` | Level 0 |

Research questions:

- What is the minimum useful memory card schema?
- How are memories extracted from conversations?
- How are conflicting memories handled?
- How are memories deleted or disabled?
- How is project memory separated from user memory?
- How should memories be ranked for prompt injection?

Expected Nexus output:

- `MemoryCard v2`
- memory scope model: user/project/system/self
- memory confidence and decay fields
- memory review UI requirements

## 6. Provider and model routing

Provider work should support more than chat.

| Project | Repository | Borrowing target | Nexus module | Initial level |
| --- | --- | --- | --- | --- |
| LiteLLM | `github.com/BerriAI/litellm` | provider normalization and routing | `nexus-provider` | Level 0 |
| Vercel AI SDK | `github.com/vercel/ai` | streaming abstractions and provider ergonomics | frontend/provider bridge | Level 0/1 |
| Mastra | `github.com/mastra-ai/mastra` | model/provider integration patterns | `nexus-provider` | Level 0 |
| OpenHands providers | `github.com/All-Hands-AI/OpenHands` | agent-oriented model config | `nexus-provider` | Level 0 |

Research questions:

- How should chat, code, embedding, STT, TTS, vision, and realtime providers share config?
- How should models be selected per task type?
- How should cost, speed, privacy, and quality preferences be represented?
- How should provider failures fall back?

Expected Nexus output:

- provider family expansion
- per-role model routing
- provider health model
- streaming result format

## 7. OpenClaw / Hermes-style agent host ideas

Nexus should study personal agent host systems for inspiration, but must maintain stronger control boundaries.

Borrowable ideas:

- skill system
- local gateway
- persistent background host
- long-term personal memory
- multiple external channels
- agent-to-agent messaging
- local automation patterns

Risks to study carefully:

- overly broad local permissions
- untrusted skills
- unsafe marketplace assumptions
- weak secret isolation
- hidden network behavior
- insufficient auditability

Expected Nexus output:

- future `nexus-skill` design
- capability package manifest
- skill permission model
- gateway safety model

## 8. Research deliverables

Create detailed notes under:

```text
docs/research/
  001-code-agents.md
  002-browser-agents.md
  003-memory-systems.md
  004-agent-orchestration.md
  005-provider-routing.md
  006-openclaw-hermes-style-hosts.md
  007-license-risk-register.md
```

Each note should end with:

- what Nexus should copy conceptually
- what Nexus may embed as code
- what Nexus must avoid
- affected modules
- proposed next implementation task

## 9. First research sprint

Recommended first sprint:

1. Roo Code and Cline for approval-centered coding UX
2. Stagehand for browser executor semantics
3. Mem0 for memory model design
4. OpenCode or Aider for patch-first development loops
5. OpenHands for event stream and sandbox lessons

The sprint should produce implementation-ready schemas before any large third-party code is embedded.

Status:

- `docs/research/001-code-agents.md` is now started and should be expanded with project-specific source reviews before code is copied or vendored.


Code-agent source review status:

- `docs/research/001a-roo-code-source-review.md` records the first Roo Code review. The immediate implementation takeaway is a Nexus-native dev mode catalog with mode-specific tool groups and path restrictions.
