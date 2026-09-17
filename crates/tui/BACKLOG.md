# Feature Backlog: TUI

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p tui --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p tui --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| TUI-01 | FR-001 | SCEN-001 Input Translation — 12 scenarios verified | P0 | Done | `cargo test -p tui --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| TUI-02 | FR-002 | SCEN-002 File Navigation — 4 scenarios verified | P0 | Done | `cargo test -p tui --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| TUI-03 | FR-003 | SCEN-003 Lint Actions — 10 scenarios verified | P0 | Done | `cargo test -p tui --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| TUI-04 | FR-004 | SCEN-004 Domain Aggregate Facade — 8 scenarios verified | P0 | Done | `cargo test -p tui --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Press 'j' in normal mode | TuiEvent::MoveDown | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 's' in normal mode | TuiEvent::ActionScan | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 's' with Ctrl | TuiEvent::ActionSecurity | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Type 'a' in search mode | TuiEvent::SearchInput('a') | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Type 'a' in path dialog | TuiEvent::PathInput('a') | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Unknown key | TuiEvent::None | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'D' (uppercase) in normal mode | TuiEvent::ActionDuplicates | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'd' (lowercase) in normal mode | TuiEvent::ActionDoctor | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'p' with Ctrl | TuiEvent::ActionDependencies | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'y' with Ctrl | TuiEvent::CopyToFile | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press '?' in normal mode | TuiEvent::ToggleHelp | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press '/' in normal mode | TuiEvent::ToggleSearch | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Navigate into directory | Entries loaded, selection reset | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Navigate into file | Preview loaded (up to 100 lines) | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Navigate back from root | No-op (clamped) | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Enter empty directory | "Empty or inaccessible" status | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'c' (check) on file | Code analysis results in preview | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 's' (scan) on directory | Background scan with progress updates | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'f' (fix) with dry-run | Dry-run preview output | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'w' (watch) | "Watch mode not supported in TUI" message | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'D' (duplicates) on directory | No-op (not yet delegated) | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'o' (orphan) on directory | Orphan file detection results in pre… | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 't' (ci) on directory | CI threshold validation results | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'd' (doctor) globally | Toolchain diagnostics output | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Scan in progress, press 'c' | Action blocked, no output change | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| Press 'p' (dependencies) on file | Dependency report or CLI fallback m… | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| check() with code_analysis aggregate | LintExecutionResult with violat… | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| scan() delegates to collect_scan | LintExecutionResult with full scan … | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| fix() without fix_orchestrator | CLI fallback message in output | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| fix() with fix_orchestrator | Fix result with mode prefix | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| security() with maintenance aggregate | Security scan result in output | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| dependencies() with maintenance aggregate | Dependency report in outpu… | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| doctor() without maintenance | CLI fallback message | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |
| version() always returns success | Version string in output | Automated | `tests/tui/` | cargo test -p tui | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p tui --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 34 scenarios mapped; all Automated via `cargo test -p tui` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
