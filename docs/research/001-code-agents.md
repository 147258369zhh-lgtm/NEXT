# Research 001: Code Agents

Last updated: 2026-05-21

This note defines the first code-agent research track for Nexus.

Nexus should not hand-build every coding-agent capability. The goal is to study the strongest open-source coding agents, borrow implementation ideas, and eventually embed selected modules behind the `nexus-dev` patch runner boundary.

## 1. Nexus target

The target module is `crates/nexus-dev`.

The target boundary is:

```text
DevTaskSpec -> PatchPlanSchema -> PatchRunner -> PatchRunnerOutput -> DevExecutionOutput
```

A third-party code agent should enter Nexus as a patch runner or as a deeper dev runtime only after its license, architecture, and safety model are understood.

## 2. Candidate projects

### 2.1 Roo Code

Repository: `github.com/RooCodeInc/Roo-Code`

Primary value:

- mode-based coding workflow
- MCP/tool integration patterns
- checkpoint and task continuity ideas
- strong approval-centered UX
- clear separation between task intent and tool action

Nexus borrowing targets:

- mode model for `code.inspect`, `code.patch`, `code.verify`, and `self.evolve`
- checkpoint metadata for rollback-capable patch runners
- approval language for high-risk code actions
- tool/action schema style

Initial integration level: Level 0 / Level 1.

Do not embed large chunks yet. First extract concepts and schemas.

### 2.2 Cline

Repository: `github.com/cline/cline`

Primary value:

- human-in-the-loop code editing
- terminal/file action approval UX
- VS Code-centric agent loop
- practical task state presentation

Nexus borrowing targets:

- approval prompts for file and command actions
- compact task step presentation
- file edit lifecycle
- terminal command risk framing

Initial integration level: Level 0 / Level 1.

Cline is useful for UX and safety flow even if Nexus does not share its IDE extension architecture.

### 2.3 OpenCode

Repository: `github.com/opencode-ai/opencode`

Primary value:

- terminal-native coding agent shape
- lightweight session and command flow
- potentially easier to adapt than large IDE agents

Nexus borrowing targets:

- CLI/session model
- repository context packing
- patch loop ergonomics
- compact terminal-style status stream

Initial integration level: Level 0 / Level 1.

This is a strong candidate for a future embedded runner if the architecture and license remain compatible.

### 2.4 Aider

Repository: `github.com/Aider-AI/aider`

Primary value:

- git-native patch workflow
- multi-file editing strategy
- practical repo modification loop
- diff and commit-oriented operation

Nexus borrowing targets:

- file selection and repo map ideas
- patch/diff application strategy
- git-aware workflow
- verification prompt patterns

Initial integration level: Level 0 first.

Because Aider is Python-first, Nexus should study it before deciding whether to embed code, port concepts, or keep it as an external reference.

### 2.5 OpenHands

Repository: `github.com/All-Hands-AI/OpenHands`

Primary value:

- larger repo-task runtime
- sandbox and event stream concepts
- task loop and browser/terminal integrations
- full agent environment design

Nexus borrowing targets:

- event stream shape
- sandbox lessons
- repo-task loop
- separation between runtime and UI

Initial integration level: Level 0.

Nexus should avoid importing large runtime sections early. Use it as a design reference.

## 3. Selection criteria

A candidate is suitable for embedding only if it scores well on:

- permissive license compatibility
- small enough module boundary
- clear file-edit abstraction
- strong approval hooks
- testable patch output
- minimal global state
- controllable command execution
- easy secret isolation
- maintainable dependency tree
- no hidden cloud/service coupling

## 4. Runner categories

Nexus patch runners should fall into these categories:

### 4.1 Native scaffold runner

Current default.

- implemented in Nexus
- safe dry-run
- no file mutation
- used to validate schema and UI

### 4.2 Embedded agent runner

Reserved boundary for code borrowed from projects such as Roo Code, Cline, OpenCode, or Aider.

- runs inside Nexus process or controlled child module
- emits `PatchRunnerOutput`
- must respect approval policy
- should return patch artifacts, not silently mutate files by default

### 4.3 Vendored specialist runner

A copied or vendored submodule for a narrow function.

Examples:

- repo map generator
- diff planner
- patch parser
- test target recommender

### 4.4 External bridge runner

Allowed as an intermediate development bridge, but not the final design preference.

Examples:

- CLI command invocation
- local service process

This can validate behavior before deeper embedding.

## 5. Recommended first implementation

The next implementation target should be an embedded-agent adapter skeleton that can hold source metadata and emit a handoff plan.

It should expose:

- source project name
- repository URL
- license status
- integration level
- intended borrowed modules
- required permissions
- mutation mode
- approval requirement
- dry-run support

The actual third-party code should not be copied until a focused source review is complete.

## 6. Open questions

- Which project has the cleanest reusable patch application model?
- Which project has the safest approval UX for local file mutation?
- Which project has the lowest dependency and license risk?
- Should Nexus port ideas into Rust, embed TypeScript modules, or vendor Python logic selectively?
- How should code-agent checkpoints map into Nexus audit and memory?

## 7. Immediate next steps

1. Extend `PatchRunnerDescriptor` with source and license metadata.
2. Add an embedded-agent runner skeleton.
3. Add a source review document for Roo Code.
4. Compare Roo Code and Cline approval flows.
5. Decide the first small module to port or reimplement.

## 8. Project source reviews

- `docs/research/001a-roo-code-source-review.md`: first Roo Code source review and Nexus mode/tool-filtering takeaways.
