# HOW TO MAKE BACKLOG.md

> **Purpose**: Record what is true right now for one feature or shared folder,
> and on what evidence.
>
> **Audience**: Engineers, QA, Tech Lead.
>
> **Scope**: One BACKLOG.md per feature folder and per shared folder.
>
> **Location**: Inside the feature's directory or inside `shared/`.
>
> **Length**: 50–500 lines
>
> **Not a feature → still needs BACKLOG.** Feature folders (with an
> `agent_*_orchestrator`) carry `FRD.md` + `BACKLOG.md`. Shared/kernel folders
> carry `DATA.md` + `BACKLOG.md`. Surface folders (DESIGN-only) carry
> `DESIGN.md` + `BACKLOG.md`. Every folder type has a BACKLOG.

---

## Rules

Five rules. Each one prevents a specific failure mode.

1. **Definitions live once.** State vocabulary and status policy belong in
the root file (ROADMAP.md). Feature backlogs cite them; they never repeat
them (`state-vocab-restated`, `undefined-state-vocab`).
2. **Feature backlogs carry file-specific content only.** No policy prose, no
state tables, no copied paragraphs from the root.
3. **Every folder has a BACKLOG.** An `FRD.md` pairs with `BACKLOG.md` in
feature folders; `DATA.md` pairs with `BACKLOG.md` in shared folders;
`DESIGN.md` pairs with `BACKLOG.md` in surface folders. The root
`PRD.md` pairs with root `ROADMAP.md`, not a sibling `BACKLOG.md`.
4. **ID scopes are explicit.** Each folder uses its own prefix
(`RENDER-01`, `SCRIPT-01`). A row citing `FR-006` is checked against the
spec (`orphan-fr-id`).
5. **Status is verified, not self-reported.** A Done or Released row must
contain a backtick code span naming a command and a commit hash
(`done-without-evidence`).

---




## Workflow

1. **Create file** → `BACKLOG.md` in feature directory.
2. **Section: Current Condition** — document baseline state.
3. **Section: Backlog** — list items with status (New/In Progress/Done).
4. **Section: Scenario Evidence** — link to tests/passing gates.
5. **Verify** → `lint-arwaky-cli docs` passes; all sections present.

## Template

Copy, fill, delete nothing.

```markdown
# Feature Backlog: <Feature Name>

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md), if applicable
State / Health: values from root [ROADMAP.md](../../ROADMAP.md) — do not redefine here.
Last Updated: <YYYY-MM-DD>

## Current Condition

- Done: <command re-run → counts, at <commit>, on <date>>
- In Progress: <None | item>
- Blocked: <None | item and what clears it>
- Next Action: <the one concrete next thing; cite the row ID>

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| <SCOPE>-01 | P0 | Done | On Track | — | — | <YYYY-MM-DD> |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| <scenario copied from the spec> | Automated | tests/<file> | test_<name> | <commit> |

Kind values: Automated, Proxy, Manual, Gap.

## Blockers

<None | each blocker and what would clear it>

## Dependencies

<None | rows owned elsewhere that rows here wait on>

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | <test command> → <result> at <commit> |
| Scenario evidence | Done | <N> of <N> scenarios mapped |
| Docs | Done | [FRD.md](FRD.md) is specification-only. |

## Deferred

<None | item + reason>

## Change Log

| Date | Change | By |
|---|---|---|
| <YYYY-MM-DD> | <what moved here, with the commit> | @<owner> |
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.

| Section           | Why it belongs here                                     |
| ----------------- | ------------------------------------------------------- |
| Header links      | Connects the backlog to its spec and shared root policy.|
| Current Condition | Gives the truth before the table, not after it.         |
| Backlog           | The work rows for this folder only.                     |
| Scenario Evidence | Maps FRD scenarios to real tests or honest gaps.        |
| Blockers          | Names what stops work and what clears it.               |
| Dependencies      | Names rows owned elsewhere that this folder waits on.   |
| Release Readiness | Shows whether the folder can ship, with evidence.       |
| Deferred          | Records intentional non-work so it is not re-litigated. |
| Change Log        | Records when and why the condition changed.             |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: pairing, columns, unknown states, unevidenced Done, scenario coverage.
```
