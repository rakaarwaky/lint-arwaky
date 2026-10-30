# CONTRIBUTING Workflow

Follow these steps in order. The output must match the Template exactly.

## 0. Choose the entry scenario

The entry point depends on the starting state. In every scenario the CONTRIBUTING.md is a contribution contract: it states how changes are accepted, never what the current source code does. The user's `<instruction>` overrides the code — code never edits the spec.

### Scenario 1 — Update existing CONTRIBUTING.md
**When:** a `CONTRIBUTING.md` already exists at the project root and needs to be improved.
**Argument hint:** `--improve --docpath <docpath>`
`<docpath>` is the path to the existing `CONTRIBUTING.md`. `<instruction>` describes what to change. The delegate reads the current CONTRIBUTING.md, applies the improvement, and keeps every section stable; new sections are only added when the improvement declares new contribution paths. Never derive contribution rules from source code — apply the instruction against the existing spec.

**Special case — `--docpath all`:** improves every existing `CONTRIBUTING.md` in the workspace in parallel. The orchestrator discovers all `CONTRIBUTING.md` files at project roots, then fans out one delegate subagent per `CONTRIBUTING.md`, each applying the same `<instruction>` to its own CONTRIBUTING.md while keeping that file's sections stable. The same shared rule applies: no source-code derivation, instruction overrides code.

### Scenario 2 — Create new CONTRIBUTING.md, no source code yet
**When:** the project has no implementation and no `CONTRIBUTING.md`.
**Argument hint:** `--design --docpath <docpath>`
`<docpath>` is the target path for the new `CONTRIBUTING.md`. `<instruction>` describes how contributions are accepted. The delegate enumerates the contribution-path set from the instruction and the project's ROADMAP/ARCHITECTURE context, then writes the CONTRIBUTING.md purely from the promised behavior. No source code exists to consult, so the instruction is the sole source of truth.

### Scenario 3 — Create CONTRIBUTING.md for a project that already has source code
**When:** source code exists but there is no `CONTRIBUTING.md`, or the contract must be written independently of the build.
**Argument hint:** `--make --docpath <docpath>`
`<docpath>` is the target path for the new `CONTRIBUTING.md`. `<instruction>` describes how contributions are accepted. Existing source code is reference only: it may inform naming of caller-observable identifiers (a CLI flag, an output line, an exit code), but the CONTRIBUTING.md never names implementation detail — SQL schemas, API signatures, class names, library choices — and never states how the current build works. Where code and instruction conflict, the instruction wins.

## 1. Identify the contribution paths
Before writing, list the distinct kinds of change this project accepts — a new feature module, a bug fix, a vendored/upstream dependency, a tooling change, or a docs-only update. One section per path. Delete the ones that do not apply.

## 2. Create file
`CONTRIBUTING.md` at repo root.

## 3. Section: Contribution Overview
how to contribute, what mindset contributors should have, and what "ready to merge" looks like.

## 4. Section: Contribution Guidelines
contribution rules.

## 5. Section: Workflow
`Step | Action | Detail` contribution pipeline.

## 6. Section: Commit Convention
`Type | Usage | Example` conventional commit prefixes.

## 7. Section: Reference
link to PRD.md and ARCHITECTURE.md.

## 8. Verify
`lint-arwaky-cli docs` passes; every command block runs without absolute personal paths; templates use placeholders.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: dead-link, absolute-path, secret-in-docs, doc-length.
# Manual: every command runs; templates use placeholders; no absolute personal paths.
```
