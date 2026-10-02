# HOW TO MAKE FRD.md

> **Purpose**: Tell an engineer exactly what to build and tell QA exactly what
> to test — without a single follow-up question to the author.
>
> **Audience**: Engineers, QA, Tech Lead.
>
> **Scope**: One FRD per feature folder
>
> **Location**: Inside the feature's directory
>
> **Length**: 50–500 lines
>
> **Not a feature → no FRD.** Cross-cutting kernel / shared layers
> (e.g. `modules/shared`) are not features: they have **no `FRD.md` and no
> `BACKLOG.md`**. Creating either under `shared/` fails the gate with
> `feature-doc-in-shared`. Pairing (`spec-without-backlog` /
> `backlog-without-spec`) applies only to feature folders.

---

## Rules

0. **Not a feature → no FRD.** A folder is only a valid feature folder when it contains an `agent_*_orchestrator` file. Folders without an orchestrator are not features and must not carry `FRD.md` or `BACKLOG.md`. Cross-cutting kernel / shared layers (e.g. `modules/shared`) have no orchestrator and therefore have **no `FRD.md` and no `BACKLOG.md`**. Creating either under `shared/` fails the gate with `feature-doc-in-shared`. Pairing (`spec-without-backlog` / `backlog-without-spec`) applies only to feature folders.

1. **Requirement IDs are the contract.** `FR-<name>-<number>`,
unique within the feature and stable forever.
2. **A requirement is testable, or it is a wish.**
State input, output, business rules, edge cases, error handling.
3. **The API contract is two tables under one section.** `Protocol API`
lists **one row per capability method** the feature's protocol exposes
(every method on the protocol trait). `Aggregate API` lists the
**single `execute` entry point** — one row only, the composite verb the
orchestrator exposes to the surface. Columns for both tables: Method,
Input, Output, Error, Event, Description — real signatures, not invented
capability names.
3b. **The API Contract subsections are a closed set.** Level 3 under
`## API Contract` is exactly `### Protocol API` then `### Aggregate API`,
in that order, once each — nothing else. Each subsection must carry its own
table with all six columns; prose, a partial column set, or a second copy of
a subsection all fail. **Per-protocol detail goes in the rows of the
`Protocol API` table, never in a heading.** Adding `### IParserProtocol`,
`### IGraphProtocol`, and so on — one level-3 heading per protocol class — is
the workaround this rule closes (`api_h3_unexpected`, AES602): the heading
narrates a seam split the single table refused to make, so the contract reads
as if the seams existed when the rows do not. There is one `Protocol API`
table, and every protocol method is a row in it.
3c. **The whole level-3 set is closed to three shapes, document-wide.**
An FRD's only level-3 headings are `### FR-<FEATURENAME>-NNN: <name>`,
`### Protocol API`, and `### Aggregate API`. Every other level-3 heading
fires `h3_off_template` (AES602) — **wherever it sits**, not only under
`## API Contract`, so moving an invented section into another parent section
is not an escape (`api_h3_unexpected` reads only the API Contract subtree, so
it cannot see this). A table you wanted to head with
`### IToolResolutionProtocol (12 operations)` is the same information as a
`Protocol API` row: move it into that table, fold it into the section that owns
it, or demote it to a level-4 heading — level 4 and deeper are free-form and
are where detail that must not become a section belongs. The sanctioned set is
transcribed from the template below; `crates/doc-rules/scripts/check_doc_consistency.py` compares
the two so they cannot drift apart again.
3a. **The FR count must match the protocol class count.** The number of
`### FR-<Feature>-NNN:` headings and the number of `pub trait I*Protocol`
declarations across the feature's contract protocol files must be equal
(`protocol_count_mismatch`, AES601).
One protocol class is one capability seam, and one seam is one
requirement. **A single protocol file may declare many protocol classes** —
count the classes, never the files. Method counts are irrelevant: a
protocol class carrying nine methods for nine commands is still one class
and therefore one FR. When the two counts differ, fix it in whichever
direction preserves the most spec intent:

| # | Direction | Use when | Effect |
|---|-----------|----------|--------|
| 1 | **Split methods into more classes** | One class holds several genuinely different capabilities and an FR already describes each one separately. | classes up, FRs unchanged |
| 2 | **Merge methods into one class** | One class is too fat — its methods collapse into fewer coherent seams — and the FRs already match the smaller count. | classes down, FRs unchanged |
| 3 | **Merge FRs down to the class count** | FRs describe orchestrator dispatch, traversal, or sub-steps of one capability rather than distinct capabilities. | FRs down |
| 4 | **Split FRs up to the class count** | One class legitimately covers several capabilities but the FRs are lumped together, or the classes were split per direction 1 and the FRs must follow. | FRs up |

Order of preference: direction **1** or **2** when the code shape is
genuinely wrong, direction **3** when the extra FRs are infrastructure
narrative, direction **4** last. Never pad or delete a real requirement
just to satisfy the count — if neither direction preserves the spec,
split the class.
4. **Scenarios are stated here; evidence lives in the backlog.**
One scenario per bullet, so `scenario-evidence-count` can match them.
5. **Non-functional numbers live here.**
The PRD says "fast." This file says *what* the feature guarantees and
*how* you measure it.
6. **Assumptions and constraints are written down.**
Every implicit assumption is a requirement someone discovers later and
calls a bug.
7. **Cross-link the pair** in `## Reference`. A reader landing on either
file must immediately see promise and claim.
8. **No description of current behaviour.** A paragraph about what the
source does today is a second copy of the code and always staler.
9. **No source-file names.** An FRD is a stateless spec: never name
`.py` / `.rs` / `.ts` files, module paths, or implementation symbols.
Refer to roles (orchestrator, capability, gateway) and behaviour only, so
the document survives every refactor (`spec-source-path`).

---

## Workflow

1. **Create file** → `FRD.md` in feature directory.
2. **Section: System Overview** — one-paragraph context.
3. **Section: Functional Requirements** — numbered FRs with scenario tables.
4. **Section: API Contract** — request/response shapes.
5. **Section: Test Scenarios** — scenarios with acceptance criteria.
6. **Verify** → `lint-arwaky-cli docs` passes; FR table has all required columns.

## Template

Copy, fill, delete nothing.

```markdown
# FRD — <feature-name>

## Reference

- PRD: <link to root PRD.md>
- Backlog: [BACKLOG.md](BACKLOG.md) 

## System Overview

<One diagram or ≤ 3 sentences: where this feature sits, what calls it, what it calls.>

## Functional Requirements

### FR-<FEATURENAME>-001: <Short imperative name>

- **Description**: <what it does — one sentence>
- **Input**: <shape, source>
- **Output**: <shape, destination>
- **Business Rules**: <validation logic, constraints>
- **Edge Cases**: <boundary conditions and their handling>
- **Error Handling**: <what fails, what the caller sees>

### FR-<FEATURENAME>-XXX: <Short imperative name>
### FR-<FEATURENAME>-XXX: <Short imperative name>
### FR-<FEATURENAME>-XXX: <Short imperative name>
### FR-<FEATURENAME>-XXX: <Short imperative name>
- …

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| <capability method> | <input> | <output> | <error> | <event> | one row per method on the protocol trait |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| <execute> | <request> | <response> | <error> | <event> | single composite entry point |


## Integration Points

| System | Direction | Purpose | Failure mode |
|--------|-----------|---------|--------------|
| <name> | <in / out>  | <one sentence> | <condition> -> <fallback> |


## Non-functional Requirements

| Metric       | Target      | Measurement method        |
|--------------|-------------|---------------------------|
| <metric name> |  <value>  | <tool>  |


## Test Scenarios

- <Scenario stated as observable behaviour + expected result.>

## Assumptions & Constraints

- <Assumption or constraint — name it, scope it, date it if it expires.>

## Glossary

- **Term**: <one definition, one meaning>
```

The template's level-3 headings are exactly the requirement headings, `### Protocol
API`, and `### Aggregate API`. Detail that needs a heading of its own belongs at
level 4, which is free-form — an FRD that invents a level-3 section fires
`h3_off_template`.

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.

| Section                       | Why it belongs here                               |
| ----------------------------- | ------------------------------------------------- |
| Reference                     | Separates spec promise from backlog claim.        |
| System Overview               | Orients the reader before details begin.          |
| Functional Requirements       | The testable promise. Its only level-3 headings are `### FR-<FEATURENAME>-NNN:`. |
| API Contract                  | Protocol + Aggregate surfaces integrators build against. Holds exactly two level-3 subsections. |
| Integration Points            | Names every outside system that can fail you.     |
| Non-functional Requirements   | Feature-level numbers the PRD deliberately omits. |
| Test Scenarios                | Promises the backlog must evidence. A bullet list — not a heading per scenario. |
| Assumptions &amp; Constraints | Implicit requirements made explicit.              |
| Glossary                      | One meaning per term; rows and code agree.        |

The level-3 set is closed document-wide (Rule 3c): only
`### FR-<FEATURENAME>-NNN: <name>`, `### Protocol API`, and
`### Aggregate API`. Anything else fires `h3_off_template`.

## Verify

```bash
lint-arwaky-cli docs
# path form: lint-arwaky-cli docs .
# Checks: IDs, orphan refs, scenario coverage, status leak, sections, links.
```

On any violation the gate prints `[FAIL] <code> <path>: <message>` and exits
non-zero; every finding gates (strict is the only mode — no advisory tier).
