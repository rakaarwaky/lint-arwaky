# Feature Backlog: Orphan Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p orphan_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p orphan_rules --lib --tests` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| ORPH-01 | P0 | Done | On Track | None | — | 2026-09-17  |
| ORPH-02 | P0 | Done | On Track | None | — | 2026-09-17  |
| ORPH-03 | P0 | Done | On Track | None | — | 2026-09-17  |
| ORPH-04 | P0 | Done | On Track | None | — | 2026-09-17  |
| ORPH-05 | P0 | Done | On Track | None | — | 2026-09-17  |
| ORPH-06 | P0 | Done | On Track | None | — | 2026-09-17  |
| ORPH-07 | P0 | Done | On Track | None | — | 2026-09-17  |
| ORPH-08 | P0 | Done | On Track | None | — | 2026-09-17  |
| ORPH-09 | P0 | Done | On Track | None | — | 2026-09-17  |
| ORPH-10 | P0 | Done | On Track | None | — | 2026-09-17  |

## Scenario Evidence

| Scenario | Rule / expected | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|---|
| Workspace with 100 files, 5 orphans across 3 layers | All 5 detected, … | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Circular imports between two capabilities | Both reachable, neither fl… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Workspace with zero entry points | All non-barrel files flagged as orp… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Cross-crate imports (crate A imports from crate B) | Graph resolves co… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Configuration disabled | Full orphan scan returns empty immediately | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| File with parse failure | Flagged as orphan (fail-strict) | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Python `__init__.py` package marker | Skipped, not flagged | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| TypeScript barrel `index.ts` re-exports | Skipped, not flagged | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Rust `mod.rs` re-exports | Skipped, not flagged | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Rust `lib.rs` library root | Skipped, not flagged | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Taxonomy file imported by a contract file | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Taxonomy file imported only by other taxonomy files | Orphan (no non-t… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Taxonomy file with no inbound links | Orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Taxonomy file imported by capabilities file | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Protocol with implementation | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Protocol with no implementation | Orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Aggregate re-exported in barrel file | Not orphan (public API) | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Aggregate implemented by agent | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Contract file with no traits (only type aliases) | Not orphan (nothing… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Capability struct referenced in container file | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Capability file transitively reachable from entry point | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Capability file not in alive set, not in any container | Orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Capability imported by other capabilities, chain reaches container | N… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Utility imported by a capabilities file | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Utility imported only by other utilities | Orphan (utility chain = dea… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Utility with no inbound links | Orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Utility imported by agent file | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Agent aggregate called by container file | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Agent aggregate not called by any container/lib | Orphan (HIGH) | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Agent with no aggregate implementation | Not orphan (skip check) | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Agent with aggregate traits, none found in containers | Orphan (HIGH) | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Smart surface (`_command`) reachable from entry point | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Smart surface not reachable from any entry point | Orphan (HIGH) | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Utility surface (`_hook`) reachable from entry point | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Utility surface not reachable from any entry point | Orphan (MEDIUM) | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Passive surface (`_component`) reachable from entry point | Not orphan | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Passive surface not reachable from any entry point | Orphan (LOW) | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Surface file with unclassifiable suffix | Skipped (no check) | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Config `check_orphan: false` for a layer | No violations for that laye… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Config with exceptions list | Excepted files produce no violations | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Config with `ignored_paths: ["tests"]` | `tests/` segment files produc… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Config with AES501 disabled | No taxonomy orphan violations | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Config with custom entry point patterns | Additional entry points reco… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| 10,000 file workspace | Completes in under 5 seconds | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |
| Contract analyzer with 50 traits × 500 files | Completes in under 2 se… | Automated | `tests/orphan-rules/` | cargo test -p orphan_rules | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p orphan_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 46 scenarios mapped; all Automated via `cargo test -p orphan_rules` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
