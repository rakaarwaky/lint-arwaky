# Feature Backlog: CLI Commands

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p cli_commands --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| CLIC-01 | FR-001 | SCEN-001 - Check/Scan — 5 scenarios verified | P0 | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CLIC-02 | FR-002 | SCEN-002 - CI — 4 scenarios verified | P0 | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CLIC-03 | FR-003 | SCEN-003 - Fix — 5 scenarios verified | P0 | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CLIC-04 | FR-004 | SCEN-004 - Doctor — 3 scenarios verified | P0 | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CLIC-05 | FR-005 | SCEN-005 - Security — 3 scenarios verified | P0 | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CLIC-06 | FR-006 | SCEN-006 - Dependencies — 3 scenarios verified | P0 | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CLIC-07 | FR-007 | SCEN-007 –FR-011 - Setup & Config — 6 scenarios verified | P0 | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| CLIC-08 | FR-008 | SCEN-008 –FR-014 - Git, Watch, Individual — 4 scenarios verified | P0 | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| `check` / `scan` run full pipeline | Correct exit 0/1/2 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Non-existent path | Exit 2 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Workspace member discovery + `--member` | Correct member targeted | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| No workspace members | Falls back to single-scan | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Pipeline fails for one workspace | Warning logged, others continue | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Score ≥ threshold, no CRITICAL | Exit 0 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Score ≥ threshold, CRITICAL present | Exit 1 (auto-fail) | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Score < threshold | Exit 1 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Score exactly at threshold | Exit 0 (passes) | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `fix` applies remove/replace/rename | Reports fixed count | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `fix --dry-run` | Preview only, no changes | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| No violations before fix | Reports 0 fixed | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| All violations fixed | "all violations resolved" | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Violations remain after fix | Exit 1 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| All tools installed | All OK, exit 0 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Some tools missing | MISSING listed, still exit 0 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Doctor internal failure | Exit 2 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Tool not installed | Exit 3 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| No vulnerabilities | Exit 0 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Vulnerabilities found | Exit 1, findings listed | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Normal project | Lists up to 30 deps | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| > 30 dependencies | Truncated with "... and N more" | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| No dependency file | Error, exit 2 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `init` creates config for detected languages | Config files created | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `install` partial failure | Exit 1 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `mcp-config` correct JSON per client | Valid JSON output | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `config-show` redacts secrets | AWS keys / base64 redacted | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `config-show` no config found | "Run lint-arwaky init" message | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `adapters` lists enabled adapters | Bullet list or "(none enabled)" | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `git-diff` analyzes only changed files | Correct subset scanned | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `watch` monitors and re-scans | Re-scan on file change | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| `watch` handler setup fails | Exit 2 | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |
| Individual linters (quality/import/naming/role/orphan/external) | Corr… | Automated | `tests/cli-commands/` | cargo test -p cli_commands | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p cli_commands --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 33 scenarios mapped; all Automated via `cargo test -p cli_commands` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
