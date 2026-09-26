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
holds **exactly one row**: one method for the whole feature folder that
covers every capability (not one row per leaf). `Aggregate API` holds one
row per public method the feature's agent exposes (the feature
orchestrator / agent, designed for the target — not a snapshot of today's
code). Columns for both tables: Method, Input, Output, Error, Event,
Description — real signatures, not invented capability names.
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
6. **Verify** → `aa check docs` passes; FR table has all required columns.

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
| <method> | <input> | <output> | <error> | <event> | one row only — one method for all capabilities |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| <method> | <input> | <output> | <error> | <event> | <one sentence> |


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

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.


| Section                       | Why it belongs here                               |
| ----------------------------- | ------------------------------------------------- |
| Reference                     | Separates spec promise from backlog claim.        |
| System Overview               | Orients the reader before details begin.          |
| Functional Requirements       | The testable promise                              |
| API Contract                  | Protocol + Aggregate surfaces integrators build against. |
| Integration Points            | Names every outside system that can fail you.     |
| Non-functional Requirements   | Feature-level numbers the PRD deliberately omits. |
| Test Scenarios                | Promises the backlog must evidence.               |
| Assumptions &amp; Constraints | Implicit requirements made explicit.              |
| Glossary                      | One meaning per term; rows and code agree.        |




## Verify

```bash
aa check docs
# path form: aa check docs .
# Checks: IDs, orphan refs, scenario coverage, status leak, sections, links.
```

On any violation the gate prints `[FAIL] <code> <path>: <message>` and exits
non-zero; every finding gates (strict is the only mode — no advisory tier).

