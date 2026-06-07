# FRD — Self-Lint Target (`lint-arwaky-cli check .`)

> **PRD Reference**: [FR-004](PRD.md) — Self-lint target — project audits itself
> **Dependency**: FR-001 (6-layer AES), FR-002 (Config), FR-003 (Source parsing)
> **Status**: ✅ COMPLETE — Full self-lint pipeline with 31 AES rules, CLI commands, multi-language support
> **Self-lint**: `lint-arwaky-cli check .` — project audits `src-rust/` against all AES rules

## 1. Problem Statement

Sebelum self-lint:

| Issue | Description |
|-------|-------------|
| **No dogfooding** | Project tidak bisa ngecek kepatuhan arsitekturnya sendiri |
| **No CI gate** | PR bisa masuk dengan pelanggaran arsitektur |
| **Manual review** | AES violation cuma ke detect pas code review |
| **No score** | Nggak ada ukuran kuantitatif kesehatan codebase |
| **No reporting** | Violation nggak dilaporkan secara terstruktur |

## 2. Konsep Dasar

Self-lint = jalanin semua 31 AES rules terhadap `src-rust/` (kode project itu sendiri). Hasilnya:
1. **Score**: Mulai 100, dikurangi per pelanggaran
2. **Violations**: Daftar lengkap dengan file, baris, rule, severity
3. **CRITICAL auto-fail**: Kalau ada CRITICAL → exit non-zero

```
Proyek ngelinting dirinya sendiri — dogfooding.
Kalau ada bug di rule AES001, self-lint akan detect sendiri.
```

## 3. Mekanisme Kerja — Step by Step

### 3.1 Entry Point

```
User: lint-arwaky-cli check .
    │
    ▼
cli_main_entry.rs: main()
    │
    ├─► Clap parse args → Commands::Check { path: Some("."), git_diff: false }
    │
    ├─► handle_check(".", false)
    │     │
    │     ├─► lint_path(".")
    │     │
    │     └─► Cetak hasil:
    │           ├─► "Score: 87.5 / 100"
    │           ├─► "CRITICAL: 0 | HIGH: 3 | MEDIUM: 5 | LOW: 2"
    │           └─► Daftar violation per file
    │
    └─► ExitCode::Success (0) atau Failure (1)
```

### 3.2 Pipeline Lint

```
lint_path(project_root)
    │
    ├─► Step 1: Find source directory
    │     ArchLintHandler.find_source_dir(project_root)
    │     ├─► Cek: src-rust/ → Rust ✅
    │     ├─► Cek: src-python/ → Python
    │     ├─► Cek: src-javascript/ → JavaScript
    │     └─► Cek: src/ (generic)
    │
    ├─► Step 2: Load config
    │     ConfigLoaderOrchestrator.load_project_config(project_root)
    │     ├─► detect_language(".") → Rust
    │     ├─► read_config(".", "rust") → lint_arwaky.config.rust.yaml
    │     └─► parse → ArchitectureConfig
    │
    ├─► Step 3: Run all checks
    │     LintCheckingCoordinator.run_all_checks(ArchitectureConfig, source_dir)
    │     │
    │     │  ╔══════════════════════════════════════╗
    │     │  ║  3a. File discovery                  ║
    │     │  ╚══════════════════════════════════════╝
    │     │  Walk source_dir → kumpulkan semua *.rs / *.py / *.js *.ts
    │     │
    │     │  ╔══════════════════════════════════════╗
    │     │  ║  3b. Per-file checks (22 rules)      ║
    │     │  ╚══════════════════════════════════════╝
    │     │  Untuk SETIAP file:
    │     │  ├── Layer detection
    │     │  │     detect_layer() → cari file di layer apa
    │     │  │
    │     │  ├── AES003: Naming convention
    │     │  │     Regex: ^word_word_word\.rs$ ?
    │     │  │     → "architecture_compliance_analyzer.rs" ✅
    │     │  │     → "my_file.rs" ❌ (cuma 2 kata)
    │     │  │
    │     │  ├── AES004: File too large (>500 lines)
    │     │  │     get_line_count() > 500 ? → FLAG
    │     │  │
    │     │  ├── AES005: File too short (<10 lines)
    │     │  │     get_line_count() < 10 ? → FLAG
    │     │  │
    │     │  ├── AES006: Primitive usage
    │     │  │     find_primitive_violations() → cari String/i32 di domain
    │     │  │
    │     │  ├── AES008: Contract suffix
    │     │  │     Cek: kalau di contract/ → suffix _port/_protocol/_aggregate?
    │     │  │
    │     │  ├── AES009: Mandatory struct/trait
    │     │  │     get_raw_symbols() → ada struct/trait/enum?
    │     │  │
    │     │  ├── AES011: Suffix mismatch
    │     │  │     Cek: suffix file ada di allowed list layer?
    │     │  │
    │     │  ├── AES014: Bypass comment
    │     │  │     find_bypass_comments() → #[allow, unwrap(), panic!
    │     │  │
    │     │  ├── AES015: Unused import
    │     │  │     find_unused_imports() → symbol ada tapi nggak dipakai
    │     │  │
    │     │  ├── AES016: Dead inheritance
    │     │  │     Struct {} atau trait {} kosong? → FLAG
    │     │  │
    │     │  ├── AES021: Agent role
    │     │  │     Kalau _container → cuma wiring?
    │     │  │     Kalau _orchestrator → stateless?
    │     │  │
    │     │  ├── AES022: Surface role
    │     │  │     Smart surface → wajib delegate via container
    │     │  │     Passive surface → cuma taxonomy import
    │     │  │
    │     │  ├── AES023: Surface direct import
    │     │  │     extract_imports() → ada "use crate::infrastructure::" ?
    │     │  │
    │     │  ├── AES024: Agent any-bypass
    │     │  │     Cek: ada `dyn Any` atau `Box<Any>` di agent?
    │     │  │
    │     │  ├── AES025: MCP schema
    │     │  │     MCP files → ada docstring + JSON Schema?
    │     │  │
    │     │  ├── AES026: Forbidden inheritance
    │     │  │     Contract Aggregate jangan `impl PortTrait for ...`
    │     │  │
    │     │  ├── AES027: Mandatory inheritance
    │     │  │     Setiap file → implements contract trait?
    │     │  │
    │     │  ├── AES030: Capability method exists
    │     │  │     Dispatch catalog → method ada di class?
    │     │  │
    │     │  ├── AES031: Single bottleneck
    │     │  │     Semua dispatch ke 1 class? → FLAG
    │     │  │
    │     │  ├── AES032: Missing VO
    │     │  │     Capability call → ada parameter VO?
    │     │  │
    │     │  └── AES033: Constant purity
    │     │        File _constant → cuma pub const/pub static?
    │     │
    │     │  ╔══════════════════════════════════════╗
    │     │  ║  3c. Cross-file checks (9 rules)     ║
    │     │  ╚══════════════════════════════════════╝
    │     │  ├── AES001: Import layer violation
    │     │  │     Cocokkan setiap import dengan aturan import per layer
    │     │  │
    │     │  ├── AES002: Mandatory import missing
    │     │  │     Layer wajib import taxonomy? → cek extract_imports
    │     │  │
    │     │  ├── AES007: Layer import alias
    │     │  │     Contract imports harus via barrel (mod.rs)
    │     │  │
    │     │  ├── AES010: Root layer
    │     │  │     Root files → hanya entry point?
    │     │  │
    │     │  ├── AES012: Barrel completeness
    │     │  │     mod.rs → export semua file di layer?
    │     │  │
    │     │  ├── AES013: Internal all forbidden
    │     │  │     Non-mod.rs → pub mod / pub use? → FLAG
    │     │  │
    │     │  ├── AES018: Surface hierarchy
    │     │  │     Utility surface import Smart surface? → FLAG
    │     │  │
    │     │  ├── AES019: Passive surface violation
    │     │  │     Passive surface import agent/contract? → FLAG
    │     │  │
    │     │  └── AES020: Circular dependency
    │     │        Graph analysis → ada cycle di import?
    │     │
    │     └── 3d. Kumpulkan semua violations
    │
    ├─► Step 4: Compute score
    │     ArchitectureGovernanceEntity
    │     ├─► Score = 100.0
    │     ├─► Untuk setiap violation:
    │     │     LOW     → score -= 1
    │     │     MEDIUM  → score -= 2
    │     │     HIGH    → score -= 3
    │     │     CRITICAL → score -= 5
    │     ├─► score = max(0, score)
    │     ├─► Kalau ada CRITICAL → is_passing = false
    │     └─► Kalau score < threshold (ci mode) → exit non-zero
    │
    └─► Step 5: Cetak report
          ├─► Format: text (default), JSON (--format json), SARIF, JUnit
          ├─► Group by severity
          └─► Tampilkan per file dengan line number
```

### 3.3 CI Mode

```
lint-arwaky-cli ci . --threshold 80
    │
    └─► Sama kayak check, tapi:
          ├─► Kalau score < 80 → exit code 1 (CI FAIL)
          ├─► Output JSON (machine-readable)
          └─► Exit code:
                0 = PASS (score >= threshold, no CRITICAL)
                1 = FAIL (score < threshold OR ada CRITICAL)
```

### 3.4 Git Diff Mode

```
lint-arwaky-cli check . --git-diff
    │
    └─► Hanya check file yang berubah di git working tree
          ├─► git diff --name-only → daftar file modified
          └─► Jalankan check hanya untuk file-file itu
```

## 4. File-file Kunci

### Surface
| File | Fungsi |
|------|--------|
| `cli_core_command.rs` | Definisikan semua CLI subcommands via Clap |
| `cli_check_command.rs` | `CheckCommandsSurface` — entry point check/scan |
| `cli_main_entry.rs` | `handle_check()`, `handle_scan()`, `handle_ci()`, `lint_path()` |

### Agent
| File | Fungsi |
|------|--------|
| `architecture_lint_orchestrator.rs` | `run_self_lint(project_root)` — find source + load config + run checks |
| `lint_checking_coordinator.rs` | `run_all_checks(config, src)` — orchestrate 31 AES rules |
| `dependency_injection_container.rs` | Wire semua dependencies |

### Capabilities (10 checkers)
| File | AES Rules |
|------|-----------|
| `architecture_compliance_analyzer.rs` | Layer detection |
| `architecture_import_checker.rs` | AES001, AES002, AES023 |
| `architecture_naming_checker.rs` | AES003, AES008, AES011 |
| `architecture_internal_checker.rs` | AES012, AES013 |
| `architecture_metric_checker.rs` | AES004, AES005, AES006 |
| `architecture_cycle_analyzer.rs` | AES020 |
| `architecture_orphan_analyzer.rs` | AES017 |
| `architecture_inheritance_checker.rs` | AES026, AES027 |
| `surface_hierarchy_checker.rs` | AES018, AES019, AES022 |
| `architecture_lint_handler.rs` | `ArchLintHandler` — implement `IArchLintProtocol` |

## 5. Score Computation Detail

```
Raw score = 100.0

Violation: AES001 | HIGH | infrastructure/foo.rs:42
  Score -= 3 → 97.0

Violation: AES014 | CRITICAL | capabilities/bar.rs:15
  Score -= 5 → 92.0
  is_passing = false ← CRITICAL auto-fail!

Violation: AES003 | LOW | agent/my_file.rs
  Score -= 1 → 91.0

Final score = max(0, 91.0) = 91.0
Status: FAIL (CRITICAL ditemukan)
```

## 6. Report Formats

| Format | Output | Use Case |
|--------|--------|----------|
| Text | Human readable table | Local dev |
| JSON | `{"score": 91.0, "violations": [...]}` | Machine parsing |
| SARIF | SARIF 2.1.0 JSON | GitHub Code Scanning |
| JUnit | JUnit XML | Jenkins/CI pipeline |

## 7. Acceptance Criteria

| # | Kriteria | Status |
|---|----------|--------|
| AC001 | `lint-arwaky-cli check .` jalan tanpa error | ✅ |
| AC002 | Deteksi 153+ violations di codebase sendiri | ✅ |
| AC003 | 31 AES codes (AES001–AES033, 028/029 reserved) | ✅ |
| AC004 | Score: start 100, deduct per severity, CRITICAL = fail | ✅ |
| AC005 | `scan` command = AES + external adapters (clippy, ruff, eslint) | ✅ |
| AC006 | `ci` mode dengan threshold + exit code | ✅ |
| AC007 | Report formats: text, JSON, SARIF, JUnit | ✅ |
| AC008 | `git-diff` — hanya check file berubah | ✅ |
| AC009 | `cargo check --bin lint-arwaky-cli` lulus | ✅ |
| AC010 | `cargo test` — semua tests lulus | ✅ |
