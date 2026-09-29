# ROADMAP — Lint Arwaky

State / Health: defined below — all feature backlogs cite these, never repeat them.
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo test --workspace --lib --tests` → 0 failures at `29c71083` (2026-09-17); `lint-arwaky-cli check .` → 0 violations. CI self-lint job green.
- In Progress: None.
- Blocked: None
- Next Action: v3.7.0 release gate — WS-13, WS-14; `aa docs check . --strict` clean.

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

### ID prefixes

`WS-` (this file) · `SHAR` shared · `CONF` config-system · `FILE` filesystem · `NAMI` naming-rules · `IMPO` import-rules · `QUAL` quality-rules · `ROLE` role-rules · `ORPH` orphan-rules · `EXTE` external-lint · `AUTO` auto-fix · `REPO` report-formatter · `DISP` dispatcher · `CLIC` cli-commands · `MCPP` mcp-server · `GITH` git-hooks · `FILW` file-watch · `PRJS` project-setup · `MAINT` maintenance · `TUIC` tui.

## Feature Roll-up

One table per feature: tier, spec, backlog, current state. Feature-level detail and scenario evidence stay in each feature's `BACKLOG.md`.

| Feature | Tier | Spec | Backlog | State | Health | Next Action |
|---|---|---|---|---|---|---|
| `crates/shared` | P0 | — | — | Done | On Track | — |
| `crates/config-system` | P0 | [FRD](crates/config-system/FRD.md) | [BACKLOG](crates/config-system/BACKLOG.md) | Done | On Track | — |
| `crates/filesystem` | P0 | [FRD](crates/filesystem/FRD.md) | [BACKLOG](crates/filesystem/BACKLOG.md) | Done | On Track | — |
| `crates/naming-rules` | P0 | [FRD](crates/naming-rules/FRD.md) | [BACKLOG](crates/naming-rules/BACKLOG.md) | Done | On Track | — |
| `crates/import-rules` | P0 | [FRD](crates/import-rules/FRD.md) | [BACKLOG](crates/import-rules/BACKLOG.md) | Done | On Track | — |
| `crates/quality-rules` | P0 | [FRD](crates/quality-rules/FRD.md) | [BACKLOG](crates/quality-rules/BACKLOG.md) | Done | On Track | — |
| `crates/role-rules` | P0 | [FRD](crates/role-rules/FRD.md) | [BACKLOG](crates/role-rules/BACKLOG.md) | Done | On Track | — |
| `crates/orphan-rules` | P0 | [FRD](crates/orphan-rules/FRD.md) | [BACKLOG](crates/orphan-rules/BACKLOG.md) | Done | On Track | — |
| `crates/structure-rules` | P0 | [FRD](crates/structure-rules/FRD.md) | [BACKLOG](crates/structure-rules/BACKLOG.md) | Done | On Track | — |
| `crates/dispatcher` | P0 | [DESIGN](crates/dispatcher/DESIGN.md) | — | Done | On Track | — |
| `crates/cli-commands` | P0 | [DESIGN](crates/cli-commands/DESIGN.md) | — | Done | On Track | — |
| `crates/external-lint` | P1 | [FRD](crates/external-lint/FRD.md) | [BACKLOG](crates/external-lint/BACKLOG.md) | Done | On Track | — |
| `crates/auto-fix` | P1 | [FRD](crates/auto-fix/FRD.md) | [BACKLOG](crates/auto-fix/BACKLOG.md) | Done | On Track | — |
| `crates/report-formatter` | P1 | [FRD](crates/report-formatter/FRD.md) | [BACKLOG](crates/report-formatter/BACKLOG.md) | Done | On Track | — |
| `crates/mcp-server` | P1 | [DESIGN](crates/mcp-server/DESIGN.md) | — | Done | On Track | — |
| `crates/git-hooks` | P1 | [FRD](crates/git-hooks/FRD.md) | [BACKLOG](crates/git-hooks/BACKLOG.md) | Done | On Track | — |
| `crates/file-watch` | P1 | [FRD](crates/file-watch/FRD.md) | [BACKLOG](crates/file-watch/BACKLOG.md) | Done | On Track | — |
| `crates/project-setup` | P1 | [FRD](crates/project-setup/FRD.md) | [BACKLOG](crates/project-setup/BACKLOG.md) | Done | On Track | — |
| `crates/maintenance` | P1 | [FRD](crates/maintenance/FRD.md) | [BACKLOG](crates/maintenance/BACKLOG.md) | Done | On Track | — |
| `crates/tui` | P2 | [DESIGN](crates/tui/DESIGN.md) | — | Done | On Track | — |
| Root (PRD) | P0 | [PRD.md](PRD.md) | this file | In Progress | At Risk | WS-13, WS-14 |

## Backlog

Cross-cutting and workspace-level rows only. Anything that belongs to one crate goes in that crate's backlog; per-crate test gates live in the crate `BACKLOG.md` `Current Condition`.

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| WS-01 | § PRD | All 29 AES rules enforced and self-lint clean | P0 | Done | `lint-arwaky-cli check .` → 0 violations at `29c71083` (2026-09-17); AES700 group added in #332 and AES704/AES705 in #333; CI self-lint job green | @raka | None | 2026-09-28 |
| WS-03 | § PRD | CLI `check` / `scan` / `fix` / `ci` commands | P0 | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-04 | § PRD | MCP server with 5 tools, full CLI parity | P1 | Done | `cargo test -p mcp_server --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-09 | § PRD | TUI file browser | P2 | Done | `cargo test -p tui --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| WS-10 | § PRD | Windows support | P2 | Deferred | Out of scope for v3.x; no Windows CI runner. Revisit when a cross-platform build matrix is added. | Unassigned | None | 2026-09-17 |
| WS-11 | § PRD | Deeper monorepo performance optimizations | P2 | Deferred | 10k-file target met; optimize only when a real bottleneck is measured. | Unassigned | None | 2026-09-17 |
| WS-12 | § PRD | AES700 folder-structure group (AES701–AES703) | P0 | Done | Shared purity, feature health, surface purity; `check .` → 0, `workspaces-good` → 0, 27 codes per language (#332, #411) | @raka | None | 2026-09-29 |
| WS-13 | § PRD | v3.7.0 candidate artifact manifest: tag, binary inventory, platform coverage, checksums, provenance | P1 | Ready | Not recorded as of 2026-09-29 | Unassigned | None | 2026-09-29 |
| WS-14 | § PRD | v3.7.0 role sign-offs (Product, Engineering, QA, Documentation, Operations) | P1 | Ready | No approvals recorded as of 2026-09-29 | Unassigned | WS-13 | 2026-09-29 |

## Branches in Flight

None.

## Risk Register

- **Risk:** `aa docs check` still reports errors in `FRD.md` files (doc-length overruns, doc-thin). **Mitigation:** WS-04 row in feature backlogs; fix before v3.7.0 release.

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial master backlog created; 19 feature backlogs added | @raka |
| 2026-09-29 | Consolidated: merged Feature Index + Roll-up; dropped restated per-crate rows (WS-02, WS-05–WS-08); deferred reasons folded into Actual Condition; release criteria became WS-13/WS-14 | @raka |
