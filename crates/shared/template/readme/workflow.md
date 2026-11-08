# README Workflow

Follow these steps in order. The output must match the Template exactly.

## 0. Choose the entry scenario

The entry point depends on the starting state. In every scenario the README.md is a user guide: it states what the project does, never what the current source code does. The user's `<instruction>` overrides the code — code never edits the spec.

### Scenario 1 — Update existing README.md

**When:** a `README.md` already exists at the repo or package root and needs to be improved.
**Argument hint:** `--improve --docpath <docpath>`
`<docpath>` is the path to the existing `README.md`. `<instruction>` describes what to change. The delegate reads the current README.md, applies the improvement, and keeps every section stable; new sections are only added when the improvement declares new user-facing information. Never derive user-facing content from source code — apply the instruction against the existing spec.

**Special case — `--docpath all`:** improves every existing `README.md` in the workspace in parallel. The orchestrator discovers all `README.md` files at repo/package roots, then fans out one delegate subagent per `README.md`, each applying the same `<instruction>` to its own README.md while keeping that file's sections stable. The same shared rule applies: no source-code derivation, instruction overrides code.

### Scenario 2 — Create new README.md, no source code yet

**When:** the project has no implementation and no `README.md`.
**Argument hint:** `--design --docpath <docpath>`
`<docpath>` is the target path for the new `README.md`. `<instruction>` describes what the project does and who it serves. The delegate enumerates the section set from the instruction and the project's ROADMAP/ARCHITECTURE context, then writes the README.md purely from the promised behavior. No source code exists to consult, so the instruction is the sole source of truth.

### Scenario 3 — Create README.md for a project that already has source code

**When:** source code exists but there is no `README.md`, or the guide must be written independently of the build.
**Argument hint:** `--make --docpath <docpath>`
`<docpath>` is the target path for the new `README.md`. `<instruction>` describes what the project does and who it serves. Existing source code is reference only: it may inform naming of caller-observable identifiers (a CLI flag, an output line, an exit code), but the README.md never names implementation detail — SQL schemas, API signatures, class names, library choices — and never states how the current build works. Where code and instruction conflict, the instruction wins.

## 1. Create file

`README.md` at repo or package root.

## 2. Section: Project Overview

what this project does, who it serves, core value.

## 3. Section: Prerequisites

pinned build dependencies.

## 4. Section: Quick Start

`Step | Action | Command` table.

## 5. Section: Reference

required H2 with empty placeholder table.

## 6. Verify

`lint-arwaky-cli docs` passes; quick start works.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: readme-section-missing, dead-link, absolute-path, secret-in-docs, doc-length.
```
