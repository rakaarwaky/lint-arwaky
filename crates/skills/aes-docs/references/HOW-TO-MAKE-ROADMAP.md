# HOW TO MAKE ROADMAP.md

> **Purpose**: Index features, own shared policy, and record
> workspace-level condition.
>
> **Audience**: Tech Lead, PM, Engineers.
>
> **Boundary**: The single source of truth for workspace state,
> cross-cutting risks, and feature tracking.
>
> **Scope**: Exactly one ROADMAP.md at the project root.
>
> **Location**: Project root.
>
> **Length**: 50–500 lines

---

## Rules




Four rules. Each one prevents a specific failure mode.

1. **Single source of truth.** State vocabulary, Health vocabulary, and
 Status Policy live here and nowhere else. Feature backlogs cite them.
2. **Index every feature.** Every feature directory must appear in the
 Feature Index table, linking to its FRD.md and BACKLOG.md
 (`no-master-roadmap`).
3. **Cross-cutting rows only.** The workspace backlog is for items that
 span multiple features or infrastructure.
4. **ID scopes are explicit.** The root file uses a workspace prefix
 (`WS-01`). Feature prefixes are documented in the Status Policy table.

---




## Workflow

1. **Create file** → `ROADMAP.md` at repo root.
2. **Section: Current State** — where we are now.
3. **Section: Roadmap** — phased milestones with dates.
4. **Section: Status Policy** — how to update progress.
5. **Verify** → `aa check docs` passes; all phases have status.

## Template

Copy, fill, delete nothing.

```markdown
# ROADMAP — <workspace name>

State / Health: vocabulary below. Last Updated: <YYYY-MM-DD>

## Current Condition

- Done: <evidence> · In Progress: <None | item> · Blocked: <None | item>
- Next: <open row IDs>

## State Definitions

| State | Meaning |
|---|---|
| Idea | Not examined; no spec. |
| Refinement | Being specced. |
| Ready | Specified; not started. |
| In Progress | Active now. |
| Blocked | Name the blocker in Actual Condition. |
| In Review | PR open. |
| QA | Awaiting verification pass. |
| Done | Command + commit evidence. |
| Released | Shipped. |
| Deferred | Out of scope; reason in Actual Condition. |

| Health | Meaning |
|---|---|
| On Track | No threat to the gate. |
| At Risk | Gaps may miss the gate. |
| Blocked | Cannot proceed. |
| Ready for QA | Open rows clear; sweep left. |
| Ready for Release | Evidence recorded. |
| Released | Shipped. |

## Status Policy

- Verified, not self-reported: re-run command; cite commit hash (not "today").
- Same PR updates every backlog row the change invalidates.
- Prefixes: workspace `WS-` · feature `<SCOPE>-` (feature rows stay in their BACKLOG).

## Feature Roll-up

One table: every feature. Feature detail stays in each feature's BACKLOG.

| ID | Item | Priority | Spec | Backlog | State | Health | Owner | Next | Updated |
|---|---|---|---|---|---|---|---|---|---|
| modules/<a> | Feature | P0 | [FRD](modules/<a>/FRD.md) | [BACKLOG](modules/<a>/BACKLOG.md) | In Progress | On Track | — | <SCOPE>-01 | <YYYY-MM-DD> |

## Branches in Flight

| Branch | Backlog IDs | State |
|---|---|---|
| <prefix>/<name> | <IDs> | <state / PR #n> |

## Risk Register

- Risk: <what ships broken>. Mitigation: <row ID>.
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.

| Section            | Why it belongs here                                                |
| ------------------ | ------------------------------------------------------------------ |
| Current Condition  | Workspace truth in one glance.                                     |
| State Definitions  | SSOT for state + health vocabulary. Feature backlogs cite it.      |
| Status Policy      | SSOT for verification + ID prefixes.                               |
| Feature Roll-up    | Index + state + `WS-*` in one place; feature rows stay in BACKLOG. |
| Branches in Flight | Active branches and the rows they own.                             |
| Risk Register      | Workspace-level what-could-ship-broken.                            |

---

## Verify

```bash
aa check docs .
# Checks: master root presence, feature pairing, unknown states, missing sections.
```
