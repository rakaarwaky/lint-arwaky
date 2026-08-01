# Plan: naming-rules — filesystem contract + consolidate overlaps

## Goal
1. Hapus direct dependency naming-rules ke shared I/O utilities
2. Consolidate duplicate functions ke `utility_filesystem_io.rs`
3. Agent pakai `IFilesystemAggregate` contract

## Overlap Analysis

| naming-rules | filesystem_io | Status | Action |
|-------------|---------------|--------|--------|
| `walk_recursive` | `walk_source_files` | DUPLICATE | Hapus — agent pakai contract |
| `filter_source_files` | ❌ tidak ada | UNIQUE | PINDAH ke `utility_filesystem_io` |
| `get_stem` | `get_file_stem` | DUPLICATE | Delegate ke `get_file_stem` |
| `get_suffix` | ❌ | UNIQUE | Tetap di `utility_naming_checker` |
| `layer_keys` | ❌ | UNIQUE | Tetap di `utility_naming_checker` |
| `detect_layer` | ❌ | UNIQUE | Tetap di `utility_naming_checker` |
| `is_exception` | ❌ | UNIQUE | Tetap di `utility_naming_checker` |
| `file_level_result` | ❌ | UNIQUE | Tetap di `utility_naming_checker` |
| `string_filename_result` | ❌ | UNIQUE | Tetap di `utility_naming_checker` |

---

## Changes

### Change 1: Tambah `filter_source_files` ke `utility_filesystem_io.rs`
**File**: `crates/shared/src/filesystem/utility_filesystem_io.rs`

Tambah setelah `is_source_ext`:
```rust
/// Filter FilePathList to only include source files.
pub fn filter_source_files(files: &FilePathList) -> FilePathList {
    let filtered: Vec<FilePath> = files
        .values
        .iter()
        .filter(|f| {
            let path = Path::new(&f.value);
            path.extension()
                .and_then(|e| e.to_str())
                .map(|ext| is_source_ext(ext))
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    FilePathList::new(filtered)
}
```

Perlu tambah import: `use crate::common::taxonomy_paths_vo::FilePathList;`

### Change 2: Delegate `get_stem` ke `get_file_stem`
**File**: `crates/shared/src/naming-rules/utility_naming_checker.rs`

Sebelum:
```rust
pub fn get_stem(filename: &str) -> Option<&str> {
    match filename.rfind('.') {
        Some(pos) if pos > 0 => Some(&filename[..pos]),
        _ => Some(filename),
    }
}
```

Sesudah:
```rust
pub fn get_stem(filename: &str) -> Option<&str> {
    Some(crate::filesystem::utility_filesystem_io::get_file_stem(filename))
}
```

### Change 3: Contract — tambah `discover_files()`
**File**: `crates/shared/src/filesystem/contract_filesystem_aggregate.rs`

```rust
/// FR-001 light: Walk + filter only. No parse/extract/graph.
fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry>;
```

### Change 4: Implement `discover_files()` di filesystem crate
**File**: `crates/filesystem/src/agent_filesystem_orchestrator.rs`

```rust
fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry> {
    let extensions = Language::extensions();
    self.walker.walk(root, ignored, extensions)
}
```

### Change 5: Agent — tambah filesystem aggregate ke deps
**File**: `crates/naming-rules/src/agent_naming_orchestrator.rs`

```rust
pub struct NamingOrchestratorDeps {
    pub naming_convention_checker: Arc<dyn INamingConventionChecker>,
    pub suffix_prefix_checker: Arc<dyn ISuffixPrefixChecker>,
    pub config: Arc<ArchitectureConfig>,
    pub layer_map: Arc<LayerMapVO>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,  // NEW
}
```

### Change 6: Agent — gunakan contract, hapus I/O imports
**File**: `crates/naming-rules/src/agent_naming_orchestrator.rs`

Sebelum:
```rust
let all_files = shared::naming_rules::utility_naming_filesystem::walk_recursive(
    target, Some(&self.ignored_patterns),
);
let files = shared::naming_rules::utility_file_filter::filter_source_files(&all_files);
```

Sesudah:
```rust
let entries = self.deps.filesystem.discover_files(
    target_path, &self.ignored_patterns.values,
);
let file_paths: Vec<FilePath> = entries
    .iter()
    .filter(|f| !f.content.is_empty())
    .filter_map(|f| FilePath::new(f.path.to_string_lossy().to_string()).ok())
    .collect();
let files = shared::common::FilePathList::new(file_paths);
```

Hapus:
```rust
- use shared::naming_rules::utility_naming_filesystem;
- use shared::naming_rules::utility_file_filter;
```

Tambah:
```rust
+ use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
```

### Change 7: Container — inject filesystem aggregate
**File**: `crates/naming-rules/src/root_naming_rules_container.rs`

```rust
pub struct NamingContainer {
    // ... existing ...
    filesystem: Arc<dyn IFilesystemAggregate>,  // NEW
}
```

Update `orchestrator()`:
```rust
filesystem: self.filesystem.clone(),  // NEW
```

### Change 8: Hapus I/O utilities dari shared
**File**: `crates/shared/src/naming-rules/utility_naming_filesystem.rs` → DELETE
**File**: `crates/shared/src/naming-rules/utility_file_filter.rs` → DELETE

Hapus dari `crates/shared/src/naming-rules/mod.rs`:
```rust
- pub mod utility_naming_filesystem;
- pub mod utility_file_filter;
```

### Change 9: Update capabilities import
**File**: `crates/naming-rules/src/capabilities_suffix_prefix_checker.rs`

Tambah import:
```rust
+ use shared::filesystem::utility_filesystem_io::filter_source_files;
```

Jika capabilities pakai `filter_source_files`, update ke `utility_filesystem_io`.

### TETAP DI SHARED:
- `utility_naming_checker.rs` — `get_stem` sekarang delegate ke `get_file_stem`
- `get_suffix`, `layer_keys`, `detect_layer`, `is_exception`, `file_level_result`, `string_filename_result` — tetap

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check --workspace`
2. `cargo nextest run -p naming-rules-lint-arwaky`
3. `cargo nextest run -p shared-lint-arwaky`
4. `grep -r "utility_naming_filesystem\|utility_file_filter" crates/*/src/` — zero

## Files Modified
1. `crates/shared/src/filesystem/utility_filesystem_io.rs` — add `filter_source_files`
2. `crates/shared/src/naming-rules/utility_naming_checker.rs` — delegate `get_stem`
3. `crates/shared/src/filesystem/contract_filesystem_aggregate.rs` — add `discover_files()`
4. `crates/filesystem/src/agent_filesystem_orchestrator.rs` — implement `discover_files()`
5. `crates/naming-rules/src/agent_naming_orchestrator.rs` — use contract
6. `crates/naming-rules/src/root_naming_rules_container.rs` — inject filesystem
7. `crates/shared/src/naming-rules/utility_naming_filesystem.rs` — DELETE
8. `crates/shared/src/naming-rules/utility_file_filter.rs` — DELETE
9. `crates/shared/src/naming-rules/mod.rs` — remove 2 pub mod
