# ROADMAP Workflow

Follow these steps in order. The output must match the Template exactly.

## 0. Choose the entry scenario

The entry point depends on the starting state. In every scenario the ROADMAP.md is a progress plan: it states what ships next, never what the current source code does. The user's `<instruction>` overrides the code — code never edits the spec.

### Scenario 1 — Update existing ROADMAP.md

**When:** a `ROADMAP.md` already exists at the repo root and needs to be improved.
**Argument hint:** `--improve --docpath <docpath>`
`<docpath>` is the path to the existing `ROADMAP.md`. `<instruction>` describes what to change. The delegate reads the current ROADMAP.md, applies the improvement, and keeps every roadmap item ID stable; new items are only added when the improvement declares new deliverables. Never derive roadmap items from source code — apply the instruction against the existing spec.

**Special case — `--docpath all`:** improves every existing `ROADMAP.md` in the workspace in parallel. The orchestrator discovers all `ROADMAP.md` files at repo roots, then fans out one delegate subagent per `ROADMAP.md`, each applying the same `<instruction>` to its own ROADMAP.md while keeping that file's IDs stable. The same shared rule applies: no source-code derivation, instruction overrides code.

### Scenario 2 — Create new ROADMAP.md, no source code yet

**When:** the project has no implementation and no `ROADMAP.md`.
**Argument hint:** `--design --docpath <docpath>`
`<docpath>` is the target path for the new `ROADMAP.md`. `<instruction>` describes what ships next. The delegate enumerates the roadmap-item ID set from the instruction and the project's PRD/ARCHITECTURE context, then writes the ROADMAP.md purely from the promised behavior. No source code exists to consult, so the instruction is the sole source of truth.

### Scenario 3 — Create ROADMAP.md for a project that already has source code

**When:** source code exists but there is no `ROADMAP.md`, or the plan must be written independently of the build.
**Argument hint:** `--make --docpath <docpath>`
`<docpath>` is the target path for the new `ROADMAP.md`. `<instruction>` describes what ships next. Existing source code is reference only: it may inform naming of caller-observable identifiers (a CLI flag, an output line, an exit code), but the ROADMAP.md never names implementation detail — SQL schemas, API signatures, class names, library choices — and never states how the current build works. Where code and instruction conflict, the instruction wins.

## 1. Create file

`ROADMAP.md` at repo root.

## 2. Section: Progress Overview

one paragraph of progress summary.

## 3. Section: Roadmap Status

`PR-<PROJECT-NAME-ID>-NNN` items, each carrying Priority / State / Dependencies / Updated.

## 4. Section: Reference

link to PRD.md and ARCHITECTURE.md.

## 5. Verify

`lint-arwaky-cli docs` passes; every required section is present.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: master root presence, feature pairing, unknown states, missing sections.
```
