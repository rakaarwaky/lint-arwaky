# AGENTS Rules

**Purpose:** Define the operational guide for AI agents working in this workspace — what they must know before generating code, docs, or suggestions.

**Audience:** AI agents, workspace maintainers, anyone configuring agent behavior.

**Scope:** Exactly one AGENTS.md per workspace.

**Location:** Project root.

**Length:** 50–500 lines (flat budget shared by every document type).

## Rules

1. **Commands must match CI.** Every printed command must be copy-pasteable and identical to the CI gate.
2. **No absolute personal paths.** Use `$HOME`, `${workspaceFolder}`, or repo-relative paths (`absolute-path`).
3. **No secrets or credentials.** Never put tokens, passwords, or keys in this file.
4. **Do not restate other documents.** Use links instead.
5. **Mark optional sections clearly.** Do not force fake sections like pipeline diagrams just to fill a template.
6. **Use** `main`, `.worktrees/`.
7. **Respect the length budget.** Target 50–500 lines.
