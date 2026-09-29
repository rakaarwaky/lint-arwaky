# ROADMAP — Lint Arwaky

## Current Condition

- Todo: None
- In Progress: None
- Blocked: None

## Feature Roll-up

One unified table: every feature + cross-cutting items, sorted by priority.

| ID | Priority | Feature | State | Health | Dependencies | Next Action | Spec | Backlog | Updated |
|---|---:|---|---|---|---|---|---|---|---|
| SHAR | P0 | `crates/shared` | Done | On Track | — | — | — | — | 2026-09-29 |
| CONF | P0 | `crates/config-system` | Done | On Track | — | — | [FRD](crates/config-system/FRD.md) | [BACKLOG](crates/config-system/BACKLOG.md) | 2026-09-29 |
| FILE | P0 | `crates/filesystem` | Done | On Track | — | — | [FRD](crates/filesystem/FRD.md) | [BACKLOG](crates/filesystem/BACKLOG.md) | 2026-09-29 |
| NAMI | P0 | `crates/naming-rules` | Done | On Track | — | — | [FRD](crates/naming-rules/FRD.md) | [BACKLOG](crates/naming-rules/BACKLOG.md) | 2026-09-29 |
| IMPO | P0 | `crates/import-rules` | Done | On Track | — | — | [FRD](crates/import-rules/FRD.md) | [BACKLOG](crates/import-rules/BACKLOG.md) | 2026-09-29 |
| QUAL | P0 | `crates/quality-rules` | Done | On Track | — | — | [FRD](crates/quality-rules/FRD.md) | [BACKLOG](crates/quality-rules/BACKLOG.md) | 2026-09-29 |
| ROLE | P0 | `crates/role-rules` | Done | On Track | — | — | [FRD](crates/role-rules/FRD.md) | [BACKLOG](crates/role-rules/BACKLOG.md) | 2026-09-29 |
| ORPH | P0 | `crates/orphan-rules` | Done | On Track | — | — | [FRD](crates/orphan-rules/FRD.md) | [BACKLOG](crates/orphan-rules/BACKLOG.md) | 2026-09-29 |
| STRU | P0 | `crates/structure-rules` | Done | On Track | — | — | [FRD](crates/structure-rules/FRD.md) | [BACKLOG](crates/structure-rules/BACKLOG.md) | 2026-09-29 |
| DISP | P0 | `crates/dispatcher` | Done | On Track | — | — | [DESIGN](crates/dispatcher/DESIGN.md) | — | 2026-09-29 |
| CLIC | P0 | `crates/cli-commands` | Done | On Track | — | — | [DESIGN](crates/cli-commands/DESIGN.md) | — | 2026-09-29 |
| EXTE | P1 | `crates/external-lint` | Done | On Track | — | — | [FRD](crates/external-lint/FRD.md) | [BACKLOG](crates/external-lint/BACKLOG.md) | 2026-09-29 |
| AUTO | P1 | `crates/auto-fix` | Done | On Track | — | — | [FRD](crates/auto-fix/FRD.md) | [BACKLOG](crates/auto-fix/BACKLOG.md) | 2026-09-29 |
| REPO | P1 | `crates/report-formatter` | Done | On Track | — | — | [FRD](crates/report-formatter/FRD.md) | [BACKLOG](crates/report-formatter/BACKLOG.md) | 2026-09-29 |
| MCPP | P1 | `crates/mcp-server` | Done | On Track | — | — | [DESIGN](crates/mcp-server/DESIGN.md) | — | 2026-09-29 |
| GITH | P1 | `crates/git-hooks` | Done | On Track | — | — | [FRD](crates/git-hooks/FRD.md) | [BACKLOG](crates/git-hooks/BACKLOG.md) | 2026-09-29 |
| FILW | P1 | `crates/file-watch` | Done | On Track | — | — | [FRD](crates/file-watch/FRD.md) | [BACKLOG](crates/file-watch/BACKLOG.md) | 2026-09-29 |
| PRJS | P1 | `crates/project-setup` | Done | On Track | — | — | [FRD](crates/project-setup/FRD.md) | [BACKLOG](crates/project-setup/BACKLOG.md) | 2026-09-29 |
| MAINT | P1 | `crates/maintenance` | Done | On Track | — | — | [FRD](crates/maintenance/FRD.md) | [BACKLOG](crates/maintenance/BACKLOG.md) | 2026-09-29 |
| TUIC | P2 | `crates/tui` | Done | On Track | — | — | [DESIGN](crates/tui/DESIGN.md) | — | 2026-09-29 |

## Status Policy

- Status is **verified, not self-reported**. A row reaches `Done` only after someone re-ran the evidence command and read its output.
- A recorded verification names a **commit hash**, not "today".
- A PR that merges a fix updates **every** row that fix invalidates, in the same PR.

## State Definitions

| State | Meaning |
|---|---|
| Idea | Captured, not yet examined; no spec exists for it. |
| Refinement | Being specced; a spec or product decision is needed first. |
| Ready | Specified enough to start; nobody has started it. |
| In Progress | Someone is in it now. |
| Blocked | Cannot proceed; name the blocker in `Actual Condition`. |
| In Review | PR open, awaiting review/CI. |
| QA | Implemented; awaiting a verification pass against evidence. |
| Done | Evidenced complete — cites the command/commit/PR. |
| Released | Done and shipped in a release. |
| Deferred | Intentionally out of current scope; reason in Next Action. |

## Health Definitions

| Health | Meaning |
|---|---|
| On Track | Nothing threatens the tier's scope. |
| At Risk | Open gaps could compromise the tier's gate. |
| Blocked | Work cannot proceed; name the blocker. |
| Ready for QA | No open rows; a verification sweep is outstanding. |
| Ready for Release | All evidence for the feature is recorded. |
| Released | Shipped. |

## Risk Register

- **Risk:** `aa docs check` still reports errors in `FRD.md` files (doc-length overruns, doc-thin). **Mitigation:** Fix before v3.7.0 release.
- **Risk (closed 2026-09-17):** Broken doc tests after documentation upgrade. **Resolved by** `bb715442`: all doc tests pass; self-lint clean.

## Change Log

| Date | Change |
|---|---|
| 2026-09-17 | Initial master backlog created; 19 feature backlogs added |
| 2026-09-29 | Consolidated: single table with Spec/Backlog columns, removed WS-* cross-cutting rows |
