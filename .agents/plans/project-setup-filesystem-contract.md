# Plan: project-setup — filesystem contract

## Goal
Hapus direct dependency project-setup capabilities ke shared I/O utilities. Semua I/O via `IFilesystemAggregate` contract.

## Problem
- `capabilities_setup_processor.rs` line 226: `shared::filesystem::utility_filesystem_io::write_file(filename, content)` — DIRECT I/O

## Capabilities Analysis
| File | Status |
|------|--------|
| `capabilities_setup_processor.rs` | ✗ DIRECT I/O — `write_file` |
| `capabilities_setup_installer_adapter.rs` | ✓ Zero I/O — uses ISetupInstallerProtocol |

---

## Changes

### Change 1: Inject filesystem aggregate ke setup processor
**File**: `crates/project-setup/src/capabilities_setup_processor.rs`

Tambah dependency:
```rust
pub struct SetupManagementProcessor {
    installer: Arc<dyn ISetupInstallerProtocol>,
    filesystem: Arc<dyn IFilesystemAggregate>,  // NEW
}
```

Update constructor:
```rust
pub fn new(installer: Arc<dyn ISetupInstallerProtocol>, filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
    Self { installer, filesystem }
}
```

### Change 2: Ganti direct `write_file` call
**File**: `crates/project-setup/src/capabilities_setup_processor.rs`

Sebelum (line 226):
```rust
shared::filesystem::utility_filesystem_io::write_file(filename, content)
```

Sesudah:
```rust
self.filesystem.write_string(
    std::path::Path::new(filename),
    content,
)
```

### Change 3: Update container wiring
**File**: `crates/project-setup/src/root_project_setup_container.rs`

```rust
pub fn new() -> Self {
    let installer = Arc::new(crate::capabilities_setup_installer_adapter::SetupInstallerAdapter::new());
    let filesystem: Arc<dyn IFilesystemAggregate> = Arc::new(filesystem::FilesystemOrchestrator::new());
    let protocol = Arc::new(
        crate::capabilities_setup_processor::SetupManagementProcessor::new(installer, filesystem)
    );
    // ...
}
```

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p project-setup-lint-arwaky`
2. `cargo nextest run -p project-setup-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/project-setup/src/` — zero

## Files Modified
1. `crates/project-setup/src/capabilities_setup_processor.rs`
2. `crates/project-setup/src/root_project_setup_container.rs`
