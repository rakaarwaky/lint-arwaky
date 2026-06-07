# FRD — Track Quality Trends Over Time

> **PRD Reference**: [FR-006](PRD.md) — Track quality trends over time
> **Dependency**: FR-004 (Self-lint)
> **Status**: ⚠️ PARTIAL — Score computation + history check implemented; persistence + analysis are stubs

## 1. Problem Statement

Before quality trends:

| Issue | Description |
|-------|-------------|
| **No historical data** | Each lint run ephemeral |
| **No trend analysis** | No comparison current vs previous |
| **No persistence** | Scores never saved to disk |
| **No CI memory** | No regression detection across builds |

## 2. Solution Overview

Persist lint scores as JSON-lines, compare current vs previous, report trend direction.

| Layer | Role |
|-------|------|
| **Taxonomy** | `LintScoreVO`, `ArchitectureGovernanceEntity` |
| **Contract** | `IMetricsProviderPort`, `IProjectGovernanceProtocol` |
| **Infrastructure** | `MetricsProvider` (reads/writes `.lint-history.json`), `TrendsAdapter` (stub) |
| **Agent** | `AnalysisOrchestrator::get_trends()` |
| **Surfaces** | `cli_analysis_command.rs`, `cli_main_entry.rs` — `trends` subcommand |

### Target Flow

```
lint-arwaky-cli trends [path]
  └─► AnalysisOrchestrator.get_trends()
        ├─► Run self-lint → current score
        ├─► MetricsProvider.get_history() → previous scores
        ├─► analyze_quality_trend() → IMPROVING / STABLE / DECLINING
        ├─► MetricsProvider.save_score(current)
        └─► Report: score, delta, all-time high/low
```

## 3. Key Files

| File | Layer | Role |
|------|-------|------|
| `taxonomy/lint_score_vo.rs` | Taxonomy | Score value |
| `taxonomy/lint_severity_vo.rs` | Taxonomy | `score_impact()` |
| `taxonomy/architecture_governance_entity.rs` | Taxonomy | Score + compliance |
| `contract/metrics_provider_port.rs` | Contract | History I/O |
| `contract/project_governance_protocol.rs` | Contract | Trend analysis |
| `infrastructure/python_metrics_adapter.rs` | Infrastructure | `MetricsProvider` |
| `infrastructure/python_analysis_adapter.rs` | Infrastructure | `TrendsAdapter` |
| `agent/analysis_execution_orchestrator.rs` | Agent | `get_trends()` |
| `surfaces/cli_analysis_command.rs` | Surfaces | CLI `trends` |
| `surfaces/cli_main_entry.rs` | Surfaces | `handle_trends()` |

### History File

`.lint-history.json` (JSON-lines):

```json
{"score":92.0,"timestamp":"2026-06-01T10:00:00Z","violations":28,"critical":0}
{"score":88.5,"timestamp":"2026-06-02T14:30:00Z","violations":35,"critical":1}
```

## 4. Acceptance Criteria

| # | Kriteria | Status |
|---|----------|--------|
| AC001 | `trends` shows current score | ✅ Working |
| AC002 | Score from severity (100 - deductions) | ✅ Working |
| AC003 | History saved to `.lint-history.json` | ⚠️ Stub |
| AC004 | Trend direction: IMPROVING/STABLE/DECLINING | ❌ Missing |
| AC005 | All-time high/low tracked | ❌ Missing |
| AC006 | Delta (current - previous) displayed | ❌ Missing |
| AC007 | Auto-create history file on first run | ❌ Missing |
| AC008 | `MetricsProvider.get_history()` reads history | ✅ Working |
| AC009 | `cargo check --bin lint-arwaky-cli` passes | ✅ |
| AC010 | `cargo test` passes | ✅ |
