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
3. **No ambiguity.** A PRD must be definitive. Do not leave open questions, assumptions. If you cannot decide, do not write the PRD and ask user.
4. **Every goal is testable or a wish.** "Fast" is a slogan. "p95 render under 2 s on a 4-core CI runner" is a goal, because a single run can prove it wrong.
5. **Nothing about the current build.** Whether something shipped is a backlog claim with evidence, not a PRD sentence.
6. **One observable behaviour per requirement.** Phrased so a reviewer can check it without asking.
7. **No acceptance criteria outside tables.** No test steps anywhere outside the PR aspect tables. Functional test steps belong in FRD.
8. **A PRD is a promise, not a build report.** No status, progress, or "done" language; state belongs in ROADMAP and BACKLOG.
9. **No implementation identifiers.** Code, class, or schema names never appear, unless a caller can observe the identifier directly.
10. **Purity test.** If a row could be edited by an engineer without a stakeholder noticing, it is not a PRD row. Rewrite it as observable behaviour or delete it.
11. **Priority-ordered IDs.** `PR-<PROJECT-NAME-ID>-NNN` entries are listed in strict priority order: all P0 entries first, then P1, then P2. The `NNN` counter increments across all entries; the first entry is `-001` and is always P0. No P1 entry may appear before a P0 entry, and no P2 entry before a P1. Within the same tier, NNN order is the author's call, but it must not jump tiers.
12. **Priority tier is the User Stories table's Priority column.** Each `#### User Stories` row's `Priority` cell is the entry's tier. If multiple rows carry different tiers within one entry, the entry's tier is the highest-priority (lowest P-number) row in that table.

## Seed rows

Each `### PR-<PROJECT-NAME-ID>-NNN:` entry carries three H4 aspect subsections in canonical order: `#### User Stories`, `#### Proposed Solution`, `#### Main User Flow`. `User Stories` is a `| As a | I want | So that | Priority |` table seeded with at least one real row (priority `P0`/`P1`/`P2`). `Proposed Solution` states one observable behaviour per entry — no implementation detail, SQL, API signatures, or class names. `Main User Flow` is a numbered list of user-facing steps (1., 2., 3.). Every aspect seeds at least one real content row/bullet/step; add further rows only when a second real fact exists — never a placeholder cell. This is the `PRD_ASPECT_SEED_MIN` contract (open-ended at 1 per aspect).

## Priority ordering

`PR-<PROJECT-NAME-ID>-NNN` entries must be listed in priority order: all P0 entries first, then P1, then P2. The NNN counter is global across tiers; the first entry (`-001`) is always P0. No P1 entry may appear before a P0 entry, and no P2 entry before a P1 entry. Within the same tier the author may reorder entries, but tier jumps are forbidden.
