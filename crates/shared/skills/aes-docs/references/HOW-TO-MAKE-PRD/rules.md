# PRD Rules

**Purpose:** Let any stakeholder state the problem and the exclusions in under 30 seconds — without asking an engineer.
**Audience:** Stakeholders, PM, Design, Engineering leads.
**Scope:** Exactly one per project root.
**Location:** Project root.
**Length:** 50–1000 lines.

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
