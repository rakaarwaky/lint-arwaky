# 📄 Feature Requirements Document (FRD)
**Feature Name:** Quality Trends Tracking (`trends` subcommand)
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
This document defines the quality trends tracking feature that records lint scores over time and displays historical comparison. It builds on FR-006 (Quality Trends baseline) and extends it with dedicated CLI command and enhanced trend visualization.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli trends [path]` — show quality trends over time
- Score persistence to `.lint-history.json` (JSON-lines format)
- Trend direction analysis: IMPROVING, STABLE, DECLINING
- All-time high/low tracking with dates
- Delta display (current score vs previous)
- Score history chart (sparkline)

**Out-of-Scope:**
- External dashboard integration
- Real-time trend streaming
- Predictive trend analysis (ML forecasting)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Score** | Quality score 0–100 computed from lint violations (deductions) |
| **Delta** | Difference between current score and previous score |
| **Trend direction** | IMPROVING (delta > +1), STABLE (delta -1..+1), DECLINING (delta < -1) |
| **All-time high** | Maximum score across all history entries |
| **All-time low** | Minimum score across all history entries |
| **Sparkline** | Inline ASCII chart showing score over last N entries |

## 3. Feature Overview
### 3.1 Background & Problem
Lint results were ephemeral — each run computed a score and immediately discarded it. There was no way to see if code quality was improving or declining. CI runs had no memory of previous results, making regression detection impossible.

### 3.2 Business Goals
- Track codebase health trajectory over time
- Detect quality regressions between builds
- Provide visibility into quality trends at a glance
- Enable data-driven refactoring decisions

### 3.3 Target Users
- **Developers**: Monitor their project's quality trajectory
- **Engineering Managers**: Track team productivity improvements
- **CI/CD Pipelines**: Detect regressions between builds

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `lint-arwaky-cli trends .` to see my project's quality score and how it has changed over time.
- **US-002:** As a CI pipeline, I want trend data to persist between builds so I can detect if a PR introduces a regression.
- **US-003:** As a developer, I want a sparkline chart showing the last 10 scores so I can visualize the trend at a glance.

### 4.2 Use Cases & Workflow
**Trends Pipeline:**
```
lint-arwaky-cli trends .
  │
  ├─► 1. Self-lint → compute current score (0–100)
  │
  ├─► 2. Read history from .lint-history.json
  │     ├── If file doesn't exist → create it
  │     └── Parse JSON-lines: [{"score":92,"timestamp":"..."}, ...]
  │
  ├─► 3. Analyze trend
  │     ├── delta = current_score - previous_score
  │     ├── delta > +1   → "IMPROVING"
  │     ├── delta < -1   → "DECLINING"
  │     └── else         → "STABLE"
  │
  ├─► 4. Compute all-time stats
  │     ├── all_time_high = max(scores) with date
  │     └── all_time_low  = min(scores) with date
  │
  ├─► 5. Generate sparkline (last 10 entries)
  │     └── ASCII bar chart: ▁▂▃▄▅▆▇█
  │
  ├─► 6. Save current score
  │     └── Append {"score":85,"timestamp":"2026-06-09T10:00:00Z",...}
  │
  └─► 7. Display report
```

**Example Output:**
```
$ lint-arwaky-cli trends .
📈 Quality Trends — /home/user/project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Current score:  85.0 / 100
  Previous:       87.5 (Jun 7)  |  Delta: -2.5
  Trend:          DECLINING ⚠️
  ─────────────────────────────────────────────────
  All-time high:  92.0 (Jun 1)
  All-time low:   80.5 (May 28)
  ─────────────────────────────────────────────────
  Sparkline (last 10):
  █▇▇▆▆▅▇▆▆▅
  May 30 ──────────────────── Jun 8
  ─────────────────────────────────────────────────
  History: 42 entries over 30 days
```

### 4.3 Business Rules
- Score range: 0–100 (100 = perfect, 0 = all rules violated)
- History stored as JSON-lines (append-only, no DB)
- Trend thresholds: IMPROVING (delta > +1), STABLE (-1..+1), DECLINING (delta < -1)
- History file auto-created on first run at project root
- Max history entries: 365 (configurable)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | History read latency (1000 entries) | < 10ms |
| NFR-002 | History write latency (append) | < 5ms |
| NFR-003 | History file size (10,000 entries) | < 5MB |
| NFR-004 | Score computation latency | < 500ms |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli trends .
📈 Quality Trends — /home/user/project
  Score:  85.0/100  │  Δ -2.5  │  DECLINING ⚠️
  High:  92.0 (Jun 1)  │  Low:  80.5 (May 28)
  █▇▇▆▆▅▇▆▆▅  42 entries
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | `lint-arwaky-cli trends .` runs | Pipeline completes | Current score, delta, and trend direction displayed | Pending Review |
| AC-002 | History file does not exist | First `trends` run | `.lint-history.json` created, entry written | Pending Review |
| AC-003 | Current score = 85, previous = 87.5 | Trend analysis runs | delta = -2.5 → DECLINING shown | Pending Review |
| AC-004 | History has 42 entries | Stats computed | All-time high and low displayed with dates | Pending Review |
| AC-005 | Score is 85.0 | `save_score()` runs | `{"score":85.0,...}` appended to history | Pending Review |

## 8. Empirical Findings (Code Audit)
### 8.1 Current Implementation
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| Trends CLI command | `cli-commands/surface_analysis_command.rs` | 156 | **FULLY IMPLEMENTED** |
| Analysis orchestrator | `code-analysis/agent_analysis_orchestrator.rs` | — | **FULLY IMPLEMENTED** |
| Metrics provider port | `metrics-service/contract_metrics_port.rs` | — | **STUB** — trait only |
| Score computation | `output-report/taxonomy_score_constant.rs` | — | **FULLY IMPLEMENTED** |

### 8.2 Bugs Found
1. **DummyContainer** — `agent_analysis_orchestrator.rs:18-20` creates a no-op container
2. **History path hardcoded** — `.lint-arwaky-trends.json` not configurable
3. **No Rust metrics provider** — only Python adapter exists

### 8.3 What Needs to Be Added
- Replace `DummyContainer` with proper DI wiring
- Configurable history path (`.lint-arwaky/` directory)
- Rust metrics provider implementing `IMetricsProviderPort`
- Sparkline generation utility
- History retention limit (365 entries)

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-006 (Quality Trends) | Baseline trends feature | Overlap in functionality | Extend, don't duplicate |
| FR-004 (Self-Lint) | Trends requires self-lint for score | Self-lint failure = no score | Graceful error, existing history |
| File system | History file I/O | Concurrent writes corrupt file | Append-only, single-process lock |

## 10. Appendices
- `cli-commands/surface_analysis_command.rs` — CLI `trends` command
- `code-analysis/agent_analysis_orchestrator.rs` — `get_trends()` orchestrator
- `metrics-service/contract_metrics_port.rs` — `IMetricsProviderPort` trait
- `output-report/taxonomy_score_constant.rs` — `compute_score()` function
