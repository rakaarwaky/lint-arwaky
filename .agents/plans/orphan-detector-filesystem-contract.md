# Plan: orphan-detector — filesystem contract

## Goal
Hapus direct dependency orphan-detector capabilities ke shared I/O utilities. Semua I/O via `IFilesystemAggregate` contract.

## Problem
- `capabilities_orphan_graph_resolver.rs` line 582: `shared::filesystem::utility_filesystem_io::scan_directory(src_dir)` — **DIRECT I/O di capabilities!**
- `agent_orphan_orchestrator.rs` — sudah pakai `IFilesystemAggregate` contract ✓
- `utility_orphan_io.rs` — shared internal, sudah migrate ke `filesystem::utility_filesystem_io`

## Capabilities Analysis
| File | Status |
|------|--------|
| `capabilities_orphan_graph_resolver.rs` | ✗ DIRECT I/O — `scan_directory` |
| `capabilities_orphan_parser_dispatcher.rs` | ✓ Zero I/O — parses content |
| `capabilities_orphan_surfaces_analyzer.rs` | ✓ Zero I/O — analyzes data |
| `capabilities_orphan_rust_checker.rs` | ✓ Zero I/O |
| `capabilities_orphan_python_checker.rs` | ✓ Zero I/O |
| `capabilities_orphan_ts_checker.rs` | ✓ Zero I/O |

---

## Changes

### Change 1: Inject filesystem aggregate ke graph resolver
**File**: `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

Tambah dependency:
```rust
pub struct OrphanGraphResolver {
    filesystem: Arc<dyn IFilesystemAggregate>,
}
```

### Change 2: Ganti direct `scan_directory` call
**File**: `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

Sebelum (line 582):
```rust
let entries = shared::filesystem::utility_filesystem_io::scan_directory(src_dir);
```

Sesudah:
```rust
let entries = self.filesystem.scan_directory(src_dir);
```

### Change 3: Update constructor
**File**: `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

Update `new()` untuk terima `filesystem` parameter.

### Change 4: Update container wiring
**File**: `crates/orphan-detector/src/root_orphan_detector_container.rs`

Inject filesystem aggregate ke `OrphanGraphResolver::new()`.

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p orphan-detector-lint-arwaky`
2. `cargo nextest run -p orphan-detector-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/orphan-detector/src/` — zero

## Files Modified
1. `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`
2. `crates/orphan-detector/src/root_orphan_detector_container.rs`
