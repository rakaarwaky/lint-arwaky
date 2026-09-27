# Feature Backlog: Shared

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p shared --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p shared --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| SHAR-01 | FR-SHARED-001 | All scenarios verified | P0 | Done | `cargo test -p shared --lib --tests` → 0 failures at `29c71083` | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Every public contract trait in the crate compiles cleanly with the `Send` and `Sync` auto-traits in scope | Automated | `tests/shared/` | cargo test -p shared | `29c71083` |
| Expanding the `string_value_object!` macro produces a type that implements `Display` and a borrowed-string conversion | Automated | `tests/shared/` | cargo test -p shared | `29c71083` |
| Expanding the `primitive_value_object!` macro produces a type that implements `Display` and is `Copy` | Automated | `tests/shared/` | cargo test -p shared | `29c71083` |
| The `score_impact` method on every severity variant returns the documented numeric weight | Automated | `tests/shared/` | cargo test -p shared | `29c71083` |
| The `ExitCode` constants map to the exact values 0, 1, 2, and 3 in the documented order | Automated | `tests/shared/` | cargo test -p shared | `29c71083` |
| The language detector recognises `.rs`, `.py`, `.ts`, and `.js` extensions and returns the correct language enum for each | Automated | `tests/shared/` | cargo test -p shared | `29c71083` |
| The compliance-score computation yields a value that is always within the 0-to-100 range for any valid input set | Automated | `tests/shared/` | cargo test -p shared | `29c71083` |
| The path-ignored check filters paths that match `**/*.ext` glob patterns while leaving non-matching paths untouched | Automated | `tests/shared/` | cargo test -p shared | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p shared --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 8 scenarios mapped; all Automated via `cargo test -p shared` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
