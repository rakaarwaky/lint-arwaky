# FRD — filesystem

## System Overview

The filesystem crate centralizes all file I/O, AST parsing, and dependency graph construction for lint-arwaky. It replaces scattered `scan_directory` / `read_to_string` / `read_lintable_file` calls across 29 files (74 call sites) with a single, cached, parallel pipeline.

```
┌──────────────────────────────────────────────────────────────────────┐
│                    Filesystem Service                                │
│  (capabilities layer — walk, parse, graph, cache)                   │
├──────────────┬──────────────┬──────────────┬────────────────────────┤
│ FileWalker   │ FileParser   │ ImportExtractor│ DependencyGraph     │
│ (ignore)     │ (tree-sitter)│ (tree-sitter) │ (petgraph+dashmap)  │
│              │              │               │                      │
│ Walk once    │ Parse AST    │ Extract mod/  │ Build import graph   │
│ gitignore-   │ parallel     │ use/from/     │ Cycle detection      │
│ aware        │ rayon        │ require/import│ Reachability query   │
├──────────────┴──────────────┴──────────────┴────────────────────────┤
│ FileCache (DashMap<Path, String>) — read once, use everywhere       │
│ ASTCache  (DashMap<Path, Tree>)  — parse once, query everywhere     │
└──────────────────────────────────────────────────────────────────────┘
```

## Functional Requirements

### FR-001: File Discovery (ignore-based walk)

- **Description**: Walk project directory tree using `ignore` crate (gitignore-aware, parallel walk). Produces a flat list of source files filtered by extension.
- **Input**: Root path, ignored paths, allowed extensions
- **Output**: `Vec<FileEntry>` — path, extension, size, last modified
- **Business Rules**:
  - Uses `ignore::WalkBuilder` for parallel, gitignore-aware walking
  - Filters by extension: `.rs`, `.py`, `.ts`, `.js`, `.jsx`, `.tsx`
  - Respects `.gitignore`, `.ignore`, and `ignored_paths` from config
  - Skips files > 2 MiB (same as current `MAX_LINT_FILE_BYTES`)
  - Skips hidden directories (`.git`, `.venv`, `node_modules`, `target`)
- **Edge Cases**:
  - Symlinks: follow if target is within workspace, skip otherwise
  - Empty directories: return empty list, no error
  - Permission denied: log warning, skip file, continue walk
- **Error Handling**: Non-fatal — skip inaccessible files, return partial results

### FR-002: File Content Cache

- **Description**: Read all discovered files into memory once. Subsequent reads serve from cache with zero I/O.
- **Input**: `Vec<FileEntry>` from FR-001
- **Output**: `FileCache` — `DashMap<PathBuf, String>`
- **Business Rules**:
  - Populates cache in parallel via `rayon::par_iter`
  - Only reads files within size limit (2 MiB)
  - Cache is immutable after population (no mutation after scan start)
  - Thread-safe reads via `DashMap::get()`
  - Memory budget: configurable max total bytes (default 512 MiB)
- **Edge Cases**:
  - File modified between walk and read: use read timestamp, skip if stale
  - File deleted between walk and read: skip silently
  - Cache full: stop populating, log warning, serve partial cache
- **Error Handling**: Non-fatal — unreadable files excluded from cache

### FR-003: AST Parsing (tree-sitter)

- **Description**: Parse cached file contents into ASTs using tree-sitter. Parallel parsing via rayon.
- **Input**: `FileCache` from FR-002
- **Output**: `ASTCache` — `DashMap<PathBuf, Tree>`
- **Business Rules**:
  - Uses `tree-sitter` with language-specific grammars:
    - `tree-sitter-rust` for `.rs`
    - `tree-sitter-python` for `.py`
    - `tree-sitter-typescript` for `.ts`, `.tsx`
    - `tree-sitter-javascript` for `.js`, `.jsx`
  - Parsing is parallel via `rayon::par_iter`
  - AST is cached after first parse (parse-once, query-many)
  - Incremental parsing not needed (files are small, full parse is fast)
- **Edge Cases**:
  - Parse error (syntax error in source): store partial tree, log warning
  - Unsupported extension: skip parsing, return None
  - Tree-sitter grammar not available: fall back to regex-based extraction
- **Error Handling**: Non-fatal — parse errors produce partial AST, don't block scan

### FR-004: Import/Dependency Extraction

- **Description**: Extract import statements from ASTs. Normalize to absolute module paths.
- **Input**: `ASTCache` from FR-003, language per file
- **Output**: `Vec<ImportEntry>` — source file, import path, import type (mod/use/from/require)
- **Business Rules**:
  - **Rust**: Extract `use` and `mod` statements → resolve to module path
  - **Python**: Extract `import` and `from ... import` → resolve to module path
  - **TypeScript/JavaScript**: Extract `import`, `export ... from`, `require()` → resolve to module path
  - Normalize relative paths (`./foo`, `../bar`) to absolute module paths
  - Handle barrel re-exports (`mod.rs`, `__init__.py`, `index.ts`)
  - Skip external dependencies (crates.io, npm packages) — only internal imports
- **Edge Cases**:
  - Dynamic imports (`import()`): extract string literal, mark as dynamic
  - Conditional imports (`#[cfg(...)]`): extract both branches
  - Star imports (`use foo::*`): mark as wildcard, don't resolve individual items
- **Error Handling**: Non-fatal — unresolvable imports logged, excluded from graph

### FR-005: Dependency Graph Construction

- **Description**: Build a directed graph of file-to-file dependencies from extracted imports.
- **Input**: `Vec<ImportEntry>` from FR-004
- **Output**: `DependencyGraph` — petgraph `DiGraph<FileNode, ImportEdge>`
- **Business Rules**:
  - Nodes: each source file (keyed by absolute path)
  - Edges: import relationship (source file → imported file)
  - Edge weight: import type (use/from/require/mod)
  - Parallel graph construction via `DashMap` → merge into `petgraph::DiGraph`
  - Supports queries:
    - `dependents(file)` → who imports this file?
    - `dependencies(file)` → what does this file import?
    - `cycles()` → find circular dependencies
    - `reachable(from, to)` → is there a path?
    - `orphan_files()` → files with no dependents (nothing imports them)
- **Edge Cases**:
  - External dependencies (not in workspace): nodes created but marked external
  - Broken imports (file not found): edge created with `resolved = false`
  - Duplicate imports: single edge, deduplicated
- **Error Handling**: Non-fatal — broken imports create unresolved edges

### FR-006: FilesystemService Facade

- **Description**: Single entry point that orchestrates FR-001 through FR-005. Returns a `FilesystemResult` containing everything rule crates need.
- **Input**: Scan root path, config (ignored paths, extensions)
- **Output**: `FilesystemResult { files, cache, asts, imports, graph }`
- **Business Rules**:
  - Pipeline: walk → cache → parse → extract → graph (sequential stages, parallel within each stage)
  - Each stage logs timing for performance profiling
  - Result is immutable after construction (read-only queries only)
  - Implements `IFilesystemServiceProtocol` trait for DI
- **Edge Cases**:
  - Empty project: return empty result, no error
  - Single file: still run full pipeline (consistency)
  - Very large project (>10k files): stream results if memory budget exceeded
- **Error Handling**: Return partial result with diagnostics for failed stages

## Non-Functional Requirements

### NFR-001: Performance

- Full scan of 1,660 files should complete in **< 5 seconds** (target 4x speedup from current 19.7s)
- File discovery (walk): **< 500ms** for 1,660 files
- File reading (cache): **< 2s** for 1,660 files (parallel)
- AST parsing: **< 2s** for 1,660 files (parallel)
- Import extraction: **< 500ms** (query cached ASTs)
- Graph construction: **< 200ms** (DashMap + petgraph)

### NFR-002: Memory

- Total memory budget: configurable, default **512 MiB** for file cache
- AST cache: ~2x file size overhead (tree-sitter trees are compact)
- Graph: ~100 bytes per edge, ~1KB per node
- For 1,660 files with ~500 avg lines: ~150 MiB total estimated

### NFR-003: Compatibility

- Must implement existing traits: `ICodeAnalysisAggregate`, `IOrphanAggregate`, etc.
- Rule crates continue to work unchanged — they receive `FilesystemResult` instead of doing their own I/O
- Backward compatible — `run_code_analysis()` still works, just faster internally
