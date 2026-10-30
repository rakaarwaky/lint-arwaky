# BACKLOG Workflow

Follow these steps in order. The output must match the Template exactly.

## 0. Choose the entry scenario

The entry point depends on the starting state. In every scenario the BACKLOG.md is a tracking plan: it states what remains to be done, never what the current source code does. The user's `<instruction>` overrides the code — code never edits the spec.

### Scenario 1 — Update existing BACKLOG.md
**When:** a `BACKLOG.md` already exists in the feature directory and needs to be improved.
**Argument hint:** `--improve --docpath <docpath>`
`<docpath>` is the path to the existing `BACKLOG.md`. `<instruction>` describes what to change. The delegate reads the current BACKLOG.md, applies the improvement, and keeps every item ID stable; new items are only added when the improvement declares new deliverables. Never derive tracking items from source code — apply the instruction against the existing spec.

**Special case — `--docpath all`:** improves every existing `BACKLOG.md` in the workspace in parallel. The orchestrator discovers all `BACKLOG.md` files under feature directories, then fans out one delegate subagent per `BACKLOG.md`, each applying the same `<instruction>` to its own BACKLOG.md while keeping that file's IDs stable. The same shared rule applies: no source-code derivation, instruction overrides code.

### Scenario 2 — Create new BACKLOG.md, no source code yet
**When:** the feature has no implementation and no `BACKLOG.md`.
**Argument hint:** `--design --docpath <docpath>`
`<docpath>` is the target path for the new `BACKLOG.md`. `<instruction>` describes what must be done. The delegate enumerates the item ID set from the instruction and the feature's PRD/FRD/ROADMAP/DESIGN context, then writes the BACKLOG.md purely from the promised behavior. No source code exists to consult, so the instruction is the sole source of truth.

### Scenario 3 — Create BACKLOG.md for a project that already has source code
**When:** source code exists but there is no `BACKLOG.md`, or the plan must be written independently of the build.
**Argument hint:** `--make --docpath <docpath>`
`<docpath>` is the target path for the new `BACKLOG.md`. `<instruction>` describes what must be done. Existing source code is reference only: it may inform naming of caller-observable identifiers (error codes, event names, output paths), but the BACKLOG.md never names `.py`/`.rs`/`.ts`/`.tsx` files and never states how the current build works. Where code and instruction conflict, the instruction wins.

## 1. Create file
`BACKLOG.md` in feature directory.

## 2. Section: Progress Overview
document baseline state.

## 3. Section: Backlog Status
`{<DATA-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}` items, each carrying State / Done / In Progress / Risk / Updated.

## 4. Section: Reference
link to PRD.md, FRD.md, ROADMAP.md, ARCHITECTURE.md.

## 5. Verify
`lint-arwaky-cli docs` passes; all sections present.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: pairing, columns, unknown states, unevidenced Done, scenario coverage.
```
