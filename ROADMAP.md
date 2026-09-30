# ROADMAP — Lint Arwaky

## Current Condition

- Todo: None
- In Progress: None
- Blocked: None
- Release Status: v3.7.1 Release Candidate sign-offs recorded in `DEPLOY.md` (verified at commit `8d4342a`).
- UAT Status: Persona-level acceptance scenarios (Developer, DevOps, AI Agent) verified clean (see `TEST.md` Section 4).
- Document Gate: `lint-arwaky-cli docs .` reports 0 document invariant violations across all crates and root documents.

## Feature Roll-up

One unified table: every feature + cross-cutting items, sorted by priority.

| ID | Priority | Feature | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|---|
| FR-SHAR | P0 | `crates/shared` | Done | On Track | — | — | 2026-09-30 |
| FR-CONF | P0 | `crates/config-system` | Done | On Track | — | — | 2026-09-30 |
| FR-FILE | P0 | `crates/filesystem` | Done | On Track | — | — | 2026-09-30 |
| FR-NAMI | P0 | `crates/naming-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-IMPO | P0 | `crates/import-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-QUAL | P0 | `crates/quality-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-ROLE | P0 | `crates/role-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-ORPH | P0 | `crates/orphan-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-STRU | P0 | `crates/structure-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-DISP | P0 | `crates/dispatcher` | Done | On Track | — | — | 2026-09-30 |
| FR-CLIC | P0 | `crates/cli-commands` | Done | On Track | — | — | 2026-09-30 |
| FR-EXTE | P1 | `crates/external-lint` | Done | On Track | — | — | 2026-09-30 |
| FR-AUTO | P1 | `crates/auto-fix` | Done | On Track | — | — | 2026-09-30 |
| FR-REPO | P1 | `crates/report-formatter` | Done | On Track | — | — | 2026-09-30 |
| FR-MCPP | P1 | `crates/mcp-server` | Done | On Track | — | — | 2026-09-30 |
| FR-GITH | P1 | `crates/git-hooks` | Done | On Track | — | — | 2026-09-30 |
| FR-FILW | P1 | `crates/file-watch` | Done | On Track | — | — | 2026-09-30 |
| FR-PRJS | P1 | `crates/project-setup` | Done | On Track | — | — | 2026-09-30 |
| FR-MAINT | P1 | `crates/maintenance` | Done | On Track | — | — | 2026-09-30 |
| FR-TUIC | P2 | `crates/tui` | Done | On Track | — | — | 2026-09-30 |

## Status Policy

- Status is **verified, not self-reported**. A row reaches `Done` only after someone re-ran the evidence command and read its output.
- A recorded verification names a **commit hash**, not "today".
- A PR that merges a fix updates **every** row that fix invalidates, in the same PR.
- Business outcome metrics (PRD.md Section 2) are audited per release cycle against recorded git commits.

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

- **Risk (re-verified 2026-09-30, commit `8d4342a` — closed):** Document invariant audit via `lint-arwaky-cli docs .` reports 0 violations across all `FRD.md` and workspace document chain files (doc-length overruns and doc-thin checks resolved in PR #500 / PR #520). Definition of Done document gate passes clean.
- **Risk (closed 2026-09-17):** Broken doc tests after documentation upgrade. **Resolved by** `bb715442`: all doc tests pass; self-lint clean.
- **Risk (historical gate waiver — closed 2026-09-29):** AES607 counting bug in PR #335 false-failed legitimate PRs (#344, #350, #351). A temporary branch-protection waiver was granted under the Gate Waiver Process (AGENTS.md) and resolved via PR #500 / PR #520 commit `8d4342a`.

## Change Log

| Date | Change |
|---|---|
| 2026-09-17 | Initial master backlog created; 19 feature backlogs added |
| 2026-09-29 | Consolidated: single table with FR-ID format |
| 2026-09-29 | Streamlined columns: removed Feature/Spec/Backlog; added shared DATA.md + surface BACKLOG requirements |
| 2026-09-29 | Resolved AGENTS.md section contract conflict (#493 vs #494): adopted 12 strict H2 headings (restoring Related Documents) in PR #500 / PR #509 with AES605 test enforcement |
| 2026-09-30 | Addressed 15 Business Analyst audit issues (BA-1-01 through BA-5-03): added outcome metrics, evidence citations, UAT plan, deterministic bad-workspace floors, gate waiver policy, and release checklist |
