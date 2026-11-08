# PRD Workflow

Follow these steps in order. The output must match the Template exactly.

## 0. Choose the entry scenario

The entry point depends on the starting state. In every scenario the PRD is a promise: it states what the project must deliver and why, never what the current source code does. The user's `<instruction>` overrides the code — code never edits the spec.

### Scenario 1 — Update existing PRD

**When:** a `PRD.md` already exists at the project root and needs to be improved.
**Argument hint:** `--improve --docpath <docpath>`
`<docpath>` is the path to the existing `PRD.md`. `<instruction>` describes what to change. The delegate reads the current PRD, applies the improvement, and keeps every PR ID stable; new PR IDs are only added when the improvement declares new deliverables. Never derive requirements from source code — apply the instruction against the existing spec.

**Special case — `--docpath all`:** improves every existing `PRD.md` in the workspace in parallel. The orchestrator discovers all `PRD.md` files, then fans out one delegate subagent per `PRD.md`, each applying the same `<instruction>` to its own PRD while keeping that PRD's IDs stable. The same shared rule applies: no source-code derivation, instruction overrides code.

### Scenario 2 — Create new PRD, no source code yet

**When:** the project has no implementation and no `PRD.md`.
**Argument hint:** `--design --docpath <docpath>`
`<docpath>` is the target path for the new `PRD.md`. `<instruction>` describes what the project must deliver. The delegate enumerates the PR ID set from the instruction and the project's ROADMAP/ARCHITECTURE context, then writes the PRD purely from the promised behaviour. No source code exists to consult, so the instruction is the sole source of truth.

### Scenario 3 — Create PRD for a project that already has source code

**When:** source code exists but there is no `PRD.md`, or the spec must be written independently of the build.
**Argument hint:** `--make --docpath <docpath>`
`<docpath>` is the target path for the new `PRD.md`. `<instruction>` describes what the project must deliver. Existing source code is reference only: it may inform naming of caller-observable identifiers (a CLI flag, an output line, an exit code), but the PRD never names implementation detail — SQL schemas, API signatures, class names, library choices — and never states how the current build works. Where code and instruction conflict, the instruction wins.

## 1. Declare the PR ID set first

Enumerate which `PR-<PROJECT-NAME-ID>-NNN` entries the project needs and why they must exist, derived from the project's top-level deliverable folders and roadmap promises. The count N is decided here, explicitly, not guessed later. State the total N in the workflow.

## 2. Fan out one delegate subagent per PR ID

Spawn exactly N subagents, one per PR ID; each delegate researches its own subject (the folder's FRD, BACKLOG, and ARCHITECTURE docs plus architecture context) and writes ONLY its own `### PR-<PROJECT-NAME-ID>-NNN` heading plus its three H4 aspect subsections (`#### User Stories`, `#### Proposed Solution`, `#### Main User Flow`), seeded per the seed-row contract. If the PRD needs 15 PR IDs, spawn 15 subagents — one per ID, never one agent writing many entries.

## 3. Integrate

Merge the N drafts into `## Project Requirements` in priority order: P0 entries first (NNN 001, 002, …), then P1, then P2. The NNN counter is global and sequential — no gaps, no tier jumps. The integrator polishes against the Rules above (no impl detail, testable metrics, explicit exclusions, open-ended seed rows) and does not invent entries for IDs that were never declared.

## 4. Verify

```bash
lint-arwaky-cli docs .
```

**Shared rule across all scenarios:** the PRD is a promise, not a build report. No status, progress, or "done" language; state belongs in ROADMAP and BACKLOG. No implementation identifiers — code, class, or schema names never appear unless a caller can observe the identifier directly (a CLI flag, an output line, an exit code).
