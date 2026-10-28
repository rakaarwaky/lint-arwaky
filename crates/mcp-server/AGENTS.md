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

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| README       | What the tool does, every command, install | [README.md](README.md) |
| CONTRIBUTING | Branch, commit, and PR conventions         | [CONTRIBUTING.md](CONTRIBUTING.md) |
| ARCHITECTURE | Layer contract for every crate             | [ARCHITECTURE.md](ARCHITECTURE.md) |
| ROADMAP      | What ships next, in order                  | [ROADMAP.md](ROADMAP.md) |
| PRD          | What the product does and why              | [PRD.md](PRD.md) |
