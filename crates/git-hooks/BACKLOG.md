# Feature Backlog: Git Hooks

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo test -p git-hooks-lint-arwaky --lib --tests` → 0 failures at `2310bc50` (2026-09-29, post-refactor).
  Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p git-hooks-lint-arwaky --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| GITH-01 | FR-GitHooks-001 | SCEN-001 - Git Diff Detection — 14 scenarios verified | P0 | Done | `cargo test -p git-hooks-lint-arwaky --lib --tests` → 0 failures at `2310bc50` (2026-09-29) | @raka | None | 2026-09-29 |
| GITH-02 | FR-GitHooks-002 | SCEN-002 - Hook Installation — 7 scenarios verified | P0 | Done | `cargo test -p git-hooks-lint-arwaky --lib --tests` → 0 failures at `2310bc50` (2026-09-29) | @raka | None | 2026-09-29 |
| GITH-03 | FR-GitHooks-003 | SCEN-003 - Hook Uninstallation — 3 scenarios verified | P0 | Done | `cargo test -p git-hooks-lint-arwaky --lib --tests` → 0 failures at `2310bc50` (2026-09-29) | @raka | None | 2026-09-29 |
| GITH-04 | FR-GitHooks-004 | SCEN-004 - Project Config Initialization — 7 scenarios verified | P0 | Done | `cargo test -p git-hooks-lint-arwaky --lib --tests` → 0 failures at `2310bc50` (2026-09-29) | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Default branch from `origin/HEAD` | Correct branch detected | Automated | `tests/unit_git_hooks_diff_checker.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| `symbolic-ref` fails | Defaults to "main" | Automated | `tests/unit_git_hooks_diff_checker.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Changed files via `origin/main...HEAD` | Correct file list | Automated | `tests/unit_git_hooks_diff_checker.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| All branch variants empty | Fallback to `HEAD` diff | Automated | `tests/unit_git_hooks_diff_checker.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| All diff strategies fail | Fallback to `ls-files` | Automated | `tests/unit_git_hooks_diff_checker.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Lintable filter: .rs, .py, .ts, .js, .jsx, .tsx | Included | Automated | `tests/unit_git_hooks_diff_checker.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Non-lintable: .md, .toml, .json, .png, .lock | Excluded | Automated | `tests/unit_git_hooks_diff_checker.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Empty diff | total_changed: 0 | Automated | `tests/unit_git_hooks_diff_checker.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Detached HEAD | Fallback strategies handle | Automated | `tests/unit_git_hooks_diff_checker.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Renamed files classified via `--diff-filter=R` | Old/new paths parsed | Automated | `tests/unit_git_hooks_diff_checker.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Changed files with violations | Lint results returned | Automated | `tests/acceptance_git_hooks.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| No changed files | Empty result list | Automated | `tests/acceptance_git_hooks.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Changed file with parse failure | Skipped by linters, no warning | Automated | `tests/acceptance_git_hooks.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| All changed files non-lintable | Empty result list | Automated | `tests/acceptance_git_hooks.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Normal install | Hook script created with correct executable | Automated | `tests/unit_git_hooks_hook_adapter.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| `.git/hooks/` missing | Directory created | Automated | `tests/unit_git_hooks_hook_adapter.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Hook file already exists | Overwritten | Automated | `tests/unit_git_hooks_hook_adapter.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Not a git repo | SuccessStatus(false), no error | Automated | `tests/unit_git_hooks_hook_adapter.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Unix permissions | 0o755 set | Automated | `tests/unit_git_hooks_hook_adapter.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Windows | Permission setting skipped | Automated | `tests/unit_git_hooks_hook_adapter.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Empty executable path | Defaults to "lint-arwaky-cli" | Automated | `tests/unit_git_hooks_hook_adapter.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Hook exists | Removed, SuccessStatus(true) | Automated | `tests/unit_git_hooks_hook_adapter.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Hook doesn't exist | SuccessStatus(true), idempotent | Automated | `tests/unit_git_hooks_hook_adapter.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Not a git repo | SuccessStatus(false) | Automated | `tests/unit_git_hooks_hook_adapter.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Config not present | Default config created, success | Automated | `tests/unit_git_hooks_hook_manager.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Config already exists | Idempotent, "ALREADY_EXISTS" status | Automated | `tests/unit_git_hooks_hook_manager.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Add ignore rule | Rule added to config | Automated | `tests/unit_git_hooks_hook_manager.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Remove ignore rule | Rule removed from config | Automated | `tests/unit_git_hooks_hook_manager.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Config file not found | Error suggesting `lint-arwaky-cli init` | Automated | `tests/unit_git_hooks_hook_manager.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Rule already exists (add) | No-op, "already present" | Automated | `tests/unit_git_hooks_hook_manager.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |
| Write failure | Error description returned | Automated | `tests/acceptance_git_hooks.rs` | cargo test -p git-hooks-lint-arwaky | `2310bc50` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p git-hooks-lint-arwaky --lib --tests` → 0 failures at `2310bc50` (2026-09-29) |
| Scenario evidence | Done | 31 scenarios mapped; all Automated via `cargo test -p git-hooks-lint-arwaky` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Restructured FRD: 7 utility/mixed FRs → 4 business-capability FRs. Removed FR-005 (Diff Data Comparison), FR-006 (Ignore Rule Management), FR-007 (Config Initialization). Merged FR-001 + FR-004 → single IDiffDetectionProtocol seam. Merged FR-006 + FR-007 → single IConfigInitProtocol seam. Removed IDiffDataProtocol, IIgnoreRuleProtocol, IHookCheckProtocol from shared contract. | @raka |
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
