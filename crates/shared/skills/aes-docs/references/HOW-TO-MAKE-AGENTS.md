---
trigger: always
description: "{Project} operational guide."
---

# HOW TO MAKE [AGENTS.md](http://AGENTS.md)

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

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| README       | What the tool does, every command, install | [README.md](README.md) |
| CONTRIBUTING | Branch, commit, and PR conventions         | [CONTRIBUTING.md](CONTRIBUTING.md) |
| ARCHITECTURE | Layer contract for every crate             | [ARCHITECTURE.md](ARCHITECTURE.md) |
| ROADMAP      | What ships next, in order                  | [ROADMAP.md](ROADMAP.md) |
| PRD          | What the product does and why              | [PRD.md](PRD.md) |
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason. The H2 set is closed by AES605: the five required H2s are
`Agent Overview`, `Runtime`, `Project Structure`, `Commands`, `Reference`. Global agent
behavior (autonomy, safety, work loop, merge guard) is carried in the
global agent guide (`~/.qwen/QWEN.md` or equivalent) and is not
repeated as H2s in `AGENTS.md`.

| Section               | Why it belongs here                                                                                          |
| --------------------- | ------------------------------------------------------------------------------------------------------------ |
| Frontmatter           | Loads the file unconditionally in rule-style harnesses. Skip when harness discovers plain `AGENTS.md`.       |
| Agent Overview        | Tells the agent what it must know before generating code, docs, or suggestions in this workspace.            |
| Runtime               | Language, Environment, Artifacts, Package Manager, Version Probe, Setup, Activation — `Aspect &#124; Detail`.   |
| Project Structure     | `Path &#124; Purpose` per crate/workspace member, so the next session navigates the tree without crawling it.    |
| Commands              | `Category &#124; Command &#124; Scope &#124; CI Gate` — every gate verbatim as CI runs it. Highest-value section. Never skip. |
| Reference             | `Artifact &#124; Purpose &#124; Link` — README, CONTRIBUTING, ARCHITECTURE, ROADMAP, PRD.                                      |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: agents-section-missing, ci-command-drift, absolute-path, secret-in-docs, dead-link, doc-length.

```
