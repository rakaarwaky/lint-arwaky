# Plan: shared — Tech-Lead

## Summary

The `shared` crate is the foundation layer (180 files, ~8,300 LOC) providing taxonomy VOs, contract traits, and utility functions consumed by every feature crate. The crate is architecturally sound in its zero-bypass policy and consistent macro usage, but has significant structural debt: 5 contract traits violate SRP (10+ methods each), ~30 raw primitive usages in contract signatures violate AES402, 2 files contain async code conflicting with the no-async project rule, and several taxonomy files contain behavioral logic that should be in capabilities or utility layers. The highest-priority fixes are the async anomaly (project rule violation) and the `IFileSystemIOProtocol` SRP violation (28 methods in one trait).

---

## Findings

### Security

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟢 INFO | `utility_command_runner.rs` executes shell commands — safe because it takes `name` and `args` separately (no shell injection) | `src/common/utility_command_runner.rs` | No action needed — already safe pattern |

**No critical security vulnerabilities found.** No credential leaks, no injection vectors, no path traversal risks. The command runner uses `tokio::process::Command` (not shell string interpolation).

---

### Performance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 2 | 🟢 INFO | `taxonomy_common_vo.rs` at 721 lines — largest file but contains 16+ separate types, no single hot function | `src/common/taxonomy_common_vo.rs` | Consider splitting into domain-grouped files (severity, path, lint) for maintainability |
| 3 | 🟢 INFO | `InboundLinkMap::get_importers()` (~60 lines) tries 4 fallback strategies sequentially — acceptable since the file set is bounded | `src/filesystem/taxonomy_filesystem_vo.rs` | No action needed — strategies are ordered by cost |

**No O(n²) algorithms, no unnecessary allocations, no performance bottlenecks detected.** All functions operate on bounded collections with linear complexity.

---

### Error Handling

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 4 | 🟢 INFO | All 180 files have **zero** `unwrap()`, `expect()`, `panic!()`, `todo!()`, `unreachable!()`, or `#[allow(...)]` — exemplary | All files | Maintain this standard |

**Error handling is excellent across the entire crate.** All fallible paths use `unwrap_or` / `unwrap_or_default` / `unwrap_or_else` with safe defaults. No bypass patterns detected. This is the strongest dimension of the codebase.

---

### SOLID

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 5 | 🟡 WARNING | **`IFileSystemIOProtocol` has 28 methods** — mixes path ops (15), dir ops (3), file I/O (7), process exec (3), timing (1) | `src/filesystem/contract_filesystem_io_protocol.rs` | Split into 4 focused traits: `IPathOpsProtocol` (15), `IDirOpsProtocol` (3), `IFileReadWriteProtocol` (7), `IProcessExecProtocol` (3+timing) |
| 6 | 🟡 WARNING | **`IFilesystemAggregate` has 18 methods** — mixes cache access, file discovery, graph building, workspace queries | `src/filesystem/contract_filesystem_aggregate.rs` | Split into `IFileCacheAggregate` (cache reads), `IFileDiscoveryAggregate` (walk + discover), `IGraphBuilderAggregate` (graph ops) |
| 7 | 🟡 WARNING | **`SetupManagementAggregate` has 19 methods, `ISetupManagementProtocol` has 17 methods** — mixes MCP config generation (7), adapter installation, language detection, file I/O, HTTP checks | `src/project_setup/contract_setup_aggregate.rs`, `contract_setup_protocol.rs` | Split into `IMcpConfigProtocol` (7 config methods), `IAdapterInstallerProtocol`, `ILanguageDetectionProtocol`, `IConfigFileManagerProtocol` |
| 8 | 🟡 WARNING | **`IToolResolutionProtocol` has 13 methods** — mixes JS, Cargo, Python, and general PATH resolution | `src/filesystem/contract_tool_resolution_protocol.rs` | Split into `IJsToolResolutionProtocol`, `ICargoToolResolutionProtocol`, `IPathResolutionProtocol` |
| 9 | 🟡 WARNING | **`AppState` in taxonomy has 15+ behavioral mutation methods** — UI navigation, scroll, search filtering, scan progress are capabilities-level behavior | `src/tui/taxonomy_state_vo.rs` | Split into `AppStateData` (pure VO with fields only) and move behavioral methods to tui capabilities layer |
| 10 | 🟡 WARNING | **`IdentifierVisitor` + `extract_idents_from_stream()` in taxonomy** — implements `syn::visit::Visit` and recursive token-stream parsing, which is parser implementation (capabilities behavior) | `src/orphan_rules/taxonomy_orphan_parse_result_vo.rs` | Move `IdentifierVisitor` and `extract_idents_from_stream()` to `orphan-rules` crate as a utility or capabilities function |
| 11 | 🟡 WARNING | **`InboundLinkMap::get_importers()` in taxonomy** — ~60 lines of multi-strategy lookup logic is behavioral, not a domain type | `src/filesystem/taxonomy_filesystem_vo.rs` | Move `get_importers()` to a utility function in `filesystem` crate or `shared::common::utility_*.rs` |
| 12 | 🟡 WARNING | **`GitHooksAggregate` has 6 of 8 methods with default bodies in the trait** — blurs contract/capabilities boundary | `src/git_hooks/contract_git_hooks_aggregate.rs` | Move default implementations to a helper struct in capabilities layer; keep trait as pure signatures |

---

### Code Quality

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 13 | 🟡 WARNING | **`#[async_trait]` + `tokio` in `IWatchProviderProtocol`** — project rule says "no async runtime (std::thread/rayon, not tokio)" but this contract uses `async fn` and `tokio::sync::broadcast` | `src/file_watch/contract_provider_protocol.rs` | Either (a) document this as an approved exception for file-watch I/O, or (b) refactor to sync `start()`/`stop()` with a `std::thread::spawn` channel internally |
| 14 | 🟡 WARNING | **`utility_command_runner.rs` has 2 `pub async fn` using `tokio::process::Command`** — introduces tokio into the lowest-level crate | `src/common/utility_command_runner.rs` | Remove async variants or move to a crate that already depends on tokio (e.g., `file-watch`). The sync variants (`run_command`, `run_command_in_dir`) remain. |
| 15 | 🟡 WARNING | **`HashMap<String, String>` as `content_map` in 6+ contract signatures** — raw primitives instead of VOs | `src/import_rules/contract_*.rs`, `src/orphan_rules/contract_orphan_protocol.rs` | Create `ContentMap` VO (wraps `HashMap<String, String>`) in `taxonomy_common_vo.rs` and use in all contract signatures |
| 16 | 🟡 WARNING | **`&[String]` for `ignored` parameter in 4+ contracts** — raw `String` slice instead of `PatternList` | `src/filesystem/contract_filesystem_aggregate.rs`, `src/orphan_rules/contract_orphan_aggregate.rs` | Use `&PatternList` (already exists in taxonomy) |
| 17 | 🟡 WARNING | **`file: &str` / `content: &str` in 6 quality_rules contracts** — raw primitives instead of `&FilePath` / `&ContentString` | `src/quality_rules/contract_bypass_checker_protocol.rs`, `contract_class_protocol.rs`, `contract_dead_inheritance_protocol.rs`, `contract_line_protocol.rs`, `src/orphan_rules/contract_orphan_parser_protocol.rs` | Use `&FilePath` and `&ContentString` VOs |
| 18 | 🟡 WARNING | **`layer: &str` in role_rules contracts** — raw string instead of VO | `src/role_rules/contract_agent_role_protocol.rs`, `contract_capabilities_role_protocol.rs` | Use `&Identity` or `&LayerNameVO` VO |
| 19 | 🟢 INFO | **No-op functions in `utility_path_normalization.rs`** — both `normalize_path` and `resolve_capabilities_path` are identity stubs | `src/common/utility_path_normalization.rs` | Remove the file or add doc comments explaining they are intentional placeholders |
| 20 | 🟢 INFO | **`is_generator_enabled()` always returns `true`** — dead feature-flag stub | `src/common/utility_value_object_generator.rs` | Remove or document as intentional |
| 21 | 🟢 INFO | **Strategy 5 dead code in `detect_module_layer`** — loop body only calls `continue`, never returns | `src/common/utility_layer_detector.rs` | Remove Strategy 5 block (lines ~124-138) and deduplicate with Strategies 2-4 |
| 22 | 🟢 INFO | **Code duplication in `extract_typescript_method_signatures`** — `has_primitive` check block duplicated verbatim in two places | `src/common/utility_signature_parser.rs` | Extract into a helper function `check_primitive_in_context()` |
| 23 | 🟢 INFO | **`resolve_module_path_to_layer` does `std::fs::read_dir`** — I/O in utility layer violates purity | `src/common/utility_layer_detector.rs` | Move to `filesystem` crate or accept a `&[FileEntry]` parameter instead |
| 24 | 🟢 INFO | **`FileEntry::from_path()` calls `path.metadata()`** — filesystem I/O in taxonomy layer | `src/tui/taxonomy_file_entry_vo.rs` | Move `from_path()` to tui capabilities layer |
| 25 | 🟢 INFO | **`WatchEvent::new()` calls `SystemTime::now()`** — system clock I/O in taxonomy layer | `src/file_watch/taxonomy_watch_event_vo.rs` | Accept `timestamp: Timestamp` parameter instead of calling system clock |

---

### Maintainability

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 26 | 🟢 INFO | **`taxonomy_filesystem_vo.rs` at 498 lines with 35+ types** — too many types in one file | `src/filesystem/taxonomy_filesystem_vo.rs` | Split into: `taxonomy_file_entry_vo.rs`, `taxonomy_parse_metadata_vo.rs`, `taxonomy_import_vo.rs`, `taxonomy_graph_vo.rs`, `taxonomy_scan_config_vo.rs` |
| 27 | 🟢 INFO | **`taxonomy_common_vo.rs` at 721 lines with 16+ types** — exceeds 500-line readability threshold | `src/common/taxonomy_common_vo.rs` | Split into domain groups: `taxonomy_severity_vo.rs` (Score, Severity, Threshold), `taxonomy_layout_vo.rs` (LineNumber, ColumnNumber, Count), `taxonomy_pattern_vo.rs` (PatternList, SuffixVO, LanguageVO) |
| 28 | 🟢 INFO | **Taxonomy VOs defined in contract files** — `WorkspaceType` in `contract_workspace_detector_protocol.rs`, `ToolOutput` in `contract_tool_executor_protocol.rs`, `PackageManagerStatus` in `contract_setup_protocol.rs` | Various contract files | Move these types to their respective `taxonomy_*.rs` files |
| 29 | 🟢 INFO | **`utility_signature_parser.rs` at 390 lines, 7 public functions** — combines Rust/Python/TS signature parsers plus cross-cutting checks | `src/common/utility_signature_parser.rs` | Split into: `utility_rust_signature_parser.rs`, `utility_python_signature_parser.rs`, `utility_ts_signature_parser.rs`, keep `signature_uses_forbidden_primitive` as a cross-cutting utility |
| 30 | 🟢 INFO | **`is_path_ignored` at 90 lines** — single function handles 6+ distinct glob pattern types | `src/common/utility_path_filter.rs` | Extract each pattern type into a helper function |

---

## Action Items

- [ ] **CRITICAL** [#13] Reconcile `async_trait`/`tokio` usage with no-async project rule — document exception or refactor
- [ ] **CRITICAL** [#14] Remove async functions from `utility_command_runner.rs` or move to tokio-aware crate
- [ ] **HIGH** [#5] Split `IFileSystemIOProtocol` (28 methods) into 4 focused protocol traits
- [ ] **HIGH** [#6] Split `IFilesystemAggregate` (18 methods) into 3 focused aggregate traits
- [ ] **HIGH** [#7] Split `SetupManagementAggregate`/`ISetupManagementProtocol` (17-19 methods) into 4 focused traits
- [ ] **HIGH** [#15] Create `ContentMap` VO and replace `HashMap<String, String>` in 6+ contract signatures
- [ ] **MEDIUM** [#10] Move `IdentifierVisitor` from taxonomy to orphan-rules crate
- [ ] **MEDIUM** [#11] Move `InboundLinkMap::get_importers()` to utility/capabilities
- [ ] **MEDIUM** [#9] Split `AppState` behavioral methods to tui capabilities
- [ ] **MEDIUM** [#8] Split `IToolResolutionProtocol` (13 methods)
- [ ] **MEDIUM** [#16] Replace `&[String]` with `&PatternList` in 4+ contracts
- [ ] **MEDIUM** [#17] Replace `file: &str`/`content: &str` with `&FilePath`/`&ContentString` in quality_rules contracts
- [ ] **MEDIUM** [#18] Replace `layer: &str` with `&Identity` in role_rules contracts
- [ ] **LOW** [#12] Move default method bodies out of `GitHooksAggregate` trait
- [ ] **LOW** [#19-25] Remove dead code, deduplicate helpers, move I/O out of taxonomy/utility
- [ ] **LOW** [#26-30] Split large files (filesystem_vo, common_vo, signature_parser), move VOs from contract files

---

## Fixed Code

### Fix #14: Remove async from utility_command_runner.rs

```rust
// src/common/utility_command_runner.rs
// REMOVE these two functions entirely:
// - pub async fn run_command_async(...)
// - pub async fn run_command_in_dir_async(...)
// Keep only the sync variants: run_command, run_command_in_dir
```

**Rationale:** The shared crate is the lowest-level crate. Introducing tokio here propagates the async runtime to all consumers. The sync variants are sufficient — consumers that need async can spawn on a thread.

### Fix #13: Refactor IWatchProviderProtocol to sync

```rust
// src/file_watch/contract_provider_protocol.rs
// BEFORE:
#[async_trait::async_trait]
pub trait IWatchProviderProtocol: Send + Sync {
    async fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError>;
    async fn stop(&self) -> Result<(), WatchServiceError>;
    async fn is_available(&self) -> BooleanVO;
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<WatchEvent>;
}

// AFTER:
pub trait IWatchProviderProtocol: Send + Sync {
    fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError>;
    fn stop(&self) -> Result<(), WatchServiceError>;
    fn is_available(&self) -> BooleanVO;
    fn subscribe(&self) -> std::sync::mpsc::Receiver<WatchEvent>;
}
```

**Rationale:** If async is truly needed for file-watch, document it as an approved exception in ARCHITECTURE.md. Otherwise, sync with `std::thread` + `mpsc` aligns with the project's concurrency model.

### Fix #15: Create ContentMap VO

```rust
// Add to src/common/taxonomy_common_vo.rs or a new taxonomy_common_collection_vo.rs
use std::collections::HashMap;

/// Map of file path to file content — used across import-rules and orphan-rules contracts.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentMap(pub HashMap<String, String>);

impl ContentMap {
    pub fn new() -> Self { Self::default() }
    pub fn get(&self, path: &str) -> Option<&String> { self.0.get(path) }
    pub fn insert(&mut self, path: String, content: String) { self.0.insert(path, content); }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn keys(&self) -> impl Iterator<Item = &String> { self.0.keys() }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> { self.0.iter() }
}
```

### Fix #5: Split IFileSystemIOProtocol

```rust
// BEFORE: 1 trait, 28 methods in contract_filesystem_io_protocol.rs
// AFTER: 4 focused traits

pub trait IPathOpsProtocol: Send + Sync {
    fn path_exists(&self, path: &Path) -> bool;
    fn is_dir(&self, path: &Path) -> bool;
    fn is_file(&self, path: &Path) -> bool;
    fn should_ignore(&self, path: &Path, ignored: &PatternList) -> bool;
    fn canonicalize(&self, path: &Path) -> Option<PathBuf>;
    fn is_symlink(&self, path: &Path) -> bool;
    fn metadata(&self, path: &Path) -> Option<std::fs::Metadata>;
    fn is_source_file(&self, path: &Path) -> bool;
    fn get_file_stem(&self, path: &Path) -> String;
    fn get_basename(&self, path: &Path) -> String;
    fn get_parent(&self, path: &Path) -> Option<PathBuf>;
    fn is_python_file(&self, path: &Path) -> bool;
}

pub trait IDirOpsProtocol: Send + Sync {
    fn scan_directory_with_ignored(&self, root: &Path, ignored: &PatternList) -> Vec<FilePath>;
    fn is_ignored_dir(&self, name: &str) -> bool;
    fn read_dir_entries_as_pathbuf(&self, path: &Path) -> Vec<PathBuf>;
}

pub trait IFileReadWriteProtocol: Send + Sync {
    fn read_to_string(&self, path: &Path) -> Result<String, FileSystemError>;
    fn write_string(&self, path: &Path, content: &str) -> Result<(), FileSystemError>;
    fn copy_file(&self, from: &Path, to: &Path) -> Result<(), FileSystemError>;
    fn create_dir_all(&self, path: &Path) -> Result<(), FileSystemError>;
    fn remove_dir_all(&self, path: &Path) -> Result<(), FileSystemError>;
    fn remove_file(&self, path: &Path) -> Result<(), FileSystemError>;
}

pub trait IProcessExecProtocol: Send + Sync {
    fn run_git_command(&self, args: &[&str]) -> GitCommandResult;
    fn parse_output_lines(&self, output: &str) -> ParsedLines;
    fn run_external_command_in(&self, name: &str, args: &[&str], dir: &Path) -> (String, String, bool);
}
```

**Note:** The aggregate trait (`IFilesystemAggregate`) should then compose all 4 sub-traits via supertrait bounds.

### Fix #17: Replace raw primitives in quality_rules contracts

```rust
// BEFORE: src/quality_rules/contract_bypass_checker_protocol.rs
pub trait IBypassCheckerProtocol: Send + Sync {
    fn check_bypass_comments(&self, file: &str, content: &str) -> Vec<LintResult>;
    fn check_cargo_toml(&self, file: &str, content: &str) -> Vec<LintResult>;
}

// AFTER:
pub trait IBypassCheckerProtocol: Send + Sync {
    fn check_bypass_comments(&self, file: &FilePath, content: &ContentString) -> Vec<LintResult>;
    fn check_cargo_toml(&self, file: &FilePath, content: &ContentString) -> Vec<LintResult>;
}
```

Apply the same pattern to `IMandatoryClassProtocol`, `IDeadInheritanceProtocol`, `ILineCheckerProtocol`, and `IOrphanParserProtocol`.
