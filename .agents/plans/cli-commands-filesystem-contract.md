# Plan: cli-commands — filesystem contract

## Goal
Hapus direct dependency cli-commands surface layer ke shared I/O utilities. Surface pakai `IFilesystemAggregate` contract.

## Problem
- `surface_orphan_action.rs` line 100: `shared::filesystem::utility_filesystem_io::find_workspace_root(...)` — DIRECT I/O di surface layer

## Capabilities Analysis
| File | Status |
|------|--------|
| `surface_orphan_action.rs` | ✗ DIRECT I/O — `find_workspace_root` |
| `surface_check_command.rs` | ✓ Already uses `IFilesystemAggregate` |
| `surface_output_component.rs` | ✓ Already uses `IFilesystemAggregate` |
| `surface_setup_command.rs` | ✓ Already uses `IFilesystemAggregate` |

---

## Changes

### Change 1: Surface — gunakan contract, hapus direct import
**File**: `crates/cli-commands/src/surface_orphan_action.rs`

Tambah dependency ke struct atau terima sebagai parameter:
```rust
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
```

Sebelum (line 100):
```rust
let ws_top_root = shared::filesystem::utility_filesystem_io::find_workspace_root(
    &ws_abs.to_string_lossy(),
);
```

Sesudah:
```rust
let ws_top_root = filesystem.workspace_root(&ws_abs.to_string_lossy());
```

### Change 2: Container — inject filesystem aggregate
**File**: `crates/cli-commands/src/root_cli_container.rs`

Container sudah punya `filesystem: Arc<dyn IFilesystemAggregate>`. Pass ke `surface_orphan_action` saat construction.

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p cli-commands-lint-arwaky`
2. `cargo nextest run -p cli-commands-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/cli-commands/src/` — zero

## Files Modified
1. `crates/cli-commands/src/surface_orphan_action.rs`
2. `crates/cli-commands/src/root_cli_container.rs`
