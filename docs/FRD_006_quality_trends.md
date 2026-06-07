# FRD — Track Quality Trends Over Time

> **PRD Reference**: [FR-006](PRD.md) — Track quality trends over time
> **Dependency**: FR-004 (Self-lint)
> **Status**: ✅ **PRODUCTION-READY** — `handle_trends()` fully implemented: self-lint → read history → compare → trend analysis → save to history. MetricsProvider stored in DI container and used via contract. Trend direction (IMPROVING/STABLE/DECLINING), all-time high/low, delta displayed.

## 1. Problem Statement

Before quality trends:

| Issue | Description |
|-------|-------------|
| **No history** | Every lint run's results vanish — can't see progress |
| **No trend analysis** | Can't compare today's score vs yesterday's |
| **No persistence** | Score is never saved to disk |
| **No CI memory** | CI runs don't know the previous build's score |
| **No regression detection** | Can't detect when quality drops |

## 2. Basic Concept

Each time `lint-arwaky-cli trends` is run:
1. **Self-lint** → get current score
2. **Read history** from `.lint-history.json` file
3. **Compare** current vs previous → determine trend
4. **Save** current score to history
5. **Report**: score, delta, all-time high/low, trend direction

```
Score 100 ┤
          │
          │        ◉ 92.0
          │    ◉ 88.5    ◉ 91.0
          │                        ◉ 87.5
          │                              ◉ 85.0 ← now
Score 0  ┼──────────────────────────────────────
          Jun 1     Jun 2     Jun 7     Jun 8

Trend: DECLINING (92 → 85 in 7 days)
```

## 3. How It Works

### 3.1 Target Flow (Not Fully Working Yet)

```
User: lint-arwaky-cli trends [path]
    │
    ▼
cli_main_entry.rs → handle_trends(path)
    │
    ├─► Self-lint → ArchitectureGovernanceEntity
    │     └─► Score: 85.0, violations: 42, critical: 1
    │
    ├─► Read history file
    │     MetricsProvider.get_history()
    │     ├─► Read .lint-history.json (JSON-lines)
    │     └─► Parse each line:
    │           {"score":92.0,"timestamp":"2026-06-01T10:00:00Z","violations":28}
    │           {"score":88.5,"timestamp":"2026-06-02T14:30:00Z","violations":35}
    │           {"score":91.0,"timestamp":"2026-06-07T09:15:00Z","violations":30}
    │           {"score":87.5,"timestamp":"2026-06-07T16:00:00Z","violations":42}
    │
    ├─► Trend analysis
    │     analyze_quality_trend(current, previous)
    │     ├─► delta = 85.0 - 87.5 = -2.5
    │     ├─► trend:
    │     │     delta > +1  → "IMPROVING"
    │     │     delta -1..+1 → "STABLE"
    │     │     delta < -1  → "DECLINING"  ← -2.5 → DECLINING
    │     └─► all-time-high = max(history.scores) = 92.0
    │         all-time-low = min(history.scores) = 85.0
    │
    ├─► Save current score
    │     MetricsProvider.save_score(current_score)
    │     └─► Append: {"score":85.0,"timestamp":"2026-06-08T11:00:00Z","violations":42}
    │
    └─► Print report
          ├─► "Current score: 85.0 / 100"
          ├─► "Previous score: 87.5 (Jun 7)"
          ├─► "Delta: -2.5 — DECLINING ⚠️"
          ├─► "All-time high: 92.0 (Jun 1)"
          ├─► "All-time low: 85.0 (Jun 8) ← NOW"
```

### 3.2 Current Implementation — What Works

```rust
// cli_main_entry.rs:268
fn handle_trends(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);                    // ✅ resolve path

    let results = lint_path(&root);                     // ✅ self-lint
    let score = compute_score(&results);                // ✅ compute score
    // ✅ Display current score, violations count, critical count

    // ✅ Uses MetricsProvider via DI container (AES023 compliant)
    let container = Arc::new(DependencyInjectionContainer::new(...));
    let metrics = container.metrics_provider();
    let history = rt.block_on(mp.get_history());        // ✅ Read history

    // ✅ Trend analysis
    let delta = score - prev_score;                     // ✅ Delta
    let trend = if delta > 1.0 { "IMPROVING" }          // ✅ Trend direction
          else if delta < -1.0 { "DECLINING" }
          else { "STABLE" };

    // ✅ All-time high/low tracking
    // ✅ Save current score to history
    rt.block_on(mp.save_metric(entry));

    ExitCode::SUCCESS
}
```

### 3.3 Implementation Complete

| Step | Status | Implementation |
|------|--------|----------------|
| Self-lint | ✅ | Score computed from violations |
| Read history | ✅ | `MetricsProvider.get_history()` called via DI container |
| Trend analysis | ✅ | Delta, trend direction (IMPROVING/STABLE/DECLINING) |
| Save history | ✅ | `MetricsProvider.save_metric()` called via DI container |
| Detailed report | ✅ | Current score, previous score, delta, trend, all-time high/low |

### 3.4 History File Format

File: `.lint-history.json` (JSON-lines — one JSON object per line)

```json
{"score":92.0,"timestamp":"2026-06-01T10:00:00Z","violations":28,"critical":0}
{"score":88.5,"timestamp":"2026-06-02T14:30:00Z","violations":35,"critical":1}
{"score":91.0,"timestamp":"2026-06-07T09:15:00Z","violations":30,"critical":0}
```

JSON-lines format was chosen because:
- Append-only — easy to add data
- No database needed
- Can be read streaming
- Human-readable

## 4. Key Files

| File | Status | Function |
|------|--------|----------|
| `taxonomy/lint_score_vo.rs` | ✅ | Score value wrapper |
| `taxonomy/lint_severity_vo.rs` | ✅ | `score_impact()` — LOW:1, MEDIUM:2, HIGH:3, CRITICAL:5 |
| `taxonomy/architecture_governance_entity.rs` | ✅ | Score + violations + compliance |
| `contract/metrics_provider_port.rs` | ✅ | `get_history()`, `save_metric()`, `get_trend_summary()` |
| `contract/project_governance_protocol.rs` | ✅ | `analyze_quality_trend(current, previous) → LintResultList` |
| `infrastructure/python_metrics_adapter.rs` | ✅ | `MetricsProvider` — read/write `.lint-history.json` |
| `infrastructure/python_analysis_adapter.rs` | ✅ | `TrendsAdapter` — trends analysis |
| `agent/analysis_execution_orchestrator.rs` | ✅ | Orchestrates trend analysis |
| `surfaces/cli_analysis_command.rs` | ✅ | Displays trend analysis results |
| `surfaces/cli_main_entry.rs` | ✅ | `handle_trends()` — full pipeline via DI container + MetricsProvider |

## 5. Acceptance Criteria

| # | Criteria | Status | Mechanism |
|---|----------|--------|-----------|
| AC001 | `trends` displays current score | ✅ | `compute_score()` from violations |
| AC002 | Score from severity (100 - deductions) | ✅ | `ArchitectureGovernanceEntity.add_result()` |
| AC003 | History saved to `.lint-history.json` | ✅ | `MetricsProvider.save_metric()` called via DI container |
| AC004 | Trend direction: IMPROVING/STABLE/DECLINING | ✅ | Computed from delta (>+1 IMPROVING, <-1 DECLINING, else STABLE) |
| AC005 | All-time high/low tracked | ✅ | `fold(f64::max)` / `fold(f64::min)` over history |
| AC006 | Delta current - previous displayed | ✅ | `score - prev_score` formatted with sign |
| AC007 | History auto-create on first run | ✅ | `MetricsProvider.save_metric()` creates file with `OpenOptions::create(true)` |
| AC008 | `MetricsProvider.get_history()` reads history | ✅ | JSON-lines parsing works |
| AC009 | `cargo check --bin lint-arwaky-cli` passes | ✅ | |
| AC010 | `cargo test` passes | ✅ | |

