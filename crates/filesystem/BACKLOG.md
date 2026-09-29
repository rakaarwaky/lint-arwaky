# Feature Backlog: Filesystem

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p filesystem --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p filesystem --lib --tests` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| FILE-01 | P0 | Done | On Track | None | — | 2026-09-17  |
| FILE-02 | P0 | Done | On Track | None | — | 2026-09-17  |
| FILE-03 | P0 | Done | On Track | None | — | 2026-09-17  |
| FILE-04 | P0 | Done | On Track | None | — | 2026-09-17  |
| FILE-05 | P0 | Done | On Track | None | — | 2026-09-17  |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Valid Rust file | parse_ok = true, full metadata | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Rust file with syntax error | parse_ok = false, warning | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Empty file | parse_ok = true, empty metadata | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| `use crate::foo::Bar` | Resolved import entry | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| `use foo::*` | Wildcard import entry | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| `#[cfg(test)] use foo::Bar` | Not extracted | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| External dependency | Not extracted | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Barrel re-export through `mod.rs` | Resolved to original source | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| 1,000 files parsed in parallel | Completes in < 1s | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| A imports B | Edge A → B | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Circular imports | Both edges exist | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| `struct Foo` in A | Definition: "Foo" → A | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| `impl IBar for Foo` | Implementation: "IBar" → [A] | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Barrel re-export | Resolved to original source | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| File with no incoming edges | Identified as orphan | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Circular dependency chain | Cycle detected via SCC | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Workspace with 100 .rs files | All 100 discovered | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| File in .gitignore | Not discovered | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Symlink pointing outside workspace | Skipped | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Empty directory | Empty list | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Non-UTF-8 file | Skipped with warning | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Read existing file | Returns content | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Write + read back | Content matches | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Scan with ignored patterns | Ignored files excluded | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| node_modules/.bin/eslint exists | Command resolved | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Binary in system PATH | Available = true | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Config file present | Detected = true | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Cargo.toml in ancestor | Found | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Start from crates/some-crate/src | Finds workspace root | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Path with Cargo.toml (no workspace) | is_member = true | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Path with Cargo.toml nearby | language = Rust | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |
| Leaf member detection | No sub-members | Automated | `tests/filesystem/` | cargo test -p filesystem | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p filesystem --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 32 scenarios mapped; all Automated via `cargo test -p filesystem` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
