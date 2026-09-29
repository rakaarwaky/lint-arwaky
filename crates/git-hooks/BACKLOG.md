# Feature Backlog: Git Hooks

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo test -p git_hooks --lib --tests` → 0 failures at `HEAD` (post-refactor).
  Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p git_hooks --lib --tests` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| GITH-01 | P0 | Done | On Track | None | — | 2026-09-29  |
| GITH-02 | P0 | Done | On Track | None | — | 2026-09-29  |
| GITH-03 | P0 | Done | On Track | None | — | 2026-09-29  |
| GITH-04 | P0 | Done | On Track | None | — | 2026-09-29  |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Default branch from `origin/HEAD` | Correct branch detected | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| `symbolic-ref` fails | Defaults to "main" | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Changed files via `origin/main...HEAD` | Correct file list | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| All branch variants empty | Fallback to `HEAD` diff | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| All diff strategies fail | Fallback to `ls-files` | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Lintable filter: .rs, .py, .ts, .js, .jsx, .tsx | Included | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Non-lintable: .md, .toml, .json, .png, .lock | Excluded | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Empty diff | total_changed: 0 | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Detached HEAD | Fallback strategies handle | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Renamed files classified via `--diff-filter=R` | Old/new paths parsed | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Changed files with violations | Lint results returned | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| No changed files | Empty result list | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Changed file with parse failure | Skipped by linters, no warning | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| All changed files non-lintable | Empty result list | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Normal install | Hook script created with correct executable | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| `.git/hooks/` missing | Directory created | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Hook file already exists | Overwritten | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Not a git repo | SuccessStatus(false), no error | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Unix permissions | 0o755 set | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Windows | Permission setting skipped | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Empty executable path | Defaults to "lint-arwaky-cli" | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Hook exists | Removed, SuccessStatus(true) | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Hook doesn't exist | SuccessStatus(true), idempotent | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Not a git repo | SuccessStatus(false) | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Config not present | Default config created, success | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Config already exists | Idempotent, "ALREADY_EXISTS" status | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Add ignore rule | Rule added to config | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Remove ignore rule | Rule removed from config | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Config file not found | Error suggesting `lint-arwaky-cli init` | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Rule already exists (add) | No-op, "already present" | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |
| Write failure | Error description returned | Automated | `tests/git-hooks/` | cargo test -p git_hooks | `HEAD` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p git_hooks --lib --tests` → 0 failures |
| Scenario evidence | Done | 31 scenarios mapped; all Automated via `cargo test -p git_hooks` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Restructured FRD: 7 utility/mixed FRs → 4 business-capability FRs. Removed FR-005 (Diff Data Comparison), FR-006 (Ignore Rule Management), FR-007 (Config Initialization). Merged FR-001 + FR-004 → single IDiffDetectionProtocol seam. Merged FR-006 + FR-007 → single IConfigInitProtocol seam. Removed IDiffDataProtocol, IIgnoreRuleProtocol, IHookCheckProtocol from shared contract. | @raka |
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
