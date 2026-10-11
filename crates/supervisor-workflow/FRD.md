# FRD — supervisor-workflow

---

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)

## System Overview

The supervisor-workflow crate provides a 5-step GitHub issue supervisor
pipeline for delegating work to subagents: discover open issues,
select the least-busy subset, open a worktree per issue, build a
subagent brief and poll CI until green (with bounded retries), and
merge the PR. All external I/O (GitHub API, orca worktree subprocess,
gh CLI) is mocked so the pipeline is deterministic and fully
unit-testable without a live network, a live Orca session, or a live
GitHub install.

- **Architecture & Data Flow**

```text
MockIssueDiscovery ──► SupervisorOrchestrator ──► SupervisorCycleReport
                            │
                            ├─► MockWorktreeManager (deterministic per issue number)
                            └─► MockPrMonitor (seedable CI-status sequence)
```

## Functional Requirements

### FR-SupervisorWorkflow-001: GitHub Issue Discovery

- **Description**: Surface open GitHub issues shaped like
  `github.com/rakaarwaky/lint-arwaky`'s listings, up to a caller
  supplied bound, with no network I/O. The issue batch is ranked by
  total engagement (`comments_count + reactions_total`) so the
  least-busy issues come first.
- **Input**: `max: Count` (an i64-backed count VO; not a Rust primitive, per AES402).
- **Output**: `Vec<GitHubIssueVo>`, capped at `max`, in ascending
  engagement order.
- **Business Rules**:
  - The mock ships exactly 5 hardcoded, realistic-looking issues with
    plausible issue numbers, labels (`bug`, `enhancement`,
    `good first issue`, ...), comment counts, and reaction counts.
  - `max` truncates the ranked list from the head; selection is stable
    so equal-engagement issues keep their input order.
- **Edge Cases**:
  - `max = 0` → empty list, no error.
  - `max > 5` → the full 5-issue list.
- **Error Handling**: The mock never errors; the real
  reqwest-backed implementation will surface a `SupervisorError::Discovery`
  when the GitHub API is unreachable.

---

### FR-SupervisorWorkflow-002: Worktree Management

- **Description**: Open one orca worktree per selected issue and
  record its handle (id, absolute path, branch name), deterministically
  derived from the issue number, so a subagent has a checkout to work in.
- **Input**: `&GitHubIssueVo`.
- **Output**: `WorktreeHandle`.
- **Business Rules**:
  - Branch name is `rakaarwaky/supervisor-issue-<number>`.
  - Path is `/home/raka/orca/workspaces/lint-arwaky/supervisor-issue-<number>`.
  - Id is `lint-arwaky::supervisor-issue-<number>`.
  - No subprocess, no filesystem access — the mock is pure.
- **Edge Cases**:
  - The same issue number always yields the same handle.
  - Distinct issue numbers yield distinct branches and paths.
- **Error Handling**: The mock never errors; the real orca-backed
  implementation will surface a `SupervisorError::Worktree` when a
  worktree create fails.

---

### FR-SupervisorWorkflow-003: PR Monitoring and Merging

- **Description**: Build the subagent's brief text (issue number,
  title, description, worktree path) for each issue, poll PR CI up
  to a bounded retry count until the status settles, and merge the PR
  once CI is green.
- **Input**: `&PrInfo`; the retry budget is read from the shared
  taxonomy constant.
- **Output**: The final `CIStatus` observed, and a boolean `merged`
  flag in the per-issue outcome.
- **Business Rules**:
  - The brief is the exact text handed off to a subagent prompt.
  - CI polling stops early on `Passing`; `Failing` and `Pending`
    each consume a retry; `Unknown` settles immediately as
    not-merged.
  - After the retry budget is exhausted, the final status is
    recorded and the issue is marked not-merged.
  - The merge is attempted only when `pr.state == "open"`.
  - The mock PR monitor cycles through a caller-seeded status sequence
    so tests can simulate flaky CI deterministically.
- **Edge Cases**:
  - A closed PR is rejected by the merge step.
  - The mock PR monitor's seed may be shorter than the retry budget;
    it wraps to the head of the sequence.
- **Error Handling**: A transport error from the PR monitor settles
  the issue as `Unknown` without merging; a merge rejection records
  `merged = false` and a warning.

---

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `discover_issues` | `max: Count` | `Vec<GitHubIssueVo>` | `SupervisorError` | — | FR-SupervisorWorkflow-001. |
| `create_worktree` | `&GitHubIssueVo` | `WorktreeHandle` | `SupervisorError` | — | FR-SupervisorWorkflow-002. |
| `check_ci_status` | `&PrInfo` | `CIStatus` | `SupervisorError` | — | FR-SupervisorWorkflow-003. |
| `merge_pr` | `&PrInfo` | `()` | `SupervisorError` | — | FR-SupervisorWorkflow-003. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `execute` | `SupervisorRequest` | `SupervisorResponse` | — | — | Single composite entry point over the feature. |

## Integration Points

| System | Direction | Purpose | Failure mode |
| --- | --- | --- | --- |
| `shared-supervisor` crate | in | Supply `GitHubIssueVo`, `CIStatus`, `WorktreeHandle`, `PrInfo`, `SubagentBrief`, `SupervisorCycleReport`, `SupervisorIssueOutcome`, `SupervisorError`, the `SUP_VISIBILITY_MAX_CI_RETRIES` constant, and the three `I*Protocol` contracts plus the `ISupervisorAggregate` aggregate | A type or trait is missing at compile time → the build fails before any cycle runs |
| `shared-common` crate | in | Supply the `Count` value object that bounds the issue-discovery call, keeping the contract primitive-free (AES402) | A missing type at compile time → the build fails |
| GitHub REST API | in (real impl) | Surface open issues and poll PR CI | The API is unreachable → `SupervisorError::Discovery` or `SupervisorError::PrMonitor`; the cycle records the failing outcome and continues with the next issue |
| `orca worktree create` | in (real impl) | Open one worktree per selected issue | The worktree create fails → `SupervisorError::Worktree`; the issue's outcome records `final_ci_status = Unknown`, `merged = false` |
| `gh pr merge` | in (real impl) | Merge a PR once CI is green | The PR is not mergeable → `SupervisorError::Merge`; the issue's outcome records `merged = false` |
| Subagent prompt surface | out | Hand off the rendered `SubagentBrief` text as the subagent's task | The subagent does not start → no side effect in this crate; the cycle report still records the issue's outcome |

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| Cycle determinism | Two runs of the same cycle produce byte-identical reports | Run the cycle twice and diff the serialized reports |
| No network in tests | Zero external I/O in the unit/integration/e2e/smoke suite | Assert the suite runs offline |
| No filesystem in tests | Zero filesystem I/O in the mock worktree manager | Assert the mock's `create_worktree` reads and writes nothing |
| CI retry bound | At most 3 consecutive `Failing`/`Pending` polls before the cycle gives up | Time a flaky-CI scenario and assert the poll count |
| Thread safety | All three capability seams are `Send + Sync` | Compile-time assertion in the `contract_supervisor_cycle` test |

## Test Scenarios

- **SCEN-001 — Issue Discovery** — e.g. `max = 3` → 3 issues, in ranked order
- **SCEN-002 — Worktree Determinism** — e.g. same issue number → identical handle
- **SCEN-003 — Flaky CI Recovery** — e.g. seed `[Failing, Failing, Passing]` → merged on the 3rd poll
- **SCEN-004 — CI Give-Up** — e.g. seed `[Failing × 3]` → not merged, `final_ci_status = Failing`
- **SCEN-005 — Full Cycle** — e.g. default container, `max = 5` → 5 outcomes, all merged

- **SCEN-001 — Issue Discovery**

FRD Ref: FR-SupervisorWorkflow-001

| # | Scenario | Expected |
| - | - | - |
| 1 | `max = 3` | 3 issues returned, ranked by engagement |
| 2 | `max = 0` | empty list, no error |
| 3 | `max > 5` | all 5 issues returned |

- **SCEN-002 — Worktree Determinism**

FRD Ref: FR-SupervisorWorkflow-002

| # | Scenario | Expected |
| - | - | - |
| 1 | Issue #42, called twice | identical `WorktreeHandle` |
| 2 | Issue #42 vs #43 | distinct branch names and paths |

- **SCEN-003 — Flaky CI Recovery**

FRD Ref: FR-SupervisorWorkflow-003

| # | Scenario | Expected |
| - | - | - |
| 1 | Seed `[Failing, Failing, Passing]` | `final_ci_status = Passing`, `merged = true` |
| 2 | Seed `[Failing, Failing, Failing]` | `final_ci_status = Failing`, `merged = false` |

- **SCEN-004 — CI Give-Up**

FRD Ref: FR-SupervisorWorkflow-003

| # | Scenario | Expected |
| - | - | - |
| 1 | Seed `[Failing × 3]` | retry budget exhausted, issue not merged |
| 2 | Transport error on first poll | settles as `Unknown`, not merged |

- **SCEN-005 — Full Cycle**

FRD Ref: FR-SupervisorWorkflow-001

| # | Scenario | Expected |
| - | - | - |
| 1 | Default container, `max = 5` | 5 outcomes, all merged, all `Passing` |
| 2 | Default container, `max = 0` | empty report |

## Assumptions & Constraints

- The mock issue list is hardcoded and realistic-looking; it is a
  stand-in for a real `reqwest`-based GitHub API client.
- The mock worktree manager is pure: no subprocess, no filesystem.
  A real implementation will shell out to `orca worktree create`.
- The mock PR monitor cycles through a caller-seeded status sequence
  so tests can simulate flaky CI deterministically. A real
  implementation will poll the GitHub Checks API.
- PR opening is a "record only" mock step: the head branch becomes
  the worktree branch and CI polling starts immediately.
- CI retries: up to 3 consecutive `Failing`/`Pending` polls, read from
  `SUP_VISIBILITY_MAX_CI_RETRIES` in the shared taxonomy; `Passing`
  settles, `Unknown` records and treats the issue as not-merged.
- No async runtime in this crate; all mock I/O is synchronous.
- The real implementation of each capability is a follow-up behind a
  feature flag; see BACKLOG.md.

---

## Glossary

- **AES**: Agentic Engineering System — the 7-layer coding convention
- **Supervisor**: The orchestrator that drives the 5-step pipeline
- **Capability**: A mock implementation of one `I*Protocol` contract
- **Subagent Brief**: The text handed off to a subagent as its task
- **CI Status**: One of `Unknown`, `Pending`, `Passing`, `Failing`
- **Worktree**: A git worktree opened for one issue, modeled by `WorktreeHandle`
- **Retry Budget**: The maximum number of consecutive non-green CI polls before giving up, read from `SUP_VISIBILITY_MAX_CI_RETRIES`
