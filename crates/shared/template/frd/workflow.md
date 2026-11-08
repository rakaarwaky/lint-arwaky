# FRD Workflow

Follow these steps in order. The output must match the Template exactly.

The FRD is a **design specification**, not passive documentation. Every requirement is written as a prescriptive, testable obligation ("shall") that tells an engineer exactly what to build and tells QA exactly what to test — without a follow-up question to the author. The user's `<instruction>` overrides the code — code never edits the spec.

## 0. Choose the entry scenario

The entry point depends on the starting state. In every scenario the FRD states what the feature **must do**, never what the current source code does.

### Scenario 1 — Update existing FRD

**When:** an `FRD.md` already exists for the feature and needs to be improved.
**Argument hint:** `--improve --docpath <docpath>`
`<docpath>` is the path to the existing `FRD.md`. `<instruction>` describes what to change. The delegate reads the current FRD, applies the improvement, and keeps every FR ID stable; new FR IDs are only added when the improvement declares new deliverables. Never derive requirements from source code — apply the instruction against the existing spec.

**Special case — `--docpath all`:** improves every existing `FRD.md` in the workspace in parallel. The orchestrator discovers all `FRD.md` files under feature directories, then fans out one delegate subagent per `FRD.md`, each applying the same `<instruction>` to its own FRD while keeping that FRD's IDs stable. The same shared rule applies: no source-code derivation, instruction overrides code.

### Scenario 2 — Create new FRD, no source code yet

**When:** the feature has no implementation and no `FRD.md`.
**Argument hint:** `--design --docpath <docpath>`
`<docpath>` is the target path for the new `FRD.md`. `<instruction>` describes what the feature must do. The delegate enumerates the FR ID set from the instruction and the feature's PRD/ROADMAP/DESIGN context, then writes the FRD purely from the promised behaviour. No source code exists to consult, so the instruction is the sole source of truth.

### Scenario 3 — Create FRD for a project that already has source code

**When:** source code exists but there is no `FRD.md`, or the spec must be written independently of the build.
**Argument hint:** `--make --docpath <docpath>`
`<docpath>` is the target path for the new `FRD.md`. `<instruction>` describes what the feature must do. Existing source code is reference only: it may inform naming of caller-observable identifiers (error codes, event names, output paths), but the FRD never names `.py`/`.rs`/`.ts`/`.tsx` files and never states how the current build works. Where code and instruction conflict, the instruction wins.

## Design principles (applied to every FR entry)

1. **Prescriptive, not descriptive.** Write what the feature *shall* do, not what it *does* or *is about*. Replace "PRD.md states business goals" with "The docs command shall audit PRD.md and report a finding when..."
2. **Testable, or it is a wish.** Every entry must state input, output, business rules, edge cases, error handling, an observable test result, and a falsifiable metric.
3. **One observable behaviour per entry.** A reviewer can check it without asking; its `Test` subsection carries observable behaviour + expected result (never "tests pass"), its `Metric` subsection a falsifiable target value.
4. **Spec purity.** The FRD states what the feature must do, never how the current build does it. Implementation identifiers (capability names, protocol trait / interface names, capability method names) appear only under the `Capability` / `Protocol` / `Method` aspect subsections and in `## Public Interface`; every other subsection states behaviour in caller-observable terms.
5. **No source-file extensions.** Never name `.py`, `.rs`, `.ts`, `.tsx`.
6. **No status leaks.** No shipped/backlog language (released, done, in progress, v3.8.0) in spec prose.
7. **Requirement IDs are the contract.** `FR-<FEATURE-NAME-ID>-NNN`, unique within the feature and stable forever.

## 1. Declare the FR ID set first

Enumerate which `FR-<FEATURE-NAME-ID>-NNN` entries the feature needs and why, derived from the feature's deliverables and roadmap promises. The count N is decided here, explicitly, not guessed later. State the total N in the workflow.

## 2. Fan out one delegate subagent per FR ID

Spawn exactly N subagents, one per FR ID; each delegate researches its own requirement (the feature's docs: DESIGN, BACKLOG, PRD plus architecture context) and writes ONLY its own `### FR-<FEATURE-NAME-ID>-NNN` heading plus its `**Description:**` paragraph and its five H4 subsections — `Business Logic`, `Edge Cases`, `Technical Specification`, `Test`, `Metric` — seeding each per the **Seed rows** contract in the Template section. If the FRD needs 15 FR IDs, spawn 15 subagents — one per ID, never one agent writing many entries.

Each delegate writes prescriptive requirements:

- `**Description:**` — one behaviour the feature shall deliver
- `#### Business Logic` — validation logic and constraints the feature shall enforce
- `#### Edge Cases` — boundary conditions and how the feature shall handle them
- `#### Technical Specification` — one `| Aspect | Detail |` table fixed to six rows: Input, Output, Protocol, Method, Event, Error
- `#### Test` — `| Context | When Action | Then Observable Result |` table with observable behaviour + expected result
- `#### Metric` — `| Target Metric | Value |` table with a falsifiable target value

## 3. Integrate

Merge the N drafts into `## Functional Requirements` in ID order; the integrator polishes against the Rules above (testable, ID contract, no source extensions, no status leaks, spec purity) and does not invent entries for IDs that were never declared. Fill Feature Overview, Public Interface and Reference here too.

## 4. Verify

```bash
lint-arwaky-cli docs .
```

**Shared rule across all scenarios:** the FRD is spec, not status. No shipped/backlog language, no source-file extensions, and no implementation identifiers outside `Capability`, `Protocol`, `Method`, and `## Public Interface`.
