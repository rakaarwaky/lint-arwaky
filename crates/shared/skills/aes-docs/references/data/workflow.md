# DATA Workflow

Follow these steps in order. The output must match the Template exactly, and the result must satisfy every rule in `rules.md`. The `exemplar.md` file shows a filled DATA.md to calibrate against.

## 0. Choose the entry scenario

The entry point depends on the starting state. In every scenario the DATA.md is a data contract: it states what the shared layer manages, never what the current source code does. The user's `<instruction>` overrides the code — code never edits the spec. No DATA.md is placed outside a `shared/` folder.

### Scenario 1 — Update existing DATA.md

**When:** a `DATA.md` already exists in the shared/ directory and needs to be improved.
**Argument hint:** `--improve --docpath <docpath>`

`<docpath>` is the path to the existing `DATA.md`. `<instruction>` describes what to change. The delegate reads the current DATA.md, applies the improvement, and keeps every existing data-concept ID stable; new IDs are only added when the instruction declares new data concepts. Existing IDs are never removed, and their titles change only when the instruction explicitly renames them. The section order from the template and the AES rules are preserved. No source-code derivation: the instruction is applied against the existing spec.

**Special case — `--docpath all`:** improves every existing `DATA.md` in the workspace in parallel. The orchestrator discovers all `DATA.md` files under `shared/` directories, then fans out one delegate subagent per `DATA.md`. Each delegate applies the same `<instruction>` to its own DATA.md while keeping that file's IDs stable. The same shared rule applies to every delegate: no source-code derivation, instruction overrides code. After all delegates report, the orchestrator re-runs Verify on every touched file before completing.

### Scenario 2 — Create new DATA.md, no source code yet

**When:** the shared layer has no implementation and no `DATA.md`.
**Argument hint:** `--design --docpath <docpath>`

`<docpath>` is the target path for the new `DATA.md`, inside a `shared/` directory. `<instruction>` describes what the shared layer manages. The delegate enumerates the data-concept ID set from the instruction and the feature's PRD/ROADMAP/DESIGN context, then writes the DATA.md purely from the promised behavior. No source code exists to consult, so the instruction is the sole source of truth. The document must be paired with a `BACKLOG.md` in the same folder so that every `DT-SHARED-NNN` ID has a matching row (AES601 backlog alignment).

### Scenario 3 — Create DATA.md for a project that already has source code

**When:** source code exists but there is no `DATA.md`, or the contract must be written independently of the build.
**Argument hint:** `--make --docpath <docpath>`

`<docpath>` is the target path for the new `DATA.md`, inside a `shared/` directory. `<instruction>` describes what the shared layer manages. Existing source code is reference only: it may inform naming of caller-observable identifiers (error codes, event names, output paths), but the DATA.md never names `.py`/`.rs`/`.ts`/`.tsx` files, class names, or type signatures, and never states how the current build works. Where code and instruction conflict, the instruction wins. The document must be paired with a `BACKLOG.md` in the same folder so that every `DT-SHARED-NNN` ID has a matching row.

## 1. Create file

Create `DATA.md` at `<docpath>` inside the crate's `shared/` directory.

## 2. Section: Data Overview

One paragraph: what data shapes the shared layer manages, who reads and writes them, and what business domain they serve. No code.

## 3. Items: `### DT-SHARED-NNN: <short imperative name>`

One H3 per data concept under `## Data Requirements`. Number IDs sequentially from `001` and keep titles short and imperative. Each item owns a `| Type | Description |` table with the concept name and a plain-language description of the value object.

## 4. Section: Reference

Link to `PRD.md`, `BACKLOG.md`, `ROADMAP.md`, and `ARCHITECTURE.md` in the Reference table, each with its purpose and a relative link.

## 5. Verify

Run `lint-arwaky-cli docs` on the crate containing the DATA.md. The document passes when:

- All `DT-SHARED-NNN` IDs are well-formed and every declared ID has a matching row in the same folder's `BACKLOG.md`.
- Section order matches the template: `Data Overview`, `Data Requirements`, `Reference`.
- No source-code identifiers, class names, or type signatures appear in the document.
- No `shared/` folder outside the target carries a DATA.md in this run.

```bash
lint-arwaky-cli docs .
# Checks: IDs, orphan refs, section order, status leak, links.
```
