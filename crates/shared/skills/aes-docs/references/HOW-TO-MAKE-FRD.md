# HOW TO MAKE FRD.md

> **Purpose**: Tell an engineer exactly what to build and tell QA exactly what
> to test — without a single follow-up question to the author.
>
> **Audience**: Engineers, QA, Tech Lead.
>
> **Scope**: One FRD per feature folder.
>
> **Location**: Inside the feature's directory.
>
> **Length**: 50–500 lines.
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

---

## Workflow

1. **Create file** → `FRD.md` in feature directory.
2. **Section: Feature Overview** — one diagram or ≤ 3 sentences: where
   this feature sits, what calls it, what it calls.
3. **Section: Functional Requirements** — `FR-<FEATURE-NAME-ID>-NNN`
   items, each carrying an `Aspect | Detail` entry table.
4. **Section: Public Interface** — `Aspect | Detail` table.
5. **Section: Reference** — link to PRD.md, BACKLOG.md, ROADMAP.md,
   ARCHITECTURE.md.
6. **Verify** → `lint-arwaky-cli docs` passes; FR table has all required
   columns.

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

Every section is required unless marked optional. Each exists for one
reason.

| Section              | Why it belongs here                                                                       |
| -------------------- | ----------------------------------------------------------------------------------------- |
| Feature Overview     | Orients the reader before the requirements begin.                                         |
| Functional Requirements | The testable promise; each item is `FR-<FEATURE-NAME-ID>-NNN` with an `Aspect &#124; Detail` entry table. |
| Public Interface     | Agent / Aggregate / Method / Request / Response surface.                                  |
| Reference            | Connects the spec to PRD, BACKLOG, ROADMAP, ARCHITECTURE.                                 |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: IDs, orphan refs, section shape, links.
```
