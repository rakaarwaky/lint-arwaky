# FRD — filesystem (v3.0.0)

---

## System Overview

The filesystem crate produces data for all feature crates. Each Functional Requirement is defined by **what data it produces** and **for whom**.

### Data Production Map

| FR | Output Data | Consumers |
| --- | --- | --- |
| FR-001 | `Vec<FilePath>` or `Vec<FileEntry>` | naming-rules, code-analysis |
| FR-002 | `Vec<FileEntry>` with parse metadata | role-rules |
| FR-003 | `Vec<ImportEntry>` | import-rules |
| FR-004 | `GraphData` (graph + maps) | orphan-detector |
| FR-005 | Workspace info (root, member, language) | cli-commands, external-lint, orphan-detector |
| FR-006 | Tool info (paths, availability) | external-lint, maintenance |
| FR-007 | Cached file content | orphan-detector |

### Pipeline Dependency

```
FR-001 (discovery)
  └→ FR-002 (parsing)
       ├→ FR-003 (imports)
       │    └→ FR-004 (graph)
       └→ FR-004 (graph, also uses FR-002 output)

FR-005, FR-006, FR-007 — independent
```

---

## Functional Requirements

### FR-001: File Discovery

**What it produces**: Source file paths, optionally with content.

| Mode | Output Type | Consumer |
| --- | --- | --- |
| Lightweight | `Vec<FilePath>` — paths only | naming-rules |
| Full | `Vec<FileEntry>` — paths + content + language + extension | code-analysis |

**Input**: Root path, ignored paths (from config).

**Business Rules**:

- Walk project directory tree using `ignore` crate (gitignore-aware, parallel walk).
- Scans `crates/`, `packages/`, `modules/` subdirectories at root level.
- Filters by extension: `.rs`, `.py`, `.ts`, `.js`, `.jsx`, `.tsx`.
- Respects `.gitignore`, `.ignore`, and `ignored_paths` from config.
- Skips hidden directories (`.git`, `.venv`, `node_modules`, `target`, `dist`, `build`, `__pycache__`).
- **Lightweight mode**: Returns only file paths. No file reading.
- **Full mode**: Reads file content into `FileEntry.content` (UTF-8). Non-UTF-8 files are skipped with warning.
- No file size limit — all files regardless of size are processed.

**Edge Cases**:

- Symlinks: follow if target is within workspace root, skip otherwise.
- Empty directories: return empty list, no error.
- Permission denied: log warning, skip file, continue walk.
- Non-UTF-8 content (full mode): skip file, log warning.
- Empty files: included with empty content string.

**Error Handling**: Non-fatal — skip inaccessible files, return partial results.

---

### FR-002: AST Parsing

**What it produces**: File entries enriched with parse metadata and `parse_ok` flag.

| Output | Type | Consumer |
| --- | --- | --- |
| Parsed entries | `Vec<FileEntry>` with parse metadata | role-rules, FR-003, FR-004 |

**Input**: `Vec<FileEntry>` from FR-001 (full mode).

**Business Rules**:

- Uses `tree-sitter` with language-specific grammars:
  - `tree-sitter-rust` for `.rs`
  - `tree-sitter-python` for `.py`
  - `tree-sitter-typescript` for `.ts`, `.tsx`
  - `tree-sitter-javascript` for `.js`, `.jsx`
- Parsing is parallel via `rayon::par_iter`.
- Each `FileEntry` is enriched with:
  - `parse_ok: bool` — whether parsing succeeded.
  - `parse_metadata` — language-specific structured data:
    - **Rust**: `ItemUse`, `ItemMod`, `ItemImpl`, `ItemStruct`/`ItemEnum`/`ItemTrait`/`ItemType`, `ItemFn`.
    - **Python**: import statements, class declarations, function definitions.
    - **TypeScript/JavaScript**: import/export statements, class declarations, interface declarations, type alias declarations, function definitions.
- Files with `parse_ok = false` produce a `PARSE_WARN` diagnostic.
- No regex fallback.

**Edge Cases**:

- Syntax error: tree-sitter produces partial tree, `parse_ok = false`.
- Empty file: `parse_ok = true`, empty metadata.
- Macro-generated code: invisible to parser.

**Error Handling**: Non-fatal — parse errors produce `parse_ok = false` + `PARSE_WARN`.

---

### FR-003: Import Data Extraction

**What it produces**: Flat list of import entries.

| Output | Type | Consumer |
| --- | --- | --- |
| Import entries | `Vec<ImportEntry>` | import-rules, FR-004 |

**Input**: `Vec<FileEntry>` with parse metadata from FR-002.

**Business Rules**:

- **Rust**: Extract `use`/`mod` statements. Resolve `crate::`/`super::`/`self::`. Handle grouped imports, glob imports, `pub use` re-exports.
- **Python**: Extract `import X`/`from X import Y`. Resolve relative imports.
- **TypeScript/JavaScript**: Extract `import`/`export`/`require()`. Resolve extensionless imports.
- Normalize relative paths to workspace-root-relative module paths.
- Handle barrel re-exports (`mod.rs`, `__init__.py`, `index.ts`).
- Skip external dependencies (crates.io, npm).
- Skip conditional imports (`#[cfg(...)]`).

**Edge Cases**:

- Dynamic imports: extract if static, skip if variable-based.
- Unresolvable imports: `resolved = false`.
- Files with `parse_ok = false`: no imports extracted.

**Error Handling**: Non-fatal — unresolvable imports logged, marked `resolved = false`.

---

### FR-004: Graph Data Construction

**What it produces**: Structured graph data with 4 components.

| Output Component | Type | Description |
| --- | --- | --- |
| DiGraph | `petgraph::DiGraph` | Forward import graph (file → files it imports) |
| ReverseLinkIndex | `HashMap<PathBuf, Vec<PathBuf>>` | Reverse import map (file → files that import it) |
| DefinitionMap | `HashMap<String, Vec<PathBuf>>` | Symbol → defining file |
| ImplMap | `HashMap<String, Vec<PathBuf>>` | Trait/interface → implementor files |

**Consumer**: orphan-detector

**Input**: `Vec<ImportEntry>` from FR-003 + `Vec<FileEntry>` with parse metadata from FR-002.

**Business Rules**:

- **DiGraph**: Nodes = source files, Edges = import relationships. Parallel construction via `DashMap`.
- **ReverseLinkIndex**: Invert all DiGraph edges. Barrel re-exports resolved to original source.
- **DefinitionMap**: From parse metadata — struct/enum/trait/type/class/interface name → file.
- **ImplMap**: From parse metadata — trait/interface → implementor files.
- **Graph queries**: `dependents(file)`, `dependencies(file)`, `reachable(from)`.

**Edge Cases**:

- Circular imports: cycles exist but don't cause errors.
- Broken imports: edge with `resolved = false`.
- Files with `parse_ok = false`: nodes but no edges.

**Error Handling**: Non-fatal — broken imports create unresolved entries.

---

### FR-005: Workspace Detection

**What it produces**: Workspace metadata.

| Output | Type | Consumers |
| --- | --- | --- |
| Workspace root | `Option<PathBuf>` | cli-commands, orphan-detector |
| Member status | `bool` | cli-commands |
| Leaf member status | `bool` | cli-commands |
| Source directory | `PathBuf` | code-analysis |
| Language from path | `ConfigLanguage` | config-system |
| Language by walking | `(bool, bool, bool)` | external-lint |

**Input**: Start path (string or Path).

**Business Rules**:

- **Workspace root** (`workspace_root`, `find_workspace_root_from_path`): Walk up looking for `crates/`/`packages/`/`modules/` + manifest.
- **Member detection** (`is_member_path`): Cargo.toml without `[workspace]`, or `__init__.py`/`pyproject.toml`, or `package.json`.
- **Leaf member** (`is_leaf_member_path`): Member without sub-members.
- **Source dir** (`detect_source_dir`): Check `packages/`/`crates/`/`modules/` in order.
- **Language from path** (`detect_language_from_path`): Check manifest markers.
- **Language by walking** (`detect_languages`): Walk + check extensions. Early-terminate.

**Error Handling**: Non-fatal — returns `None`/`Err` for unresolvable cases.

---

### FR-006: Tool Resolution

**What it produces**: Tool availability and resolved paths.

| Output | Type | Consumers |
| --- | --- | --- |
| Executable in PATH | `bool` | external-lint, maintenance |
| Local bin available | `bool` | external-lint, maintenance |
| JS tool command | `Option<Vec<String>>` | external-lint |
| Working directory | `FilePath` | external-lint |
| Config file present | `bool` | external-lint, maintenance |
| Cargo manifest present | `Option<String>` | external-lint |

**Input**: Tool name, working directory, arguments.

**Business Rules**:

- **PATH detection** (`is_executable_in_path`, `is_binary_available`): Check system PATH.
- **Local bin** (`has_local_bin`): Check `node_modules/.bin/`.
- **JS tool** (`resolve_js_cmd`): Local binary only, no npx/bunx fallback.
- **Working dirs** (`resolve_js_working_dir`, `resolve_cargo_working_dir`, `resolve_cargo_lock_working_dir`): Walk up to find project root.
- **Config detection** (`has_config_file`): Check for `.eslintrc`, `tsconfig.json`, etc.
- **Manifest detection** (`has_cargo_toml`, `has_cargo_lock`): Check for Cargo files.

**Error Handling**: Non-fatal — return false/None for unavailable tools.

---

### FR-007: File Cache

**What it produces**: Cached file content for fast repeated reads.

| Output | Type | Consumer |
| --- | --- | --- |
| Cached content | `ContentString` | orphan-detector |

**Input**: File paths.

**Business Rules**:

- **DashMap cache** (pipeline): Parallel population via rayon. Thread-safe lookup.
- **Bounded HashMap cache** (ad-hoc): `MAX_CACHE_ENTRIES = 20,000`. Simple threshold, no LRU.
- **String-keyed cache** (code-analysis compatibility): Separate cache for string-keyed lookups.
- **Cache cascade**: `read_cached()` checks DashMap → bounded → disk.

**Edge Cases**:

- Concurrent access: DashMap handles thread safety.
- Cache full: silently skip insertion, serve from disk.
- Empty content: not stored.

**Error Handling**: Non-fatal — cache misses fall through to disk reads.

---

## API Contract

### FR-001: File Discovery

| Method | Output | Used By |
| --- | --- | --- |
| `discover_source_files(root, ignored)` | `Vec<FilePath>` | naming-rules |
| `discover_files(root, ignored)` | `Vec<FileEntry>` | code-analysis, import-rules, orphan-detector |
| `collect_source_files(root_dir, ignored)` | `Vec<FilePath>` | code-analysis |
| `collect_file_entries(files)` | `Vec<(PathBuf, String)>` | code-analysis |
| `read_lintable_file(path)` | `Result<Option<String>, String>` | code-analysis |

### FR-002: AST Parsing

| Method | Output | Used By |
| --- | --- | --- |
| `run_pipeline(root, ignored)` | — (triggers FR-001→FR-004) | mcp-server |
| `file_list()` | `&[FileEntry]` | — (available) |
| `parsed_file_list()` | `&[FileEntry]` | role-rules (via parameter) |
| `parse_warnings()` | `&[ParseWarning]` | mcp-server, cli-commands |

### FR-003: Import Data Extraction

| Method | Output | Used By |
| --- | --- | --- |
| `import_list()` | `&[ImportEntry]` | import-rules (via parameter) |
| `all_imports()` | `&[ImportEntry]` | — (available) |
| `imports_for(path)` | `Vec<ImportEntry>` | — (available) |

### FR-004: Graph Data Construction

| Method | Output | Used By |
| --- | --- | --- |
| `dependency_graph()` | `&HashMap<PathBuf, Vec<PathBuf>>` | import-rules (via parameter) |
| `reverse_import_map()` | `&HashMap<PathBuf, Vec<PathBuf>>` | orphan-detector (via parameter) |
| `symbol_definitions()` | `&HashMap<String, Vec<PathBuf>>` | orphan-detector (via parameter) |
| `trait_implementations()` | `&HashMap<String, Vec<PathBuf>>` | orphan-detector (via parameter) |
| `depends_on(from, to)` | `bool` | — (available) |
| `cycles()` | `Vec<Vec<PathBuf>>` | — (available) |
| `orphan_files()` | `Vec<PathBuf>` | — (available) |

### FR-005: Workspace Detection

| Method | Output | Used By |
| --- | --- | --- |
| `workspace_root(start)` | `Option<PathBuf>` | cli-commands |
| `find_workspace_root_from_path(start)` | `Result<PathBuf, io::Error>` | orphan-detector |
| `is_member_path(path)` | `bool` | cli-commands |
| `is_leaf_member_path(path)` | `bool` | cli-commands |
| `detect_source_dir(project_root)` | `PathBuf` | code-analysis |
| `detect_language_from_path(path)` | `ConfigLanguage` | config-system |
| `detect_languages(root)` | `(bool, bool, bool)` | external-lint |

### FR-006: Tool Resolution

| Method | Output | Used By |
| --- | --- | --- |
| `resolve_js_cmd(exec, args, wd)` | `Option<Vec<String>>` | external-lint |
| `resolve_js_working_dir(path)` | `FilePath` | external-lint |
| `resolve_cargo_working_dir(path)` | `FilePath` | external-lint |
| `resolve_cargo_lock_working_dir(path)` | `FilePath` | external-lint |
| `is_executable_in_path(exec)` | `bool` | external-lint, maintenance |
| `is_binary_available(bin_name)` | `bool` | tui |
| `has_local_bin(wd, exec)` | `bool` | external-lint, maintenance |
| `has_config_file(dir)` | `bool` | external-lint, maintenance |
| `has_cargo_toml(path)` | `Option<String>` | external-lint |
| `has_cargo_lock(path)` | `Option<String>` | external-lint |
| `has_python_files_recursive(path)` | `bool` | external-lint |
| `default_working_dir(path)` | `FilePath` | external-lint |
| `noop_apply_fix()` | `Result<ComplianceStatus, LinterOperationError>` | external-lint |

### FR-007: File Cache

| Method | Output | Used By |
| --- | --- | --- |
| `read_cached(path)` | `ContentString` | orphan-detector |
| `cache_populate(files)` | — | — (internal) |
| `cache_get(path)` | `Option<String>` | — (internal) |
| `cache_contains(path)` | `bool` | — (internal) |
| `cache_memory_bytes()` | `usize` | — (internal) |
| `cache_clear()` | — | — (internal) |

### File I/O (utility, not FR)

| Method | Output | Used By |
| --- | --- | --- |
| `read_file(path)` | `Option<String>` | config-system |
| `read_to_string(path)` | `Result<String, io::Error>` | config-system, auto-fix, git-hooks, maintenance |
| `write_string(path, content)` | `Result<(), io::Error>` | auto-fix, git-hooks |
| `write_text_to_file(path, text)` | `Result<(), String>` | tui |
| `copy_file(src, dst)` | `Result<u64, io::Error>` | cli-commands |
| `remove_file(path)` | `Result<(), io::Error>` | git-hooks |
| `create_dir_all(path)` | `Result<(), io::Error>` | git-hooks |
| `scan_directory(dir)` | `Vec<PathBuf>` | config-system, orphan-detector, maintenance |
| `scan_directory_with_ignored(dir, ignored)` | `Vec<PathBuf>` | tui |
| `read_dir_entries_as_pathbuf(dir)` | `Result<Vec<PathBuf>, io::Error>` | project-setup |
| `run_git_command(args, dir)` | `(String, String, bool)` | git-hooks |
| `run_external_command_in(name, args, dir)` | `(String, String, bool)` | maintenance |
| `parse_output_lines(output)` | `Vec<String>` | git-hooks |
| `path_exists(path)` | `bool` | config-system, git-hooks |
| `is_file(path)` | `bool` | external-lint, maintenance, git-hooks, project-setup |
| `is_dir(path)` | `bool` | orphan-detector, project-setup, git-hooks |
| `canonicalize(path)` | `Result<PathBuf, io::Error>` | config-system |
| `canonicalize_path_str(path)` | `String` | external-lint |
| `symlink_metadata(path)` | `Result<Metadata, io::Error>` | config-system |
| `should_ignore(path, ignored)` | `bool` | orphan-detector |
| `is_ignored_dir(dir, ignored)` | `bool` | orphan-detector |
| `timing()` | `&ScanTiming` | — (available) |

---

## Consumer Access Pattern

| Consumer | FRs Used | Methods |
| --- | --- | --- |
| **naming-rules** | FR-001 | `discover_source_files()` |
| **code-analysis** | FR-001, FR-005 | `collect_file_entries()`, `collect_source_files()`, `detect_source_dir()`, `read_lintable_file()` |
| **role-rules** | FR-002 | `parsed_file_list()` (via parameter) |
| **import-rules** | FR-001, FR-003, FR-004 | `discover_files()`, `import_list()`, `dependency_graph()` (via parameter) |
| **orphan-detector** | FR-002, FR-003, FR-004, FR-005, FR-007 | `parsed_file_list()`, `import_list()`, `dependency_graph()`, `reverse_import_map()`, `symbol_definitions()`, `trait_implementations()`, `read_cached()`, `find_workspace_root_from_path()`, `discover_files()`, `scan_directory()`, `is_dir()`, `is_ignored_dir()`, `should_ignore()` |
| **cli-commands** | FR-005 | `workspace_root()`, `is_member_path()`, `is_leaf_member_path()`, `canonicalize()`, `copy_file()` |
| **config-system** | utility | `read_file()`, `read_to_string()`, `symlink_metadata()`, `canonicalize()`, `path_exists()`, `scan_directory()` |
| **external-lint** | FR-005, FR-006 | `detect_languages()`, `resolve_js_cmd()`, `resolve_js_working_dir()`, `is_executable_in_path()`, `has_local_bin()`, `has_config_file()`, `has_cargo_toml()`, `has_cargo_lock()`, `resolve_cargo_working_dir()`, `resolve_cargo_lock_working_dir()`, `canonicalize_path_str()`, `is_file()`, `default_working_dir()`, `has_python_files_recursive()`, `noop_apply_fix()` |
| **tui** | utility | `write_text_to_file()`, `scan_directory_with_ignored()`, `is_binary_available()` |
| **mcp-server** | FR-002 | `run_pipeline()`, `parse_warnings()` |
| **auto-fix** | utility | `read_to_string()`, `write_string()`, `copy_file()` |
| **project-setup** | utility | `is_dir()`, `is_file()`, `read_dir_entries_as_pathbuf()` |
| **maintenance** | FR-006, utility | `run_external_command_in()`, `is_file()`, `read_to_string()`, `scan_directory()`, `is_executable_in_path()`, `has_local_bin()`, `has_config_file()` |
| **git-hooks** | utility | `path_exists()`, `is_file()`, `is_dir()`, `read_to_string()`, `write_string()`, `create_dir_all()`, `remove_file()`, `run_git_command()`, `parse_output_lines()` |
| **report-formatter** | — | (no filesystem usage) |
| **file-watch** | — | (uses notify crate directly) |

---

## Non-functional Requirements

- **Performance**: Pipeline processes 1,000 files in < 2s. 10,000 files in < 10s. Accessor calls O(1).
- **Memory**: Bounded by total workspace size. Cache capped at 20,000 entries.
- **Accuracy**: Full AST via tree-sitter for all languages. No regex fallback.
- **Concurrency**: Pipeline parallel via rayon + DashMap. Trait is `Send + Sync`.
- **Configurability**: Hardcoded conventions (workspace structure, extensions). Configurable via YAML (ignored paths, workspace dirs).

---

## Test Scenarios

### FR-001: File Discovery

| # | Scenario | Expected | Rule |
| --- | --- | --- | --- |
| 1 | Workspace with 100 .rs files | All 100 discovered | FR-001 |
| 2 | File in .gitignore | Not discovered | FR-001 |
| 3 | Symlink pointing outside workspace | Skipped | FR-001 |
| 4 | Empty directory | Empty list | FR-001 |
| 5 | Non-UTF-8 file (full mode) | Skipped with warning | FR-001 |

### FR-002: AST Parsing

| # | Scenario | Expected | Rule |
| --- | --- | --- | --- |
| 1 | Valid Rust file | parse_ok = true, full metadata | FR-002 |
| 2 | Rust file with syntax error | parse_ok = false, PARSE_WARN | FR-002 |
| 3 | Empty file | parse_ok = true, empty metadata | FR-002 |
| 4 | 1,000 files parsed in parallel | Completes in < 1s | FR-002 |

### FR-003: Import Data Extraction

| # | Scenario | Expected | Rule |
| --- | --- | --- | --- |
| 1 | `use crate::foo::Bar` | ImportEntry with resolved path | FR-003 |
| 2 | `use foo::*` | is_wildcard = true | FR-003 |
| 3 | `#[cfg(test)] use foo::Bar` | Not extracted | FR-003 |
| 4 | External dependency | Not extracted | FR-003 |

### FR-004: Graph Data Construction

| # | Scenario | Expected | Rule |
| --- | --- | --- | --- |
| 1 | A imports B | Edge A → B | FR-004 |
| 2 | Circular imports | Both edges exist | FR-004 |
| 3 | `struct Foo` in A | DefinitionMap: "Foo" → A | FR-004 |
| 4 | `impl IBar for Foo` | ImplMap: "IBar" → [A] | FR-004 |

### FR-005: Workspace Detection

| # | Scenario | Expected | Rule |
| --- | --- | --- | --- |
| 1 | Start from crates/some-crate/src | Finds workspace root | FR-005 |
| 2 | Path with Cargo.toml (no [workspace]) | is_member_path = true | FR-005 |
| 3 | Path with Cargo.toml nearby | detect_language = Rust | FR-005 |

### FR-006: Tool Resolution

| # | Scenario | Expected | Rule |
| --- | --- | --- | --- |
| 1 | node_modules/.bin/eslint exists | resolve_js_cmd returns command | FR-006 |
| 2 | Binary in system PATH | is_executable_in_path = true | FR-006 |
| 3 | has_config_file with .eslintrc | Returns true | FR-006 |

### FR-007: File Cache

| # | Scenario | Expected | Rule |
| --- | --- | --- | --- |
| 1 | cache_populate + cache_get | Returns content | FR-007 |
| 2 | read_cached with DashMap hit | No disk I/O | FR-007 |
| 3 | Bounded cache at 20K entries | New inserts skipped | FR-007 |

---

## Glossary

| Term | Definition |
| --- | --- |
| **FilePath** | Value object containing only the file path string |
| **FileEntry** | Value object: path + content + language + extension + parse metadata + parse_ok |
| **ImportEntry** | Value object: source file + target module + symbols + type + flags |
| **GraphData** | Composite: DiGraph + ReverseLinkIndex + DefinitionMap + ImplMap |
| **ParseMetadata** | Structured AST-derived data per language |
| **PARSE_WARN** | Warning for files that failed to parse |
| **Barrel file** | Re-export file (`mod.rs`, `__init__.py`, `index.ts`) |
| **IFilesystemAggregate** | Trait exposing all filesystem capabilities |

## Reference

- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
