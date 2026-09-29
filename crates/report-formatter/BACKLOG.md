# Feature Backlog: Report Formatter

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo nextest run --workspace --lib --tests` → 2048 passed, 0 failed at `cc63389a` (2026-09-29). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run --workspace --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| REPO-01 | FR-ReportFormatter-001 | SCEN-001 - Text Format — 4 scenarios verified | P0 | Done | `cargo nextest run -p report-formatter-lint-arwaky` → 0 failures at `cc63389a` (2026-09-29) | @raka | None | 2026-09-29 |
| REPO-02 | FR-ReportFormatter-002 | SCEN-002 - JSON Format — 4 scenarios verified | P0 | Done | `cargo nextest run -p report-formatter-lint-arwaky` → 0 failures at `cc63389a` (2026-09-29) | @raka | None | 2026-09-29 |
| REPO-03 | FR-ReportFormatter-003 | SCEN-003 - SARIF Format — 7 scenarios verified | P0 | Done | `cargo nextest run -p report-formatter-lint-arwaky` → 0 failures at `cc63389a` (2026-09-29) | @raka | None | 2026-09-29 |
| REPO-04 | FR-ReportFormatter-004 | SCEN-004 - JUnit Format — 6 scenarios verified | P0 | Done | `cargo nextest run -p report-formatter-lint-arwaky` → 0 failures at `cc63389a` (2026-09-29) | @raka | None | 2026-09-29 |
| REPO-05 | FR-ReportFormatter-001 through 004 | SCEN-005 - Orchestrator Routing — direct dispatch via specific protocol methods | P0 | Done | `cargo nextest run -p report-formatter-lint-arwaky` → 0 failures at `cc63389a` (2026-09-29) | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Report with AES violations | Human-readable output with severity badges | Automated | `tests/unit_report_formatter_text.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Report with external lint results | External section with tool-native codes | Automated | `tests/unit_report_formatter_text.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Report with PARSE_WARN diagnostics | Warnings section, visually distinct | Automated | `tests/unit_report_formatter_text.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Empty report | "Total violations: 0" clean output | Automated | `tests/unit_report_formatter_text.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Score line included | Compliance score shown when present | Automated | `tests/unit_report_formatter_text.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Normal report | Valid pretty-printed JSON | Automated | `tests/unit_report_formatter_json.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Empty results | Valid JSON with empty arrays, zero summary | Automated | `tests/unit_report_formatter_json.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Report with external results | `external_results` array populated | Automated | `tests/unit_report_formatter_json.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Score embedded | `summary.score` field present | Automated | `tests/unit_report_formatter_json.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Normal report | Valid SARIF 2.1.0 with tool metadata | Automated | `tests/unit_report_formatter_sarif.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| CRITICAL/HIGH severity | SARIF level "error" | Automated | `tests/unit_report_formatter_sarif.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| MEDIUM severity | SARIF level "warning" | Automated | `tests/unit_report_formatter_sarif.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| LOW/INFO severity | SARIF level "note" | Automated | `tests/unit_report_formatter_sarif.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| PARSE_WARN diagnostic | SARIF level "note" | Automated | `tests/unit_report_formatter_sarif.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Line number 0 | Clamped to 1 | Automated | `tests/unit_report_formatter_sarif.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Empty results | Valid SARIF with empty results array | Automated | `tests/unit_report_formatter_sarif.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Normal violations | `<failure>` elements present | Automated | `tests/unit_report_formatter_junit.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| INFO severity violations | Clean `<testcase>` without `<failure>` | Automated | `tests/unit_report_formatter_junit.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| PARSE_WARN diagnostics | `<testcase>` with `<skipped>` | Automated | `tests/unit_report_formatter_junit.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Special characters in message | Properly XML-escaped | Automated | `tests/unit_report_formatter_junit.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Test/failure counts | Match actual results | Automated | `tests/unit_report_formatter_junit.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Empty results | Valid XML with 0 tests, 0 failures | Automated | `tests/unit_report_formatter_junit.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Orchestrator routes Text | `format_text` called on TextFormatter | Automated | `tests/integration_report_formatter.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Orchestrator routes JSON | `format_json` called on JsonFormatter | Automated | `tests/integration_report_formatter.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Orchestrator routes SARIF | `format_sarif` called on SarifFormatter | Automated | `tests/integration_report_formatter.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |
| Orchestrator routes JUnit | `format_junit` called on JunitFormatter | Automated | `tests/integration_report_formatter.rs` | `cargo nextest run -p report-formatter-lint-arwaky` | `cc63389a` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo nextest run -p report-formatter-lint-arwaky` → 82 passed, 0 failures at `cc63389a` (2026-09-29) |
| Self-lint | Done | `lint-arwaky-cli check .` → 0 violations at `cc63389a` |
| Scenario evidence | Done | 25 scenarios mapped; all Automated |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
| 2026-09-29 | Removed FR-005/006/007 (delegation, fallback, xml-escape). 4 FRs = 4 protocols = 4 capabilities. Orchestrator routes directly. | @raka |
