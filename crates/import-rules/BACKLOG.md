# Feature Backlog: Import Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p import_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p import_rules --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| IMPO-01 | FR-001 | AES203 - Unused Import — 5 scenarios verified | P0 | Done | `cargo test -p import_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| IMPO-02 | FR-002 | AES204 - Dummy Import — 10 scenarios verified | P0 | Done | `cargo test -p import_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| IMPO-03 | FR-003 | AES205 - Circular Dependency — 4 scenarios verified | P0 | Done | `cargo test -p import_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| IMPO-04 | FR-004 | Configuration — 3 scenarios verified | P0 | Done | `cargo test -p import_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Import declared but never referenced in code | AES203 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import declared and used in code | No violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import used only in comments | AES203 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import used only inside macro body (non-derive) | No violation (exempt… | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import used in`#[derive(...)]` | No violation (detected) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Function named`_use_serialization()` containing import reference | AES… | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Function named`dummy_helper()` containing import reference | AES204 vi… | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Trait impl with all method bodies =`todo!()` | AES204 violation (dummy… | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Trait impl with 1 real method + 1`todo!()` method | No violation (has … | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import`Foo` only referenced inside `_use_foo()`, not in real logic | A… | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import`Bar` referenced in `_use_bar()` AND in real function | No viola… | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| `pub use` re-export | No violation (public API) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Taxonomy file has`_use_vo()` referencing taxonomy VO, VO not used in r… | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Surface file calls`lint_path(` directly | AES204 violation (surface lo… | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Barrel file (`mod.rs`) with re-exports | No violation (exempt) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Two layers importing each other | AES205 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Linear dependency chain | No violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Self-import (file imports itself) | No violation (silently ignored) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Indirect cycle (A → B → C → A) | AES205 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Rule disabled in config | No violation for that rule | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| File in exceptions list | No violation for that file | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| File matches multiple conditions | Checked against all matched conditi… | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p import_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 22 scenarios mapped; all Automated via `cargo test -p import_rules` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
