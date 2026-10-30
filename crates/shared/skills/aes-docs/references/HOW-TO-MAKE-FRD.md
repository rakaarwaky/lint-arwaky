# HOW TO MAKE FRD.md

**Purpose:** Tell an engineer exactly what to build and tell QA exactly what to test — without a single follow-up question to the author.
**Audience:** Engineers, QA, Tech Lead.
**Scope:** One FRD per feature folder.
**Location:** Inside the feature's directory.
**Length:** 50–1000 lines.

---

## Rules

General rules for writing an FRD. These apply across all sections.

1. **Not a feature → no FRD.** A folder is only a valid feature folder when it contains an `agent_*_orchestrator` file. Folders without an orchestrator are not features and must not carry `FRD.md` or `BACKLOG.md`. Cross-cutting kernel / shared layers (e.g. `modules/shared`) have no orchestrator and therefore have **no `FRD.md` and no `BACKLOG.md`**. Pairing (`spec-without-backlog` / `backlog-without-spec`) applies only to feature folders.
2. **Requirement IDs are the contract.** `FR-<FEATURE-NAME-ID>-NNN`, unique within the feature and stable forever.
3. **A requirement is testable, or it is a wish.** State input, output, business rules, edge cases, error handling.
4. **Nothing about the current build.** Shipped/backlog status is a BACKLOG.md claim with evidence, never an FRD sentence.
5. **One observable behaviour per entry.** A reviewer can check it without asking; its `Test` subsection carries observable behaviour + expected result (never "tests pass"), its `Metric` subsection a falsifiable target value.
6. **No acceptance criteria outside the entry tables.** Each FR entry's twelve H4 aspect subsections — every numbered and marker Detail row, including the seed rows — IS the acceptance specification; don't duplicate acceptance prose elsewhere.
7. **No source-file extensions in a spec.** Never name `.py`, `.rs`, `.ts`, `.tsx`.
8. **Spec purity — the FRD is a promise.** The FRD states what the feature must do, never how the current build does it. Implementation identifiers (capability names, protocol trait / interface names, capability method names) appear only under the `Capability`, `Protocol`, and `Method` aspect subsections and in `## Public Interface`; every other subsection states behaviour in caller-observable terms.
9. **Caller-observable identifiers may stay.** Error codes, event names, output destination paths, and similar caller-visible vocabulary belong in the `Input`, `Output`, `Error`, and `Event` subsections; they are part of the promise, not implementation.
10. **Purity test.** If a sentence in any subsection names an implementation identifier that is not one of the `Capability` / `Protocol` / `Method` / Public-Interface identifiers above, or leaks build status, move the identifier or cut the sentence.

A `DESIGN.md` in the feature folder is a legitimate input to enumerating FR IDs, but it is not one of the four required Reference links.

---

## Template

Copy, fill, delete nothing.

```markdown
# FRD — <feature-name>

## Feature Overview

<One diagram or ≤ 3 sentences: where this feature sits, what calls it, what it calls.>

## Functional Requirements

### FR-<FEATURE-NAME-ID>-001: <Short imperative name>

#### Description
| No | Detail |
|----|--------|
| 1 | <what it does — one sentence> |
| 2 | <optional second observation> |
| ∞ | <open-ended: more observations may be added> |

#### Business Rules
| No | Detail |
|----|--------|
| 1 | <validation logic, constraints> |
| 2 | <validation logic, constraint 2> |
| ∞ | <open-ended> |

#### Edge Cases
| No | Detail |
|----|--------|
| 1 | <boundary conditions and their handling> |
| 2 | <boundary condition 2 and its handling> |
| ∞ | <open-ended> |

#### Capability
| No | Detail |
|----|--------|
| 1 | <capability name> |

#### Protocol
| No | Detail |
|----|--------|
| 1 | <protocol trait / interface> |

#### Method
| No | Detail |
|----|--------|
| 1 | <capability method> |
| 2 | <second method, if any> |
| ∞ | <open-ended> |

#### Input
| No | Detail |
|----|--------|
| 1 | <shape, source> |
| 2 | <second field, if any> |

#### Output
| No | Detail |
|----|--------|
| 1 | <shape, destination> |
| 2 | <second field, if any> |
| ∞ | <open-ended> |

#### Error
| No | Detail |
|----|--------|
| 1 | <what fails, what the caller sees> |
| 2 | <second failure mode, if any> |

#### Event
| No | Detail |
|----|--------|
| 1 | <domain event emitted, if any> |
| 2 | <second event, if any> |
| ∞ | <open-ended> |

#### Test
| No | Detail |
|----|--------|
| 1 | <observable behaviour + expected result> |
| 2 | <second observable behaviour + expected result> |
| ∞ | <open-ended> |

#### Metric
| No | Detail |
|----|--------|
| 1 | <target value> |
| 2 | <second target value, if any> |
| ∞ | <open-ended> |

### FR-<FEATURE-NAME-ID>-002: <Short imperative name>
### FR-<FEATURE-NAME-ID>-003: <Short imperative name>
### FR-<FEATURE-NAME-ID>-004: <Short imperative name>
### FR-<FEATURE-NAME-ID>-005: <Short imperative name>

## Public Interface

| Aspect     | Detail            |
|------------|-------------------|
| Agent      | <agent name>      |
| Aggregate  | <aggregate name>  |
| Method     | <execute>         |
| Request    | <request>         |
| Response   | <response>        |

## Reference

| Artifact     | Purpose                                         | Link |
|--------------|-------------------------------------------------|------|
| PRD          | <Business goals, user stories, and scope>       | [PRD.md](PRD.md) |
| Backlog      | <Task tracking, epics, and implementation status>| [BACKLOG.md](BACKLOG.md) |
| Roadmap      | <Timeline, phasing, and dependencies>           | [ROADMAP.md](ROADMAP.md) |
| Architecture | <System boundaries, tech stack, and infra>      | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

**Seed rows.** `∞` is a literal final seed row — not a placeholder — that marks an open-ended aspect. In every open-ended aspect subsection it must appear as the final `∞` Detail row (in the raw table: a row whose No cell is `∞` and whose Detail cell may carry a free-text note): Description, Business Rules, Edge Cases, Method, Output, Event, Test, Metric. Each open-ended aspect seeds a minimum of 3 total rows: numbered rows `1` and `2` plus the literal `∞` row; alternatively, at least 3 numbered rows with the `∞` row dropped. Bounded aspects carry no `∞` and seed exactly their count: `Capability` and `Protocol` seed exactly 1 row; `Input` and `Error` seed exactly 2 rows. When filling an FR entry, add numbered rows above the `∞` row until the aspect is complete, then keep `∞` as the closing marker (or, once 3+ numbered rows exist, drop it).

---

## Section Contract

Every section is mandatory; each explains what to fill and what must not appear.

### Feature Overview

| Aspect | Detail |
|---|---|
| **What it is** | One diagram or ≤ 3 sentences locating the feature. |
| **Length** | One diagram or ≤ 3 sentences. |
| **Must contain** | Where the feature sits, what calls it, what it calls. |
| **Weak** | "This feature does reporting." |
| **Falsifiable** | "Sits in `features/reporting/`; called by `agent_reporting_orchestrator`; calls `storage::append`." |

### Functional Requirements

Each `FR-<FEATURE-NAME-ID>-NNN` entry is an H3 heading followed by twelve H4 aspect subsections in the canonical order: Description, Business Rules, Edge Cases, Capability, Protocol, Method, Input, Output, Error, Event, Test, Metric. Every H4 subsection carries its own numbered and marker Detail table; open-ended aspects seed a minimum of 3 rows ending in the literal `∞` marker row, bounded aspects seed exactly their count with no `∞`.

| Aspect | Detail |
|---|---|
| **What it is** | `FR-<FEATURE-NAME-ID>-NNN` items, each an H3 heading plus 12 H4 aspect subsections, each with its own numbered and marker Detail table. |
| **Naming** | Short imperative name after the ID. |
| **Description** | One behaviour — what it does. Open-ended. |
| **Business Rules** | Validation logic, constraints. Open-ended. |
| **Edge Cases** | Boundary conditions and their handling. Open-ended. |
| **Capability** | Capability name from the architecture. |
| **Protocol** | Protocol trait / interface. |
| **Method** | Capability method. Open-ended. |
| **Input** | Shape, source. |
| **Output** | Shape, destination. Open-ended. |
| **Error** | What fails, what the caller sees. |
| **Event** | Domain event emitted, if any. Open-ended. |
| **Test** | Observable behaviour + expected result; never "tests pass". Open-ended. |
| **Metric** | Target value, falsifiable. Open-ended. |
| **Seed minimums** | Open-ended aspects: 3 total rows (`1`, `2`, `∞`) or ≥3 numbered rows. Bounded-one: exactly 1 row, no `∞`. Bounded-two: exactly 2 rows, no `∞`. |
| **Must NOT contain** | Source extensions (`.py`, `.rs`, `.ts`, `.tsx`), build status. |

### Public Interface

| Aspect | Detail |
|---|---|
| **What it is** | Agent / Aggregate / Method / Request / Response surface. |
| **Agent** | `<agent name>` |
| **Aggregate** | `<aggregate name>` |
| **Method** | `<execute>` |
| **Request** | `<request>` |
| **Response** | `<response>` |
| **Must NOT contain** | Internals of those calls. |

### Reference

| Aspect | Detail |
|---|---|
| **What it is** | Connects the FRD to PRD, Backlog, Roadmap, Architecture. |
| **Must contain** | Exactly four rows: PRD, Backlog, Roadmap, Architecture. |
| **PRD** | Business goals, user stories, scope. Links to PRD.md. |
| **Backlog** | Task tracking, epics, implementation status. Links to BACKLOG.md. |
| **Roadmap** | Timeline, phasing, dependencies. Links to ROADMAP.md. |
| **Architecture** | System boundaries, tech stack, infra. Links to ARCHITECTURE.md. |
| **Must NOT contain** | Additional links beyond the four artifacts. |

---

## Exemplar

| Template Section | Weak (do not write) | Falsifiable (write like this) |
|---|---|---|
| Feature Overview | "This feature does reporting." | "Sits in `features/reporting/`; called by `agent_reporting_orchestrator`; calls `storage::append`." |
| Description | "Handles reporting." | "Emits a `report_generated` event when the export completes." |
| Business Rules | "Reports must be correct." | "A report may only be generated for a date range whose start precedes its end." |
| Edge Cases | "Edge cases don't matter." | "A report for a date range whose start is after its end returns `Err(InvalidRange)` with code 400." |
| Capability | "ReportingService.run()" | Capability `report_export`. |
| Protocol | "Reports are exported through the reporting service." | Protocol `ReportExportPort`. |
| Method | "Runs the reporting export." | Method `execute`. |
| Input | "Takes args, makes PDF." | Input: date range, caller id. |
| Output | "Makes a PDF." | Output: PDF bytes to `exports/`. |
| Error | "Fails gracefully." | "Missing caller id returns `Err(Unauthorized)` with code 401 to the caller." |
| Event | (none) | Emits `report_generated{report_id, date_range}`. |
| Test | "tests pass" | "Generating a report for 2024-01-01..2024-01-07 yields a PDF whose first page lists exactly seven daily totals." |
| Metric | "fast" | "p95 generation under 2 s on a 4-core CI runner." |
| Reference | (missing links) | Links to PRD.md, BACKLOG.md, ROADMAP.md, ARCHITECTURE.md. |

---

## Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Declare the FR ID set first** — enumerate which `FR-<FEATURE-NAME-ID>-NNN` entries the feature needs and why, derived from the feature's deliverables and roadmap promises. The count N is decided here, explicitly, not guessed later. State the total N in the workflow.
2. **Fan out one delegate subagent per FR ID** — spawn exactly N subagents, one per FR ID; each delegate researches its own requirement (the feature's docs: DESIGN, BACKLOG, PRD plus architecture context) and writes ONLY its own `### FR-<FEATURE-NAME-ID>-NNN` heading plus its twelve H4 aspect subsections with their numbered and marker Detail tables, seeding each aspect per the **Seed rows** contract in the Template section. If the FRD needs 15 FR IDs, spawn 15 subagents — one per ID, never one agent writing many entries.
3. **Integrate** — merge the N drafts into `## Functional Requirements` in ID order; the integrator polishes against the Rules above (testable, ID contract, no source extensions, no status leaks, spec purity) and does not invent entries for IDs that were never declared. Fill Feature Overview, Public Interface and Reference here too.
4. **Verify** — run `lint-arwaky-cli docs .` plus the parity/structure checks listed in the Verify section.

---

## Verify

```bash
lint-arwaky-cli docs .
```
