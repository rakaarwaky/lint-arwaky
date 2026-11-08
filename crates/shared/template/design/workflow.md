# DESIGN Workflow

Follow these steps in order. The output must match the Template exactly.

## 0. Choose the entry scenario

The entry point depends on the starting state. In every scenario the DESIGN.md is a design contract: it states what the design looks like, never what the current source code does. The user's `<instruction>` overrides the code — code never edits the spec.

### Scenario 1 — Update existing DESIGN.md

**When:** a `DESIGN.md` already exists in the surface/ folder and needs to be improved.
**Argument hint:** `--improve --docpath <docpath>`
`<docpath>` is the path to the existing `DESIGN.md`. `<instruction>` describes what to change. The delegate reads the current DESIGN.md, applies the improvement, and keeps every design-requirement ID stable; new IDs are only added when the improvement declares new design requirements. Never derive design requirements from source code — apply the instruction against the existing spec.

**Special case — `--docpath all`:** improves every existing `DESIGN.md` in the workspace in parallel. The orchestrator discovers all `DESIGN.md` files under surface/ folders, then fans out one delegate subagent per `DESIGN.md`, each applying the same `<instruction>` to its own DESIGN.md while keeping that file's IDs stable. The same shared rule applies: no source-code derivation, instruction overrides code.

### Scenario 2 — Create new DESIGN.md, no source code yet

**When:** the surface has no implementation and no `DESIGN.md`.
**Argument hint:** `--design --docpath <docpath>`
`<docpath>` is the target path for the new `DESIGN.md`. `<instruction>` describes what the design looks like. The delegate enumerates the design-requirement ID set from the instruction and the feature's PRD/BACKLOG/ROADMAP/ARCHITECTURE context, then writes the DESIGN.md purely from the promised behavior. No source code exists to consult, so the instruction is the sole source of truth.

### Scenario 3 — Create DESIGN.md for a project that already has source code

**When:** source code exists but there is no `DESIGN.md`, or the contract must be written independently of the build.
**Argument hint:** `--make --docpath <docpath>`
`<docpath>` is the target path for the new `DESIGN.md`. `<instruction>` describes what the design looks like. Existing source code is reference only: it may inform naming of caller-observable identifiers (error codes, event names, output paths), but the DESIGN.md never names `.py`/`.rs`/`.ts`/`.tsx` files and never states how the current build works. Where code and instruction conflict, the instruction wins.

## 1. Create file

`DESIGN.md` in the surface/ folder.

## 2. Section: Design Overview

one paragraph of what the design looks like.

## 3. Section: Design Requirements

one H3 per requirement, `### DS-<DESIGN-NAME-ID>-NNN: <short imperative name>`; fill the item's table.

## 4. Section: Reference

link to PRD.md, BACKLOG.md, ROADMAP.md, ARCHITECTURE.md.

## 5. Verify

`lint-arwaky-cli docs` passes; section order is correct.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: IDs, orphan refs, section order, status leak, links.
```
