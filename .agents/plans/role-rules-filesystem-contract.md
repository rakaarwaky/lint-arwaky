# Plan: role-rules — filesystem contract

## Goal
Hapus direct dependency role-rules ke shared I/O utilities. Agent pakai `IFilesystemAggregate` contract. Capabilities zero I/O tetap.

## Problem
- `agent_role_orchestrator.rs` line 176: `shared::filesystem::utility_filesystem_io::is_path_ignored` — DIRECT I/O
- `agent_role_orchestrator.rs` line 16: sudah pakai `IFilesystemAggregate` contract ✓

## Capabilities Analysis (all zero I/O ✓)
| File | Status |
|------|--------|
| `capabilities_taxonomy_role_auditor.rs` | ✓ Zero I/O |
| `capabilities_contract_role_auditor.rs` | ✓ Zero I/O |
| `capabilities_utility_role_auditor.rs` | ✓ Zero I/O |
| `capabilities_capabilities_role_auditor.rs` | ✓ Zero I/O |
| `capabilities_agent_role_auditor.rs` | ✓ Zero I/O |
| `capabilities_surface_role_auditor.rs` | ✓ Zero I/O |

---

## Changes

### Change 1: Agent — ganti `is_path_ignored` ke contract
**File**: `crates/role-rules/src/agent_role_orchestrator.rs`

Sebelum (line 176):
```rust
shared::filesystem::utility_filesystem_io::is_path_ignored(path, &self.ignored_paths)
```

Sesudah:
```rust
self.deps.filesystem.should_ignore(path, &self.ignored_paths)
```

### Change 2: Hapus direct import
**File**: `crates/role-rules/src/agent_role_orchestrator.rs`

Tidak perlu hapus — `is_path_ignored` dipanggil inline, bukan via import statement. Cukup ganti panggilannya.

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p role-rules-lint-arwaky`
2. `cargo nextest run -p role-rules-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/role-rules/src/` — zero

## Files Modified
1. `crates/role-rules/src/agent_role_orchestrator.rs` — ganti 1 line
