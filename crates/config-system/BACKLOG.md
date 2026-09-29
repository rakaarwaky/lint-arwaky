# Feature Backlog: Config System

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo nextest run -p config-system-lint-arwaky` → 138 passed, 0 failed (2026-09-29)
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run -p config-system-lint-arwaky` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| CONF-01 | FR-ConfigSystem-001 | SCEN-001 — Config Discovery and Loading (15 scenarios) | P0 | Done | `cargo nextest run -p config-system-lint-arwaky` → 138 passed, 0 failed at `eaee07f5` (2026-09-29) | @raka | None | 2026-09-29 |
| CONF-02 | FR-ConfigSystem-002 | SCEN-002 — Workspace Type Detection (9 scenarios) | P0 | Done | `cargo nextest run -p config-system-lint-arwaky` → 138 passed, 0 failed at `eaee07f5` (2026-09-29) | @raka | None | 2026-09-29 |
| CONF-03 | FR-ConfigSystem-003 | SCEN-003 — Workspace Member Discovery (4 scenarios) | P0 | Done | `cargo nextest run -p config-system-lint-arwaky` → 138 passed, 0 failed at `eaee07f5` (2026-09-29) | @raka | None | 2026-09-29 |
| CONF-04 | FR-ConfigSystem-004 | SCEN-004 (Config Merging) + SCEN-005 (Validation) — 11 scenarios total | P0 | Done | `cargo nextest run -p config-system-lint-arwaky` → 138 passed, 0 failed at `eaee07f5` (2026-09-29) | @raka | None | 2026-09-29 |
| CONF-05 | FR-ConfigSystem-005 | SCEN-006 — Ignored Paths Resolution (4 scenarios) | P0 | Done | `cargo nextest run -p config-system-lint-arwaky` → 138 passed, 0 failed at `eaee07f5` (2026-09-29) | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Last verified |
|---|---|---|---|
| Config exists at project root | Automated | acceptance_FR_001.rs | eaee07f5 |
| Config not at root, exists at parent (depth 1) | Automated | acceptance_FR_001.rs | eaee07f5 |
| Config not at root/parent, exists at XDG user | Automated | acceptance_FR_001.rs | eaee07f5 |
| Config only at XDG system dir | Automated | acceptance_FR_001.rs | eaee07f5 |
| No config anywhere | Automated | acceptance_FR_001.rs | eaee07f5 |
| Symlink pointing outside project root | Automated | acceptance_FR_001.rs | eaee07f5 |
| YAML parse failure at priority 1 | Automated | acceptance_FR_001.rs | eaee07f5 |
| Permission denied at priority 1 | Automated | acceptance_FR_001.rs | eaee07f5 |
| TOML with `[tool.lint-arwaky]` | Automated | acceptance_FR_009.rs | eaee07f5 |
| TOML without `[tool]` section | Automated | acceptance_FR_009.rs | eaee07f5 |
| Invalid TOML syntax | Automated | acceptance_FR_009.rs | eaee07f5 |
| Rust/Python/TypeScript workspace detection | Automated | acceptance_FR_002.rs | eaee07f5 |
| Unknown language | Automated | acceptance_FR_002.rs | eaee07f5 |
| Directory with Cargo.toml | Automated | acceptance_FR_002.rs | eaee07f5 |
| Directory with pyproject.toml | Automated | acceptance_FR_002.rs | eaee07f5 |
| Directory with package.json | Automated | acceptance_FR_002.rs | eaee07f5 |
| Parent dir is `crates/`/`packages/`/`modules/` | Automated | acceptance_FR_002.rs | eaee07f5 |
| No markers anywhere | Automated | acceptance_FR_002.rs | eaee07f5 |
| Both Cargo.toml and package.json | Automated | acceptance_FR_002.rs | eaee07f5 |
| Directory with `__init__.py` only | Automated | acceptance_FR_002.rs | eaee07f5 |
| Root with crates/foo, crates/bar | Automated | acceptance_FR_004.rs | eaee07f5 |
| Root with no workspace dirs | Automated | acceptance_FR_004.rs | eaee07f5 |
| Root is `crates/` itself | Automated | acceptance_FR_004.rs | eaee07f5 |
| I/O error on one member dir | Automated | acceptance_FR_004.rs | eaee07f5 |
| Config with empty layers array | Automated | acceptance_FR_005.rs | eaee07f5 |
| Duplicate rule values | Automated | acceptance_FR_005.rs | eaee07f5 |
| Config error during load | Automated | acceptance_FR_005.rs | eaee07f5 |
| Empty ignored_paths in config | Automated | acceptance_FR_005.rs | eaee07f5 |
| Scoped rule `agent(container\|registry)` | Automated | acceptance_FR_005.rs | eaee07f5 |
| Score threshold 50.0 | Automated | acceptance_FR_006.rs | eaee07f5 |
| Score threshold 0.0 / 100.0 | Automated | acceptance_FR_006.rs | eaee07f5 |
| Score threshold -1.0 / 101.0 | Automated | acceptance_FR_006.rs | eaee07f5 |
| Unknown adapter name | Automated | acceptance_FR_006.rs | eaee07f5 |
| No config ignored paths | Automated | acceptance_FR_008.rs | eaee07f5 |
| Config adds "tests" | Automated | acceptance_FR_008.rs | eaee07f5 |
| Config adds ".git" (already default) | Automated | acceptance_FR_008.rs | eaee07f5 |
| Config adds empty string | Automated | acceptance_FR_008.rs | eaee07f5 |

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
