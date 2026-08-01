# Plan: external-lint — filesystem contract

## Goal
Hapus direct dependency external-lint capabilities ke shared I/O utilities. Capabilities pakai `IFilesystemAggregate` contract atau shared utility functions tetap via contract.

## Problem
- `capabilities_external_lint_adapter.rs` line 15: `use shared::filesystem::utility_filesystem_io as ext_io;` — ALIAS import
  - `ext_io::canonicalize_path` (line 21, 82)
  - `ext_io::has_python_files` (line 47)
  - `ext_io::is_executable_in_path` (line 51)
  - `ext_io::has_local_bin` (line 64)
  - `ext_io::has_config_file` (line 92)
  - `ext_io::has_cargo_toml` (line 136)
  - `ext_io::has_cargo_lock` (line 147)
- `capabilities_js_prettier_adapter.rs` line 48: `shared::filesystem::utility_filesystem_io::is_file(...)` — DIRECT I/O
- `capabilities_js_tsc_adapter.rs` line 51: `shared::filesystem::utility_filesystem_io::is_file(...)` — DIRECT I/O
- `capabilities_js_eslint_adapter.rs` line 49: `shared::filesystem::utility_filesystem_io::is_file(...)` — DIRECT I/O

## Capabilities Analysis
| File | Status |
|------|--------|
| `capabilities_external_lint_adapter.rs` | ✗ DIRECT I/O — 8 function calls |
| `capabilities_js_prettier_adapter.rs` | ✗ DIRECT I/O — `is_file` |
| `capabilities_js_tsc_adapter.rs` | ✗ DIRECT I/O — `is_file` |
| `capabilities_js_eslint_adapter.rs` | ✗ DIRECT I/O — `is_file` |
| `capabilities_py_ruff_adapter.rs` | ✓ Zero I/O |
| `capabilities_py_bandit_adapter.rs` | ✓ Zero I/O |
| `capabilities_py_mypy_adapter.rs` | ✓ Zero I/O |
| `capabilities_rs_clippy_adapter.rs` | ✓ Zero I/O |
| `capabilities_rs_rustfmt_adapter.rs` | ✓ Zero I/O |
| `capabilities_rs_cargo_audit_adapter.rs` | ✓ Zero I/O |

**Note**: `ext_io` functions (`canonicalize_path`, `has_python_files`, `is_executable_in_path`, `has_local_bin`, `has_config_file`, `has_cargo_toml`, `has_cargo_lock`) are stateless path helpers. Contract sudah punya `canonicalize` dan `is_file`. Sisa functions perlu ditambah ke contract atau dipertahankan sebagai shared utility.

---

## Changes

### Change 1: JS adapters — inject filesystem aggregate
**File**: `crates/external-lint/src/capabilities_js_prettier_adapter.rs`
**File**: `crates/external-lint/src/capabilities_js_tsc_adapter.rs`
**File**: `crates/external-lint/src/capabilities_js_eslint_adapter.rs`

Tambah `filesystem: Arc<dyn IFilesystemAggregate>` ke struct. Ganti:
```rust
// Sebelum:
if shared::filesystem::utility_filesystem_io::is_file(Path::new(path_str))
// Sesudah:
if self.filesystem.is_file(Path::new(path_str))
```

### Change 2: ExternalLintUtilityAdapter — inject filesystem aggregate
**File**: `crates/external-lint/src/capabilities_external_lint_adapter.rs`

```rust
pub struct ExternalLintUtilityAdapter {
    filesystem: Arc<dyn IFilesystemAggregate>,  // NEW
}
```

Ganti `ext_io::canonicalize_path` → `self.filesystem.canonicalize(...)`.
Ganti `ext_io::is_file` → `self.filesystem.is_file(...)`.
Ganti `ext_io::path_exists` → `self.filesystem.path_exists(...)`.

### Change 3: Path helpers sudah ditambahkan ke contract
**File**: `crates/shared/src/filesystem/contract_filesystem_aggregate.rs` — DONE
**File**: `crates/filesystem/src/agent_filesystem_orchestrator.rs` — DONE

6 functions sudah ditambahkan:
- `has_python_files(dir: &Path) -> bool`
- `has_config_file(dir: &Path) -> bool`
- `has_cargo_toml(path_str: &str) -> Option<String>`
- `has_cargo_lock(path_str: &str) -> Option<String>`
- `is_executable_in_path(executable: &str) -> bool`
- `has_local_bin(working_dir: &Path, executable: &str) -> bool`

### Change 4: Update container wiring
**File**: `crates/external-lint/src/root_external_lint_container.rs`

Inject `filesystem` ke `ExternalLintUtilityAdapter` dan semua JS adapters.

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p external-lint-lint-arwaky`
2. `cargo nextest run -p external-lint-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/external-lint/src/` — zero

## Files Modified
1. `crates/shared/src/filesystem/contract_filesystem_aggregate.rs` — add 6 path helper methods
2. `crates/filesystem/src/agent_filesystem_orchestrator.rs` — implement 6 path helpers
3. `crates/external-lint/src/capabilities_external_lint_adapter.rs`
4. `crates/external-lint/src/capabilities_js_prettier_adapter.rs`
5. `crates/external-lint/src/capabilities_js_tsc_adapter.rs`
6. `crates/external-lint/src/capabilities_js_eslint_adapter.rs`
7. `crates/external-lint/src/root_external_lint_container.rs`
