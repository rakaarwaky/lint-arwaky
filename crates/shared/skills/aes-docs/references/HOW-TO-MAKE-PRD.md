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

**Pairing.** Every `PR-` item declared in the PRD's `Project Requirements`
must have a matching `PR-<PROJECT-NAME-ID>-NNN` entry in the root
`ROADMAP.md` `Roadmap Status` section. A `PR-` item needs its own
`FRD.md` + `BACKLOG.md` pair. An out-of-scope item is marked `Won't Do`
in the root or simply absent.

---

## Workflow

1. **Create file** → `PRD.md` at repo root or feature directory.
2. **Section: Problem Statement** — one paragraph describing the gap.
3. **Section: Project Requirements** — `PR-<PROJECT-NAME-ID>-NNN` items,
   each carrying an `Aspect | Detail` entry table.
4. **Section: Reference** — link to ROADMAP.md and ARCHITECTURE.md.
5. **Verify** → `lint-arwaky-cli docs` passes; goals are measurable.

## Template

Copy, fill, delete nothing.

```markdown
# PRD — <project-name>

## Problem Statement

<One paragraph. Name the pain, who feels it, and what breaks if nothing changes.
Do not list features here.>

## Project Requirements

### PR-<PROJECT-NAME-ID>-001: <Short imperative name>

| Aspect         | Detail                                                        |
|----------------|---------------------------------------------------------------|
| Description    | <one behaviour — what it does>                                |
| User Story     | <as a..., I want..., so that...>                              |
| In Scope       | <boundary — what this project will deliver>                   |
| Out of Scope   | <explicit exclusions — name them so the argument ends here>   |
| Acceptance     | <observable check>                                            |
| Success Metric | <how to measure this requirement succeeds>                    |

### PR-<PROJECT-NAME-ID>-002: <Short imperative name>
### PR-<PROJECT-NAME-ID>-003: <Short imperative name>
### PR-<PROJECT-NAME-ID>-004: <Short imperative name>
### PR-<PROJECT-NAME-ID>-005: <Short imperative name>

## Reference

| Artifact     | Purpose                                          | Link |
|--------------|--------------------------------------------------|------|
| Roadmap      | <Timeline, phasing, and dependencies>            | [ROADMAP.md](ROADMAP.md) |
| Architecture | <System boundaries, tech stack, and infra>       | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.

| Section               | Why it belongs here                                                                                 |
| --------------------- | --------------------------------------------------------------------------------------------------- |
| Problem Statement     | The reason the project exists, in one paragraph. Watch for a feature list instead of a problem.     |
| Project Requirements  | The promise, in `PR-<PROJECT-NAME-ID>-NNN` items, each with Description / User Story / In Scope / Out of Scope / Acceptance / Success Metric. |
| Reference             | Connects the spec to its plan and its boundaries.                                                  |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: placement, sections, links, length, hygiene.
# Manual: every Requirement row has all six Aspect labels; no impl detail (grep CREATE TABLE|POST /|fn |interface);
# out-of-scope non-empty; every Priority has acceptance criteria.
```
