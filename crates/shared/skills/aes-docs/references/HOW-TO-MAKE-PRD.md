# HOW TO MAKE PRD.md

**Purpose:** Let any stakeholder state the problem and the exclusions in under 30 seconds — without asking an engineer.
**Audience:** Stakeholders, PM, Design, Engineering leads.
**Scope:** Exactly one per project root.
**Location:** Project root.
**Length:** 50–500 lines.

---

## Rules

General rules for writing a PRD. These apply across all sections.

1. **No implementation detail.** SQL schemas, API signatures, class names, library choices belong in FRD.md. If a reader needs the stack to understand the problem, they are reading the wrong file.
2. **No ambiguity.** A PRD must be definitive. Do not leave open questions, assumptions, or "TBD" entries. If you cannot decide, do not write the PRD yet.
3. **Every goal is falsifiable.** "Fast" is a slogan. "p95 render under 2 s on a 4-core CI runner" is a goal, because a single run can prove it wrong.
4. **One behaviour per requirement.** Phrased so a reviewer can check it without asking.
5. **Nothing about the current build.** Whether something shipped is a backlog claim with evidence, not a PRD sentence.
6. **No priority in PRD.** Priority and phasing belong in ROADMAP.md. PRD only defines what and why.
7. **No acceptance criteria in PRD.** Functional test steps belong in FRD.md. PRD defines boundaries and measurement.

---

## Template

Copy, fill, delete nothing. This is the canonical structure.

```markdown
# PRD — <project-name>

## Problem Statement

<Up to 3 paragraphs. Name the pain, who feels it, and what breaks if nothing changes.
Do not list features here.>

## Project Requirements

### PR-<PROJECT-NAME-ID>-001: <Short imperative name>

| Aspect         | Detail                                                      |
|----------------|---------------------------------------------------------------|
| Description    | <one behaviour — what it does>                              |
| User Story     | <as a..., I want..., so that...>                            |
| In Scope       | <functional boundary — what must exist>                     |
| Out of Scope   | <functional boundary — what must not exist. Name them so the argument ends here> |
| Success Metric | <how to measure the outcome of this requirement>            |

### PR-<PROJECT-NAME-ID>-002: <Short imperative name>
### PR-<PROJECT-NAME-ID>-003: <Short imperative name>
### PR-<PROJECT-NAME-ID>-004: <Short imperative name>
### PR-<PROJECT-NAME-ID>-005: <Short imperative name>

## Reference

| Artifact     | Purpose                                          | Link|
|--------------|--------------------------------------------------|------|
| Roadmap      | <Timeline, phasing, and dependencies>            | [ROADMAP.md](ROADMAP.md)|
| Architecture | <System boundaries, tech stack, and infra>       | [ARCHITECTURE.md](ARCHITECTURE.md)|
```

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
| **Falsifiable** | "Six documents describe one feature and disagree about what is finished; telling whether FR-006 shipped requires reading the source." |

---

### Project Requirements

| Aspect | Detail |
|---|---|
| **What it is** | The promise, in PR-<PROJECT-NAME-ID>-NNN items. Each item carries a five-row table. |
| **Naming** | Short imperative name. |
| **Description** | One behaviour — what it does. No implementation. |
| **User Story** | As a [specific persona], I want [action], so that [outcome]. Persona must be specific enough to disagree with. |
| **In Scope** | Functional boundary — what must exist. Defines what the system will deliver. |
| **Out of Scope** | Functional boundary — what must not exist. Name exclusions explicitly so the argument ends here. Written per requirement only, no global out-of-scope. |
| **Success Metric** | Measures the outcome of the requirement. Must be falsifiable. Does NOT describe functional steps (that belongs in FRD.md). |

---

### Reference

| Aspect | Detail |
|---|---|
| **What it is** | Connects the PRD to its plan and technical boundaries. |
| **Must contain** | Exactly two rows: Roadmap and Architecture. |
| **Roadmap** | Timeline, phasing, and dependencies. Links to ROADMAP.md. |
| **Architecture** | System boundaries, tech stack, infra. Links to ARCHITECTURE.md. |
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
| Success Metric    | "High quality."                 | "Exit 1 on any status line found in an FRD.md; exit 0 on a clean tree."                                                               |
| Reference         | (missing links)                 | Roadmap → ROADMAP.md, Architecture → ARCHITECTURE.md                                                                                  |

---

## Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Declare the PR ID set first** — enumerate which `PR-<PROJECT-NAME-ID>-NNN` entries the project needs and why they must exist, derived from the project's top-level deliverable folders and roadmap promises. The count N is decided here, explicitly, not guessed later. State the total N in the workflow.
2. **Fan out one delegate subagent per PR ID** — spawn exactly N subagents, one per PR ID; each delegate researches its own subject (the folder's docs: FRD/BACKLOG/DESIGN plus architecture context) and writes ONLY its own `### PR-<PROJECT-NAME-ID>-NNN` heading plus its five-row entry table. If the PRD needs 15 PR IDs, spawn 15 subagents — one per ID, never one agent writing many entries.
3. **Integrate** — merge the N drafts into `## Project Requirements` in ID order; the integrator polishes against the Rules above (no impl detail, falsifiable metrics, explicit exclusions) and does not invent entries for IDs that were never declared.
4. **Verify** — run `lint-arwaky-cli docs` and the parity/structure checks already listed in the Verify section.

---

## Verify

```bash
lint-arwaky-cli docs .
```

---
