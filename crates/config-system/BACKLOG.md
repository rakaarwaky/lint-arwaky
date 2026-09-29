# Feature Backlog: Config System

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo nextest run -p config_system_lint_arwaky` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run -p config_system_lint_arwaky` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| CONF-01 | P0 | Done | On Track | None | — | 2026-09-29  |
| CONF-02 | P0 | Done | On Track | None | — | 2026-09-29  |
| CONF-03 | P0 | Done | On Track | None | — | 2026-09-29  |
| CONF-04 | P0 | Done | On Track | None | — | 2026-09-29  |
| CONF-05 | P0 | Done | On Track | None | — | 2026-09-29  |

## Scenario Evidence

| Scenario | Kind | Test file | Last verified |
|---|---|---|---|
| Config exists at project root | Automated | acceptance_FR_001.rs | current |
| Config not at root, exists at parent (depth 1) | Automated | acceptance_FR_001.rs | current |
| Config not at root/parent, exists at XDG user | Automated | acceptance_FR_001.rs | current |
| Config only at XDG system dir | Automated | acceptance_FR_001.rs | current |
| No config anywhere | Automated | acceptance_FR_001.rs | current |
| Symlink pointing outside project root | Automated | acceptance_FR_001.rs | current |
| YAML parse failure at priority 1 | Automated | acceptance_FR_001.rs | current |
| Permission denied at priority 1 | Automated | acceptance_FR_001.rs | current |
| TOML with `[tool.lint-arwaky]` | Automated | acceptance_FR_009.rs | current |
| TOML without `[tool]` section | Automated | acceptance_FR_009.rs | current |
| Invalid TOML syntax | Automated | acceptance_FR_009.rs | current |
| Rust/Python/TypeScript workspace detection | Automated | acceptance_FR_002.rs | current |
| Unknown language | Automated | acceptance_FR_002.rs | current |
| Directory with Cargo.toml | Automated | acceptance_FR_002.rs | current |
| Directory with pyproject.toml | Automated | acceptance_FR_002.rs | current |
| Directory with package.json | Automated | acceptance_FR_002.rs | current |
| Parent dir is `crates/`/`packages/`/`modules/` | Automated | acceptance_FR_002.rs | current |
| No markers anywhere | Automated | acceptance_FR_002.rs | current |
| Both Cargo.toml and package.json | Automated | acceptance_FR_002.rs | current |
| Directory with `__init__.py` only | Automated | acceptance_FR_002.rs | current |
| Root with crates/foo, crates/bar | Automated | acceptance_FR_004.rs | current |
| Root with no workspace dirs | Automated | acceptance_FR_004.rs | current |
| Root is `crates/` itself | Automated | acceptance_FR_004.rs | current |
| I/O error on one member dir | Automated | acceptance_FR_004.rs | current |
| Config with empty layers array | Automated | acceptance_FR_005.rs | current |
| Duplicate rule values | Automated | acceptance_FR_005.rs | current |
| Config error during load | Automated | acceptance_FR_005.rs | current |
| Empty ignored_paths in config | Automated | acceptance_FR_005.rs | current |
| Scoped rule `agent(container\|registry)` | Automated | acceptance_FR_005.rs | current |
| Score threshold 50.0 | Automated | acceptance_FR_006.rs | current |
| Score threshold 0.0 / 100.0 | Automated | acceptance_FR_006.rs | current |
| Score threshold -1.0 / 101.0 | Automated | acceptance_FR_006.rs | current |
| Unknown adapter name | Automated | acceptance_FR_006.rs | current |
| No config ignored paths | Automated | acceptance_FR_008.rs | current |
| Config adds "tests" | Automated | acceptance_FR_008.rs | current |
| Config adds ".git" (already default) | Automated | acceptance_FR_008.rs | current |
| Config adds empty string | Automated | acceptance_FR_008.rs | current |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | All scenarios covered by automated tests |
| Scenario evidence | Done | 35 scenarios mapped across 5 FRs |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Rewrote FRD: consolidated 10 mixed FRs into 5 pure business capabilities; moved utility concerns (caching, language mapping, TOML parsing) out of FR scope | @raka |
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
