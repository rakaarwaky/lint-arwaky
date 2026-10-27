# HOW TO MAKE README.md

> **Purpose**: Answer *how do I get this running, and where do I look
> next*.
>
> **Audience**: A developer who just cloned the repo, possibly you in six
> months.
>
> **Scope**: Onboarding: prerequisites, quick start, commands.
>
> **Location**: Project root.
>
> **Length**: 50–500 lines (flat budget shared by every document type).

---

## Rules

1. **The test is time, not completeness.** Clone → install → build → run
   in under 10 minutes. If the Quick Start cannot clear that, the
   missing step is the bug in this file.
2. **Every command was actually run, and says how to confirm it worked.**
   A reader who cannot tell success from silence will assume failure and
   go ask a human.
3. **Prerequisites are the ones the build fails without**, with versions
   pinned. Optional tooling listed as required teaches readers to
   distrust the list.

---

## Workflow

1. **Create file** → `README.md` at repo or package root.
2. **Section: Project Overview** — what this project does, who it serves,
   core value.
3. **Section: Prerequisites** — pinned build dependencies.
4. **Section: Quick Start** — `Step | Action | Command` table.
5. **Section: Reference** — required H2 with empty placeholder table.
6. **Verify** → `lint-arwaky-cli docs` passes; quick start works.

## Template

Copy, fill, delete nothing.

```markdown
# README — <project-name>

## Project Overview

<One paragraph: what this project does, who it serves, and the core value it delivers.>

## Prerequisites

- <requiretment>
- <requiretment>

## Quick Start

| Step | Action | Command  |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.

| Section         | Why it belongs here                                                     |
| --------------- | ----------------------------------------------------------------------- |
| Project Overview| Decides in five seconds whether to keep reading.                        |
| Prerequisites   | Prevents a build failure with no cause. Watch for optional tooling listed as required. |
| Quick Start     | The 10-minute promise, as `Step | Action | Command`.                 |
| Reference       | Required H2; carries an empty `| Artifact | Purpose | Link |` table when no artifact is mandated. |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: readme-section-missing, dead-link, absolute-path, secret-in-docs, doc-length.
```
