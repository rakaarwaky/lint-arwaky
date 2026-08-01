# Plan: import-rules — filesystem contract

## Goal
Hapus direct dependency import-rules ke shared I/O utilities. Agent pakai `IFilesystemAggregate` contract. Capabilities zero I/O tetap.

## Problem
- `agent_import_orchestrator.rs` line 13: `use shared::filesystem::utility_filesystem_io::{path_exists, read_file, walk_source_files}`
- `agent_import_orchestrator.rs` line 61: `shared::filesystem::utility_filesystem_io::find_workspace_root`
- `collect_files()` method calls `walk_source_files` directly
- `run_audit()` calls `path_exists` and `read_file` directly

## Capabilities Analysis (zero I/O ✓)
| File | Status |
|------|--------|
| `capabilities_import_mandatory_checker.rs` | ✓ Zero I/O — receives content_map |
| `capabilities_import_forbidden_checker.rs` | ✓ Zero I/O — receives content_map |
| `capabilities_import_unused_checker.rs` | ✓ Zero I/O — receives content_map |
| `capabilities_import_cycle_checker.rs` | ✓ Zero I/O — receives content_map |
| `capabilities_import_dummy_checker.rs` | ✓ Zero I/O — receives content_map |

---

## Changes

### Change 1: Contract — tambah methods untuk import-rules
**File**: `crates/shared/src/filesystem/contract_filesystem_aggregate.rs`

```rust
/// Check if target path exists.
fn path_exists(&self, path: &Path) -> bool;

/// Find workspace root by walking up from start path.
fn workspace_root(&self, start: &str) -> Option<PathBuf>;

/// Read file content. Checks cache first, falls back to disk.
fn read_file(&self, path: &Path) -> Option<String>;
```

### Change 2: Agent deps — tambah filesystem aggregate
**File**: `crates/import-rules/src/agent_import_orchestrator.rs`

```rust
pub struct ImportOrchestratorDeps {
    pub mandatory: Arc<dyn IImportMandatoryProtocol>,
    pub forbidden: Arc<dyn IImportForbiddenProtocol>,
    pub unused: Arc<dyn IUnusedImportProtocol>,
    pub cycle: Arc<dyn ICycleImportProtocol>,
    pub dummy: Arc<dyn IDummyImportCheckerProtocol>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,  // NEW
}
```

### Change 3: Agent — gunakan contract, hapus direct imports
**File**: `crates/import-rules/src/agent_import_orchestrator.rs`

Hapus:
```rust
- use shared::filesystem::utility_filesystem_io::{path_exists, read_file, walk_source_files};
```

Tambah:
```rust
+ use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
```

Update `run_audit()`:
```rust
// Sebelum:
if !path_exists(target.value()) { ... }
// Sesudah:
if !self.deps.filesystem.path_exists(target.as_ref()) { ... }
```

Update `run_audit()` content reading:
```rust
// Sebelum:
read_file(f.value()).ok().map(|c| (f.value().to_string(), c))
// Sesudah:
self.deps.filesystem.read_file(Path::new(f.value())).map(|c| (f.value().to_string(), c))
```

Update `collect_files()`:
```rust
// Sebelum:
walk_source_files(path, &mut files, &ignored);
// Sesudah:
let entries = self.deps.filesystem.discover_files(path, &ignored);
let files: Vec<FilePath> = entries.iter()
    .filter_map(|f| FilePath::new(f.path.to_string_lossy().to_string()).ok())
    .collect();
```

Update `find_workspace_root`:
```rust
// Sebelum:
shared::filesystem::utility_filesystem_io::find_workspace_root(target.value())
// Sesudah:
self.deps.filesystem.workspace_root(target.value())
```

### Change 4: Container — inject filesystem aggregate
**File**: `crates/import-rules/src/root_import_rules_container.rs`

Tambah field `filesystem: Arc<dyn IFilesystemAggregate>` dan inject ke orchestrator.

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p import-rules-lint-arwaky`
2. `cargo nextest run -p import-rules-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/import-rules/src/` — zero

## Files Modified
1. `crates/shared/src/filesystem/contract_filesystem_aggregate.rs`
2. `crates/import-rules/src/agent_import_orchestrator.rs`
3. `crates/import-rules/src/root_import_rules_container.rs`
