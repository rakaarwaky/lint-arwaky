# FRD — Apply Safe Auto-Fixes (Rust + Python + JS/TS)

> **PRD Reference**: [FR-005](PRD.md) — Apply safe auto-fixes
> **Dependency**: FR-003 (Source parsing)
> **Status**: ⚠️ PARTIAL — Naming renamer implemented; `apply_fix` on adapters + orchestrator are stubs

## 1. Problem Statement

Sebelum auto-fix:

| Issue | Description |
|-------|-------------|
| **No automated fix** | Semua violation diperbaiki manual satu per satu |
| **No pipeline** | Tidak ada orchestrated fix — tiap fix jalan sendiri |
| **No dry-run** | Tidak bisa preview perubahan sebelum dieksekusi |
| **No audit trail** | Tidak tercatat apa yang diperbaiki dan kapan |

## 2. Konsep Dasar

Auto-fix = setelah self-lint mendeteksi violation, sistem bisa **memperbaiki sendiri** violation yang aman (safe). Yang tidak aman tetap manual.

**Fixable** (bisa otomatis):
- AES003 (naming) → rename symbol
- AES014 (bypass) → hapus `#[allow(...)]` / `noqa`
- AES015 (unused import) → hapus baris import

**Not fixable** (perlu manual):
- AES004 (file terlalu besar) → refactor
- AES006 (primitif di domain) → bungkus VO
- AES001 (import violation) → keputusan arsitektur

## 3. Mekanisme Kerja

### 3.1 Target Flow

```
User: lint-arwaky-cli fix .
    │
    ▼
cli_fix_command.rs → FixCommandsSurface.fix(path)
    │
    ├─► Self-lint dulu → dapat daftar violations
    │
    ├─► Kelompokkan violation berdasarkan fixability:
    │     ├── Fixable otomatis:
    │     │     ├── AES003 (naming) → NamingRenamerProcessor.rename_symbol()
    │     │     ├── AES014 (bypass) → hapus baris #[allow(...)] / noqa
    │     │     └── AES015 (unused) → hapus baris import
    │     │
    │     └── Manual (dilaporkan ke user):
    │           ├── AES004 (size) → refactor
    │           ├── AES006 (primitive) → bungkus VO
    │           └── ...
    │
    ├─► Eksekusi fix otomatis (kalau bukan dry-run)
    │
    └─► Laporkan:
          ├─► "3 violations fixed automatically"
          └─► "5 violations require manual fix — see above"
```

### 3.2 Naming Renamer — Satu-satunya yang Working

File: `capabilities/naming_renamer_processor.rs` (98 lines)

```
rename_symbol(root_dir, old_name, new_name)
    │
    ├─► Walk semua file di root_dir
    │
    ├─► Baca file line by line
    │
    ├─► Untuk setiap line:
    │     ├─► Skip kalau line adalah:
    │     │     ├─► Single-line comment (// atau #)
    │     │     ├─► Multi-line comment (/* */ masih terbuka)
    │     │     ├─► String literal ('...' atau "...")
    │     │     ├─► Triple-quoted string ("""...""")
    │     │     └─► Template literal (`...`)
    │     │
    │     └─► Replace old_name → new_name (regex word boundary)
    │
    ├─► Write file kalau ada perubahan
    │
    └─► Return count modified files
```

**Contoh**:
```
Sebelum: auth_token_vo.rs → is_symbol_exported(path, symbol)
Sesudah:  auth_token_vo.rs → check_symbol_exported(path, symbol)
(dengan asumsi rename is_symbol → check_symbol)
```

### 3.3 Adapter apply_fix — Semua Stub

Setiap linter adapter punya method `apply_fix()`:

```rust
// contract/linter_adapter_port.rs
pub trait ILinterAdapterPort: Send + Sync {
    async fn scan(&self, path: &DirectoryPath) -> Result<...>;
    async fn apply_fix(&self, path: &FilePath) -> Result<...>;  // NEW
    fn fixable_error_codes(&self) -> Vec<String>;                // NEW
    async fn preview_fix(&self, path: &FilePath) -> Result<...>; // NEW
}
```

Implementasi saat ini — SEMUA STUB:

```rust
// infrastructure/python_ruff_adapter.rs
impl ILinterAdapterPort for PythonRuffAdapter {
    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, ...> {
        // TODO: panggil ruff check --fix
        Ok(ComplianceStatus::new(false))  // ← STUB: return false
    }
}
```

Yang seharusnya:
```rust
// Target implementasi:
async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, ...> {
    let output = Command::new("ruff")
        .args(["check", "--fix", &path.value])
        .output()?;
    Ok(ComplianceStatus::new(output.status.success()))
}
```

### 3.4 Fix Orchestrator — Stub

File: `agent/lint_fix_orchestrator.rs` (20 lines)

```rust
pub struct LintFixOrchestrator { ... }

impl LintFixOrchestratorAggregate for LintFixOrchestrator {
    fn execute(&self, path: &FilePath) -> FixResult {
        // TODO:
        // 1. Run self-lint
        // 2. Classify violations by fixability
        // 3. Call NamingRenamerProcessor for AES003
        // 4. Call adapter.apply_fix() for AES014/AES015
        // 5. Collect results
        // 6. Return FixResult
        FixResult::success("No fixes applied (stub)")  // ← STUB
    }
}
```

### 3.5 CLI Surface — Stub

File: `surfaces/cli_fix_command.rs` (56 lines)

```rust
pub async fn fix(&self, path: &str) {
    // Current: print warning, fallback ke check
    println!("Applying safe fixes to {path}...");
    println!("Fix command is not fully wired yet — falling back to check");
    // self.container.get_fix_orchestrator().execute(path)  ← COMMENTED OUT
}
```

## 4. File-file Kunci

| File | Baris | Status | Fungsi |
|------|-------|--------|--------|
| `taxonomy/fix_result_vo.rs` | 28 | ✅ | `FixResult { output, error }` |
| `taxonomy/fix_applied_event.rs` | 29 | ✅ | `FixApplied { path, adapter, error_code, changes, timestamp }` |
| `contract/lint_fix_aggregate.rs` | 5 | ✅ | `LintFixOrchestratorAggregate::execute(path) → FixResult` |
| `contract/linter_adapter_port.rs` | 15 | ✅ | `apply_fix()`, `preview_fix()`, `fixable_error_codes()` |
| `capabilities/naming_renamer_processor.rs` | 98 | ✅ **Working** | Project-wide symbol rename |
| `infrastructure/rust_linter_adapter.rs` | — | ⚠️ Stub | `apply_fix` return false |
| `infrastructure/python_ruff_adapter.rs` | — | ⚠️ Stub | `apply_fix` return false |
| `infrastructure/python_mypy_adapter.rs` | — | ⚠️ Stub | `apply_fix` return false |
| `infrastructure/python_bandit_adapter.rs` | — | ⚠️ Stub | `apply_fix` return false |
| `infrastructure/javascript_linter_adapter.rs` | — | ⚠️ Stub | `apply_fix` return false |
| `agent/lint_fix_orchestrator.rs` | 20 | ⚠️ Stub | Orchestrator return success dummy |
| `surfaces/cli_fix_command.rs` | 56 | ⚠️ Stub | Fallback ke check |

## 5. Acceptance Criteria

| # | Kriteria | Status |
|---|----------|--------|
| AC001 | `fix .` jalanin lint + auto-fix pipeline | ❌ Stub — cuma println |
| AC002 | AES003 naming violation fix via `NamingRenamerProcessor` | ✅ Working — rename project-wide |
| AC003 | AES014 bypass comments dihapus otomatis | ❌ Stub — `apply_fix` return false |
| AC004 | AES015 unused imports dihapus | ❌ Stub |
| AC005 | `apply_fix()` on all 5 adapters | ❌ Stub — semua return false |
| AC006 | Dry-run `--dry-run` preview changes | ❌ Missing |
| AC007 | `FixAppliedEvent` dicatat | ❌ Stub — orchestrator belum panggil event |
| AC008 | Non-fixable violations dilapor sebagai manual steps | ❌ Stub |
| AC009 | `cargo check --bin lint-arwaky-cli` lulus | ✅ |
| AC010 | `cargo test` lulus | ✅ |
