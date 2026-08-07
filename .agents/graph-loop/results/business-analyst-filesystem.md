# Plan: filesystem — Business-Analyst

## Summary

The filesystem crate is a well-structured feature crate with 5 FRs mapped 1:1 to protocol traits (IParserProtocol, IGraphProtocol, IFileSystemIOProtocol, IToolResolutionProtocol, IWorkspaceProtocol), each implemented by a corresponding capability struct, and composed by a single aggregate (IFilesystemAggregate = 80 methods). The architecture follows AES conventions: 3-block structure, DI via protocol traits, orchestrator is pure delegation, utility files are stateless. No history in features.json (LOCKED, iteration 0). The FRD hash is consistent. However, 5 utility files contain duplicated helper functions (`text_of`, `child_by_field`, `extract_use_path`, `extract_scoped_path`, `extract_js_string_child`) that need consolidation. Additionally, 2 utility files inline their own I/O helpers instead of reusing `utility_filesystem_io`. These are the primary issues to address.

## Findings

### Requirements Clarity

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| RC-1 | 🟢 INFO | FRD API Contract section shows 62 methods for IFileSystemIOProtocol but actual trait has 29 (FRD table lists 29 correctly — the section count is inconsistent) | FRD.md API Contract section | Update FRD API Contract method count to match: 6+7+29+12+8+18 = 80 |
| RC-2 | 🟢 INFO | FRD NFR says "pipeline processes 1,000 files in < 2s" but Test Scenario FR-001 #9 says "Completes in < 1s" — conflicting performance targets | FRD.md NFR + Test Scenarios | Harmonize: either both say < 1s or both say < 2s |

### Business Flow

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| BF-1 | 🟢 INFO | ~~`build_file_index_impl` barrel resolution only updates parser's internal copy~~ **RETRACTED**: Verified the code calls `parser.import_list()` AFTER `resolve_barrel_imports`, so the orchestrator's `imports` OnceLock correctly receives barrel-resolved imports | `agent_filesystem_orchestrator.rs` lines 716-720 | No action — code is correct |
| BF-2 | 🟡 WARNING | `build_orphan_graph_context` re-resolves imports inline using `resolve_import_target` instead of going through the barrel resolution pipeline, creating a second independent resolution path | `agent_filesystem_orchestrator.rs` lines 500-590 | Centralize import resolution: either use barrel-resolved imports from `resolved_import_list()` or make `resolve_import_target` a single canonical path |
| BF-3 | 🟡 WARNING | `build_orphan_graph_context` creates fake edges between all `lib.rs` files unconditionally (`for lib in &lib_rs_files { for other_lib in &lib_rs_files { ... } }`), which pollutes the dependency graph with phantom edges | `agent_filesystem_orchestrator.rs` lines 570-577 | Remove the lib.rs cross-linking or gate it behind an explicit flag, as it creates false cycles in Kosaraju SCC |

### Logic Implementation

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| LI-1 | 🟡 WARNING | `extract_grouped_use_names` handles nested braces by recursing but the `split_top_level_commas` function uses `depth` tracking that only works for single-level nesting — deeply nested `use foo::{Bar::{Baz::{Qux}, Quux}}` may split incorrectly | `utility_import_extractor.rs` lines 320-360 | Add test for 3-level nested grouped imports and verify edge case |
| LI-2 | 🟡 WARNING | `resolve_python_relative_import` only handles `.` (current dir) and `..` (parent dir) but not `...` (grandparent) or deeper relative imports | `utility_barrel_resolution.rs` lines 85-110 | Extend dot-count logic: `dot_count >= 2` should walk up `dot_count - 1` directories, not just 1 |
| LI-3 | 🟢 INFO | `read_lintable_file` in the orchestrator returns `None` for files > 2MiB — but `utility_filesystem_io::read_lintable_file` also exists with the same logic, creating two independent implementations | `agent_filesystem_orchestrator.rs` line 687 + `utility_filesystem_io.rs` line 44 | Consolidate: orchestrator should delegate to `deps.io` or use utility function directly |

### Testability & Acceptance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| TA-1 | 🟢 INFO | Memory records indicate 160 tests exist — all passing. No testability gaps identified in code structure | — | No action needed |
| TA-2 | 🟢 INFO | FRD test scenarios cover happy paths and edge cases for all 5 FRs — traceable to implementation | — | No action needed |

### Traceability (FRD→Code)

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| TR-1 | 🟡 WARNING | FRD documents `utility_import_resolution` module but `lib.rs` has `pub mod utility_import_resolution;` without a comment mapping it to a specific FR (line 25). All other utility modules have FR comments | `lib.rs` line 25 | Add comment: `// FR-001: External crate/package import resolution` |
| TR-2 | 🟢 INFO | FRD claims aggregate has 80 methods; actual count is ~80 (6+7+29+12+8+18) — consistent with contract trait | `contract_filesystem_aggregate.rs` | No action needed |
| TR-3 | 🟢 INFO | 1 FR = 1 protocol = 1 capability mapping is correctly maintained across all 5 FRs | `lib.rs` + contracts | No action needed |

## Violations (AES Rules)

| # | Rule | Severity | Issue | Location |
|---|------|----------|-------|----------|
| V-1 | AES305 (Duplication) | MEDIUM | `text_of()` duplicated in 4 files: `utility_import_extractor.rs`, `utility_ast_rust.rs`, `utility_ast_python.rs`, `utility_ast_typescript.rs` | See files |
| V-2 | AES305 (Duplication) | MEDIUM | `child_by_field()` duplicated in 4 files: `utility_import_extractor.rs`, `utility_ast_rust.rs`, `utility_ast_python.rs`, `utility_ast_typescript.rs` | See files |
| V-3 | AES305 (Duplication) | MEDIUM | `extract_use_path()` + `extract_scoped_path()` duplicated in 2 files: `utility_import_extractor.rs` + `utility_ast_rust.rs` | See files |
| V-4 | AES305 (Duplication) | MEDIUM | `extract_js_string_child()` duplicated in 2 files: `utility_import_extractor.rs` + `utility_ast_typescript.rs` | See files |
| V-5 | AES201/Architecture | LOW | `utility_barrel_resolution.rs` and `utility_import_resolution.rs` inline their own I/O functions (`read_file_safe`, `path_exists`, `is_file`, `canonicalize_path`, `scan_directory`) instead of reusing `utility_filesystem_io` — caused by AES201 preventing utility→utility imports | See files |

## Action Items

- [ ] **HIGH** Consolidate duplicated helper functions: move `text_of`, `child_by_field`, `extract_use_path`, `extract_scoped_path`, `extract_js_string_child` to shared taxonomy or utility module, eliminating AES305 violations (V-1 through V-4)
- [ ] **MEDIUM** Remove unconditional lib.rs cross-linking in `build_orphan_graph_context` or gate behind a flag to prevent phantom graph edges (BF-3)
- [ ] **MEDIUM** Fix `resolve_python_relative_import` to handle `...` and deeper relative imports (LI-2)
- [ ] **MEDIUM** Verify `split_top_level_commas` handles 3+ level nested grouped imports (LI-1)
- [ ] **LOW** Add FR comment to `pub mod utility_import_resolution;` in `lib.rs` (TR-1)
- [ ] **LOW** Update FRD method count in API Contract section and harmonize performance targets (RC-1, RC-2)
- [ ] **LOW** Consolidate inline I/O helpers in `utility_barrel_resolution` and `utility_import_resolution` into a pattern that avoids AES201 violations while reducing duplication (V-5)

## Fixed Code

### BF-1: Barrel resolution inconsistency in `agent_filesystem_orchestrator.rs`

The orchestrator stores its own copy of imports, but barrel resolution only updates the parser's copy. Fix by re-fetching after barrel resolution.

**File:** `crates/filesystem/src/agent_filesystem_orchestrator.rs`

```rust
// In build_file_index_impl, AFTER resolve_barrel_imports:
self.parse_all(&mut entries);
self.resolve_barrel_imports(&abs_root);
let _ = self.files.set(entries.clone());
// FIX: Re-fetch imports AFTER barrel resolution to get resolved versions
let _ = self.imports.set(self.deps.parser.import_list());
let _ = self
    .warnings
    .set(self.deps.parser.parse_warnings().to_vec());
```

This is actually already correct in the current code (lines 716-720). The `parser.import_list()` IS called after `resolve_barrel_imports`. **Retracted** — no fix needed for BF-1.

### V-1 through V-4: Consolidate duplicated AST helpers

Create a single shared helper module for the 5 duplicated functions. Since utility files cannot import other utility files (AES201), the correct approach per memory feedback is to consolidate into the primary consumer.

**Approach:** The `text_of` and `child_by_field` functions are tree-sitter primitives. The cleanest fix is to have `utility_ast_rust`, `utility_ast_python`, `utility_ast_typescript` each re-export the shared helpers via a macro or inline definition. However, since the project has the AES201 constraint, the pragmatic fix is:

1. Remove `text_of`/`child_by_field`/`extract_use_path`/`extract_scoped_path` from `utility_import_extractor.rs`
2. Have `utility_import_extractor.rs` call the public `extract_rust_metadata`/`extract_python_metadata`/`extract_ts_metadata` functions from the AST utility files instead of reimplementing tree-sitter traversal
3. This eliminates 4 duplicated functions from `utility_import_extractor.rs`

**Note:** Full consolidation requires a larger refactor that the developer agent should validate. The above is the recommendation direction.
