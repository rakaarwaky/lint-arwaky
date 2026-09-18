# Feature Backlog: Git Hooks

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p git_hooks --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p git_hooks --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| GITH-01 | FR-001 | SCEN-001 - Git Diff Detection — 10 scenarios verified | P0 | Done | `cargo test -p git_hooks --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| GITH-02 | FR-002 | SCEN-002 - Hook Installation — 7 scenarios verified | P0 | Done | `cargo test -p git_hooks --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| GITH-03 | FR-003 | SCEN-003 - Hook Uninstallation — 3 scenarios verified | P0 | Done | `cargo test -p git_hooks --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| GITH-04 | FR-004 | SCEN-004 - Check Execution — 4 scenarios verified | P0 | Done | `cargo test -p git_hooks --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| GITH-05 | FR-005 | SCEN-005 - Diff Data Comparison — 7 scenarios verified | P0 | Done | `cargo test -p git_hooks --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| GITH-06 | FR-006 | SCEN-006 - Ignore Rule Management — 4 scenarios verified | P0 | Done | `cargo test -p git_hooks --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| GITH-07 | FR-007 | SCEN-007 - Config Initialization — 3 scenarios verified | P0 | Done | `cargo test -p git_hooks --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Default branch from`origin/HEAD` | Correct branch detected | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| `symbolic-ref` fails | Defaults to "main" | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Changed files via`origin/main...HEAD` | Correct file list | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| All branch variants empty | Fallback to`HEAD` diff | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| All diff strategies fail | Fallback to`ls-files` | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Lintable filter: .rs, .py, .ts, .js, .jsx, .tsx | Included | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Non-lintable: .md, .toml, .json, .png, .lock | Excluded | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Empty diff | total_changed: 0 | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Detached HEAD | Fallback strategies handle | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Renamed files classified via `--diff-filter=R` | Old/new paths parsed | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Normal install | Hook script created with correct executable | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| `.git/hooks/` missing | Directory created | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Hook file already exists | Overwritten | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Not a git repo | SuccessStatus(false), no error | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Unix permissions | 0o755 set | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Windows | Permission setting skipped | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Empty executable path | Defaults to "lint-arwaky-cli" | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Hook exists | Removed, SuccessStatus(true) | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Hook doesn't exist | SuccessStatus(true), idempotent | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Not a git repo | SuccessStatus(false) | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Changed files with violations | Lint results returned | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| No changed files | Empty result list | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Changed file with parse failure | Skipped by linters, no warning | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| All changed files non-lintable | Empty result list | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Both files identical | Score 0.0, status Unchanged | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Files partially different | Score between 0.0 and 1.0, Modified | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| First file missing | MissingFirst | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Second file missing | MissingSecond | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Both paths are directories | NotAFile | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Both paths missing | BothMissing | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Same file path twice | Score 0.0, Unchanged | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Add ignore rule | Rule added to config | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Remove ignore rule | Rule removed from config | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Config file not found | Error suggesting`lint-arwaky-cli init` | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Rule already exists (add) | No-op, "already present" | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Config not present | Default config created, success | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Config already exists | Idempotent, "ALREADY_EXISTS" status | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |
| Write failure | Error description returned | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p git_hooks --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 38 scenarios mapped; all Automated via `cargo test -p git_hooks` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
