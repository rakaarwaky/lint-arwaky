# Feature Backlog: Naming Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p naming_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p naming_rules --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| NAMI-01 | FR-NAMINGRULES-001 | All scenarios verified | P0 | Done | `cargo test -p naming_rules --lib --tests` → 0 failures at `29c71083` | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Valid snake_case file, 3+ words, recognized layer prefix | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| File with uppercase characters in stem | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| File with only 2 words | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| File with hyphens in stem | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| File with dots in stem | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Barrel file or module entry point | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| File in the rule exception list | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Valid file below the configured `min_words` | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| File with an unrecognized prefix | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| File with digits in a segment | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Prefix and suffix both in the taxonomy strict allow-list | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Taxonomy prefix with a contract-layer suffix | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Contract prefix with a taxonomy-layer suffix | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Agent layer with a suffix outside its strict allow-list | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Utility layer with a flexible, non-forbidden suffix | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Utility layer with a forbidden suffix | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Capabilities layer with a forbidden suffix | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Capabilities layer with a flexible, non-forbidden suffix | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Surface layer with a suffix in its strict allow-list | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Surface layer with a suffix outside its strict allow-list | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Root layer with a suffix in its strict allow-list | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Root layer with a suffix outside its strict allow-list | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| The build script | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| File in the exception list for its layer | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| File with no suffix beyond the prefix | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Rule AES101 disabled in config | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| Rule AES102 disabled in config | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |
| File in the exceptions list | Automated | `tests/naming-rules/` | cargo test -p naming_rules | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p naming_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 28 scenarios mapped; all Automated via `cargo test -p naming_rules` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
