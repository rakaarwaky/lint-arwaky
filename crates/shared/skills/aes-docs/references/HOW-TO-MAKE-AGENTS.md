# HOW TO MAKE [AGENTS.md](http://AGENTS.md)

> **Purpose**: Tell the AI agent how to work safely, what commands to run,
> and what counts as done.
>
> **Audience**: The AI agent in the next session
>
> **Scope**: Operational constraints, commands, security, and git
> workflows.
>
> **Location**: Project root.
>
> **Length**: 50–500 lines
>

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

````markdown
---
trigger: always
description: "{Project} operational guide."
---
# {Project Name}

## Runtime

- Language: {language and pinned version}
- Environment: {environment manager and creation method}
- Artifacts: {reviewable artifact location}
- Package manager: {package manager and lockfile}

```bash
{version probe}
{environment setup}
{optional environment activation}
```

## Project Quick Facts

{project quick facts}

## Pipeline

{stage A} → {stage B} → {stage C} → {stage D}

## Project Structure

{project structure}

## Commands

Every command must match the exact CI gate.

```bash
# Tests
{whole-workspace test command}                      # what it covers
{single-package test command}                       # one unit
{single-file test command}                          # one file

# Lint / types / architecture
{formatter or linter command}                       # matches ci.yml {job name}
{type checker command}                              # exact config-file flags
{architecture scanner command}
```

## Related Documents

- {link}: {what the document answers}

---
````

## Section Contract

Every section is required unless marked optional. Each exists for one
reason. The H2 set is closed by AES605: the five required H2s are
`Runtime`, `Project Quick Facts`, `Pipeline`,
`Commands`, `Related Documents`. Global agent
behavior (autonomy, safety, work loop, merge guard) is carried in the
global agent guide (`~/.qwen/QWEN.md` or equivalent) and is not
repeated as H2s in `AGENTS.md`.

| Section             | Why it belongs here                                                                                          |
| ------------------- | ------------------------------------------------------------------------------------------------------------ |
| Frontmatter         | Loads the file unconditionally in rule-style harnesses. Skip when harness discovers plain `AGENTS.md`.       |
| Runtime             | Version pin + env isolation + package manager. Skip when nothing here is version-pinned.                     |
| Project Quick Facts | Free-form slot: any facts the project wants to surface to the next session. Fill the `{project quick facts}` slot; leave one blank line if there are none. |
| Pipeline            | Stage order and controller in one glance. Skip when not pipeline-shaped.                                     |
| Project Structure   | {why: shows the crate/workspace layout one line per member, so the next session navigates the tree without crawling it. Watch for every generated file or scratch path listed; the tree must match the real layout} |
| Commands            | The gates, verbatim as CI runs them. Highest-value section. Never skip.                                      |
| Related Documents   | One line per doc: what it answers. Never skip.                                                               |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: agents-section-missing, ci-command-drift, absolute-path, secret-in-docs, dead-link, doc-length.

```
