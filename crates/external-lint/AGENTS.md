---
trigger: always
description: "Lint Arwaky operational guide."
---
# AGENTS — Lint Arwaky

## Agent Overview

{One paragraph: what AI agents must know before generating code, docs, or suggestions in this workspace.}

## Runtime

| Aspect          | Detail                                          |
|-----------------|-------------------------------------------------|
| Language        | {language and pinned version}                   |
| Environment     | {environment manager and creation method}       |
| Artifacts       | {reviewable artifact location}                  |
| Package Manager | {package manager and lockfile}                  |
| Version Probe   | `{version probe command}`                       |
| Setup           | `{environment setup command}`                    |
| Activation      | `{optional environment activation command}`      |

## Project Structure

| Path       | Purpose              |
|------------|----------------------|
| {path}     | {what lives here}    |

## Commands

| Category    | Command                              | Scope      | CI Gate              |
|-------------|--------------------------------------|------------|----------------------|
| Test (all)  | `{whole-workspace test command}`     | All        | ci.yml: {job name}   |
| Test (unit) | `{single-package test command}`      | One unit   | ci.yml: {job name}   |
| Test (file) | `{single-file test command}`         | One file   | ci.yml: {job name}   |
| Lint        | `{formatter or linter command}`      | All        | ci.yml: {job name}   |
| Type Check  | `{type checker command}`             | All        | ci.yml: {job name}   |
| Arch Scan   | `{architecture scanner command}`     | All        | ci.yml: {job name}   |

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
| PRD          | {Business goals, user stories, and scope}  | [PRD.md](PRD.md) |
| FRD          | {Functional requirements specification}    | [FRD.md](FRD.md) |
| DATA         | {Data shapes and layer definitions}        | [DATA.md](DATA.md) |
| DESIGN       | {UI/UX and visual system}                  | [DESIGN.md](DESIGN.md) |
| Architecture | {System boundaries, tech stack, and infra} | [ARCHITECTURE.md](ARCHITECTURE.md) |
