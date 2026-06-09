# 📄 Feature Requirements Document (FRD)
**Feature Name:** Cancel Lint Job (`cancel <job_id>`)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.0

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 09/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the cancel lint job CLI command `cancel <job_id>`. It allows users to cancel a running lint job (check, scan, or fix) by its job ID. Supports graceful cancellation with cleanup of running subprocesses (external linter adapters).

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli cancel <job_id>` — cancel running job by ID
- Job listing (list active jobs with IDs)
- Graceful subprocess termination (clippy, ruff, eslint, tsc)
- `lint-arwaky-cli cancel --all` — cancel all running jobs
- Job state tracking: running, cancelling, cancelled, completed

**Out-of-Scope:**
- Job scheduling or queueing
- Persistent job history across sessions
- Remote job cancellation over MCP

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Job ID** | UUID assigned to each lint operation at start |
| **JobManager** | In-memory registry of active jobs with state |
| **JobState** | Enum: Running, Cancelling, Cancelled, Completed, Failed |
| **JobHandle** | Handle to cancel a running job and its subprocesses |
| **Subprocess** | External tool process spawned by an adapter (e.g., clippy) |

## 3. Feature Overview
### 3.1 Background & Problem
Long-running lint operations (especially `scan` with clippy on large projects) could not be interrupted gracefully. Pressing Ctrl+C would kill the process but leave subprocesses (clippy, ruff) orphaned and config files in an inconsistent state. There was no way to list active jobs or cancel a specific one.

### 3.2 Business Goals
- Allow users to cancel long-running lint jobs
- Ensure clean termination of all subprocesses
- Prevent orphaned external tool processes
- Provide visibility into running jobs

### 3.3 Target Users
- **Developers**: Cancel a long-running `scan` that was started by accident
- **CI/CD Pipelines**: Cancel stale jobs programmatically
- **AI Agents**: Cancel operations that are no longer needed

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to cancel a long-running `scan` by its job ID, so I don't have to wait for it to finish.
- **US-002:** As a developer, I want to list all running jobs with their IDs, so I know which job to cancel.
- **US-003:** As a DevOps engineer, I want cancelled jobs to clean up all subprocesses, so no orphan linters are left running.

### 4.2 Use Cases & Workflow
**Job Lifecycle:**
```
Job Created (UUID) → Running → (cancel requested) → Cancelling → Cancelled
                    → (completes) → Completed
                    → (error) → Failed
```

**Cancel Pipeline:**
```
lint-arwaky-cli cancel <job_id>
  │
  ├─► 1. Lookup job in JobManager
  │     ├── Found → continue
  │     └── Not found → error: "No job with ID <job_id>"
  │
  ├─► 2. Set job state → Cancelling
  │
  ├─► 3. Terminate subprocesses:
  │     ├── Send SIGTERM to all child processes
  │     ├── Wait 5s for graceful shutdown
  │     └── Send SIGKILL if still running
  │
  ├─► 4. Clean up temporary files / partial output
  │
  └─► 5. Set job state → Cancelled
        └── Report: "Job <job_id> cancelled"
```

**List Active Jobs:**
```
lint-arwaky-cli cancel --list
  │
  └─► Display table:
        │ Job ID                              │ Type  │ Status   │ Elapsed │
        ├─────────────────────────────────────┼───────┼──────────┼─────────┤
        │ 550e8400-e29b-41d4-a716-446655440000│ scan  │ Running  │ 12.3s   │
        │ 550e8400-e29b-41d4-a716-446655440001│ check │ Running  │ 3.1s    │
```

### 4.3 Business Rules
- Job IDs are UUID v4 generated at job creation
- Jobs are stored in an in-memory registry (lost on process exit)
- `cancel --all` cancels every job in the registry
- Subprocess termination: SIGTERM first, then SIGKILL after 5s timeout
- Cancelled jobs return partial results if available (up to cancellation point)
- Job listing shows only active (Running, Cancelling) jobs by default

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Job cancellation latency (signal sent) | < 100ms |
| NFR-002 | Subprocess cleanup reliability | 100% — no orphans |
| NFR-003 | Job registry lookup | < 10ms |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli cancel 550e8400-e29b-41d4-a716-446655440000
🛑 Cancelling job 550e8400-e29b-41d4-a716-446655440000
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Job:     scan /project (started 10:32:15)
Status:  Cancelling...
  ⏹  clippy subprocess terminated (SIGTERM)
  ⏹  ruff subprocess terminated (SIGTERM)
  ✓ Partial results saved to /tmp/lint-arwaky-partial.json

✅ Job 550e8400-... cancelled (12.3s elapsed)
```

List active jobs:
```
$ lint-arwaky-cli cancel --list
📋 Active Jobs
  │ Job ID                              │ Type  │ Status   │ Elapsed │ File              │
  ├─────────────────────────────────────┼───────┼──────────┼─────────┼───────────────────┤
  │ 550e8400-e29b-41d4-a716-446655440000│ scan  │ Running  │ 12.3s   │ /project          │
  │ 550e8400-e29b-41d4-a716-446655440001│ check │ Running  │ 3.1s    │ /other-project    │

  Use: lint-arwaky-cli cancel <job_id> to cancel a job
```

Cancel all:
```
$ lint-arwaky-cli cancel --all
🛑 Cancelling all jobs (2)
  ✓ Job 550e8400-... cancelled
  ✓ Job 550e8400-... cancelled
✅ All jobs cancelled
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Job running with clippy subprocess | `cancel <job_id>` runs | Job cancelled, clippy terminated | Pending Review |
| AC-002 | Invalid job ID | `cancel <bad_id>` runs | Error: "No job with ID <bad_id>" | Pending Review |
| AC-003 | 2 jobs running | `cancel --list` runs | Both jobs displayed with IDs | Pending Review |
| AC-004 | 2 jobs running | `cancel --all` runs | Both jobs cancelled | Pending Review |
| AC-005 | Job already completed | `cancel <job_id>` runs | Error: "Job already completed" | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI cancel command | `cli-commands/surface_cancel_command.rs` | — | **FULLY IMPLEMENTED** — cancel + list + cancel-all |
| Job manager | `pipeline-jobs/agent_job_manager.rs` | — | **FULLY IMPLEMENTED** — in-memory registry |
| Job handle | `pipeline-jobs/taxonomy_job_handle.rs` | — | **FULLY IMPLEMENTED** — cancellation token + subprocess list |
| Subprocess manager | `pipeline-jobs/infrastructure_subprocess_manager.rs` | — | **FULLY IMPLEMENTED** — SIGTERM/SIGKILL |
| Job state VO | `pipeline-jobs/taxonomy_job_state.rs` | — | **FULLY IMPLEMENTED** |

### 8.2 Bugs Found

1. **Subprocess manager does not track child processes of subprocesses** — clippy may spawn its own child processes (e.g., rustc invocations) that are not in the tracked PID list
   - **Impact**: Orphan rustc processes left running after clippy is cancelled
   - **Fix**: Use process group (PGID) termination instead of per-PID

2. **Job registry is not thread-safe** — `HashMap` in `JobManager` uses no synchronization
   - **Impact**: Race condition if cancel and job completion happen simultaneously
   - **Fix**: Wrap registry in `Arc<RwLock<HashMap>>`

3. **cancel --list shows no output when no jobs active** — empty output with no message
   - **Impact**: User may think the command failed
   - **Fix**: Print "No active jobs" message

4. **Partial results not saved** — when a job is cancelled, partial results are discarded
   - **Impact**: Work done before cancellation is lost
   - **Fix**: Save intermediate results on cancellation signal

### 8.3 What Needs to Be Added

- **Process group termination**: Cancel entire process groups, not individual PIDs
- **Thread safety**: Wrap job registry in `Arc<RwLock<>>`
- **Partial results**: Save results up to cancellation point
- **"No active jobs" message**: User-friendly empty state

### 8.4 What to Keep

- **cancel + cancel --list + cancel --all** ✅ — complete command set
- **UUID job IDs** ✅ — unique, unguessable
- **Graceful subprocess termination** ✅ — SIGTERM → SIGKILL pattern
- **Job state tracking** ✅ — Running → Cancelling → Cancelled

### 8.5 Empirical Evidence from Test Projects

- `cancel <uuid>` terminates a running `scan` with clippy subprocess
- `cancel --list` correctly shows active jobs
- Pending Review: Process group termination, thread safety

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Check) | Job pipeline must support cancellation | No cancellation signal in pipeline | Add CancellationToken to all pipeline steps |
| FR-056 (Scan) | Adapters must support termination | Adapter ignores SIGTERM | SIGKILL fallback after 5s |
| Operating system signals | SIGTERM/SIGKILL on Unix | Windows compatibility | Conditional compilation for Windows |

## 10. Appendices
- `src-rust/cli-commands/surface_cancel_command.rs` — CLI cancel command
- `src-rust/pipeline-jobs/agent_job_manager.rs` — In-memory job registry
- `src-rust/pipeline-jobs/infrastructure_subprocess_manager.rs` — Subprocess lifecycle
- `src-rust/pipeline-jobs/taxonomy_job_handle.rs` — Job handle with cancellation
- `src-rust/pipeline-jobs/taxonomy_job_state.rs` — Job state enum
