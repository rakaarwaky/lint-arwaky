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

1. **Single source of truth.** State vocabulary and Status Policy live
   here and nowhere else. Feature backlogs cite them.
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
5. **Verify** → `lint-arwaky-cli docs` passes; all phases have status.

## Template

Copy, fill, delete nothing.

```markdown
# ROADMAP — <workspace name>

State: vocabulary below. Last Updated: <YYYY-MM-DD>

## Current Condition

- Done: <evidence> · In Progress: <None | item> · Blocked: <None | item>
- Next: <open row IDs>

## State Definitions

| State | Meaning |
|---|---|
| Ready | Specified; not started. |
| In Progress | Active now. |
| In Review | PR open. |
| Done | Command + commit evidence. |

## Status Policy

- Verified, not self-reported: re-run command; cite commit hash (not "today").
- Same PR updates every backlog row the change invalidates.
- Prefixes: workspace `WS-` · feature `<SCOPE>-` (feature rows stay in their BACKLOG).

## Feature Roll-up

One table: every feature. Feature detail stays in each feature's BACKLOG.

| ID | Feature | Priority | State | Dependencies | Next Action | Updated |
|---|---|---|---|---|---|---|
| <SCOPE>-01 | Feature | P0 | In Progress | — | <next row ID> | <YYYY-MM-DD> |

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
| State Definitions  | SSOT for state vocabulary. Feature backlogs cite it.            |
| Status Policy      | SSOT for verification + ID prefixes.                               |
| Feature Roll-up    | Index + state + `WS-*` in one place; feature rows stay in BACKLOG. |
| Risk Register      | Workspace-level what-could-ship-broken.                            |

Per-feature dependency detail belongs in each feature's BACKLOG.md, and
workspace-level release history belongs in CHANGELOG.md; the ROADMAP
carries neither.

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: master root presence, feature pairing, unknown states, missing sections.
```
