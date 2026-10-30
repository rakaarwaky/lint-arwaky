# DATA Workflow

Follow these steps in order. The output must match the Template exactly.

## 0. Choose the entry scenario

The entry point depends on the starting state. In every scenario the DATA.md is a data contract: it states what the kernel manages, never what the current source code does. The user's `<instruction>` overrides the code — code never edits the spec.

### Scenario 1 — Update existing DATA.md
**When:** a `DATA.md` already exists in the shared/ directory and needs to be improved.
**Argument hint:** `--improve --docpath <docpath>`
`<docpath>` is the path to the existing `DATA.md`. `<instruction>` describes what to change. The delegate reads the current DATA.md, applies the improvement, and keeps every data-concept ID stable; new IDs are only added when the improvement declares new data concepts. Never derive data concepts from source code — apply the instruction against the existing spec.

**Special case — `--docpath all`:** improves every existing `DATA.md` in the workspace in parallel. The orchestrator discovers all `DATA.md` files under shared/ directories, then fans out one delegate subagent per `DATA.md`, each applying the same `<instruction>` to its own DATA.md while keeping that file's IDs stable. The same shared rule applies: no source-code derivation, instruction overrides code.

### Scenario 2 — Create new DATA.md, no source code yet
**When:** the kernel has no implementation and no `DATA.md`.
**Argument hint:** `--design --docpath <docpath>`
`<docpath>` is the target path for the new `DATA.md`. `<instruction>` describes what the kernel manages. The delegate enumerates the data-concept ID set from the instruction and the feature's PRD/ROADMAP/DESIGN context, then writes the DATA.md purely from the promised behavior. No source code exists to consult, so the instruction is the sole source of truth.

### Scenario 3 — Create DATA.md for a project that already has source code
**When:** source code exists but there is no `DATA.md`, or the contract must be written independently of the build.
**Argument hint:** `--make --docpath <docpath>`
`<docpath>` is the target path for the new `DATA.md`. `<instruction>` describes what the kernel manages. Existing source code is reference only: it may inform naming of caller-observable identifiers (error codes, event names, output paths), but the DATA.md never names `.py`/`.rs`/`.ts`/`.tsx` files and never states how the current build works. Where code and instruction conflict, the instruction wins.

## 1. Create file
`DATA.md` in the shared/ directory.

## 2. Section: Data Overview
one paragraph: what the kernel manages, who reads/writes it.

## 3. Items: `### DT-SHARED-NNN: <short imperative name>`
one H3 per data concept under `## Data Requirements`; fill the item's Type/Description table.

## 4. Section: Reference
link to PRD.md, BACKLOG.md, ROADMAP.md, ARCHITECTURE.md.

## 5. Verify
`lint-arwaky-cli docs` passes; section order is correct.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: IDs, orphan refs, section order, status leak, links.
```
