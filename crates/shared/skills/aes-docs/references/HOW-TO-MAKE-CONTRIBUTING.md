# HOW TO MAKE CONTRIBUTING.md

> **Purpose**: Tell anyone who wants to add code or documentation to the
> project exactly how to set up, where to work, and what gates they must clear
> before the PR lands.
>
> **Audience**: New contributors, existing engineers adding a feature, upstream
> maintainers or fork contributors.
>
> **Scope**: Local development setup, the project's contribution paths, and the
> PR verification checklist.
>
> **Location**: Project root.
>
> **Length**: 50–500 lines (flat budget shared by every document type).

---

## Rules

1. **Commands are verbatim.** Every shell command here must be copy-pasteable
   and produce the same result when run by a contributor as when run by CI.
2. **Steps are numbered and sequential.** A step that depends on a later step
   being done first breaks the chain — re-order so prerequisites come before
   dependents.
3. **Prerequisites are the ones the build fails without.** Optional tooling
   belongs in a separate note, not the setup list.
4. **Paths are relative to the repo root.** Never embed `/home/<user>/` or
   absolute host paths. Use `${PWD}` or `$(pwd)` when a script needs its own
   directory.
5. **No secrets or credentials.** Any configuration file shown must use
   placeholders (`<token>`, `<api-key>`), never real values.
6. **Templates carry comments, not filler.** A template with no guidance on
   what to fill and why is worse than no template.
7. **Respect the length budget.** Target 50–500 lines. Move verbose prose to
   linked sub-pages; keep this file scannable.
8. **Do not restate AGENTS.md.** The AGENTS.md file owns operational guardrails
   (security, worktree discipline, session memory). CONTRIBUTING.md owns the
   *what to do to ship a change*, not the *how to behave in a session*.
9. **Only include paths your project has.** A contributing guide names the
   directories and lifecycle steps that exist in this repo. If the project
   ships no submodules, no shared tool registry, and no generated manifests,
   those sections are deleted, not filled with placeholders.

---

## Workflow

1. **Identify the contribution paths.** Before writing, list the distinct
   kinds of change this project accepts — a new feature module, a bug fix, a
   vendored/upstream dependency, a tooling change, or a docs-only update.
   One section per path. Delete the ones that do not apply.
2. **Create file** → `CONTRIBUTING.md` at repo root.
3. **Section: Contribution Overview** — how to contribute, what mindset
   contributors should have, and what "ready to merge" looks like.
4. **Section: Contribution Guidelines** — contribution rules.
5. **Section: Workflow** — `Step | Action | Detail` contribution pipeline.
6. **Section: Commit Convention** — `Type | Usage | Example` conventional
   commit prefixes.
7. **Section: Reference** — link to PRD.md and ARCHITECTURE.md.
8. **Verify** → `lint-arwaky-cli docs` passes; every command block runs without
   absolute personal paths; templates use placeholders.

## Template

Copy, fill, delete what does not apply to the project.

```markdown
# CONTRIBUTING — <project-name>

## Contribution Overview

<One paragraph: how to contribute, what mindset contributors should have, and what "ready to merge" looks like.>

## Contribution Guidelines

## Workflow

| Step | Action | Detail |
|------|--------|--------|
| 1 | Fork / Branch | `<branch-naming-convention>` |
| 2 | Implement | Follow FRD spec, write tests |
| 3 | Self-review | Check against acceptance criteria in PRD |
| 4 | Submit PR | Link to BACKLOG ID, describe what changed |
| 5 | Review | <reviewer / approval criteria> |
| 6 | Merge | <merge strategy: squash / rebase / merge> |

## Commit Convention

| Type | Usage | Example |
|------|-------|---------|
| `feat` | New feature | `feat(auth): add login endpoint` |
| `fix` | Bug fix | `fix(auth): handle expired token` |
| `docs` | Documentation only | `docs(frd): update FR-AUTH-001` |
| `test` | Adding/updating tests | `test(auth): add edge case for timeout` |
| `refactor` | Code refactor, no behaviour change | `refactor(auth): extract middleware` |
| `chore` | Maintenance, deps, config | `chore: bump dependencies` |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | <Business goals, user stories, and scope> | [PRD.md](PRD.md) |
| Architecture | <System boundaries, tech stack, and infra> | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.

| Section              | Why it belongs here                                                     |
| -------------------- | ----------------------------------------------------------------------- |
| Contribution Overview| Sets contributor mindset and "ready to merge" definition.               |
| Contribution Guidelines | The contribution rules every change must respect.                      |
| Workflow             | `Step &#124; Action &#124; Detail` contribution pipeline.                             |
| Commit Convention    | `Type &#124; Usage &#124; Example` conventional-commit prefixes.                         |
| Reference            | Connects contribution policy to PRD and Architecture.                   |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: dead-link, absolute-path, secret-in-docs, doc-length.
# Manual: every command runs; templates use placeholders; no absolute personal paths.
```
