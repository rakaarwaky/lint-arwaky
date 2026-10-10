# Feature Backlog: Supervisor Workflow

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-10

## Current Condition

- Done: `cargo nextest run -p supervisor-workflow-lint-arwaky --lib --tests` → 24 passed, 0 failures
- Verified live: `rakaarwaky/lint-arwaky` has 0 open GitHub issues as of 2026-10-10
  (checked via the live REST API + search API; the mock's synthetic fallback,
  issue numbers 9001-9005, is the path actually exercised today).
- In Progress: None
- Blocked: None
- Next Action: Wire real GitHub API client behind a feature flag

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| SUP-01 | P0 | Done | On Track | None | — | 2026-10-10 |
| SUP-02 | P0 | Done | On Track | None | — | 2026-10-10 |
| SUP-03 | P0 | Done | On Track | None | — | 2026-10-10 |
| SUP-04 | P0 | Done | On Track | None | — | 2026-10-10 |
| SUP-05 | P1 | In Progress | On Track | SUP-01–SUP-04 | Wire real GitHub API client behind a feature flag | 2026-10-10 |

## Scenario Evidence

| Scenario | Rule / expected | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|---|
| Mock discovery returns 5 issues, `max = 3` → 3 | 3 issues in list order | Automated | `tests/unit_issue_selection_logic.rs` | `unit_select_least_busy_caps_at_max_issues` | 2026-10-10 |
| Least-busy selection caps at `max_issues` | two least-engaged first | Automated | `tests/unit_issue_selection_logic.rs` | `unit_select_least_busy_caps_at_max_issues` | 2026-10-10 |
| Flaky CI recovers on 3rd poll | `Passing`, `merged = true` | Automated | `tests/acceptance_supervisor_cycle.rs` | `acceptance_flaky_ci_recovers_on_third_poll_and_merges` | 2026-10-10 |
| Give up after 3 consecutive failures | `Failing`, `merged = false` | Automated | `tests/acceptance_supervisor_cycle.rs` | `acceptance_give_up_after_three_consecutive_failures` | 2026-10-10 |
| Unique worktree per issue number | 5 distinct branches | Automated | `tests/acceptance_supervisor_cycle.rs` | `acceptance_unique_worktree_per_issue_number` | 2026-10-10 |
| Root container full cycle, all merged | 5 outcomes, all merged | Automated | `tests/integration_supervisor_cycle.rs` | `full_cycle_via_root_container_reports_five_unique_branches_all_merged` | 2026-10-10 |
| Flaky-CI cycle produces a complete, serializable evidence report | 5 outcomes, each `Passing`/`merged`, JSON-serializable | Automated | `tests/e2e_supervisor_cycle.rs` | `e2e_flaky_ci_recovered_report_is_complete_and_serializable` | 2026-10-10 |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo nextest run -p supervisor-workflow-lint-arwaky --lib --tests` → 24 passed, 0 failures |
| Scenario evidence | Done | 7 scenarios mapped; all Automated via `cargo nextest run -p supervisor-workflow-lint-arwaky` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change |
|---|---|
| 2026-10-10 | Rebased mock issue data to clearly synthetic 9001-9005 identifiers (verified live repo has 0 open issues; no collision risk with real PR/issue numbers) |
| 2026-10-10 | Added `e2e_flaky_ci_recovered_report_is_complete_and_serializable` durable evidence test capturing the full `SupervisorCycleReport` through the real root-container entry point |
| 2026-10-10 | Implemented initial mock-based supervisor workflow crate, all 5 pipeline steps mocked and unit+integration tested |
