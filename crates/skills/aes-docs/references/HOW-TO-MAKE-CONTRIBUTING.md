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
3. **Section: Principles** — the non-negotiable constraints every change must
   respect (architecture, storage, single source of truth).
4. **Section: Development Setup** — clone, prerequisites, install, verify.
5. **Section per contribution path** — each as a numbered, self-contained
   step pipeline.
6. **Section: Quality Verification & PR Process** — local gates, CI, PR
   checklist.
7. **Verify** → `aa check docs` passes; every command block runs without
   absolute personal paths; templates use placeholders.

## Template

Copy, fill, delete what does not apply to the project.

```markdown
# Contributing to <project-name>

Welcome. This document covers the contribution paths below. Only the paths
this project actually supports are present.

- [Development Setup](#development-setup)
- [<Path 1 name>](#<path-1-anchor>)
- [<Path 2 name>](#<path-2-anchor>)
- [Quality Verification & PR Process](#quality-verification--pr-process)

---

## Principles to Keep in Mind

Before making changes, observe these non-negotiable rules. Each rule states
the constraint, not the tool that enforces it:

1. **<Architecture model>**: <the structural rule every change must respect,
   e.g., layer boundaries, directory ownership, module placement>.
2. **<Storage / path compliance>**: <where persistent data and caches belong,
   and where they must never be written>.
3. **<Single source of truth>**: <the manifest, registry, or index that is the
   definitive record; every add/delete must update it>.

---

## Development Setup

1. Clone the repository:
   ```bash
   git clone <repo-url>
   cd <repo>
   ```
   Add `--recurse-submodules` only if the project has submodules.
2. Verify host prerequisites:
   ```bash
   <prerequisite-check command>
   ```
3. Provision the environment:
   ```bash
   <install command>
   ```
4. Verify installation:
   ```bash
   <status command>
   ```

---

## <Contribution Path 1>

<One line: what this path covers and when to use it.>

1. <Step: the first concrete action, with the exact command.>
2. <Step: the next action that depends on the previous one.>
3. <Step: the verification command that proves the change took effect.>
4. <Step: how to commit, including the conventional-commit prefix.>

Delete this whole section if the path does not exist in the project.

---

## <Contribution Path 2>

<Repeat the shape of Path 1 for the next distinct kind of change.>

---

## Quality Verification & PR Process

Before committing or opening a PR, clear every gate below.

### 1. Run the Verification Commands

```bash
<check command>
```

<One line naming what this runs: test suite, linter, type check, architecture
scanner.> CI mirrors this check on every push via `.github/workflows/ci.yml`.

### 2. Conventional Commit Guidelines

| Prefix      | Usage                                      |
| ----------- | ------------------------------------------ |
| `feat:`     | New feature or capability                  |
| `fix:`      | Bug fix                                    |
| `chore:`    | Maintenance, dependency bump, cleanup      |
| `docs:`     | Documentation changes                      |
| `refactor:` | Refactoring without behavioral change      |

### 3. Pull Request Checklist

- [ ] All local verification commands pass.
- [ ] New files follow the project's naming and directory conventions.
- [ ] Any registry or manifest the project keeps is updated.
- [ ] No absolute paths, secrets, or machine-specific values leaked into the diff.
- [ ] The PR description names the contribution path and links the issue.
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one reason.

| Section                   | Why it belongs here                                                    |
| ------------------------- | -------------------------------------------------------------------- |
| Intro / Table of Contents | Lets contributors jump to their path. Watch for a wall of text with no navigation. |
| Principles                | Sets the non-negotiable boundary before any code is touched. Watch for rules that name tools instead of constraints. |
| Development Setup         | The 10-minute promise — clone, install, verify. Watch for steps that assume prior state. |
| One section per contribution path | Each names a distinct kind of change with a self-contained step pipeline. Watch for paths this project does not actually support. |
| Quality Verification & PR Process | The gate that protects CI. Watch for commands that differ from the CI job. |

---

## Verify

```bash
aa check docs .
# Checks: dead-link, absolute-path, secret-in-docs, doc-length.
# Manual: every command runs; templates use placeholders; no absolute personal paths.
```
