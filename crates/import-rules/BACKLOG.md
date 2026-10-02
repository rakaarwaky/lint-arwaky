# Feature Backlog: Import Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p import_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p import_rules --lib --tests` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| IMPO-01 | P0 | Done | On Track | None | — | 2026-09-17  |
| IMPO-02 | P0 | Done | On Track | None | — | 2026-09-17  |
| IMPO-03 | P0 | Done | On Track | None | — | 2026-09-17  |
| IMPO-04 | P0 | Done | On Track | None | — | 2026-09-17  |

## Scenario Evidence

| Scenario | Rule / expected | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|---|
| File imports from a forbidden layer | AES201 CRITICAL | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| File imports from an allowed layer | No violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| File imports from a layer not listed in either allowed or forbidden | AES201 WARNING (grey area) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| File with no imports | No violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Capabilities file importing utility | No violation (allowed by matrix) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Utility file importing capabilities | AES201 CRITICAL (forbidden) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Surface component file importing contract | AES201 CRITICAL (forbidden) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Surface command file importing contract aggregate | No violation (allowed) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Agent importing capabilities | AES201 CRITICAL (forbidden, via DI) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Contract protocol importing contract aggregate | AES201 CRITICAL (forbidden) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Capabilities file missing taxonomy import | AES202 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Capabilities file missing contract(protocol) import | AES202 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Capabilities file with both taxonomy and contract(protocol) imports | No violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| File in exception list | No violation (exclusion) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Taxonomy entity file missing taxonomy vo import | AES202 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import declared but never referenced in code body | AES203 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import declared and used in code | No violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import used only in comments | AES203 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import used only inside macro body (non-derive) | No violation (exempt) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import used inside `#[derive(...)]` | No violation (detected) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Function named `_use_*` containing import reference | AES204 violation (dummy function) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Function named `dummy_*` containing import reference | AES204 violation (dummy function) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Trait impl with all method bodies equal to `todo!()` | AES204 violation (dummy impl) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Trait impl with one real method plus one `todo!()` method | No violation (has real logic) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import referenced only inside a dummy function, not in real logic | AES204 violation (dummy import) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Import referenced in both a dummy function and real logic | No violation (real usage exists) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| `pub use` re-export | No violation (public API) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Taxonomy file with `_use_vo()` referencing taxonomy VO unused in real logic | AES204 violation (taxonomy intent) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Surface file calling business logic function directly | AES204 violation (surface logic bypass) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Barrel file with re-exports | No violation (exempt) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Two layers importing each other | AES205 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Linear dependency chain | No violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Self-import (file imports itself) | No violation (silently ignored) | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Indirect cycle (A -> B -> C -> A) | AES205 violation | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| Rule disabled in config | No violation for that rule | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| File in exceptions list | No violation for that file | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |
| File matching multiple scope conditions | Checked against all matched conditions | Automated | `tests/import-rules/` | cargo test -p import_rules | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p import_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 37 scenarios mapped; all Automated via `cargo test -p import_rules` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
