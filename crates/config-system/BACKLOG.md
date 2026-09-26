# Feature Backlog: Config System

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p config_system --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| CONF-01 | FR-001 | SCEN-001 - Config Discovery and Loading — 8 scenarios verified | P0 | Done | `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CONF-02 | FR-002 | SCEN-002 - Language Resolution — 2 scenarios verified | P0 | Done | `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CONF-03 | FR-003 | SCEN-003 - Workspace Detection — 9 scenarios verified | P0 | Done | `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CONF-04 | FR-004 | SCEN-004 - Workspace Members — 4 scenarios verified | P0 | Done | `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CONF-05 | FR-005 | SCEN-005 - Config Merging — 5 scenarios verified | P0 | Done | `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CONF-06 | FR-006 | SCEN-006 - Validation — 6 scenarios verified | P0 | Done | `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CONF-07 | FR-007 | SCEN-007 - Caching — 2 scenarios verified | P0 | Done | `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CONF-08 | FR-008 | SCEN-008 - Ignored Paths — 4 scenarios verified | P0 | Done | `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CONF-09 | FR-009 | SCEN-009 - TOML Parsing — 3 scenarios verified | P0 | Done | `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Config exists at project root | Loaded from project root | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Config not at root, exists at parent (depth 1) | Loaded from parent | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Config not at root/parent, exists at XDG user | Loaded from XDG user | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Config only at XDG system dir | Loaded from XDG system | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| No config anywhere | Embedded defaults used | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Symlink pointing outside project root | Rejected | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| YAML parse failure at priority 1 | Warning logged, priority 2 searched | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Permission denied at priority 1 | Warning logged, priority 2 searched | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Any language (Rust/Python/TypeScript) | `lint_arwaky.config.yaml` | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Unknown language | Empty list, embedded defaults | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Directory with Cargo.toml | Rust | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Directory with pyproject.toml | Python | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Directory with package.json | TypeScript | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Parent dir is `crates/` | Rust | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Parent dir is `packages/` | TypeScript | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Parent dir is `modules/` | Python | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| No markers anywhere | Unknown | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Both Cargo.toml and package.json | First match wins | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Directory with `__init__.py` only | Python | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Root with crates/foo, crates/bar | [crates/foo, crates/bar] | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Root with no workspace dirs | Empty vec + warning | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Root is `crates/` itself | Direct subdirectories returned | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| I/O error on one member dir | Warning logged, other members returned | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Config with empty layers array | Defaults injected + warning | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Duplicate rule values | Deduplicated by value containment | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Config error during load | Defaults used + warning | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Empty ignored_paths in config | Defaults preserved (not overridden) | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Scoped rule `agent(container\ | registry)` | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Score threshold 50.0 | Valid | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Score threshold 0.0 | Valid | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Score threshold 100.0 | Valid | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Score threshold -1.0 | Invalid | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Score threshold 101.0 | Invalid | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Unknown adapter name | Enabled (default true) | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Same config file requested twice | Parsed once, cached | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Concurrent requests for same key | Single parse (DashMap) | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| No config ignored paths | 8 universal defaults returned | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Config adds "tests" | Defaults + "tests" | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Config adds ".git" (already default) | Deduplicated, not added twice | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Config adds empty string | Filtered out | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Cargo.toml with `[tool.lint-arwaky]` | Parsed correctly | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Cargo.toml without `[tool]` | Returns None | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |
| Invalid TOML syntax | ConfigError returned | Automated | `tests/config-system/` | cargo test -p config_system | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p config_system --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 43 scenarios mapped; all Automated via `cargo test -p config_system` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
