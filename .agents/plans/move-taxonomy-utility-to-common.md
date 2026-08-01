# Plan: Move Cross-Cutting Taxonomy & Utility to common/

## Problem
Taxonomy VOs and utility functions used by 2+ crates are scattered across feature folders. They should live in `common/` for proper AES compliance.

---

## Duplicate Resolution Required

### 1. FileEntry (TWO different types!)
- **tui/taxonomy_file_entry_vo.rs**: UI display type (`name`, `full_path`, `is_dir`, `layer`, `violation_count`, `extension`, `size_bytes`)
- **filesystem/taxonomy_filesystem_vo.rs**: File metadata type (`path`, `extension`, `language`, `size`, `content`, `parse_ok`, `parse_metadata`)

**Fix**: Rename tui's to `FileEntryDisplay`, keep filesystem's as canonical `FileEntry` in common.

### 2. Language (TWO identical enums)
- **filesystem/taxonomy_filesystem_vo.rs**: `Rust, Python, TypeScript, JavaScript` + `from_extension()`
- **code-analysis/taxonomy_violation_code_analysis_vo.rs**: `Rust, JavaScript, Python, TypeScript` + `from_adapter_name()`

**Fix**: Consolidate into `common/taxonomy_language_vo.rs` with both methods.

---

## Move List

### Taxonomy → common/ (13 files)

| # | Source | File | Key Types | Consumers |
|---|--------|------|-----------|-----------|
| 1 | cli-commands | `taxonomy_result_vo.rs` | `LintResult` | 9 crates |
| 2 | filesystem | `taxonomy_filesystem_vo.rs` | `FileEntry`, `Language`, `ParseMetadata`, etc. | 7 crates |
| 3 | config-system | `taxonomy_config_vo.rs` | `ArchitectureConfig` | 6 crates |
| 4 | cli-commands | `taxonomy_format_vo.rs` | `Format` | 5 crates |
| 5 | file-watch | `taxonomy_watch_config_vo.rs` | `WatchConfig` | 3 crates |
| 6 | orphan-detector | `taxonomy_orphan_contract_vo.rs` | `OrphanFileListVO` | 2 crates |
| 7 | maintenance | `taxonomy_doctor_vo.rs` | `ToolchainDiagnostics`, `DependencyReport` | 2 crates |
| 8 | config-system | `taxonomy_source_vo.rs` | `ConfigResult` | 2 crates |
| 9 | config-system | `taxonomy_config_language_vo.rs` | `ConfigLanguage` | 2 crates |
| 10 | cli-commands | `taxonomy_protocol_vo.rs` | `TransportProtocol` | 2 crates |
| 11 | auto-fix | `taxonomy_fix_vo.rs` | `FixResult` | 2 crates |
| 12 | tui | `taxonomy_file_entry_vo.rs` | `FileEntryDisplay` (renamed) | 2 crates |
| 13 | code-analysis | `taxonomy_violation_code_analysis_vo.rs` | `Language` (consolidated) | 2 crates |

### Utility → common/ (consolidation)

| # | Source | Functions to Consolidate | Consumers |
|---|--------|-------------------------|-----------|
| 1 | filesystem/utility_filesystem_io.rs | `is_dir`, `is_file`, `path_exists`, `read_file`, `write_file`, `scan_directory`, `find_workspace_root`, `detect_languages` | 5+ crates each |
| 2 | code-analysis/utility_code_duplication_detector.rs | `collect_file_entries`, `scan_duplicate_blocks`, `build_violations` | 3-4 crates |
| 3 | naming-rules/utility_naming_checker.rs | `layer_keys` | 3 crates |
| 4 | orphan-detector/utility_orphan_filename.rs | `file_stem` | 3 crates |

---

## Execution Order

1. **Resolve duplicates first** (FileEntry, Language)
2. **Move taxonomy files** to common/
3. **Update all import paths** in consuming crates
4. **Consolidate utility functions** to common/
5. **Verify compilation**

## Files Modified
- `crates/shared/src/common/mod.rs` — add new pub mod declarations
- `crates/shared/src/common/taxonomy_*.rs` — new files (moved from feature folders)
- `crates/shared/src/{feature}/mod.rs` — remove moved pub mod + add re-exports
- All consuming crates — update import paths

## Verification
1. `CARGO_INCREMENTAL=0 cargo check --workspace`
2. `grep -r 'pub struct FileEntry' crates/shared/src/` — only in common
3. `grep -r 'pub enum Language' crates/shared/src/` — only in common
