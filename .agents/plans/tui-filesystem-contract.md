# Plan: tui — filesystem contract

## Goal
Hapus direct dependency tui capabilities ke shared I/O utilities. Capabilities pakai `IFilesystemAggregate` contract.

## Problem
- `capabilities_lint_executor.rs` line 52: `shared::filesystem::utility_filesystem_io::find_workspace_root(path)` — DIRECT I/O
- `capabilities_lint_executor.rs` line 144: `shared::filesystem::utility_filesystem_io::find_workspace_root(path)` — DIRECT I/O
- `capabilities_lint_executor.rs` line 158: `shared::filesystem::utility_filesystem_io::scan_directory_with_ignored(...)` — DIRECT I/O
- `capabilities_lint_executor.rs` line 301: `shared::filesystem::utility_filesystem_io::find_workspace_root(path)` — DIRECT I/O
- `capabilities_lint_executor.rs` line 315: `shared::filesystem::utility_filesystem_io::scan_directory_with_ignored(...)` — DIRECT I/O
- `capabilities_lint_executor.rs` line 910: `shared::filesystem::utility_filesystem_io::find_workspace_root(...)` — DIRECT I/O

**Note**: File sudah import `IFilesystemAggregate` (line 19) tapi belum wired ke struct.

## Capabilities Analysis
| File | Status |
|------|--------|
| `capabilities_lint_executor.rs` | ✗ DIRECT I/O — `find_workspace_root`, `scan_directory_with_ignored` (6 call sites) |
| `capabilities_action_handler.rs` | ✓ Zero I/O |

---

## Changes

### Change 1: Tambah filesystem aggregate ke LintExecutor struct
**File**: `crates/tui/src/capabilities_lint_executor.rs`

```rust
pub struct LintExecutor {
    code_analysis: Arc<dyn ICodeAnalysisAggregate>,
    // ... existing fields ...
    filesystem: Arc<dyn IFilesystemAggregate>,  // NEW
}
```

### Change 2: Update constructor
**File**: `crates/tui/src/capabilities_lint_executor.rs`

Tambah `filesystem` parameter ke `new()`.

### Change 3: Ganti semua direct calls (6 call sites)
**File**: `crates/tui/src/capabilities_lint_executor.rs`

```rust
// Sebelum (line 52):
let scan_root = shared::filesystem::utility_filesystem_io::find_workspace_root(path)
// Sesudah:
let scan_root = self.filesystem.workspace_root(path)

// Sebelum (line 144):
shared::filesystem::utility_filesystem_io::find_workspace_root(path)
// Sesudah:
self.filesystem.workspace_root(path)

// Sebelum (line 158):
shared::filesystem::utility_filesystem_io::scan_directory_with_ignored(...)
// Sesudah:
self.filesystem.scan_directory_with_ignored(...)

// Sebelum (line 301):
shared::filesystem::utility_filesystem_io::find_workspace_root(path)
// Sesudah:
self.filesystem.workspace_root(path)

// Sebelum (line 315):
shared::filesystem::utility_filesystem_io::scan_directory_with_ignored(...)
// Sesudah:
self.filesystem.scan_directory_with_ignored(...)

// Sebelum (line 910):
shared::filesystem::utility_filesystem_io::find_workspace_root(...)
// Sesudah:
self.filesystem.workspace_root(...)
```

### Change 4: Update container wiring
**File**: `crates/tui/src/root_tui_container.rs`

Container sudah punya `filesystem: Arc<dyn IFilesystemAggregate>`. Pass ke `LintExecutor::new(...)`.

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p tui-lint-arwaky`
2. `cargo nextest run -p tui-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/tui/src/` — zero

## Files Modified
1. `crates/tui/src/capabilities_lint_executor.rs`
2. `crates/tui/src/root_tui_container.rs`
