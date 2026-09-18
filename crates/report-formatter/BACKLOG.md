# Feature Backlog: Report Formatter

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p report_formatter --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p report_formatter --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| REPO-01 | FR-001 | SCEN-001 - Text Format — 4 scenarios verified | P0 | Done | `cargo test -p report_formatter --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| REPO-02 | FR-002 | SCEN-002 - JSON Format — 4 scenarios verified | P0 | Done | `cargo test -p report_formatter --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| REPO-03 | FR-003 | SCEN-003 - SARIF Format — 7 scenarios verified | P0 | Done | `cargo test -p report_formatter --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| REPO-04 | FR-004 | SCEN-004 - JUnit Format — 6 scenarios verified | P0 | Done | `cargo test -p report_formatter --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| REPO-05 | FR-005 | SCEN-005 –FR-007 - Orchestrator, Fallback, XML Escape — 8 scenarios verified | P0 | Done | `cargo test -p report_formatter --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Report with AES violations | Human-readable output with severity badge… | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Report with external lint results | External section with tool-native … | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Report with PARSE_WARN diagnostics | Warnings section, visually distin… | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Empty report | "0 violations" clean report | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Normal report | Valid pretty-printed JSON | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Empty results | Valid JSON with empty arrays, zero summary | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Report with external results | `external_results` array populated | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Report with PARSE_WARN | `diagnostics` array populated | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Normal report | Valid SARIF 2.1.0 with tool metadata | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| CRITICAL/HIGH severity | SARIF level "error" | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| MEDIUM severity | SARIF level "warning" | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| LOW/INFO severity | SARIF level "note" | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| PARSE_WARN diagnostic | SARIF level "note" | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Line number 0 | Clamped to 1 | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Empty results | Valid SARIF with empty results array | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Normal violations | `<failure>` elements present | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| INFO severity violations | Clean `<testcase>` without `<failure>` | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| PARSE_WARN diagnostics | `<testcase>` with `<skipped>` | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Special characters in message | Properly XML-escaped | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Test/failure counts | Match actual results | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Empty results | Valid XML with 0 tests, 0 failures | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Orchestrator routes Text | Text formatter invoked | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Orchestrator routes JSON | JSON formatter invoked | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Orchestrator routes SARIF | SARIF formatter invoked | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Orchestrator routes JUnit | JUnit formatter invoked | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Default fallback with violations | Counts by code, sorted descending | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| Default fallback empty | "Violations: 0" | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| XML escape all 5 characters | All escaped correctly | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |
| XML escape normal text | Unchanged | Automated | `tests/report-formatter/` | cargo test -p report_formatter | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p report_formatter --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 29 scenarios mapped; all Automated via `cargo test -p report_formatter` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
