# HOW TO MAKE [AGENTS.md](http://AGENTS.md)

---
trigger: always
description: "{Project} operational guide."
---

## Rules

1. **Commands must match CI.** Every printed command must be
   copy-pasteable and identical to the CI gate.
1. **No absolute personal paths.** Use `$HOME`, `${workspaceFolder}`,
   or repo-relative paths (`absolute-path`).
1. **No secrets or credentials.** Never put tokens, passwords, or
   keys in this file.
1. **Do not restate to other documents.** Use link instead.
1. **Mark optional sections clearly.** Do not force fake sections
   like pipeline diagrams just to fill a template.
1. **Use** `main`, `.worktrees/`.
1. **Respect the length budget.** Target 50–500 lines.

---

## Workflow

1. **Determine context** — Agent config for single tool or multi-agent system.
1. **Write frontmatter** — name, description, persona, tools.
1. **Write AGENTS.md** — behavior rules, guardrails, response format.
1. **Verify** → validate with command 'lint-arwaky docs'

## Template

Copy, fill, delete nothing.

```markdown
# AGENTS — <project-name>

## Agent Overview

<One paragraph: what AI agents must know before generating code, docs, or suggestions in this workspace.>

## Runtime

| Aspect          | Detail                                          |
|-----------------|-------------------------------------------------|
| Language        | <language and pinned version>                   |
| Environment     | <environment manager and creation method>       |
| Artifacts       | <reviewable artifact location>                  |
| Package Manager | <package manager and lockfile>                  |
| Version Probe   | `<version probe command>`                       |
| Setup           | `<environment setup command>`                   |
| Activation      | `<optional environment activation command>`     |

## Project Structure

| Path       | Purpose              |
|------------|----------------------|
| <path>     | <what lives here>    |
| <path>     | <what lives here>    |
| <path>     | <what lives here>    |

## Commands

| Category    | Command                              | Scope      | CI Gate              |
|-------------|--------------------------------------|------------|----------------------|
| Test (all)  | `<whole-workspace test command>`     | All        | ci.yml: <job name>   |
| Test (unit) | `<single-package test command>`      | One unit   | ci.yml: <job name>   |
| Test (file) | `<single-file test command>`         | One file   | ci.yml: <job name>   |
| Lint        | `<formatter or linter command>`      | All        | ci.yml: <job name>   |
| Type Check  | `<type checker command>`             | All        | ci.yml: <job name>   |
| Arch Scan   | `<architecture scanner command>`     | All        | ci.yml: <job name>   |

## Documentation Awareness

| Document | Agent Must                              |
|----------|-----------------------------------------|
| PRD      | Read before implementing any `PR-` item | 
| FRD      | Follow `FR-` spec exactly               | 
| DATA     | Respect `DT-` shapes and types          | 
| DESIGN   | Use `DS-` tokens and variants           | 
| ROADMAP  | Check `State` before starting work      | 
| BACKLOG  | Update status after each task           | 

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | <Business goals, user stories, and scope>  | [PRD.md](PRD.md) |
| FRD          | <Functional requirements specification>    | [FRD.md](FRD.md) |
| DATA         | <Data shapes and kernel definitions>       | [DATA.md](DATA.md) |
| DESIGN       | <UI/UX and visual system>                  | [DESIGN.md](DESIGN.md) |
| Architecture | <System boundaries, tech stack, and infra> | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason. The H2 set is closed by AES605: the six required H2s are
`Agent Overview`, `Runtime`, `Project Structure`, `Commands`,
`Documentation Awareness`, `Reference`. Global agent
behavior (autonomy, safety, work loop, merge guard) is carried in the
global agent guide (`~/.qwen/QWEN.md` or equivalent) and is not
repeated as H2s in `AGENTS.md`.

| Section               | Why it belongs here                                                                                          |
| --------------------- | ------------------------------------------------------------------------------------------------------------ |
| Frontmatter           | Loads the file unconditionally in rule-style harnesses. Skip when harness discovers plain `AGENTS.md`.       |
| Agent Overview        | Tells the agent what it must know before generating code, docs, or suggestions in this workspace.            |
| Runtime               | Language, Environment, Artifacts, Package Manager, Version Probe, Setup, Activation — `Aspect | Detail`.   |
| Project Structure     | `Path | Purpose` per crate/workspace member, so the next session navigates the tree without crawling it.    |
| Commands              | `Category | Command | Scope | CI Gate` — every gate verbatim as CI runs it. Highest-value section. Never skip. |
| Documentation Awareness| `Document | Agent Must` — maps each spec doc to the agent's duty toward it.                                |
| Reference             | `Artifact | Purpose | Link` — PRD, FRD, DATA, DESIGN, Architecture.                                      |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: agents-section-missing, ci-command-drift, absolute-path, secret-in-docs, dead-link, doc-length.

```
