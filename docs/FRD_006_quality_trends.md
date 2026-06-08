# 📄 Feature Requirements Document (FRD)
**Feature Name:** Track Quality Trends Over Time  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 08/06/2026  
**Version:** v1.0  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |

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
Previous: 87.5 (Jun 7) | Delta: -2.5 — DECLINING ⚠️
All-time high: 92.0 (Jun 1)
All-time low:  80.5 (May 28)
History: 42 entries over 30 days
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | `lint-arwaky-cli trends .` runs | Pipeline completes | Current score displayed | ✅ `handle_trends()` in `cli_main_entry.rs` lints, computes score, saves to history |
| AC-002 | History file does not exist | `trends` runs | Auto-create `.lint-history.json` | ✅ `MetricsProvider.save_metric()` creates parent dirs + appends via `OpenOptions::create(true).append(true)` |
| AC-003 | Current score = 85, previous = 87.5 | Trend analysis runs | delta = -2.5 → DECLINING | ✅ `handle_trends()` computes delta, classifies IMPROVING/DECLINING/STABLE using ±1 thresholds |
| AC-004 | History has 42 entries | Stats computed | All-time high and low shown | ✅ `handle_trends()` computes `all_time_high` / `all_time_low` via `fold(f64::max)` / `fold(f64::min)` |
| AC-005 | Score is 85.0 | `save_score()` runs | `{"score":85.0,...}` appended to history | ✅ `MetricsProvider.save_metric()` appends JSON-lines entry with score, timestamp, violations, critical count |
| AC-006 | `MetricsProvider.get_history()` called | History read | Valid JSON-lines parsed | ✅ `get_history()` parses each line as json, skips empty lines |
| AC-007 | Current score computed | Self-lint completes | Score 0-100 returned | ✅ `compute_score()` in taxonomy mod |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-004 (Self-Lint) | Trends requires self-lint to compute current score | If self-lint fails, no score | Report error, don't crash |
| File system | History file I/O | Concurrent writes corrupt file | Append-only, single-process |
| `analyze_quality_trend()` | Not yet implemented | Trend direction not computed | Implement per spec |

## 9. Appendices
- `src-rust/surfaces/cli_analysis_command.rs` — CLI `trends` command
- `src-rust/surfaces/cli_main_entry.rs` — `handle_trends()` handler
- `src-rust/agent/analysis_execution_orchestrator.rs` — `get_trends()` orchestrator
- `src-rust/infrastructure/python_metrics_adapter.rs` — MetricsProvider
- `src-rust/infrastructure/python_analysis_adapter.rs` — TrendsAdapter
- `src-rust/contract/metrics_provider_port.rs` — IMetricsProviderPort trait
- `src-rust/contract/project_governance_protocol.rs` — analyze_quality_trend() trait
