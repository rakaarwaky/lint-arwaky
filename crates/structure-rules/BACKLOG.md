# Feature Backlog: Structure Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo test -p lint_arwaky_structure_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p lint_arwaky_structure_rules --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| STRUC-01 | FR-STR-001 | AES701 shared folder purity — 1 scenario verified | P0 | Done | `cargo test -p lint_arwaky_structure_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29) | @raka | None | 2026-09-29 |
| STRUC-02 | FR-STR-001 | AES702 feature folder health — 1 scenario verified | P0 | Done | `cargo test -p lint_arwaky_structure_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29) | @raka | None | 2026-09-29 |
| STRUC-03 | FR-STR-001 | AES703 surface folder purity — 1 scenario verified | P0 | Done | `cargo test -p lint_arwaky_structure_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29) | @raka | None | 2026-09-29 |
| STRUC-04 | FR-STR-001 | AES704 feature folder doc pair — 1 scenario verified | P0 | Done | `cargo test -p lint_arwaky_structure_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29) | @raka | None | 2026-09-29 |
| STRUC-05 | FR-STR-001 | AES705 surface folder DESIGN.md — 1 scenario verified | P0 | Done | `cargo test -p lint_arwaky_structure_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29) | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| A shared folder holds a capability file | AES701 CRITICAL | Automated | `tests/structure_rules/` | `cargo test -p lint_arwaky_structure_rules` | `926bd34a` |
| A feature folder holds a doc pair but no orchestrator | AES702 CRITICAL | Automated | `tests/structure_rules/` | `cargo test -p lint_arwaky_structure_rules` | `926bd34a` |
| A surface folder holds a non-taxonomy file | AES703 CRITICAL | Automated | `tests/structure_rules/` | `cargo test -p lint_arwaky_structure_rules` | `926bd34a` |
| A feature folder lacks its doc pair | AES704 CRITICAL | Automated | `tests/structure_rules/` | `cargo test -p lint_arwaky_structure_rules` | `926bd34a` |
| A surface folder lacks a DESIGN.md | AES705 CRITICAL | Automated | `tests/structure_rules/` | `cargo test -p lint_arwaky_structure_rules` | `926bd34a` |
| A clean repository reports 0 structure violations on `check .` | No violation | Automated | Self-lint | `lint-arwaky-cli check .` | `926bd34a` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p lint_arwaky_structure_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29) |
| Scenario evidence | Done | 6 scenarios mapped; all Automated |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Aligned BACKLOG.md and FRD.md to HOW-TO templates (AES606 H2 contracts) | @raka |
