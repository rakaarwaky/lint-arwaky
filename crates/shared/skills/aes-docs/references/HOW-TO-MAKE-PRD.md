# HOW TO MAKE PRD.md

**Purpose:** Let any stakeholder state the problem and the exclusions in under 30 seconds — without asking an engineer.
**Audience:** Stakeholders, PM, Design, Engineering leads.
**Scope:** Exactly one per project root.
**Location:** Project root.
**Length:** 50–1000 lines.

---

## Rules

General rules for writing a PRD. These apply across all sections.

1. **Not a feature → no PRD.** A PRD describes a project-level promise. If the entry is a single feature, it belongs in FRD, not here.
2. **No implementation detail.** SQL schemas, API signatures, class names, library choices belong in FRD. If a reader needs the stack to understand the problem, they are reading the wrong file.
3. **No ambiguity.** A PRD must be definitive. Do not leave open questions, assumptions, or "TBD" entries. If you cannot decide, do not write the PRD yet.
4. **Every goal is testable or a wish.** "Fast" is a slogan. "p95 render under 2 s on a 4-core CI runner" is a goal, because a single run can prove it wrong.
5. **Nothing about the current build.** Whether something shipped is a backlog claim with evidence, not a PRD sentence.
6. **One observable behaviour per requirement.** Phrased so a reviewer can check it without asking.
7. **No acceptance criteria outside tables.** No test steps anywhere outside the PR aspect tables. Functional test steps belong in FRD.
8. **A PRD is a promise, not a build report.** No status, progress, or "done" language; state belongs in ROADMAP and BACKLOG.
9. **No implementation identifiers.** Code, class, or schema names never appear, unless a caller can observe the identifier directly (a CLI flag, an output line, an exit code).
10. **Purity test.** If a row could be edited by an engineer without a stakeholder noticing, it is not a PRD row. Rewrite it as observable behaviour or delete it.

---

## Template

Copy, fill, delete nothing. This is the canonical structure.

```markdown
# PRD — ACME

## Problem Statement

Up to 3 paragraphs. Name the pain, who feels it, and what breaks if
nothing changes. Do not list features here.

## Project Requirements

### PR-ACME-001: Short imperative name

#### Description
| No | Detail |
|----|--------|
| 1 | One behaviour — what it does |
| 2 | Second detail |

#### User Story
| No | Detail |
|----|--------|
| 1 | As a persona, I want action, so that outcome |
| 2 | Second detail |

#### In Scope
| No | Detail |
|----|--------|
| 1 | Functional boundary — what must exist |
| 2 | Second detail |

#### Out of Scope
| No | Detail |
|----|--------|
| 1 | Functional boundary — what must not exist. Name them so the argument ends here |
| 2 | Second detail |

#### Success Metric
| No | Detail |
|----|--------|
| 1 | How to measure the outcome of this requirement |
| 2 | Second detail |

### PR-<PROJECT-NAME-ID>-002: <Short imperative name>
### PR-ACME-003: Short imperative name

## Reference

| Artifact     | Purpose                                          | Link |
|--------------|--------------------------------------------------|------|
| PRD          | <Business goals and scope>                       | [PRD.md](PRD.md) |
| Backlog      | <What is true now, with evidence>                | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | <Timeline, phasing, and dependencies>            | [ROADMAP.md](ROADMAP.md) |
| Architecture | <System boundaries, tech stack, and infra>       | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

Each `### PR-…` entry carries five H4 aspect subsections in canonical
order: Description, User Story, In Scope, Out of Scope, Success Metric.
Seed rows follow the seed-row contract in the Section Contract below.

---

## Section Contract

Every section is mandatory. Each entry below explains what that section is, how to fill it, and what must not appear.

---

### Problem Statement

| Aspect | Detail |
|---|---|
| **What it is** | The reason the project exists. |
| **Length** | Up to 3 paragraphs. |
| **Must contain** | The pain, who feels it, what breaks if nothing changes. |
| **Must NOT contain** | Feature lists, solutions, implementation detail, technical jargon. |
| **Weak** | "Documentation is inefficient." |
| **Falsifiable** | "Six documents describe one feature and disagree about what is finished; telling whether PR-ACME-003 shipped requires reading the source." |

---

### Project Requirements

Each requirement is one `### PR-<PROJECT-NAME-ID>-NNN:` heading (the
requirement ID is the contract; do not renumber or reorder existing IDs)
followed by five H4 aspect subsections in canonical order: Description,
User Story, In Scope, Out of Scope, Success Metric, each carrying one
No/Detail table seeded per the seed-row contract below. The fence above
opens each entry from its own H4 block; entries 002 and 003 are elided.

| Subsection | Contract |
|---|---|
| **Description** | One observable behaviour — what it does. No implementation. |
| **User Story** | As a [specific persona], I want [action], so that [outcome]. Persona must be specific enough to disagree with. |
| **In Scope** | Functional boundary — what must exist. Defines what the system will deliver. |
| **Out of Scope** | Functional boundary — what must not exist. Name exclusions explicitly so the argument ends here. Written per requirement only, no global out-of-scope. |
| **Success Metric** | Measures the outcome of the requirement. Must be testable or a wish. Does NOT describe functional steps (that belongs in FRD). |
| **Seed rows** | All five PRD aspects open at 2 rows: `1`, `2`. The `∞` marker row is not written in PRD docs — open-ended means any count ≥2, not an explicit infinity marker. This is the `PRD_ASPECT_SEED_MIN` contract — a design-level invariant today; the linter enforces it via `check_aspect_entry_tables` through the PRD aspect constants and the aspect-table checker (linter catch-up for H4/seed-row shape is a BACKLOG item, not yet wired). |

---

### Reference

| Aspect | Detail |
|---|---|
| **What it is** | Connects the PRD to its plan, evidence, and technical boundaries. |
| **Must contain** | Exactly four rows: PRD, Backlog, Roadmap, Architecture. |
| **PRD** | Business goals and scope. Links to PRD. |
| **Backlog** | What is true now, with evidence. Links to BACKLOG. |
| **Roadmap** | Timeline, phasing, and dependencies. Links to ROADMAP. |
| **Architecture** | System boundaries, tech stack, infra. Links to ARCHITECTURE. |
| **Must NOT contain** | Additional links, external docs, or implementation diagrams. |

---

## Exemplar

| Template Section  | Weak (do not write)             | Falsifiable (write like this)                                                                                                         |
| ----------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Problem Statement | "Documentation is inefficient." | "Six documents describe one feature and disagree about what is finished; telling whether PR-006 shipped requires reading the source." |
| Description       | "Good documentation."           | "The checker reports every invariant by file and line."                                                                               |
| User Story        | "Users want docs."              | "As an ops engineer at 3 a.m., I want one file that tells me what shipped, so that I don't grep six sources."                         |
| In Scope          | "Everything docs-related."      | "Reads markdown files under /docs and reports invariant violations to stdout."                                                        |
| Out of Scope      | (empty)                         | "Rendering, publishing, or translating docs; the checker only reads and reports."                                                     |
| Success Metric    | "High quality."                 | "Exit 1 on any status line found in an FRD; exit 0 on a clean tree."                                                                  |
| Reference         | (missing links)                 | PRD → PRD.md, Backlog → BACKLOG.md, Roadmap → ROADMAP.md, Architecture → ARCHITECTURE.md                                              |

---

## Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Declare the PR ID set first** — enumerate which `PR-<PROJECT-NAME-ID>-NNN` entries the project needs and why they must exist, derived from the project's top-level deliverable folders and roadmap promises. The count N is decided here, explicitly, not guessed later. State the total N in the workflow.
2. **Fan out one delegate subagent per PR ID** — spawn exactly N subagents, one per PR ID; each delegate researches its own subject (the folder's FRD, BACKLOG, and ARCHITECTURE docs plus architecture context) and writes ONLY its own `### PR-<PROJECT-NAME-ID>-NNN` heading plus its five H4 aspect subsections, each with its No/Detail two-column table seeded per the seed-row contract in the Section Contract. If the PRD needs 15 PR IDs, spawn 15 subagents — one per ID, never one agent writing many entries.
3. **Integrate** — merge the N drafts into `## Project Requirements` in ID order; the integrator polishes against the Rules above (no impl detail, testable metrics, explicit exclusions, open-ended seed rows) and does not invent entries for IDs that were never declared.
4. **Verify** — run `lint-arwaky-cli docs .` and the parity/structure checks already listed in the Verify section.

---

## Verify

```bash
lint-arwaky-cli docs .
```
