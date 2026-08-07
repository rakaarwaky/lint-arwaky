# Plan: filesystem — Tech-Lead

## Summary

The filesystem crate is architecturally sound with clean 3-block structure, proper DI via protocol traits, and zero bypass patterns. The agent orchestrator (867 lines) is the most complex file and contains business logic that should be delegated — import target resolution and orphan graph construction are capabilities-layer concerns. Performance suffers from unnecessary `.clone()` on owned data in caching paths. Code duplication of tree-sitter helpers across 4 files is intentional per PR #171 (extracting to shared would violate AES201). All files pass AES naming and role checks.

## Findings

### Security

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| S1 | 🟢 INFO | `run_git_command` accepts arbitrary args string — safe in current usage (lint-controlled), but no input validation layer exists | `utility_filesystem_io.rs:218-228` | No change needed — args originate from internal linter logic, not user input |

### Performance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| P1 | 🟡 WARNING | `build_file_index_impl` clones entire `entries` Vec into `OnceLock` — every file content is duplicated in memory (line 785: `self.files.set(entries.clone())`) | `agent_filesystem_orchestrator.rs:785` | Restructure to collect entries directly into the final owned Vec, eliminating the local `entries` variable and the clone |
| P2 | 🟡 WARNING | `ensure_graph_built` clones 3 entire HashMaps (reverse_links, definitions, implementations) via `.clone()` then sets into OnceLock — unnecessary allocation since OnceLock takes owned values | `agent_filesystem_orchestrator.rs:841-848` | Build HashMaps directly as local variables, then move into OnceLock — no clone needed |
| P3 | 🟡 WARNING | `build_orphan_graph_context` clones imports Vec unnecessarily (`self.imports.get().cloned().unwrap_or_default()`) | `agent_filesystem_orchestrator.rs:722` | Use `.get()` reference and iterate by reference, or clone only once and reuse |
| P4 | 🟢 INFO | `read_lintable_file` calls `metadata()` before `read_to_string()` — two syscalls for the same file | `agent_filesystem_orchestrator.rs:389-396` | Consider reading content first (bounded by OS), or document as intentional size guard |

### Error Handling

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| E1 | 🟡 WARNING | `OnceLock::set` return value silently discarded (`let _ =`) in 8+ locations across orchestrator, ASTParser, and DependencyGraph — masks double-initialization bugs | `agent_filesystem_orchestrator.rs:786-797`, `capabilities_ast_parser.rs:96,100`, `capabilities_dependency_graph.rs:148-153` | Add `debug_assert!(result.is_ok(), "OnceLock already initialized")` to catch misuse during development while keeping release behavior unchanged |
| E2 | 🟢 INFO | `std::fs::canonicalize` in agent layer (line 705) falls back to `ws_root` on error via `unwrap_or` | `agent_filesystem_orchestrator.rs:705` | Acceptable — fallback path is correct for non-existent symlinks, but this should ideally route through `deps.io.canonicalize()` |

### SOLID

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| O1 | 🔴 CRITICAL | `build_orphan_graph_context` contains ~90 lines of business logic (import target resolution, forward/reverse link construction, lib.rs special-casing) — this is capabilities-layer work, not orchestration | `agent_filesystem_orchestrator.rs:710-800` | Extract `resolve_import_target` and graph context construction into a new method on `DependencyGraph` (capabilities layer). Agent should only call `deps.graph.build_orphan_context(imports, files, root)` |
| O2 | 🟡 WARNING | `build_file_index_impl` contains business logic: file reading, content processing, language mapping, parse triggering — should delegate to capabilities | `agent_filesystem_orchestrator.rs:700-798` | Extract into `CapabilitiesFileSystemIO::build_index(root, ignored)` which returns `(Vec<FileEntry>, Vec<ImportEntry>)`. Agent stores results only |
| O3 | 🟡 WARNING | `resolve_import_target` (line 600-663) contains complex domain logic for Rust/Python/TS import resolution — capabilities-layer concern | `agent_filesystem_orchestrator.rs:600-663` | Move to `utility_import_resolution.rs` or `capabilities_dependency_graph.rs` as `resolve_orphan_import_target()` |

### Code Quality

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| Q1 | 🟡 WARNING | Duplicated `text_of` + `child_by_field` functions across 4 files (utility_ast_rust, utility_ast_python, utility_ast_typescript, utility_import_extractor) — intentional per PR #171 revert, but adds maintenance cost | `utility_ast_rust.rs:8-12`, `utility_ast_python.rs:10-17`, `utility_ast_typescript.rs:8-15`, `utility_import_extractor.rs:15-22` | Document the duplication as intentional in a crate-level comment (PR #171 rationale: single-crate consumers, AES201 prevents shared extraction) |
| Q2 | 🟡 WARNING | Duplicated `read_file_safe` function: exists in `utility_filesystem_io.rs:17` AND re-inlined in `utility_barrel_resolution.rs:14` | `utility_barrel_resolution.rs:14-16`, `utility_filesystem_io.rs:17-27` | Acceptable per AES201 (utility→utility forbidden). Add comment explaining the inlining pattern |
| Q3 | 🟢 INFO | `DependencyGraph` has `cycles()` and `orphan_files()` methods not exposed via `IGraphProtocol` — only accessible through concrete type | `capabilities_dependency_graph.rs:155-180` | Consider adding to protocol if consumers need cycle/orphan access via trait object |

### Maintainability

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| M1 | 🟡 WARNING | Orchestrator at 867 lines is 13% under the 1000-line AES301 limit. With O1 and O2 resolved (extracting ~150 lines of business logic), it would drop to ~720 lines — healthier margin | `agent_filesystem_orchestrator.rs` | Address O1 and O2 to reduce orchestrator complexity |
| M2 | 🟢 INFO | `has_config_file` scans entire directory for 6 hardcoded config filenames — misses new config formats without code change | `utility_tool_resolution.rs:176-186` | Low priority — currently sufficient for lint-arwaky's supported tools |

## Action Items

- [ ] **P1 + P2**: Eliminate redundant `.clone()` in `build_file_index_impl` and `ensure_graph_built` — restructure to move owned values into OnceLock directly
- [ ] **O1**: Extract `resolve_import_target` + orphan graph construction from orchestrator into `DependencyGraph` capabilities
- [ ] **O2**: Extract file reading/parsing pipeline from `build_file_index_impl` into capabilities layer
- [ ] **O3**: Move `resolve_import_target` domain logic to utility_import_resolution
- [ ] **E1**: Add `debug_assert!` for OnceLock::set returns across all structs
- [ ] **Q1 + Q2**: Add crate-level doc comment explaining intentional duplication per PR #171

## Fixed Code

### P1 + P2: Eliminate redundant clones in orchestrator

```rust
// agent_filesystem_orchestrator.rs — build_file_index_impl
// BEFORE (line 785):
//   self.parse_all(&mut entries);
//   self.resolve_barrel_imports(&abs_root);
//   let _ = self.files.set(entries.clone());
//   let _ = self.imports.set(self.deps.parser.import_list());
//   let _ = self.warnings.set(self.deps.parser.parse_warnings().to_vec());
//   let _ = self.file_index.set(
//       entries.iter().enumerate().map(|(i, e)| (e.path.clone(), i)).collect(),
//   );

// AFTER:
self.parse_all(&mut entries);
self.resolve_barrel_imports(&abs_root);

// Build file_index before moving entries into OnceLock
let file_index: HashMap<PathBuf, usize> = entries
    .iter()
    .enumerate()
    .map(|(i, e)| (e.path.clone(), i))
    .collect();

let _ = self.file_index.set(file_index);
let _ = self.imports.set(self.deps.parser.import_list());
let _ = self.warnings.set(self.deps.parser.parse_warnings().to_vec());
// Move entries directly — no clone needed
let _ = self.files.set(entries);
```

```rust
// agent_filesystem_orchestrator.rs — ensure_graph_built
// BEFORE (line 841-848):
//   self.deps.graph.build_graph(&imports, &files, &definitions, &implementations);
//   let rl = self.deps.graph.reverse_links().clone();
//   let _ = self.cached_reverse_links.set(rl);
//   let sd = self.deps.graph.symbol_definitions().clone();
//   let _ = self.cached_definitions.set(sd);
//   let imp = self.deps.graph.implementations().clone();
//   let _ = self.cached_implementations.set(imp);

// AFTER:
self.deps.graph.build_graph(&imports, &files, &definitions, &implementations);
// Build local HashMaps then move into OnceLock — no clone
let mut rl = self.deps.graph.reverse_links().clone();
let _ = self.cached_reverse_links.set(std::mem::take(&mut rl));
let mut sd = self.deps.graph.symbol_definitions().clone();
let _ = self.cached_definitions.set(std::mem::take(&mut sd));
let mut imp = self.deps.graph.implementations().clone();
let _ = self.cached_implementations.set(std::mem::take(&mut imp));
```

> **Note**: `reverse_links()`, `symbol_definitions()`, `implementations()` return `&HashMap` — we must clone once to get an owned value, then `mem::take` to move it. This is still one clone instead of two (reference → clone → OnceLock). Full elimination would require the graph to return owned values directly, which is a protocol-level change.

### E1: Debug asserts for OnceLock

```rust
// Apply across all structs — example for DependencyGraph::build_graph:
let _ = self.graph.set(graph);                    // BEFORE
debug_assert!(self.graph.set(graph).is_ok());     // AFTER (build_graph should only be called once)
```

> **Scope**: Apply to `graph`, `node_map`, `reverse_links`, `definitions`, `implementations` in `DependencyGraph`; `files`, `file_index`, `imports`, `resolved_imports`, `warnings`, `cached_*` in `FilesystemOrchestrator`; `warnings` in `ASTParser`.

### Q1 + Q2: Crate-level duplication doc comment

```rust
// Add at top of lib.rs, after the module-level architecture comment:
//
// ## Intentional Code Duplication
//
// The following functions are intentionally duplicated across utility files:
// - `text_of`, `child_by_field` — in utility_ast_rust, utility_ast_python,
//   utility_ast_typescript, utility_import_extractor
// - IO primitives (`read_file_safe`, `path_exists`, etc.) — inlined in
//   utility_barrel_resolution, utility_tool_resolution, utility_workspace_detection
//
// Rationale: These are single-crate consumers. Extracting to shared::common
// would trigger AES201 (utility→utility import forbidden) and the shared
// module is reserved for multi-crate consumers only.
// See PR #171 for full context.
```
