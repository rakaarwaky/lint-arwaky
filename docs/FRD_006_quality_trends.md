# 📄 Feature Requirements Document (FRD)
**Feature Name:** Track Quality Trends Over Time  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.1  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v1.1 | 09/06/2026 | Raka | Updated file paths to reflect vertical slicing (26 feature folders) | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the quality trends feature that tracks project health over time. It covers score persistence to `.lint-history.json`, trend direction analysis (IMPROVING/STABLE/DECLINING), all-time high/low tracking, and the `trends` CLI command.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli trends [path]` — show quality trends
- Score persistence to `.lint-history.json` (JSON-lines format)
- Trend direction detection: delta comparison
- All-time high/low tracking
- Delta display (current - previous)

**Out-of-Scope:**
- External monitoring or dashboard integration
- Historical data migration
- Real-time trend streaming

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **MetricsProvider** | Infrastructure adapter for reading/writing `.lint-history.json` |
| **TrendsAdapter** | Linter adapter that computes trend data |
| **analyze_quality_trend()** | Protocol method for comparing scores |
| **JSON-lines** | One JSON object per line, append-only format |
| **Trend direction** | IMPROVING (delta > +1), STABLE (delta -1..+1), DECLINING (delta < -1) |
| **All-time high/low** | Max and min score across all history entries |

## 3. Feature Overview
### 3.1 Background & Problem
Each lint run's results were ephemeral — scores were computed and immediately discarded. There was no way to see if code quality was improving or declining over time. CI runs had no memory of previous results, making regression detection impossible.

### 3.2 Business Goals
- Track codebase health over time with persistent history
- Detect quality regressions early
- Provide visibility into quality trajectory
- Enable data-driven decisions about refactoring priorities

### 3.3 Target Users
- **Developers**: Monitor their project's quality trajectory
- **Engineering Managers**: Track team productivity improvements
- **CI/CD Pipelines**: Detect regressions between builds

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `lint-arwaky-cli trends` to see my project's current score and how it has changed over time, so I can track quality improvements.
- **US-002:** As a CI pipeline, I want trend data to persist between builds, so I can detect if a PR introduces a regression.
- **US-003:** As a developer, I want to see all-time high and low scores, so I know the best and worst states of my project.

### 4.2 Use Cases & Workflow
**Trends Pipeline (Target):**
```
lint-arwaky-cli trends .
  │
  ├─► 1. Self-lint → current score
  │
  ├─► 2. Read history file
  │     MetricsProvider.get_history()
  │     → [{"score":92,"timestamp":"2026-06-01"}, ...]
  │
  ├─► 3. Analyze trend
  │     analyze_quality_trend(current, previous)
  │     ├── delta = 85.0 - 87.5 = -2.5
  │     └── delta < -1 → "DECLINING"
  │
  ├─► 4. Compute all-time stats
  │     ├── all-time high = max(scores)
  │     └── all-time low = min(scores)
  │
  ├─► 5. Save current score
  │     MetricsProvider.save_score(current)
  │     → append to .lint-history.json
  │
  └─► 6. Report
        "Current score: 85.0 / 100"
        "Previous: 87.5 (Jun 7) | Delta: -2.5 — DECLINING ⚠️"
        "All-time high: 92.0 (Jun 1) | All-time low: 80.5 (May 28)"
```

**History File Format (`.lint-history.json`):**
```json
{"score":92.0,"timestamp":"2026-06-01T10:00:00Z","violations":28,"critical":0}
{"score":88.5,"timestamp":"2026-06-02T14:30:00Z","violations":35,"critical":1}
{"score":87.5,"timestamp":"2026-06-07T16:00:00Z","violations":42,"critical":0}
```

### 4.3 Business Rules
- History stored as JSON-lines (append-only, no DB needed)
- Trend direction: IMPROVING (delta > +1), STABLE (delta -1..+1), DECLINING (delta < -1)
- History file auto-created on first run
- Score range: 0–100

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | History read latency (1000 entries) | < 10ms |
| NFR-002 | History write latency (append) | < 5ms |
| NFR-003 | History file size (10,000 entries) | < 5MB |
| NFR-004 | No database required | File-based only |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli trends .
Current score: 85.0 / 100
Previous: 87.5 (Jun 7) | Delta: -2.5 — DECLINING Pending Review
All-time high: 92.0 (Jun 1)
All-time low:  80.5 (May 28)
History: 42 entries over 30 days
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | `lint-arwaky-cli trends .` runs | Pipeline completes | Current score displayed | Pending Review `handle_trends()` in `cli_main_entry.rs` lints, computes score, saves to history |
| AC-002 | History file does not exist | `trends` runs | Auto-create `.lint-history.json` | Pending Review `MetricsProvider.save_metric()` creates parent dirs + appends via `OpenOptions::create(true).append(true)` |
| AC-003 | Current score = 85, previous = 87.5 | Trend analysis runs | delta = -2.5 → DECLINING | Pending Review `handle_trends()` computes delta, classifies IMPROVING/DECLINING/STABLE using ±1 thresholds |
| AC-004 | History has 42 entries | Stats computed | All-time high and low shown | Pending Review `handle_trends()` computes `all_time_high` / `all_time_low` via `fold(f64::max)` / `fold(f64::min)` |
| AC-005 | Score is 85.0 | `save_score()` runs | `{"score":85.0,...}` appended to history | Pending Review `MetricsProvider.save_metric()` appends JSON-lines entry with score, timestamp, violations, critical count |
| AC-006 | `MetricsProvider.get_history()` called | History read | Valid JSON-lines parsed | Pending Review `get_history()` parses each line as json, skips empty lines |
| AC-007 | Current score computed | Self-lint completes | Score 0-100 returned | Pending Review `compute_score()` in taxonomy mod |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI trends command | `cli-commands/surface_analysis_command.rs` | 156 | **FULLY IMPLEMENTED** — `trends()`, `complexity()`, `duplicates()`, `ci()`, `dependencies()` |
| Analysis orchestrator | `code-analysis/agent_analysis_orchestrator.rs` | — | **FULLY IMPLEMENTED** — `get_trends()` with history read/delta/trend classification |
| Analysis aggregate trait | `code-analysis/contract_analysis_aggregate.rs` | — | **FULLY IMPLEMENTED** — 5 method signatures |
| Metrics provider port | `metrics-service/contract_metrics_port.rs` | — | **STUB** — trait only, `get_line_count`, `get_history`, `save_metric` |
| Python metrics adapter | `language-adapters/infrastructure_py_metrics.rs` | — | **FULLY IMPLEMENTED** — Python-specific metrics collection |
| Score computation | `output-report/taxonomy_score_constant.rs` | — | **FULLY IMPLEMENTED** — `compute_score()` with severity deductions |

### 8.2 Bugs Found

1. **Analysis orchestrator uses a `DummyContainer`** (`code-analysis/agent_analysis_orchestrator.rs:18-20`)
   ```rust
   struct DummyContainer {}
   impl ServiceContainerAggregate for DummyContainer {}
   ```
   - The orchestrator creates a dummy container with NO methods implemented
   - `container()` always returns a no-op container
   - **Impact**: Any code path that calls `self.container()` receives a non-functional container

2. **Trends history file path hardcoded** — `.lint-arwaky-trends.json` in project root
   - Not configurable, not documented as an output artifact
   - Mixed with project's own files — should be in `.lint-arwaky/` directory

3. **No actual IMetricsProviderPort implementation for Rust**
   - `contract_metrics_port.rs` defines the trait
   - Only `infrastructure_py_metrics.rs` implements it (Python-specific)
   - No Rust metrics provider exists — trends for Rust projects use fallback line counting

### 8.3 What Needs to Be Added

- **Real container**: Replace `DummyContainer` with proper DI container wiring
- **Configurable history path**: Support custom output directory
- **Rust metrics provider**: Implement `IMetricsProviderPort` for Rust code analysis
- **History cleanup**: Add max entries limit or retention policy

### 8.4 What to Keep

- **Trend direction computation** ✅ — delta-based IMPROVING/STABLE/DECLINING classification
- **Score persistence** ✅ — JSON-lines append-only format, auto-creates file
- **All-time high/low tracking** ✅ — computed from full history
- **CLI integration** ✅ — `lint-arwaky-cli trends .` command works end-to-end

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli trends .` runs and produces `.lint-arwaky-trends.json`
- History file format: `{"score":N,"timestamp":"...","violations":N,"critical":N}` per line
- Trend direction correctly classifies IMPROVING/STABLE/DECLINING based on ±1 thresholds
- Pending Review: All acceptance criteria

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-004 (Self-Lint) | Trends requires self-lint to compute current score | If self-lint fails, no score | Report error, don't crash |
| File system | History file I/O | Concurrent writes corrupt file | Append-only, single-process |
| `analyze_quality_trend()` | Not yet implemented | Trend direction not computed | Implement per spec |

## 10. Appendices
- `src-rust/cli-commands/surface_trends_command.rs` — CLI `trends` command
- `src-rust/cli-commands/surface_main_entry.rs` — `handle_trends()` handler
- `src-rust/pipeline-jobs/agent_analysis_orchestrator.rs` — `get_trends()` orchestrator
- `src-rust/metrics-service/infrastructure_metrics_adapter.rs` — MetricsProvider
- `src-rust/metrics-service/infrastructure_analysis_adapter.rs` — TrendsAdapter
- `src-rust/metrics-service/contract_metrics_port.rs` — IMetricsProviderPort trait
- `src-rust/multi-project/contract_governance_protocol.rs` — analyze_quality_trend() trait
