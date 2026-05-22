# Research 001a: Roo Code Source Review

Last updated: 2026-05-21

Sources reviewed:

- `github.com/RooCodeInc/Roo-Code`
- Roo Code documentation for modes and marketplace
- DeepWiki summary of Roo Code system prompt and modes internals

## 1. Current upstream status

Roo Code is a VS Code AI coding extension. The GitHub repository currently shows the project as archived and read-only as of 2026-05-15, with Apache-2.0 licensing. It remains valuable as a source reference because the repository exposes mature ideas around modes, tool access, MCP integration, checkpoints, and approval-centered coding workflows.

Important consequence for Nexus:

- Roo Code should be treated as a design and source-reference project, not as an actively maintained dependency.
- Any borrowed code must be reviewed carefully and copied selectively.
- Nexus should prefer reimplementing concepts behind its own `nexus-dev` boundaries.

## 2. Most relevant Roo concepts

### 2.1 Modes

Roo Code supports built-in and custom modes such as Code, Architect, Ask, Debug, and Custom Modes.

Useful Nexus mapping:

| Roo concept | Nexus target |
| --- | --- |
| Code mode | `code.patch` / `code.inspect` task kind |
| Architect mode | `code.plan` / `self.evolution.plan` |
| Ask mode | provider-only read-only executor |
| Debug mode | `code.verify` / `code.diagnose` |
| Custom modes | future `nexus-skill` or `nexus-mode` manifest |

Nexus should borrow the mode concept, but represent it as runtime routing metadata rather than UI-only configuration.

### 2.2 Tool groups and file restrictions

Roo's mode system can restrict tool groups and file edit patterns. DeepWiki references file restriction behavior in `src/shared/modes.ts` and validation in `src/core/tools/validateToolUse.ts`.

Useful Nexus mapping:

- `PatchRunnerDescriptor.requires_approval`
- `PatchRunnerDescriptor.mutates_files`
- future `allowed_tool_groups`
- future `allowed_path_patterns`
- future `risk_ceiling`
- future `write_scope`

Nexus should implement this idea natively, because it fits the approval/audit model.

### 2.3 Dynamic prompt construction

Roo assembles the system prompt from the active mode, workspace context, MCP state, rules, skills, capabilities, and custom instructions.

Useful Nexus mapping:

- `nexus-brain` decides role/mode.
- `nexus-exec` owns task state.
- `nexus-provider` receives assembled prompt/context.
- `nexus-memory` injects memory context.
- `nexus-dev` contributes patch schema and runner-specific capability text.

Nexus should borrow the layered prompt assembly model, but not the exact VS Code extension implementation.

### 2.4 MCP and marketplace model

Roo marketplace supports installing MCPs and modes globally or per project. Project-level files include `.roo/mcp.json` and `.roomodes`; global settings include MCP settings and custom modes.

Useful Nexus mapping:

- future project-local `.nexus/` config
- future global Nexus profile config
- future `nexus-skill` manifest
- project/global scope distinction for embedded capability modules

Security note:

- Marketplace-style installation is powerful but risky.
- Nexus should not install executable capabilities without approval, source review, and audit.
- Nexus should avoid immediate removal without confirmation for high-impact items.

## 3. Potential source files to inspect later

These files are referenced by DeepWiki and should be reviewed before any code borrowing:

- `src/shared/modes.ts`
- `src/core/tools/validateToolUse.ts`
- `src/core/prompts/system.ts`
- `src/core/task/build-tools.ts`
- `src/core/prompts/tools/filter-tools-for-mode.ts`
- `src/core/assistant-message/*`
- `src/core/tools/*`

Expected borrowing level:

- modes schema: Level 0 / Level 1
- tool filtering algorithm: Level 0 / Level 1
- prompt section ordering: Level 0
- UI components: Level 0 only
- VS Code extension integration: do not embed

## 4. License assessment

Observed license: Apache-2.0.

Preliminary status: compatible for selective borrowing, pending file-level review.

Required before copying code:

- record upstream commit hash
- copy Apache-2.0 license and notices where required
- verify no mixed-license files in target area
- avoid assets, marketplace data, branding, and service-specific code
- avoid archived project security assumptions

## 5. Security assessment

Roo-style capabilities imply local file edits, shell commands, MCP tools, and marketplace-installed extensions.

Nexus must gate any similar behavior through:

- risk policy
- approval inbox
- audit timeline
- path restrictions
- dry-run first for patch operations
- secret redaction
- explicit source review status

Nexus should copy the idea of tool filtering by mode, but enforce it below the UI layer.

## 6. Recommended Nexus changes

### 6.1 Add mode metadata to dev tasks

Future fields:

```text
DevTaskSpec.mode_slug
DevTaskSpec.allowed_tool_groups
DevTaskSpec.allowed_path_patterns
DevTaskSpec.mode_instructions
```

### 6.2 Extend patch runner descriptors

Already started:

```text
repository
license
review_status
integration_level
requires_approval
supports_dry_run
```

Future fields:

```text
borrowed_from
upstream_commit
allowed_tool_groups
allowed_path_patterns
checkpoint_support
```

### 6.3 Add a Nexus mode manifest

Future file:

```text
.nexus/modes.json
```

Possible shape:

```json
{
  "modes": [
    {
      "slug": "code",
      "title": "Code",
      "task_kinds": ["code.inspect", "code.patch"],
      "allowed_tool_groups": ["read", "edit", "test"],
      "requires_approval_above": "L3"
    }
  ]
}
```

## 7. What Nexus should borrow now

Borrow immediately as design:

- mode taxonomy
- mode-specific tool filtering
- path restriction concept
- project/global configuration scope
- layered prompt assembly
- mode marketplace cautionary model

Do not borrow yet:

- direct VS Code extension code
- marketplace installation code
- UI implementation
- any file mutation logic
- any command execution logic

## 8. First concrete implementation target

Implement a Nexus-native dev mode catalog:

- `code.inspect`
- `code.patch`
- `code.verify`
- `code.architect`
- `self.evolve`

Each mode should declare:

- allowed task kinds
- allowed tool groups
- mutation permission
- approval requirement
- default patch runner

This should live in `nexus-dev` first and later become configurable through project/global Nexus config.

## 9. Source links

- GitHub repository: https://github.com/RooCodeInc/Roo-Code
- Roo marketplace documentation: https://roocodeinc.github.io/Roo-Code/features/marketplace/
- DeepWiki system prompt and modes page: https://deepwiki.com/RooCodeInc/Roo-Code/9-settings-and-configuration

## 10. Implementation status

Implemented in Nexus:

- `crates/nexus-dev/src/lib.rs` now exposes `DevModeDescriptor`.
- `list_dev_mode_catalog()` includes `code.inspect`, `code.patch`, `code.verify`, `code.architect`, and `self.evolve`.
- `select_dev_mode()` maps user prompts and inferred intents into a mode.
- Tauri exposes `list_dev_modes` for the control center.
- The control center displays mode slug, intent, runner, allowed tool groups, mutation flag, approval requirement, and borrowed-from note.

Next implementation target:

- enforce mode permissions inside patch runners instead of only displaying them.

## 11. Mode guard status

Implemented now:

- Patch runners receive `DevModeDescriptor` at execution time.
- `validate_mode_for_runner()` checks mutation permission, edit tool allowance, approval alignment, and target path restrictions.
- Runner logs now include mode guard outcome (`passed` or `blocked-dry-run`) and guard messages.

Current behavior:

- Guard violations are recorded in runner logs and surfaced through audit paths.
- Hard blocking can be added next by turning violations into execution errors for mutation-capable runners.

## 12. Structured guard status

Implemented now:

- `PatchRunnerOutput` contains a structured `ModeGuardReport`.
- `PatchRunnerAuditPayload` persists the guard report alongside log entries.
- Control Center parses runner audit JSON and displays guard status, mode slug, and violation count.

Next implementation target:

- enforce hard blocking for mutation-capable runners when `ModeGuardReport.status` is not `passed`.

## 13. Hard-block status

Implemented now:

- `should_hard_block_guard()` upgrades guard violations to execution errors for mutation-capable runners.
- Dry-run runners remain observable and can report guard violations without mutating files.
- Embedded-agent runner is still non-mutating today, but the hard-block path will activate automatically if a future embedded runner declares `mutates_files = true` and violates mode permissions.

This preserves the current scaffold while preparing the safety boundary for real code-agent embedding.

## 14. Mutation placeholder status

Implemented now:

- `MutationAgentPatchRunner` is registered in the patch runner catalog with `mutates_files = true` and `enabled = false`.
- It can be selected with `NEXUS_PATCH_RUNNER=mutation-placeholder` for guard-boundary testing.
- It still does not mutate files; it exists to verify hard-block behavior before a real embedded code agent is added.

Expected use:

- Use read-only modes such as `code.inspect` to confirm hard-block behavior.
- Use mutation modes such as `code.patch` or `self.evolve` to confirm guarded placeholder execution can pass.
