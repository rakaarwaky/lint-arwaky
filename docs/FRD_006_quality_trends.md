# FRD — Track Quality Trends Over Time

> **PRD Reference**: [FR-006](PRD.md) — Track quality trends over time
> **Dependency**: FR-004 (Self-lint)
> **Status**: ⚠️ PARTIAL — Score computation + history check implemented; persistence + analysis are stubs

## 1. Problem Statement

Sebelum quality trends:

| Issue | Description |
|-------|-------------|
| **No history** | Setiap lint run hasilnya lenyap — tidak bisa lihat progress |
| **No trend analysis** | Tidak bisa bandingin skor sekarang vs kemarin |
| **No persistence** | Score tidak pernah disimpan ke disk |
| **No CI memory** | CI run tidak tahu skor build sebelumnya |
| **No regression detection** | Tidak bisa deteksi kapan kualitas turun |

## 2. Konsep Dasar

Setiap kali `lint-arwaky-cli trends` dijalankan:
1. **Self-lint** → dapat current score
2. **Baca history** dari file `.lint-history.json`
3. **Bandingkan** current vs previous → tentukan trend
4. **Simpan** current score ke history
5. **Laporkan**: score, delta, all-time high/low, trend direction

```
Score 100 ┤
          │
          │        ◉ 92.0
          │    ◉ 88.5    ◉ 91.0
          │                        ◉ 87.5
          │                              ◉ 85.0 ← sekarang
Score 0  ┼──────────────────────────────────────
          Juni 1    Juni 2    Juni 7    Juni 8

Trend: DECLINING (92 → 85 dalam 7 hari)
```

## 3. Mekanisme Kerja

### 3.1 Target Flow (Belum Sepenuhnya Working)

```
User: lint-arwaky-cli trends [path]
    │
    ▼
cli_main_entry.rs → handle_trends(path)
    │
    ├─► Self-lint → ArchitectureGovernanceEntity
    │     └─► Score: 85.0, violations: 42, critical: 1
    │
    ├─► Baca history file
    │     MetricsProvider.get_history()
    │     ├─► Baca .lint-history.json (JSON-lines)
    │     └─► Parse tiap baris:
    │           {"score":92.0,"timestamp":"2026-06-01T10:00:00Z","violations":28}
    │           {"score":88.5,"timestamp":"2026-06-02T14:30:00Z","violations":35}
    │           {"score":91.0,"timestamp":"2026-06-07T09:15:00Z","violations":30}
    │           {"score":87.5,"timestamp":"2026-06-07T16:00:00Z","violations":42}
    │
    ├─► Analisis trend
    │     analyze_quality_trend(current, previous)
    │     ├─► delta = 85.0 - 87.5 = -2.5
    │     ├─► trend:
    │     │     delta > +1  → "IMPROVING"
    │     │     delta -1..+1 → "STABLE"
    │     │     delta < -1  → "DECLINING"  ← -2.5 → DECLINING
    │     └─► all-time-high = max(history.scores) = 92.0
    │         all-time-low = min(history.scores) = 85.0
    │
    ├─► Simpan current score
    │     MetricsProvider.save_score(current_score)
    │     └─► Append: {"score":85.0,"timestamp":"2026-06-08T11:00:00Z","violations":42}
    │
    └─► Cetak report
          ├─► "Current score: 85.0 / 100"
          ├─► "Previous score: 87.5 (Jun 7)"
          ├─► "Delta: -2.5 — DECLINING ⚠️"
          ├─► "All-time high: 92.0 (Jun 1)"
          ├─► "All-time low: 85.0 (Jun 8) ← NOW"
```

### 3.2 Current Implementation — Yang Berfungsi

```rust
// cli_main_entry.rs:255
fn handle_trends(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);                    // ✅ resolve path

    let results = lint_path(&root);                     // ✅ self-lint
    let score = compute_score(&results);                // ✅ compute score
    println!("Current score: {}", score);                // ✅ print score

    let history = std::path::Path::new(&root)
        .join(".lint-arwaky-trends.json");              // ✅ cek file history
    if history.exists() {
        println!("History file: {}", history.display()); // ✅ bilang ada
    } else {
        println!("No history yet — first run");          // ✅ bilang belum ada
    }

    // ❌ Tidak nulis history
    // ❌ Tidak baca history
    // ❌ Tidak analisis trend
    // ❌ Tidak compare

    ExitCode::Success
}
```

### 3.3 Yang Belum — Gap Analysis

| Step | Status | Yang Kurang |
|------|--------|-------------|
| Self-lint | ✅ | Score computed dari violations |
| Baca history | ⚠️ | `MetricsProvider.get_history()` udah bisa baca JSON-lines, tapi trends handler **tidak panggil** |
| Analisis trend | ❌ | `IProjectGovernanceProtocol.analyze_quality_trend()` — trait ada, implementasi **belum ada** |
| Simpan history | ⚠️ | `MetricsProvider.save_metric()` — method ada, tapi **tidak dipanggil** dari trends flow |
| Report detail | ❌ | Hanya print current score — delta, high/low, trend direction tidak dihitung |

### 3.4 History File Format

File: `.lint-history.json` (JSON-lines — satu JSON object per baris)

```json
{"score":92.0,"timestamp":"2026-06-01T10:00:00Z","violations":28,"critical":0}
{"score":88.5,"timestamp":"2026-06-02T14:30:00Z","violations":35,"critical":1}
{"score":91.0,"timestamp":"2026-06-07T09:15:00Z","violations":30,"critical":0}
```

Format JSON-lines dipilih karena:
- Append-only — gampang nambah data
- Tidak perlu kunci database
- Bisa dibaca streaming
- Human-readable

## 4. File-file Kunci

| File | Status | Fungsi |
|------|--------|--------|
| `taxonomy/lint_score_vo.rs` | ✅ | Score value wrapper |
| `taxonomy/lint_severity_vo.rs` | ✅ | `score_impact()` — LOW:1, MEDIUM:2, HIGH:3, CRITICAL:5 |
| `taxonomy/architecture_governance_entity.rs` | ✅ | Score + violations + compliance |
| `contract/metrics_provider_port.rs` | ✅ | `get_history()`, `save_metric()`, `get_trend_summary()` |
| `contract/project_governance_protocol.rs` | ✅ | `analyze_quality_trend(current, previous) → LintResultList` (implementasi ❌) |
| `infrastructure/python_metrics_adapter.rs` | ✅ | `MetricsProvider` — baca/tulis `.lint-history.json` |
| `infrastructure/python_analysis_adapter.rs` | ⚠️ | `TrendsAdapter` — stub (return empty) |
| `agent/analysis_execution_orchestrator.rs` | ⚠️ | `get_trends()` — panggil self-lint tapi belum baca history |
| `surfaces/cli_analysis_command.rs` | ⚠️ | Print "Quality trend: STABLE or IMPROVING" — dummy |
| `surfaces/cli_main_entry.rs` | ⚠️ | `handle_trends()` — print score doang, gak write history |

## 5. Acceptance Criteria

| # | Kriteria | Status | Mekanisme |
|---|----------|--------|-----------|
| AC001 | `trends` nampilin current score | ✅ | `compute_score()` dari violations |
| AC002 | Score dari severity (100 - deductions) | ✅ | `ArchitectureGovernanceEntity.add_result()` |
| AC003 | History disimpan ke `.lint-history.json` | ⚠️ | `MetricsProvider.save_metric()` ada tapi tidak terpanggil |
| AC004 | Trend direction: IMPROVING/STABLE/DECLINING | ❌ | `analyze_quality_trend()` trait ada, implementasi 0 |
| AC005 | All-time high/low tracked | ❌ | Tidak ada tracking |
| AC006 | Delta current - previous ditampilkan | ❌ | Hanya print current score |
| AC007 | History auto-create on first run | ❌ | History file tidak pernah di-write |
| AC008 | `MetricsProvider.get_history()` baca history | ✅ | JSON-lines parsing works |
| AC009 | `cargo check --bin lint-arwaky-cli` lulus | ✅ | |
| AC010 | `cargo test` lulus | ✅ | |
