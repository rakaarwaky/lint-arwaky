# FRD Template

Copy, fill, delete nothing.

```markdown
# FRD — <feature-name>

## Feature Overview

<One diagram or ≤ 3 sentences: where this feature sits, what calls it, what it calls.>

## Functional Requirements

### FR-<FEATURE-NAME-ID>-001: <Short imperative name>

**Description:** <what it does — one paragraph>

#### Business Logic
- <validation logic>
- <validation logic>
- <validation logic>

#### Edge Cases
- <boundary condition & handling>
- <boundary condition & handling>
- <boundary condition & handling>

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | <shape, source> |
| **Output** | <shape, destination> |
| **Protocol** | <protocol trait / interface> |
| **Method** | <capability method> |
| **Event** | <domain event emitted, if any> |
| **Error** | <what fails, what the caller sees> |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| <context> | <action> | <observable result> |

#### Metric
| Target Metric | Value |
|---------------|-------|
| <metric, e.g. Latency / Success Rate> | <target value, e.g. 200ms / 99.9%> |

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

**Seed rows.** Every FR entry seeds one `**Description:**` paragraph, one `#### Business Logic` list, one `#### Edge Cases` list, one `#### Technical Specification` table carrying exactly the six fixed rows `Input`, `Output`, `Protocol`, `Method`, `Event`, `Error` (each with real content), one `#### Test` table with a 3-column header `Context | When Action | Then Observable Result`, and one `#### Metric` table with a 2-column header `Target Metric | Value`. Never pad a table with a placeholder cell such as `—`; the linter flags an empty or dash-padded cell as `empty_detail_cell`. The `Technical Specification` table's six rows are the fixed set — a missing row or an undeclared extra row fires `entry_missing_row` / `table_extra_columns`.

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

Each `FR-<FEATURE-NAME-ID>-NNN` entry is an H3 heading followed by a `**Description:**` paragraph and five H4 subsections in the canonical order: `#### Business Logic`, `#### Edge Cases`, `#### Technical Specification`, `#### Test`, `#### Metric`. `Business Logic` and `Edge Cases` carry bullet lists; `Technical Specification` is a single `| Aspect | Detail |` table fixed to the six rows `Input`, `Output`, `Protocol`, `Method`, `Event`, `Error`; `Test` is a `| Context | When Action | Then Observable Result |` table; `Metric` is a `| Target Metric | Value |` table. Every table seeds at least one real data row; `Technical Specification` seeds exactly its six fixed rows and no extra.

| Aspect | Detail |
|---|---|
| **What it is** | `FR-<FEATURE-NAME-ID>-NNN` items, each an H3 heading, a `**Description:**` paragraph, and five H4 subsections in canonical order. |
| **Naming** | Short imperative name after the ID. |
| **Description** | One behaviour — what it does. A paragraph, open-ended. |
| **Business Logic** | Validation logic, constraints. Bullet list, open-ended. |
| **Edge Cases** | Boundary conditions and their handling. Bullet list, open-ended. |
| **Technical Specification** | One `| Aspect | Detail |` table, fixed to six rows: `Input` (shape, source), `Output` (shape, destination), `Protocol` (protocol trait / interface), `Method` (capability method), `Event` (domain event emitted, if any), `Error` (what fails, what the caller sees). |
| **Test** | `| Context | When Action | Then Observable Result |` table; observable behaviour + expected result, never "tests pass". Open-ended. |
| **Metric** | `| Target Metric | Value |` table; falsifiable target value. Open-ended. |
| **Seed minimums** | `Technical Specification`: exactly its six fixed rows, each with real content. `Test` / `Metric`: at least one data row. No placeholder cells such as `—`. |
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
