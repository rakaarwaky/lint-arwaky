# Plan: code-analysis — filesystem contract

## Goal
Hapus direct dependency code-analysis ke shared I/O utilities. Agent dan capabilities pakai `IFilesystemAggregate` contract.

## Problem
- `agent_code_analysis_orchestrator.rs` line 243,274: `shared::filesystem::utility_filesystem_io::read_lintable_file` — DIRECT I/O
- `agent_code_analysis_orchestrator.rs` line 191,213,348: `shared::code_analysis::utility_target_resolver::detect_source_dir` dan `collect_source_files` — shared utilities yang internally call filesystem
- `capabilities_code_duplication_analyzer.rs` line 39,70: `shared::code_analysis::utility_target_resolver::detect_source_dir` dan `collect_source_files` — **DIRECT I/O di capabilities!**

## Capabilities Analysis
| File | Status |
|------|--------|
| `capabilities_code_duplication_analyzer.rs` | ✗ DIRECT I/O — `detect_source_dir`, `collect_source_files` |
| `capabilities_check_bypass_checker.rs` | ✓ Zero I/O |
| `capabilities_line_checker.rs` | ✓ Zero I/O |
| `capabilities_mandatory_definition_checker.rs` | ✓ Zero I/O |
| `capabilities_dead_inheritance_checker.rs` | ✓ Zero I/O |

---

## Changes

### Change 1: Agent — gunakan contract untuk read_lintable_file
**File**: `crates/code-analysis/src/agent_code_analysis_orchestrator.rs`

Hapus:
```rust
- shared::filesystem::utility_filesystem_io::read_lintable_file
```

Tambah `filesystem: Arc<dyn IFilesystemAggregate>` ke deps.

Ganti:
```rust
// Sebelum:
shared::filesystem::utility_filesystem_io::read_lintable_file(file)
// Sesudah:
self.deps.filesystem.read_lintable_file(file)
```

### Change 2: Agent — gunakan contract untuk source dir detection
**File**: `crates/code-analysis/src/agent_code_analysis_orchestrator.rs`

`utility_target_resolver::detect_source_dir` dan `collect_source_files` adalah shared internal yang sudah pakai filesystem. Bisa keep sebagai shared internal ATAU agent panggil contract langsung.

**Opsi A (keep shared internal)**: Tidak perlu ubah — `utility_target_resolver` sudah indirectly pakai filesystem.
**Opsi B (pindah ke agent)**: Agent pakai `self.deps.filesystem.discover_files()` langsung.

### Change 3: Capabilities — inject filesystem aggregate
**File**: `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs`

Tambah `filesystem: Arc<dyn IFilesystemAggregate>` ke struct.

Ganti:
```rust
// Sebelum:
shared::code_analysis::utility_target_resolver::detect_source_dir(root)
shared::code_analysis::utility_target_resolver::collect_source_files(...)
// Sesudah:
// Agent pre-collect files, pass ke capabilities
```

Atau: capabilities terima `&[FileEntry]` dari agent, tidak perlu filesystem access.

### Change 4: Container — inject filesystem aggregate
**File**: `crates/code-analysis/src/root_code_analysis_container.rs`

---

## Verification
1. `CARGO_INCREMENTAL=0 cargo check -p code-analysis-lint-arwaky`
2. `cargo nextest run -p code-analysis-lint-arwaky`
3. `grep -r "utility_filesystem_io" crates/code-analysis/src/` — zero

## Files Modified
1. `crates/shared/src/filesystem/contract_filesystem_aggregate.rs` — add `read_lintable_file()`
2. `crates/code-analysis/src/agent_code_analysis_orchestrator.rs`
3. `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs`
4. `crates/code-analysis/src/root_code_analysis_container.rs`
