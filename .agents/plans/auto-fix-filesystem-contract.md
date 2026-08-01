# Plan: auto-fix — filesystem contract

## Goal
Hapus direct dependency auto-fix capabilities ke shared I/O utilities. Semua I/O via `IFilesystemAggregate` contract.

## Problem
- `capabilities_file_adapter.rs` line 14: `shared::filesystem::utility_filesystem_io::path_exists(...)` — DIRECT I/O
- `capabilities_file_adapter.rs` line 17: `shared::filesystem::utility_filesystem_io::read_file(...)` — DIRECT I/O
- `capabilities_file_adapter.rs` line 23: `shared::filesystem::utility_filesystem_io::write_file(...)` — DIRECT I/O
- `capabilities_file_adapter.rs` line 27: `shared::filesystem::utility_filesystem_io::path_exists(...)` — DIRECT I/O
- `capabilities_fix_processor.rs` line 222: `shared::filesystem::utility_filesystem_io::read_file(...)` — DIRECT I/O (DefaultFileAdapter)
- `capabilities_fix_processor.rs` line 228: `shared::filesystem::utility_filesystem_io::write_file(...)` — DIRECT I/O (DefaultFileAdapter)
- `capabilities_fix_processor.rs` line 232: `shared::filesystem::utility_filesystem_io::path_exists(...)` — DIRECT I/O (DefaultFileAdapter)

## Capabilities Analysis
| File | Status |
|------|--------|
| `capabilities_file_adapter.rs` | ✗ DIRECT I/O — `path_exists`, `read_file`, `write_file` |
| `capabilities_fix_processor.rs` | ✗ DIRECT I/O (DefaultFileAdapter) — `read_file`, `write_file`, `path_exists` |

---

## Changes

### Change 1: Inject filesystem aggregate ke FileAdapter
**File**: `crates/auto-fix/src/capabilities_file_adapter.rs`

```rust
pub struct FileAdapter {
    filesystem: Arc<dyn IFilesystemAggregate>,  // NEW
}
```

Update constructor:
```rust
impl FileAdapter {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }
}
```

### Change 2: Ganti direct calls di FileAdapter
**File**: `crates/auto-fix/src/capabilities_file_adapter.rs`

```rust
// Sebelum:
if !shared::filesystem::utility_filesystem_io::path_exists(&path.value) { ... }
// Sesudah:
if !self.filesystem.path_exists(std::path::Path::new(&path.value)) { ... }

// Sebelum:
shared::filesystem::utility_filesystem_io::read_file(&path.value)
// Sesudah:
self.filesystem.read_file(std::path::Path::new(&path.value))

// Sebelum:
shared::filesystem::utility_filesystem_io::write_file(&path.value, &content.value).is_ok()
// Sesudah:
self.filesystem.write_string(std::path::Path::new(&path.value), &content.value).is_ok()

// Sebelum:
shared::filesystem::utility_filesystem_io::path_exists(&path.value)
// Sesudah:
self.filesystem.path_exists(std::path::Path::new(&path.value))
```

### Change 3: Ganti DefaultFileAdapter di fix_processor
**File**: `crates/auto-fix/src/capabilities_fix_processor.rs`

```rust
struct DefaultFileAdapter {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

impl IFileAdapterProtocol for DefaultFileAdapter {
    fn read_file(&self, path: &FilePath) -> Option<ContentString> {
        self.filesystem.read_file(std::path::Path::new(&path.value))
            .map(ContentString::new)
    }

    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool {
        self.filesystem.write_string(std::path::Path::new(&path.value), &content.value).is_ok()
    }

    fn path_exists(&self, path: &FilePath) -> bool {
        self.filesystem.path_exists(std::path::Path::new(&path.value))
    }
}
```

### Change 4: Update container wiring
**File**: `crates/auto-fix/src/root_auto_fix_container.rs`

Tambah `filesystem` ke container dan inject ke `LintFixProcessor` / `FileAdapter`.

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p auto-fix-lint-arwaky`
2. `cargo nextest run -p auto-fix-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/auto-fix/src/` — zero

## Files Modified
1. `crates/auto-fix/src/capabilities_file_adapter.rs`
2. `crates/auto-fix/src/capabilities_fix_processor.rs`
3. `crates/auto-fix/src/root_auto_fix_container.rs`
