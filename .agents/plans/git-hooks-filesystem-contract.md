# Plan: git-hooks — filesystem contract

## Goal
Hapus direct dependency git-hooks capabilities ke shared I/O utilities. Semua I/O via `IFilesystemAggregate` contract.

## Problem
- `capabilities_hook_adapter.rs` line 53: `shared::filesystem::utility_filesystem_io::write_file(...)` — DIRECT I/O
- `capabilities_hook_adapter.rs` line 78: `shared::filesystem::utility_filesystem_io::path_exists(...)` — DIRECT I/O
- `capabilities_hook_adapter.rs` line 102: `shared::filesystem::utility_filesystem_io::is_dir(...)` — DIRECT I/O
- `capabilities_hook_manager.rs` — already uses `IFilesystemAggregate` ✓

## Capabilities Analysis
| File | Status |
|------|--------|
| `capabilities_hook_adapter.rs` | ✗ DIRECT I/O — `write_file`, `path_exists`, `is_dir` |
| `capabilities_hook_manager.rs` | ✓ Already uses `IFilesystemAggregate` |
| `capabilities_diff_checker.rs` | ✓ Zero I/O |

---

## Changes

### Change 1: Inject filesystem aggregate ke hook adapter
**File**: `crates/git-hooks/src/capabilities_hook_adapter.rs`

Tambah dependency:
```rust
pub struct GitHookAdapter {
    root_dir: FilePath,
    filesystem: Arc<dyn IFilesystemAggregate>,  // NEW
}
```

Update constructor:
```rust
impl GitHookAdapter {
    pub fn new(root_dir: FilePath, filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { root_dir, filesystem }
    }
}
```

### Change 2: Ganti direct I/O calls
**File**: `crates/git-hooks/src/capabilities_hook_adapter.rs`

Sebelum (line 53):
```rust
shared::filesystem::utility_filesystem_io::write_file(&hook_path, &hook_content).map_err(...)
```

Sesudah:
```rust
self.filesystem.write_string(&hook_path, &hook_content).map_err(...)
```

Sebelum (line 78):
```rust
if shared::filesystem::utility_filesystem_io::path_exists(&hook_path) {
```

Sesudah:
```rust
if self.filesystem.path_exists(&hook_path) {
```

Sebelum (line 102):
```rust
shared::filesystem::utility_filesystem_io::is_dir(&git)
```

Sesudah:
```rust
self.filesystem.is_dir(&git)
```

### Change 3: Update container wiring
**File**: `crates/git-hooks/src/root_git_hooks_container.rs`

```rust
pub fn new_default() -> Self {
    let filesystem: Arc<dyn IFilesystemAggregate> = Arc::new(filesystem::FilesystemOrchestrator::new());
    let hook_adapter: Arc<dyn IHookManagerProtocol> = Arc::new(
        crate::capabilities_hook_adapter::GitHookAdapter::new(
            FilePath::new(".".to_string()).unwrap_or_default(),
            filesystem,  // NEW
        )
    );
    Self::new(hook_adapter)
}
```

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p git-hooks-lint-arwaky`
2. `cargo nextest run -p git-hooks-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/git-hooks/src/` — zero

## Files Modified
1. `crates/git-hooks/src/capabilities_hook_adapter.rs`
2. `crates/git-hooks/src/root_git_hooks_container.rs`
