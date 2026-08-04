# FRD — orphan-rules (v1.12.1)

---

## System Overview

The orphan-rules crate identifies dead, unused, or unreachable code components across the 7-layer AES architecture. It builds its own import reachability graph and inheritance map, then performs layer-specific orphan analysis starting from valid entry points (containers, binary entries, main files).

Graph construction is delegated to the external `filesystem` aggregate via `build_orphan_graph_context(root, ignored)`, which discovers workspace files, parses each file via tree-sitter AST, resolves imports to file edges, and returns a `GraphAnalysisContext`. The Surface calls `filesystem.build_file_index(root)` to populate caches, then calls `filesystem.build_orphan_graph_context(root, ignored)` to get the analysis context. The orphan-rules crate receives pre-built graph data and performs zero I/O — it only performs business logic analysis on pre-fetched data.

### Architecture & Data Flow

```mermaid
flowchart TD
    A["Surface"] -->|"build_file_index(root)"| D["filesystem_aggregate\n(external crate)"]
    A -->|"build_orphan_graph_context(root, ignored)"| D

    subgraph FS ["filesystem crate (external)"]
        D --> E1["file_walker"]
        D --> E2["AST parser\n(imports + identifiers)"]
        D --> E3["graph builder\n(reverse links, definitions)"]
        E1 --> G1["GraphAnalysisContext\n(import graph, inbound links,\ninheritance map, file list)"]
        E2 --> G1
        E3 --> G1
    end

    G1 -->|"return"| D
    D -->|"GraphAnalysisContext\n(pre-built)"| A

    A -->|"scan_orphans(context)"| B["orphan_aggregate"]
    B --> C["orphan_orchestrator\n(zero I/O)"]

    C --> H1["taxonomy_analysis"]
    C --> H2["contract_analysis"]
    C --> H3["capabilities_analysis"]
    C --> H4["utility_analysis"]
    C --> H5["agent_analysis"]
    C --> H6["surface_analysis"]

    H1 --> I["Violations"]
    H2 --> I
    H3 --> I
    H4 --> I
    H5 --> I
    H6 --> I
    I --> J["LintResult"]
    J --> C
    C --> B
    B -->|output| A

    style A fill:#e1f5fe,stroke:#0288d1
    style FS fill:#fff3e0,stroke:#e65100
    style D fill:#fff3e0,stroke:#e65100
    style I fill:#fce4ec,stroke:#c62828
    style J fill:#f3e5f5,stroke:#7b1fa2
```

---

## Functional Requirements

### FR-001: Graph Context Construction and Dispatch

- **Description**: Build the `GraphAnalysisContext` internally via `OrphanGraphResolver` (file discovery, parsing, import edge resolution, inheritance mapping) and dispatch to analyzers.
- **Input**: `GraphAnalysisContext` (built in-crate) containing:

  - All workspace source files (workspace-root-relative paths).
  - All extracted import edges (file → file).
  - Forward import graph (file → file edges).
  - Reverse import map (file → list of importers).
  - Inheritance map (file → inherited/implemented trait, class, or interface names).
- **Output**: Built `GraphAnalysisContext` ready for analysis.
- **Business Rules**:

  - The resolver discovers workspace files across `crates/`, `packages/`, `modules/` (direct `std::fs` walks) plus the scanned file list.
  - Each file is parsed via `IOrphanParserProtocol` (shared parsers); imports, mod declarations, class bases, and trait impls feed the graph.
  - Files that fail to parse contribute no edges — they are treated as **orphan candidates** (fail-strict: cannot verify reachability without parse data).
  - All paths in the graph are workspace-root-relative.
  - Barrel files are identified and tagged for downstream skipping (see FR-010).
- **Edge Cases**:

  - Empty workspace (zero files) → empty context, no violations.
  - Unreadable files → skipped (no edges, no violations).
  - Files with parse failures → empty imports → orphan candidacy.
- **Error Handling**: Individual file read/parse failures degrade gracefully (empty edges → orphan candidacy). No explicit context validation step — components are always populated by the in-crate resolver.

---

### FR-002: Entry Point Discovery

- **Description**: Identify valid entry points that anchor the reachability graph.
- **Input**: File list from the analysis context, optional configured entry point patterns from architecture configuration.
- **Output**: Set of entry point file paths.
- **Business Rules**:

  - Default entry point patterns

    - `*_container.*`, `*_entry.*`
    - Files starting with `root_`
    - `main.rs`, `lib.rs`, `main.py`, `__main__.py`, `main.ts`, `main.js`, `index.ts`, `index.js`
  - Merges configured additional entry point patterns from architecture configuration.
  - Pattern matching uses **segment matching**: exact match, stem match, prefix match, suffix match, extension match — never substring `contains()` to prevent false positives (e.g., `germanic_utils` must not match `main`).
  - Deduplicates and sorts the final list.
- **Edge Cases**:

  - Workspace with zero entry points → all non-barrel files flagged as orphans.
  - Workspace with entry points in non-standard locations → requires config override.
- **Error Handling**: Missing or inaccessible entry point files (not in the file list) are excluded from the set.

---

### FR-003: Reachability Tracing

- **Description**: Perform BFS from all entry points through the forward import graph to determine which files are transitively reachable ("alive").
- **Input**: Entry point set and the forward import graph from the analysis context.
- **Output**: Set of all reachable file paths (alive set).
- **Business Rules**:

  - Uses breadth-first search with a visited tracker to avoid revisiting nodes.
  - A file is "alive" if it is transitively reachable from any entry point via import edges.
  - The alive set is used by capabilities, agent, and surface orphan analyzers.
  - Files whose parse produced no imports are NOT added to the alive set (cannot verify edges).
- **Edge Cases**:

  - Isolated files with no imports from any entry point → not in alive set → flagged by analyzers.
  - Entry points that import nothing → valid (they are roots, alive by definition).
  - Cycles in the graph → handled by visited set, no infinite loops.
- **Error Handling**: Cycles handled by visited set. Missing graph nodes (file in file list but not in graph) → treated as unreachable.

---

### FR-004: Taxonomy Orphan Detection (AES501)

- **Description**: Check that taxonomy layer files (`taxonomy_*`) are imported by at least one file from any other layer.
- **Input**: File path, reverse link index from the analysis context.
- **Output**: Orphan indicator result with `is_orphan` flag, reason, and severity.
- **Business Rules**:

  - A taxonomy file is orphan if no contract, capabilities, agent, utility, or surface file imports it (via `ReverseLinkIndex`).
  - Internal taxonomy-to-taxonomy imports do NOT count — at least one non-taxonomy importer is required.
  - Barrel files (`mod.rs`, `__init__.py`, `index.ts`) do not count as importers.
  - Files that fail to parse → flagged as orphan (fail-strict); no PARSE_WARN diagnostic is emitted.
- **Edge Cases**:

  - Taxonomy files imported only by other taxonomy files → flagged (no consumer outside taxonomy).
  - Taxonomy VO imported by a contract protocol → not orphan.
- **Error Handling**: Files with no detectable inbound links in `ReverseLinkIndex` → orphan candidates.

---

### FR-005: Contract Orphan Detection (AES502)

- **Description**: Check that contract files have at least one implementation or consumer, using trait extraction from parse results and whole-word reference scanning across workspace files.
- **Input**: File path, inheritance map (from the analysis context), workspace file list, content map.
- **Output**: Orphan indicator result with `is_orphan` flag, reason, and severity.
- **Business Rules**:

  - **Protocol contracts** (`_protocol` files):
    - Must be implemented by at least one capabilities file — checked by re-parsing candidate files for `impl <Trait> for ...` (Rust), class bases (Python), or class implements (TS).
    - Must be called/referenced by at least one agent, container, capabilities, or surface file — checked via whole-word content search in `agent_`/`capabilities_`/`surface_` prefixed files and container files.
    - Both conditions must be satisfied. Implementation without callers → orphan. Callers without implementation → orphan.
  - **Aggregate contracts** (`_aggregate` files):
    - Must be implemented by at least one agent file (checked via `impl`/class-base scanning).
    - Must be called/referenced by at least one surface or container file (checked via whole-word content search).
  - **Barrel re-export check**: If any trait/interface name from the contract file appears in a barrel file's re-exports, the contract is considered used as public API and is NOT flagged.
  - Whole-word matching is used for all identifier checks.
  - Files that fail to parse → flagged as orphan (fail-strict) because reachability cannot be verified.
- **Edge Cases**:

  - Protocol with implementation but zero callers → orphan.
  - Protocol with callers but no implementation → orphan.
  - Aggregate re-exported in barrel → not orphan.
  - Contract file with no traits/interfaces (e.g., only type aliases) → not orphan (nothing to check).
- **Error Handling**: Files with empty content or no trait names → not flagged (nothing to check).

---

### FR-006: Capabilities Orphan Detection (AES503)

- **Description**: Check that capability files are wired in a root container or reachable from entry points.
- **Input**: File path, alive set (from FR-003).
- **Output**: Orphan indicator result with `is_orphan` flag, reason, and severity.
- **Business Rules**:

  - Capabilities use dependency injection (`Arc<T>` in Rust, DI containers in Python/TS).
  - A capability is orphan if:
    1. Its struct/class names are not wired in any container file (container identifier check), AND
    2. The file is not in the alive set (not transitively reachable from entry points).
  - Container files are identified by suffix: `*_container.*`, `*_entry.*`.
  - Files that fail to parse → flagged as orphan (fail-strict) because reachability cannot be verified.
- **Edge Cases**:

  - Capability imported only by other capabilities in a chain → alive if any link in the chain reaches a container (BFS handles this).
  - Capability with no struct/class names → treated as potential orphan.
- **Error Handling**: Files that fail to parse → orphan (fail-strict).

---

### FR-007: Utility Orphan Detection (AES504)

- **Description**: Check that utility files are imported by at least one consumer layer (capabilities, agent, surface, or root).
- **Input**: File path, reverse link index from the analysis context.
- **Output**: Orphan indicator result with `is_orphan` flag, reason, and severity.
- **Business Rules**:

  - Check `ReverseLinkIndex` for inbound links. Classify each importer by layer prefix.
  - Valid consumer layers: `capabilities`, `agent`, `surface`, `root`.
  - If any consumer-layer importer exists → not orphan.
  - Utility-only import chains are flagged as dead code (utility importing utility does not count).
  - Files that fail to parse → flagged as orphan (fail-strict); no PARSE_WARN diagnostic is emitted.
- **Edge Cases**:

  - Utility imported by another utility that is itself orphaned → the chain is dead → orphan.
  - Utility imported by a capabilities file → not orphan.
  - Utility with no inbound links → orphan.
- **Error Handling**: Files that fail to parse → orphan (fail-strict). If `ReverseLinkIndex` has no entry for the file → orphan.

---

### FR-008: Agent Orphan Detection (AES505)

- **Description**: Check that agent orchestrator files are called by surface layer files or binary entry points, by scanning workspace file contents for aggregate references.
- **Input**: File path, workspace file list, content map.
- **Output**: Orphan indicator result with `is_orphan` flag, reason, and severity.
- **Business Rules**:

  - Extract aggregate trait/interface names implemented by the agent file (from its parse results — traits containing "Aggregate" in the name).
  - Check if any surface, entry, main, index, or container file references these aggregate names (whole-word content search).
  - Candidate reference files are pre-filtered by filename pattern: `surface_*`, `*_container.*`, `*_entry.*`, `main.*`, `lib.*`, `index.*`, `__main__.*`.
  - Agent is orphan only if **ALL** aggregates are uncalled (not ANY).
  - Agent file with no aggregate implementation → not orphan (empty aggregate list → skip check).
  - Severity: HIGH — orphaned agent means entire feature behavior is unreachable.
  - Files that fail to parse → flagged as orphan (fail-strict).
- **Edge Cases**:

  - Agent with 2 aggregates, 1 called and 1 uncalled → not orphan (not ALL uncalled).
  - Agent with 2 aggregates, both uncalled → orphan.
  - Agent with no aggregate impl → not orphan (skip).
- **Error Handling**: Files that fail to parse → orphan (fail-strict).

---

### FR-009: Surface Orphan Detection (AES506)

- **Description**: Check that surface files are reachable based on their group classification (Smart, Utility, Passive).
- **Input**: File path, alive set (from FR-003), reverse link index from the analysis context, architecture configuration.
- **Output**: Orphan indicator result with `is_orphan` flag, reason, and severity.
- **Business Rules**:

  - **Surface classification by filename suffix** (configurable via YAML):

    - **Smart**: `_command`, `_controller`, `_page`, `_router` — must be imported by entry point or container. Severity: HIGH.
    - **Utility**: `_hook`, `_store`, `_action`, `_screen` — must be imported by a Smart surface. Severity: MEDIUM.
    - **Passive**: `_component`, `_view`, `_layout`, and all other recognized surface suffixes — must be imported by Smart OR Utility surface. Severity: LOW.
  - Dependency chain: `Entry → Smart → Utility → Passive`.
  - Detection uses BFS reachability from the forward import graph and `ReverseLinkIndex`.
  - Files with **unclassifiable suffixes** (not in Smart, Utility, or Passive lists) → **skipped** (no orphan check performed).
  - Files that fail to parse → flagged as orphan (fail-strict); no PARSE_WARN diagnostic is emitted.
- **Edge Cases**:

  - Passive surface imported only by another passive surface → orphan (must be imported by Smart or Utility).
  - Smart surface not imported by any entry/container → orphan (HIGH).
  - Utility surface not imported by any Smart surface → orphan (MEDIUM).
  - Surface file with unclassifiable suffix → skipped entirely.
- **Error Handling**: Files that fail to parse → orphan (fail-strict). Unclassifiable suffix → skip (no violation, no error).

---

### FR-010: Barrel File Exception Handling

- **Description**: Skip known barrel/package marker files from orphan detection.
- **Input**: File path.
- **Output**: Skip signal (no violation produced).
- **Business Rules**:

  - `__init__.py` — Python package marker.
  - `mod.rs` — Rust module re-export.
  - `lib.rs` — Rust library root.
  - `index.ts` / `index.js` / `index.tsx` / `index.jsx` — TypeScript/JavaScript barrel files.
  - These files are package markers or re-export files, not logic.
  - Check is performed in the orchestrator before dispatching to any analyzer.
- **Edge Cases**: A file named `mod.rs` inside a deeply nested module is still skipped.
- **Error Handling**: N/A — simple filename check.

## API Contract

| Function                           | Input                                                        | Output                     | Description                                                                                  |
| ---------------------------------- | ------------------------------------------------------------ | -------------------------- | -------------------------------------------------------------------------------------------- |
| Full orphan scan                   | Target path                                                  | Lint results               | Build graph context internally, discover entry points, trace reachability, run all analyzers |
| Orphan scan with context           | Pre-built analysis context                                   | Lint results               | Orphan scan with pre-built context (avoids filesystem crate call)                            |
| Identify entry points              | File list from analysis context, configured patterns         | Set of entry point paths   | Discover all valid entry points                                                              |
| Trace reachability                 | Entry point set, import graph                                | Alive file set             | BFS from entry points through import graph                                                   |
| Check taxonomy orphan              | File path, reverse link index                                | Orphan indicator result    | AES501 — taxonomy file orphan check                                                         |
| Check contract orphan              | File path, inheritance map, workspace file list, content map | Orphan indicator result    | AES502 — contract file orphan check                                                         |
| Check capabilities orphan          | File path, alive set                                         | Orphan indicator result    | AES503 — capabilities file orphan check                                                     |
| Check utility orphan               | File path, reverse link index                                | Orphan indicator result    | AES504 — utility file orphan check                                                          |
| Check agent orphan                 | File path, workspace file list, content map                  | Orphan indicator result    | AES505 — agent file orphan check                                                            |
| Check surface orphan               | File path, alive set, reverse link index, config             | Orphan indicator result    | AES506 — surface file orphan check                                                          |
| Create default DI container        | —                                                           | Orphan detection container | Default dependency injection container                                                       |
| Create DI container with config    | Architecture configuration                                   | Orphan detection container | DI container with custom config                                                              |
| Create DI from config orchestrator | Config orchestrator reference, root directory                | Orphan detection container | Canonical DI from config orchestrator                                                        |

---

## Integration Points

- **Internal** (orphan-rules crate):

  - The orphan detection aggregate contract — aggregate trait defining the public API surface.
  - The orphan detection protocol contracts — 6 layer-specific orphan indicator protocols.
  - The orphan detection filename utility — filename parsing (stem, suffix, basename).
  - The orphan detection path utility — path resolution and segment-based ignore checking.
  - The common layer detection utility — layer detection from filename prefix.
  - The config system configuration value objects — architecture config for exceptions, rules, and orphan toggles.
  - The lint result value objects — lint result, severity, and location types.
  - The config system orchestrator aggregate — config loading from orchestrator.
  - The graph analysis context value objects — `GraphAnalysisContext`, `OrphanIndicatorResult`, `ReachabilityResult`.
- **External**:

  - **`filesystem` crate** — provides helper operations on `filesystem_aggregate`:
    - Workspace root detection (`workspace_root`), ignore filtering (`should_ignore`), directory checks (`is_dir`, `scan_directory_with_ignored`), file content reads (`read_to_string`).
    - `resolve_orphan_module_path` for Rust `#[path = "..."]` mod resolution.
  - **`shared` crate** — provides `IOrphanParserProtocol` (implemented in-crate by `OrphanParserDispatcher`, delegating to `orphan_rules::taxonomy_parser_dispatcher::parse_file_content`): Rust via `syn` AST; Python/TS via comment-aware line-based parsing. Returns `FileParseResultVO` (imports, trait defs, trait impls, class bases, mod declarations).
  - No network calls. No filesystem writes. Pure static analysis.

---

## Non-functional Requirements

- **Performance**:

  - 1,000 files < 500ms; 5,000 files < 2s; 10,000 files < 5s.
  - Graph construction and parsing are performed internally by orphan-rules (`OrphanGraphResolver`) and are included in its performance budget.
  - Orphan analysis is O(V + E) for BFS reachability + O(n) per analyzer for map lookups.
  - Contract/agent analyzers use cached search file lists and whole-word content lookups instead of re-reading files per check.
- **Memory**:

  - `GraphAnalysisContext` holds all graph data in memory. For 10,000 files with average 10 imports each, peak memory < 50MB.
  - The orchestrator additionally pre-reads all file contents into a `content_map`; the contract analyzer caches its search file list per workspace root.
- **Accuracy**:

  - **Rust** uses full AST parsing via `syn` (shared crate). **Python/TS** use comment-aware line-based parsing (shared crate) — no tree-sitter.
  - Zero false positives on transitively reachable code. A file is valid if it is transitively reachable from an entry point.
  - Comment-aware line parsing (Python/TS) and `syn` AST (Rust) reduce false positives from matches inside comments; whole-word matching is used for all identifier checks.
  - Known limitation: macro-generated code (see FR-011). Macro-generated impls are invisible → potential false orphan flags.
  - Parse failure → orphan (fail-strict). This eliminates false negatives at the cost of potential false positives for files with syntax errors.
- **Concurrency**: Thread-safe via trait object shared ownership. File-level analysis is parallelized via `rayon` (`par_iter`). Graph analysis is read-only after construction.
- **Configurability**:

  - **Hardcoded conventions (permanent, by design)**:
    - Layer detection from filename prefix (`taxonomy_*`, `contract_*`, `utility_*`, `capabilities_*`, `agent_*`, `surface_*`, `root_*`).
    - Workspace directory structure (`crates/`, `packages/`, `modules/`).
    - Barrel file names (`mod.rs`, `lib.rs`, `__init__.py`, `index.ts`).
    - Default entry point patterns (`*_container.*`, `*_entry.*`, `main.*`, `lib.*`, `index.*`).
  - **Configurable (via YAML)**:
    - Additional entry point patterns.
    - Per-layer orphan check toggle (`check_orphan`).
    - Per-rule enable/disable (AES501–AES506).
    - Per-layer exceptions.
    - Ignored paths.
    - Surface classification suffixes (Smart/Utility/Passive).

---

## Test Scenarios / QA Checklist

### Core Detection

| # | Scenario                                            | Expected                                   | Rule   |
| - | --------------------------------------------------- | ------------------------------------------ | ------ |
| 1 | Workspace with 100 files, 5 orphans across 3 layers | All 5 detected, 0 false positives          | all    |
| 2 | Circular imports between two capabilities           | Both reachable, neither flagged            | pass   |
| 3 | Workspace with zero entry points                    | All non-barrel files flagged as orphans    | all    |
| 4 | Cross-crate imports (crate A imports from crate B)  | Graph resolves correctly                   | pass   |
| 5 | Configuration disabled                              | Full orphan scan returns empty immediately | config |
| 6 | File with parse failure                             | Flagged as orphan (fail-strict)            | all    |

### Barrel Files

| # | Scenario                                 | Expected             | Rule |
| - | ---------------------------------------- | -------------------- | ---- |
| 1 | Python`__init__.py` package marker     | Skipped, not flagged | excl |
| 2 | TypeScript barrel`index.ts` re-exports | Skipped, not flagged | excl |
| 3 | Rust`mod.rs` re-exports                | Skipped, not flagged | excl |
| 4 | Rust`lib.rs` library root              | Skipped, not flagged | excl |

### AES501 — Taxonomy Orphan

| # | Scenario                                            | Expected                          | Rule   |
| - | --------------------------------------------------- | --------------------------------- | ------ |
| 1 | Taxonomy file imported by a contract file           | Not orphan                        | pass   |
| 2 | Taxonomy file imported only by other taxonomy files | Orphan (no non-taxonomy consumer) | AES501 |
| 3 | Taxonomy file with no inbound links                 | Orphan                            | AES501 |
| 4 | Taxonomy file imported by capabilities file         | Not orphan                        | pass   |

### AES502 — Contract Orphan

| # | Scenario                                          | Expected                      | Rule   |
| - | ------------------------------------------------- | ----------------------------- | ------ |
| 1 | Protocol with implementation AND callers          | Not orphan                    | pass   |
| 2 | Protocol with implementation but zero callers     | Orphan                        | AES502 |
| 3 | Protocol with callers but no implementation       | Orphan                        | AES502 |
| 4 | Aggregate re-exported in barrel file              | Not orphan (public API)       | pass   |
| 5 | Aggregate implemented by agent, called by surface | Not orphan                    | pass   |
| 6 | Contract file with no traits (only type aliases)  | Not orphan (nothing to check) | pass   |

### AES503 — Capabilities Orphan

| # | Scenario                                                           | Expected                 | Rule   |
| - | ------------------------------------------------------------------ | ------------------------ | ------ |
| 1 | Capability struct referenced in container file                     | Not orphan               | pass   |
| 2 | Capability file transitively reachable from entry point            | Not orphan               | pass   |
| 3 | Capability file not in alive set, not in any container             | Orphan                   | AES503 |
| 4 | Capability imported by other capabilities, chain reaches container | Not orphan (chain alive) | pass   |

### AES504 — Utility Orphan

| # | Scenario                                 | Expected                           | Rule   |
| - | ---------------------------------------- | ---------------------------------- | ------ |
| 1 | Utility imported by a capabilities file  | Not orphan                         | pass   |
| 2 | Utility imported only by other utilities | Orphan (utility chain = dead code) | AES504 |
| 3 | Utility with no inbound links            | Orphan                             | AES504 |
| 4 | Utility imported by agent file           | Not orphan                         | pass   |

### AES505 — Agent Orphan

| # | Scenario                                                  | Expected                      | Rule   |
| - | --------------------------------------------------------- | ----------------------------- | ------ |
| 1 | Agent aggregate called by surface file                    | Not orphan                    | pass   |
| 2 | Agent aggregate not called by any surface/entry/container | Orphan (HIGH)                 | AES505 |
| 3 | Agent with no aggregate implementation                    | Not orphan (skip check)       | pass   |
| 4 | Agent with 2 aggregates, 1 called, 1 uncalled             | Not orphan (not ALL uncalled) | pass   |
| 5 | Agent with 2 aggregates, both uncalled                    | Orphan (HIGH)                 | AES505 |

### AES506 — Surface Orphan

| # | Scenario                                                          | Expected                                | Rule   |
| - | ----------------------------------------------------------------- | --------------------------------------- | ------ |
| 1 | Smart surface (`_command`) imported by entry point              | Not orphan                              | pass   |
| 2 | Smart surface not imported by any entry/container                 | Orphan (HIGH)                           | AES506 |
| 3 | Utility surface (`_hook`) imported by Smart surface             | Not orphan                              | pass   |
| 4 | Utility surface not imported by any Smart surface                 | Orphan (MEDIUM)                         | AES506 |
| 5 | Passive surface (`_component`) imported by Smart surface        | Not orphan                              | pass   |
| 6 | Passive surface imported only by another Passive surface          | Orphan (LOW)                            | AES506 |
| 7 | Dependency chain: Entry → Smart → Utility → Passive, all alive | No violations                           | pass   |
| 8 | Remove Smart import → Utility + Passive flagged                  | Utility (MEDIUM) + Passive (LOW) orphan | AES506 |
| 9 | Surface file with unclassifiable suffix                           | Skipped (no check)                      | skip   |

### Configuration

| # | Scenario                                  | Expected                                       | Rule   |
| - | ----------------------------------------- | ---------------------------------------------- | ------ |
| 1 | Config`check_orphan: false` for a layer | No violations for that layer                   | config |
| 2 | Config with exceptions list               | Excepted files produce no violations           | config |
| 3 | Config with`ignored_paths: ["tests"]`   | `tests/` segment files produce no violations | config |
| 4 | Config with AES501 disabled               | No taxonomy orphan violations                  | config |
| 5 | Config with custom entry point patterns   | Additional entry points recognized             | config |

### Performance

| # | Scenario                                      | Expected                                   | Rule |
| - | --------------------------------------------- | ------------------------------------------ | ---- |
| 1 | 10,000 file workspace                         | Completes in under 5 seconds               | perf |
| 2 | Contract analyzer with 50 traits × 500 files | Completes in under 2 seconds (map lookups) | perf |

---

## Assumptions & Constraints

- Workspace follows AES convention with `crates/`, `packages/`, `modules/` directories.
- Naming convention validation is handled by the naming-rules crate; orphan-rules assumes filenames are correctly named.
- Entry points are identified by filename patterns (configurable), not by content analysis.
- Parsing and graph construction happen inside orphan-rules: Rust via `syn` (shared parser), Python/TS via comment-aware line-based parsing (shared parser). No tree-sitter is used.
- No network calls are required; all analysis is local filesystem.
- Configuration is loaded once and reused across all checks in a scan.
- Macro-generated code (Rust `macro_rules!`, proc macros) is not expanded — trait implementations inside macros are invisible to the detector (see FR-011).
- Parse failure → orphan (fail-strict). Files that fail to parse are flagged as orphans because reachability cannot be verified.
- Surface files with unclassifiable suffixes are skipped (no orphan check performed).
- The crate builds its own `GraphAnalysisContext` via `OrphanGraphResolver` and performs direct `std::fs` reads/walks for workspace discovery; the filesystem aggregate is used for helper operations (workspace root, ignore filtering, content reads, module path resolution).

---

## Glossary

| Term                           | Definition                                                                                                                                                                         |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **AES**                  | Agentic Engineering System — the 7-layer coding convention                                                                                                                        |
| **Orphan**               | A source file not transitively reachable from any entry point, or failing layer-specific consumer requirements                                                                     |
| **Entry point**          | A file that anchors the reachability graph (main, lib, container, entry, root)                                                                                                     |
| **Barrel file**          | A package marker or re-export file (`__init__.py`, `mod.rs`, `lib.rs`, `index.ts`)                                                                                         |
| **Alive file**           | A file reachable via BFS from any entry point through the import graph                                                                                                             |
| **DI**                   | Dependency Injection — wiring implementations to trait/interface contracts                                                                                                        |
| **Inbound link**         | A file that imports the target file (reverse import edge)                                                                                                                          |
| **AST**                  | Abstract Syntax Tree — structured representation of source code produced by a parser                                                                                              |
| **GraphAnalysisContext** | Analysis context built in-crate by`OrphanGraphResolver` containing file list, import graph, reverse link index, and inheritance map                                              |
| **InheritanceMap**       | Map of file to inherited/implemented trait, class, or interface names (class bases, class implements, trait impls)                                                                 |
| **ReverseLinkIndex**     | Map of file path to list of files that import it                                                                                                                                   |
| **ReverseLinkIndex**     | Map of file path to list of files that import it                                                                                                                                   |
| **Re-export**            | A`pub use` (Rust) or `export { X } from` (TS) that re-exports a symbol from another module                                                                                     |
| **Glob import**          | `use foo::*` (Rust) or `export * from` (TS) — imports all symbols from a module                                                                                               |
| **Smart surface**        | Surface with`_command`, `_controller`, `_page`, `_entry`, `_router` suffix — may contain orchestration                                                                  |
| **Utility surface**      | Surface with`_hook`, `_store`, `_action`, `_screen` suffix — supports smart surfaces                                                                                      |
| **Passive surface**      | Surface with`_component`, `_view`, `_layout`, or other recognized suffix — presentation-only                                                                                |
| **Filesystem crate**     | External crate providing helper operations to orphan-rules (workspace root, ignore filtering, content reads, module path resolution). Parsing and graph construction are in-crate. |
| **Segment matching**     | Path matching by splitting on`/` and comparing individual segments (not substring containment)                                                                                   |

---

## Appendix A: YAML Configuration Schema

### Top-Level Structure

```yaml
architecture:
  enabled: true
  rules:
    AES501: { ... }
    AES502: { ... }
    AES503: { ... }
    AES504: { ... }
    AES505: { ... }
    AES506: { ... }
  orphan:
    entry_points:
      - "*_container.*"
      - "*_entry.*"
      - "main.rs"
      - "lib.rs"
      - "main.py"
      - "__main__.py"
      - "main.ts"
      - "main.js"
      - "index.ts"
      - "index.js"
```

### Per-Rule Configuration

```yaml
AES501:
  enabled: true
  exceptions: []

AES502:
  enabled: true
  exceptions: []

AES503:
  enabled: true
  exceptions: []

AES504:
  enabled: true
  exceptions: []

AES505:
  enabled: true
  exceptions: []

AES506:
  enabled: true
  exceptions: []
    # Files with suffix not in any list → skipped
```

---

## Reference

- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- Filesystem crate: [FRD.md](../filesystem/FRD.md)
