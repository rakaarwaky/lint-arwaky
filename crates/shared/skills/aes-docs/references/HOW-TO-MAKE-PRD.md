# HOW TO MAKE PRD.md

> **Purpose**: Let any stakeholder state the problem, the top priorities,
> and the exclusions in under 30 seconds — without asking an engineer.
>
> **Audience**: Stakeholders, PM, Design, Engineering leads.
>
> **Scope**: Exactly one per project root. In a Rust workspace, one per
> crate (write `<crate-name>` / "this crate" instead of `<project-name>`
> / "this project").
>
> **Location**: Project root.
>
> **Length**: 50–500 lines (flat budget shared by every document type).

---

## Rules




Eight rules. Each one prevents a specific failure mode.

1. **No implementation detail.** SQL schemas, API signatures, class
names, library choices belong in `FRD.md`. If a reader needs the
stack to understand the problem, they are reading the wrong file.
2. **Every goal is falsifiable.** "Fast" is a slogan. "p95 render under
2 s on a 4-core CI runner" is a goal, because a single run can prove
it wrong.
3. **Every feature has a Priority (P0 / P1 / P2) and acceptance
criteria.** One behaviour per bullet, phrased so a reviewer can
check it without asking. Priority without ranking means everything is
P0 and nothing ships.
4. **Out of scope is written down.** Recording deliberate exclusions is
what ends the argument the next time one of them is proposed.
5. **Personas are specific enough to disagree with.** "Users" decides
nothing. "The ops engineer running this headless at 3 a.m." decides
who wins a trade-off.
6. **Open questions stay visible.** Write the question instead of
silently picking an answer. An unstated assumption in a spec becomes
a requirement nobody agreed to.
7. **Non-functional stays high-level here.** Detailed numbers belong to
the feature that carries them, in its `FRD.md`.
8. **Nothing about the current build.** Whether something shipped is a
backlog claim with evidence (`done-without-evidence`), not a PRD
sentence.

**Exemplar.** Weak fails a stakeholder; falsifiable lets them decide.

| Section      | Weak                            | Falsifiable                                                                                                                              |
| ------------ | ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Problem      | "Documentation is inefficient." | "Six documents describe one feature and disagree about what is finished; telling whether `FR-006` shipped requires reading the source."  |
| Goal         | "Docs should be high quality."  | "`aa check` fails any repo where a `Done` backlog row cites no command and commit hash."                                                 |
| Feature      | "P0 — good documentation."      | "P0 — the checker reports every invariant by file and line. Acceptance: exit 1 on a status line in an `FRD.md`, exit 0 on a clean tree." |
| Out of scope | *(empty)*                       | "Out of scope: rendering or publishing docs; the checker only reads and reports."                                                        |
| Persona      | "Users."                        | "The agent writing the docs in a fresh session, with no memory of the last one."                                                         |

Every row names a concrete subject, a measurable outcome, and a test
that could falsify it.

**Pairing.** Every feature in the PRD must be reachable from the root
`ROADMAP.md` feature index. A P0 / P1 / P2 bullet needs its own
`FRD.md` + `BACKLOG.md` pair. An out-of-scope item is marked `Won't Do`
in the root or simply absent. A goal with a metric needs a scenario in
a feature's `## Test Scenarios` that proves it.

---




## Workflow

1. **Create file** → `PRD.md` at repo root or feature directory.
2. **Section: Problem Statement** — one paragraph describing the gap.
3. **Section: Goals & Success Metrics** — measurable outcomes.
4. **Section: Scope** — in-scope vs out-of-scope.
5. **Section: Feature Requirements** — high-level capabilities.
6. **Verify** → `aa check docs` passes; goals are measurable.

## Template

Copy, fill, delete nothing.

```markdown
# PRD — <project-name>

> Product Requirements Document. Describes WHAT this project does and WHY.
> Audience: Stakeholders, PM, Design, Engineering leads.
> Condition: [ROADMAP.md](ROADMAP.md) (workspace) + each feature [BACKLOG.md](BACKLOG.md).
> This file is specification only.

## Problem Statement

<One paragraph. Name the pain, who feels it, and what breaks if nothing changes.
 Do not list features here.>

## Goals & Success Metrics

| # | Goal                        | Measurement              | Target       |
|---|-----------------------------|--------------------------|--------------|
| 1 | <outcome, not activity>     | <tool / run / metric>    | <number>     |
| 2 | …                           | …                        | …            |

## User Personas

- **<Role, e.g. "Ops engineer on-call at 3 a.m.">**:
  what they need, what they cannot do today, what "done" looks like for them.
- **<Second persona>**: …

## Scope

- **In scope**: <boundary — what this project will deliver>
- **Out of scope**: <explicit exclusions — name them so the argument ends here>

## Feature Requirements

### P0 — Must Have

- <One behaviour per bullet. End with "Acceptance: <observable check>.">

### P1 — Should Have

- <Same format.>

### P2 — Nice to Have

- <Same format.>

## Non-functional Requirements (High-level)

| Category    | Commitment                  | Detail lives in          |
|-------------|-----------------------------|--------------------------|
| Performance | <direction, e.g. "p95 < X"> | `FRD.md` of feature Y    |
| Security    | <standard or posture>       | `FRD.md` of feature Z    |
| Scalability | <order-of-magnitude target> | `FRD.md` of feature W    |

## Open Questions / Risks

| # | Question / Risk              | Owner | Deadline | Status |
|---|------------------------------|-------|----------|--------|
| 1 | <unresolved decision>        | <who> | <when>   | open   |
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.

| Section                     | Why it belongs here                                                                                           |
| --------------------------- | ------------------------------------------------------------------------------------------------------------- |
| Problem Statement           | The reason the project exists, in one paragraph. Watch for a feature list instead of a problem.               |
| Goals &amp; Success Metrics | What "worked" means, measurably. Watch for adjectives with no number behind them.                             |
| User Personas               | Decides whose need wins a trade-off. Watch for a single generic "user".                                       |
| Scope                       | The boundary that stops silent scope creep. Watch for out-of-scope left empty.                                |
| Feature Requirements        | The promise, in priority order. Watch for no acceptance criteria per feature.                                 |
| Non-functional Requirements | Stakeholders own the risk budget too. Watch for detailed numbers — those are FRD content.                     |
| Open Questions / Risks      | Unresolved decisions must be visible, not buried. Watch for deletion once someone picked an answer privately. |

---

## Verify

```bash
aa check docs .
# Checks: placement, sections, links, length, hygiene.
# Manual: every Goal has a number; no impl detail (grep CREATE TABLE|POST /|fn |interface);
# out-of-scope non-empty; every P0 has acceptance criteria.
```
