# Plan: config-system — filesystem contract

## Goal
Hapus direct dependency config-system capabilities ke shared I/O utilities. Capabilities pakai `IFilesystemAggregate` contract.

## Problem
- `capabilities_parser_provider.rs` line 17: `shared::filesystem::utility_filesystem_io::read_file(p)` — DIRECT I/O
- `capabilities_parser_provider.rs` line 40: `shared::filesystem::utility_filesystem_io::read_file(p)` — DIRECT I/O
- `capabilities_workspace_detector.rs` — already uses `IFilesystemAggregate` ✓
- `capabilities_yaml_reader.rs` — already uses `IFilesystemAggregate` ✓

## Capabilities Analysis
| File | Status |
|------|--------|
| `capabilities_parser_provider.rs` | ✗ DIRECT I/O — `read_file` (2 call sites) |
| `capabilities_workspace_detector.rs` | ✓ Already uses `IFilesystemAggregate` |
| `capabilities_yaml_reader.rs` | ✓ Already uses `IFilesystemAggregate` |
| `capabilities_rules_validator.rs` | ✓ Zero I/O |

---

## Changes

### Change 1: Inject filesystem aggregate ke parser provider
**File**: `crates/config-system/src/capabilities_parser_provider.rs`

```rust
pub struct ConfigParserProvider {
    filesystem: Arc<dyn IFilesystemAggregate>,  // NEW
}
```

Update constructor:
```rust
impl ConfigParserProvider {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }
}
```

### Change 2: Ganti direct `read_file` calls
**File**: `crates/config-system/src/capabilities_parser_provider.rs`

Sebelum (line 17):
```rust
let content = match shared::filesystem::utility_filesystem_io::read_file(p) {
```

Sesudah:
```rust
let content = match self.filesystem.read_file(std::path::Path::new(p)) {
```

Sebelum (line 40):
```rust
let content = match shared::filesystem::utility_filesystem_io::read_file(p) {
```

Sesudah:
```rust
let content = match self.filesystem.read_file(std::path::Path::new(p)) {
```

### Change 3: Update container wiring
**File**: `crates/config-system/src/root_config_system_container.rs`

Container sudah punya `filesystem: Arc<dyn IFilesystemAggregate>`. Pass ke `ConfigParserProvider::new(filesystem)`.

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p config-system-lint-arwaky`
2. `cargo nextest run -p config-system-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/config-system/src/` — zero

## Files Modified
1. `crates/config-system/src/capabilities_parser_provider.rs`
2. `crates/config-system/src/root_config_system_container.rs`
