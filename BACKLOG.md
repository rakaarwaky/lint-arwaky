# BACKLOG — Lint Arwaky

State / Health: defined below — all feature backlogs cite these, never repeat them.
Last Updated: 2026-09-17

## Feature Index

| Feature | Tier | Spec | Backlog |
|---|---|---|---|
| `crates/shared` | P0 | [FRD](crates/shared/FRD.md) | [BACKLOG](crates/shared/BACKLOG.md) |
| `crates/config-system` | P0 | [FRD](crates/config-system/FRD.md) | [BACKLOG](crates/config-system/BACKLOG.md) |
| `crates/filesystem` | P0 | [FRD](crates/filesystem/FRD.md) | [BACKLOG](crates/filesystem/BACKLOG.md) |
| `crates/naming-rules` | P0 | [FRD](crates/naming-rules/FRD.md) | [BACKLOG](crates/naming-rules/BACKLOG.md) |
| `crates/import-rules` | P0 | [FRD](crates/import-rules/FRD.md) | [BACKLOG](crates/import-rules/BACKLOG.md) |
| `crates/quality-rules` | P0 | [FRD](crates/quality-rules/FRD.md) | [BACKLOG](crates/quality-rules/BACKLOG.md) |
| `crates/role-rules` | P0 | [FRD](crates/role-rules/FRD.md) | [BACKLOG](crates/role-rules/BACKLOG.md) |
| `crates/orphan-rules` | P0 | [FRD](crates/orphan-rules/FRD.md) | [BACKLOG](crates/orphan-rules/BACKLOG.md) |
| `crates/external-lint` | P1 | [FRD](crates/external-lint/FRD.md) | [BACKLOG](crates/external-lint/BACKLOG.md) |
| `crates/auto-fix` | P1 | [FRD](crates/auto-fix/FRD.md) | [BACKLOG](crates/auto-fix/BACKLOG.md) |
| `crates/report-formatter` | P1 | [FRD](crates/report-formatter/FRD.md) | [BACKLOG](crates/report-formatter/BACKLOG.md) |
| `crates/dispatcher` | P0 | [FRD](crates/dispatcher/FRD.md) | [BACKLOG](crates/dispatcher/BACKLOG.md) |
| `crates/cli-commands` | P0 | [FRD](crates/cli-commands/FRD.md) | [BACKLOG](crates/cli-commands/BACKLOG.md) |
| `crates/mcp-server` | P1 | [FRD](crates/mcp-server/FRD.md) | [BACKLOG](crates/mcp-server/BACKLOG.md) |
| `crates/git-hooks` | P1 | [FRD](crates/git-hooks/FRD.md) | [BACKLOG](crates/git-hooks/BACKLOG.md) |
| `crates/file-watch` | P1 | [FRD](crates/file-watch/FRD.md) | [BACKLOG](crates/file-watch/BACKLOG.md) |
| `crates/project-setup` | P1 | [FRD](crates/project-setup/FRD.md) | [BACKLOG](crates/project-setup/BACKLOG.md) |
| `crates/maintenance` | P1 | [FRD](crates/maintenance/FRD.md) | [BACKLOG](crates/maintenance/BACKLOG.md) |
| `crates/tui` | P2 | [FRD](crates/tui/FRD.md) | [BACKLOG](crates/tui/BACKLOG.md) |
| Root (PRD) | P0 | [PRD.md](PRD.md) | this file |

## Current Condition

- Done: `cargo test --workspace --lib --tests` → 0 failures at `29c71083` (2026-09-17). CI self-lint job green.
- In Progress: None.
- Blocked: None
- Next Action: v3.7.0 release gate — `aa docs check . --strict` clean (verified 2026-09-17).

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
| Deferred | Intentionally out of current scope; reason in `Actual Condition`. |

## Health Definitions

| Health | Meaning |
|---|---|
| On Track | Nothing threatens the tier's scope. |
| At Risk | Open gaps could compromise the tier's gate. |
| Blocked | Work cannot proceed; name the blocker. |
| Ready for QA | No open backlog items; a verification sweep is outstanding. |
| Ready for Release | All evidence for the feature is recorded. |
| Released | Shipped. |

## Status Policy

- Status is **verified, not self-reported**. A row reaches `Done` only after someone re-ran the evidence command and read its output.
- A recorded verification names a **commit hash**, not "today".
- A PR that merges a fix updates **every** backlog row that fix invalidates, in the same PR.
- `Last Updated` moves only with a change in the file.

ID prefix table:

| Prefix | Feature |
|---|---|
| SHAR | shared |
| CONF | config-system |
| FILE | filesystem |
| NAMI | naming-rules |
| IMPO | import-rules |
| QUAL | quality-rules |
| ROLE | role-rules |
| ORPH | orphan-rules |
| EXTE | external-lint |
| AUTO | auto-fix |
| REPO | report-formatter |
| DISP | dispatcher |
| CLIC | cli-commands |
| MCPP | mcp-server |
| GITH | git-hooks |
| FILW | file-watch |
| PRJS | project-setup |
| MAINT | maintenance |
| TUIC | tui |
| WS | cross-cutting (this file) |

## Feature Roll-up

| Feature | Tier | State | Health | Next Action |
|---|---|---|---|---|
| `crates/shared` | P0 | Done | On Track | — |
| `crates/config-system` | P0 | Done | On Track | — |
| `crates/filesystem` | P0 | Done | On Track | — |
| `crates/naming-rules` | P0 | Done | On Track | — |
| `crates/import-rules` | P0 | Done | On Track | — |
| `crates/quality-rules` | P0 | Done | On Track | — |
| `crates/role-rules` | P0 | Done | On Track | — |
| `crates/orphan-rules` | P0 | Done | On Track | — |
| `crates/dispatcher` | P0 | Done | On Track | — |
| `crates/cli-commands` | P0 | Done | On Track | — |
| `crates/external-lint` | P1 | Done | On Track | — |
| `crates/auto-fix` | P1 | Done | On Track | — |
| `crates/report-formatter` | P1 | Done | On Track | — |
| `crates/mcp-server` | P1 | Done | On Track | — |
| `crates/git-hooks` | P1 | Done | On Track | — |
| `crates/file-watch` | P1 | Done | On Track | — |
| `crates/project-setup` | P1 | Done | On Track | — |
| `crates/maintenance` | P1 | Done | On Track | — |
| `crates/tui` | P2 | Done | On Track | — |

## Backlog

Cross-cutting and workspace-level rows only. Anything that belongs to one crate goes in that crate's backlog.

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| WS-01 | § PRD | All 24 AES rules enforced and self-lint clean | P0 | Done | `lint-arwaky-cli check .` → 0 violations at `29c71083` (2026-09-17); CI self-lint job green | @raka | None | 2026-09-17 |
| WS-02 | § PRD | Multi-language scan (Rust, Python, TS) | P0 | Done | `cargo test --workspace --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-03 | § PRD | CLI `check` / `scan` / `fix` / `ci` commands | P0 | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-04 | § PRD | MCP server with 5 tools, full CLI parity | P1 | Done | `cargo test -p mcp_server --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-05 | § PRD | SARIF / JUnit / JSON report formats | P1 | Done | `cargo test -p report_formatter --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-06 | § PRD | Auto-fix: remove + replace + rename | P1 | Done | `cargo test -p auto_fix --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-07 | § PRD | Git hooks pre-commit enforcement | P1 | Done | `cargo test -p git_hooks --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-08 | § PRD | Watch mode for continuous linting | P1 | Done | `cargo test -p file_watch --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-09 | § PRD | TUI file browser | P2 | Done | `cargo test -p tui --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-10 | § PRD | Windows support | P2 | Deferred | Out of scope for v3.x; no Windows CI runner. Recorded in PRD "Out of scope". | Unassigned | None | 2026-09-17 |
| WS-11 | § PRD | Deeper monorepo performance optimizations | P2 | Deferred | Current 10k-file target met; optimization deferred until real-world bottleneck identified. | Unassigned | None | 2026-09-17 |

## Blockers

None

## Dependencies

| Row | Depends on | Decision needed |
|---|---|---|
| WS-10 | — | Platform CI runner availability |
| WS-11 | WS-02 | Performance benchmark baseline |

## Release Readiness

Definition of "deployment ready" (target `v3.7.0`):

| Area | Status | Notes |
|---|---|---|
| All P0 done | Done | WS-01, WS-02, WS-03 all evidenced at `29c71083` |
| All P1 done + verified | Done | WS-04–WS-08 evidenced at `29c71083` |
| Tests pass, lint clean, build works | Done | `cargo test --workspace --lib --tests` → 0 failures; CI self-lint green at `29c71083` |
| Docs complete | Done | `aa docs check . --strict` → 0 errors, 0 warnings (verified 2026-09-17) |
| Deferred items recorded | Done | WS-10, WS-11 in Deferred state with reasons |

## Deferred

- **Windows support** (WS-10): No Windows CI runner; Linux/macOS only for v3.x. Revisit when cross-platform build matrix is added.
- **Deeper monorepo performance optimizations** (WS-11): Current performance meets the 10k-file target. Optimize only when a real bottleneck is measured.

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial master backlog created; 19 feature backlogs added | @raka |

## Branches in Flight

| Branch | Backlog IDs | State |
|---|---|---|
| `docs/embedded-skill-pack-names` | WS-01 | merged as PR #267 (`d01d5ce3`) |
| `fix/embedded-skill-pack` | WS-01 | merged as PR #266 (`7ee5b988`) |
| `refactor/consolidate-install-scripts` | WS-03 | merged as PR #271 (`bb715442`) |

## Risk Register

- **Risk:** `aa docs check` still reports errors in `FRD.md` files (doc-length overruns, doc-thin). **Mitigation:** WS-04 row in feature backlogs; fix before v3.7.0 release.
- **Risk (closed 2026-09-17):** Broken doc tests after documentation upgrade. **Resolved by** `bb715442`: all doc tests pass; self-lint clean.
