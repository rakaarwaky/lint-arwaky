# CONTRIBUTING Rules

**Purpose:** Tell anyone who wants to add code or documentation to the project exactly how to set up, where to work, and what gates they must clear before the PR lands.

**Audience:** New contributors, existing engineers adding a feature, upstream maintainers or fork contributors.

**Scope:** Local development setup, the project's contribution paths, and the PR verification checklist.

**Location:** Project root.

**Length:** 50–500 lines (flat budget shared by every document type).

## Rules

1. **Commands are verbatim.** Every shell command here must be copy-pasteable and produce the same result when run by a contributor as when run by CI.
2. **Steps are numbered and sequential.** A step that depends on a later step being done first breaks the chain — re-order so prerequisites come before dependents.
3. **Prerequisites are the ones the build fails without.** Optional tooling belongs in a separate note, not the setup list.
4. **Paths are relative to the repo root.** Never embed `/home/<user>/` or absolute host paths. Use `${PWD}` or `$(pwd)` when a script needs its own directory.
5. **No secrets or credentials.** Any configuration file shown must use placeholders (`<token>`, `<api-key>`), never real values.
6. **Templates carry comments, not filler.** A template with no guidance on what to fill and why is worse than no template.
7. **Respect the length budget.** Target 50–500 lines. Move verbose prose to linked sub-pages; keep this file scannable.
8. **Do not restate AGENTS.md.** The AGENTS.md file owns operational guardrails (security, worktree discipline, session memory). CONTRIBUTING.md owns the *what to do to ship a change*, not the *how to behave in a session*.
9. **Only include paths your project has.** A contributing guide names the directories and lifecycle steps that exist in this repo. If the project ships no submodules, no shared tool registry, and no generated manifests, those sections are deleted, not filled with placeholders.

## Section Contract

| Section | Why it belongs here |
|---------|---------------------|
| Contribution Overview | Sets contributor mindset and "ready to merge" definition. |
| Contribution Guidelines | The contribution rules every change must respect. |
| Workflow | `Step | Action | Detail` contribution pipeline. |
| Commit Convention | `Type | Usage | Example` conventional-commit prefixes. |
| Reference | Connects contribution policy to PRD and Architecture. |
