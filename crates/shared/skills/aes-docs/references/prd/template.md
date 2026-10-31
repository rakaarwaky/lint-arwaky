# PRD Template

```markdown
# PRD — <PROJECT-NAME>

## Project Overview

<One diagram and one paragraph: what the project is, who it is for, why it matters, and what happens if it is not solved.>

## Goals

- <Primary business goal>
- <Primary user goal>
- <Strategic objective>

## Non-Goals

- <Explicitly excluded scope for this project>
- <Related problems not being solved in this release>
- <Future considerations not included now>

## Target Users

| Persona | Need | Current Pain | Desired Outcome |
|---------|------|--------------|-----------------|
| <persona> | <need> | <pain> | <outcome> |

## Scope

### In Scope

- <Major capability or module included in this project>
- <User journey included>
- <Integration included>

### Out of Scope

- <Major capability excluded from this project>
- <User journey excluded>
- <Future enhancement>

---

## Project Requirements

### PR-<PROJECT-NAME-ID>-001: <Short imperative name>
<!-- Tier P0: entry -001 is always P0. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| <role> | <capability> | <benefit> | P0 |

#### Proposed Solution

<Describe the proposed solution. Focus on what the user experiences and what the system should achieve.>

#### Main User Flow

1. <Step 1>
2. <Step 2>
3. <Step 3>

### PR-<PROJECT-NAME-ID>-002: <Short imperative name>
<!-- Tier P1: the first P1 entry. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| <role> | <capability> | <benefit> | P1 |

#### Proposed Solution

<Describe the proposed solution.>

#### Main User Flow

1. <Step 1>

### PR-<PROJECT-NAME-ID>-003: <Short imperative name>
<!-- Tier P2: the first P2 entry. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| <role> | <capability> | <benefit> | P2 |

#### Proposed Solution

<Describe the proposed solution.>

#### Main User Flow

1. <Step 1>

## Reference

| Artifact     | Purpose                                          | Link |
|--------------|--------------------------------------------------|------|
| PRD          | <Business goals and scope>                       | [PRD.md](PRD.md) |
| Backlog      | <What is true now, with evidence>                | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | <Timeline, phasing, and dependencies>            | [ROADMAP.md](ROADMAP.md) |
| Architecture | <System boundaries, tech stack, and infra>       | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

**Seed rows.** Each `### PR-<PROJECT-NAME-ID>-NNN:` entry carries three H4 aspect subsections in canonical order: `#### User Stories`, `#### Proposed Solution`, `#### Main User Flow`. `User Stories` is a `| As a | I want | So that | Priority |` table seeded with at least one real row (priority `P0`/`P1`/`P2`). `Proposed Solution` states one observable behaviour per entry — no implementation detail, SQL, API signatures, or class names. `Main User Flow` is a numbered list of user-facing steps (1., 2., 3.). Every aspect seeds at least one real content row/bullet/step; add further rows only when a second real fact exists — never a placeholder cell. This is the `PRD_ASPECT_SEED_MIN` contract (open-ended at 1 per aspect).

**Priority ordering.** `PR-<PROJECT-NAME-ID>-NNN` entries must be listed in priority order: all P0 entries first, then P1, then P2. The NNN counter is global across tiers; the first entry (`-001`) is always P0. No P1 entry may appear before a P0 entry, and no P2 entry before a P1 entry. Within the same tier the author may reorder entries, but tier jumps are forbidden.
