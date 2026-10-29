# HOW TO MAKE FRD.md

**Purpose:** Tell an engineer exactly what to build and tell QA exactly what to test — without a single follow-up question to the author.
**Audience:** Engineers, QA, Tech Lead.
**Scope:** One FRD per feature folder.
**Location:** Inside the feature's directory.
**Length:** 50–500 lines.

---

## Rules

General rules for writing an FRD. These apply across all sections.

1. **Not a feature → no FRD.** A folder is only a valid feature folder when it contains an `agent_*_orchestrator` file. Folders without an orchestrator are not features and must not carry `FRD.md` or `BACKLOG.md`. Cross-cutting kernel / shared layers (e.g. `modules/shared`) have no orchestrator and therefore have **no `FRD.md` and no `BACKLOG.md`**. Pairing (`spec-without-backlog` / `backlog-without-spec`) applies only to feature folders.
2. **Requirement IDs are the contract.** `FR-<FEATURE-NAME-ID>-NNN`, unique within the feature and stable forever.
3. **A requirement is testable, or it is a wish.** State input, output, business rules, edge cases, error handling.
4. **Nothing about the current build.** Shipped/backlog status is a BACKLOG.md claim with evidence, never an FRD sentence.
5. **One observable behaviour per entry.** A reviewer can check it without asking; its `Test` row carries observable behaviour + expected result (never "tests pass"), its `Metric` row a falsifiable target value.
6. **No acceptance criteria outside the entry tables.** The 12-row `Aspect | Detail` table IS the acceptance specification; don't duplicate acceptance prose elsewhere.
7. **No source-file extensions in a spec.** Never name `.py`, `.rs`, `.ts`, `.tsx`.

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

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | <what it does — one sentence>               |
| Business Rules | <validation logic, constraints>             |
| Edge Cases     | <boundary conditions and their handling>    |
| Capability     | <capability name>                           |
| Protocol       | <protocol trait / interface>                |
| Method         | <capability method>                         |
| Input          | <shape, source>                             |
| Output         | <shape, destination>                        |
| Error          | <what fails, what the caller sees>          |
| Event          | <domain event emitted, if any>              |
| Test           | <observable behaviour + expected result>    |
| Metric         | <target value>                              |

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

| Aspect | Detail |
|---|---|
| **What it is** | `FR-<FEATURE-NAME-ID>-NNN` items, each a 12-row entry table under an Aspect-Detail header. |
| **Naming** | Short imperative name after the ID. |
| **Description** | One behaviour — what it does. |
| **Business Rules** | Validation logic, constraints. |
| **Edge Cases** | Boundary conditions and their handling. |
| **Capability** | Capability name from the architecture. |
| **Protocol** | Protocol trait / interface. |
| **Method** | Capability method. |
| **Input** | Shape, source. |
| **Output** | Shape, destination. |
| **Error** | What fails, what the caller sees. |
| **Event** | Domain event emitted, if any. |
| **Test** | Observable behaviour + expected result; never "tests pass". |
| **Metric** | Target value, falsifiable. |
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
2. **Fan out one delegate subagent per FR ID** — spawn exactly N subagents, one per FR ID; each delegate researches its own requirement (the feature's docs: DESIGN, BACKLOG, PRD plus architecture context) and writes ONLY its own `### FR-<FEATURE-NAME-ID>-NNN` heading plus its 12-row entry table. If the FRD needs 15 FR IDs, spawn 15 subagents — one per ID, never one agent writing many entries.
3. **Integrate** — merge the N drafts into `## Functional Requirements` in ID order; the integrator polishes against the Rules above (testable, ID contract, no source extensions, no status leaks) and does not invent entries for IDs that were never declared. Fill Feature Overview, Public Interface and Reference here too.
4. **Verify** — run `lint-arwaky-cli docs .` plus the parity/structure checks listed in the Verify section.

---

## Verify

```bash
lint-arwaky-cli docs .
```
