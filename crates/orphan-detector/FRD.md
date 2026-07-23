# FRD — orphan-detector

## System Overview

The orphan-detector crate identifies dead, unused, or unreachable code components across the 7-layer AES architecture. It builds an import reachability graph starting from valid entry points (containers, binary entries, main files), then flags any source file that has been orphaned. The orchestrator dispatches to 6 layer-specific analyzers (taxonomy, contract, capabilities, utility, agent, surfaces) and produces lint violations.

```
Entry Points (main.*, lib.rs, *_entry.*, *_container.*)
        │
        ▼
┌─────────────────────┐
│ Orphan Orchestrator │  ← orphan detection aggregate trait
│ (orchestrator)      │
└────┬────┬────┬──────┘
     │    │    │
     ▼    ▼    ▼
  Taxonomy  Contract  Capabilities  Utility  Agent  Surfaces
  Analyzer  Analyzer  Analyzer      Analyzer Analyzer Analyzer
```

**Scope:** All `.rs`, `.py`, `.ts`, `.js` source files in the workspace. Naming convention validation is handled by the naming rules crate — orphan-detector assumes naming is already correct and focuses solely on reachability.

## Functional Requirements

### FR-001: Import Graph Construction
- **Description**: Build a bidirectional import graph from all workspace source files, resolving cross-crate and cross-language imports.
- **Input**: List of source file paths (`Vec<String>`) and workspace root directory.
- **Output**: A graph analysis context containing the forward import graph, reverse link index, trait/class definition map, and implementation relationship map.
- **Business Rules**:
  - Scan all `crates/`, `packages/`, `modules/` subdirectories recursively for source files.
  - Resolve `crate::`, `super::`, `mod` imports (Rust); `from module import` (Python); `import { X } from` (TypeScript).
  - Expand file list to include all workspace source files for cross-crate import resolution.
- **Edge Cases**: Files with circular imports form cycles in the graph but do not cause infinite loops (BFS with visited set). Files outside supported extensions are silently skipped.
- **Error Handling**: Unreadable files are skipped with no error. Invalid paths produce no entry in the graph.

### FR-002: Entry Point Discovery
- **Description**: Identify valid entry points that anchor the reachability graph.
- **Input**: List of file paths and optional configured entry point patterns from the architecture configuration.
- **Output**: `HashSet<String>` of entry point file paths.
- **Business Rules**:
  - Default entry point patterns: `main.rs`, `lib.rs`, `main.py`, `__main__.py`, `main.ts`, `main.js`, `index.ts`, `index.js`, `*_container.*`, `*_entry.*`.
  - Merges configured additional entry point patterns from each layer definition in the architecture configuration.
  - Deduplicates and sorts the final list.
- **Edge Cases**: Workspace with zero entry points results in all files flagged as orphans. Workspace with entry points in non-standard locations requires config override.
- **Error Handling**: Missing or inaccessible entry point files are excluded from the set.

### FR-003: Reachability Tracing
- **Description**: Perform BFS from all entry points through the import graph to determine which files are transitively reachable ("alive").
- **Input**: Entry point set (`Vec<String>`) and the forward import graph.
- **Output**: `Vec<String>` of all reachable file paths.
- **Business Rules**:
  - Uses breadth-first search with a `HashSet` visited tracker to avoid revisiting nodes.
  - A file is "alive" if it is transitively reachable from any entry point.
  - The alive set is used by capabilities, agent, and surfaces orphan analyzers.
- **Edge Cases**: Isolated files with no imports from any entry point are flagged. Entry points that import nothing are valid (they are roots).
- **Error Handling**: Cycles in the graph are handled by the visited set — no infinite loops.

### FR-004: Taxonomy Orphan Detection (AES501)
- **Description**: Check that taxonomy layer files (`taxonomy_*`) are imported by at least one file from any other layer.
- **Input**: File path, root directory, reverse link map.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - A taxonomy file is orphan if no contract, capabilities, agent, utility, or surface file imports it.
  - Internal taxonomy imports (from capabilities) are valid — taxonomy types used within capabilities but not exposed in contract signatures are not orphans.
- **Edge Cases**: Taxonomy files imported only by other taxonomy files are flagged (no consumer outside taxonomy).
- **Error Handling**: Files with no detectable imports are treated as orphan candidates.

### FR-005: Contract Orphan Detection (AES502)
- **Description**: Check that contract files have at least one implementation or consumer.
- **Input**: File path, root directory, definition map, inheritance map, all workspace files.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - Protocol contracts — Must be implemented by a capabilities file AND called by an agent or capability via dependency injection.
  - Aggregate contracts — Must be implemented by an agent file AND called by a surface file via dependency injection.
  - Port contracts — Legacy, marked for removal.
  - Detects `impl Trait for Type` (Rust), `class Foo(Trait)` (Python), `class Foo implements Trait` (TS).
- **Edge Cases**: Protocol with an implementation but zero callers is still reachable (implementation exists). Port contracts are checked but deprecated.
- **Error Handling**: Files with unparseable signatures are not flagged as orphan (fail-safe).

### FR-006: Capabilities Orphan Detection (AES503)
- **Description**: Check that capability files are wired in a root container or reachable from entry points.
- **Input**: File path, root directory, reachable file paths set.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - Capabilities use dependency injection (`Arc<T>` in Rust, DI containers in Python/TS).
  - A capability is orphan if its struct/trait names do not appear in any container file AND the file is not transitively reachable.
  - Capabilities should NOT directly import agent or other capability files (enforced by role-rules, not here).
- **Edge Cases**: A capability imported only by other capabilities in a chain is alive if any link in the chain reaches a container.
- **Error Handling**: Files with no struct/trait names detectable are treated as potential orphans.

### FR-007: Utility Orphan Detection (AES504)
- **Description**: Check that utility files are imported by at least one consumer layer (agent, capability, or surface).
- **Input**: File path, root directory, all workspace files, reverse link map.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - Utility-only import chains are flagged as dead code (utility importing utility does not count).
  - Must have inbound imports from capabilities, agent, or surface files.
- **Edge Cases**: Utility imported by another utility that is itself orphaned — the chain is dead.
- **Error Handling**: Unparseable import statements cause the utility to be treated as a candidate orphan.

### FR-008: Agent Orphan Detection (AES505)
- **Description**: Check that agent orchestrator files are called by surface layer files or binary entry points.
- **Input**: File path, root directory, all workspace files.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - Extract aggregate trait names from the agent file (e.g., the implementation of the orphan detection aggregate trait).
  - Check if any surface, entry, main, index, or container file references these aggregate names.
  - Severity: HIGH — orphaned agent means entire feature behavior is unreachable.
- **Edge Cases**: Agent file with no aggregate implementation is flagged immediately.
- **Error Handling**: Files that cannot be parsed for trait implementations are flagged as HIGH severity orphans.

### FR-009: Surface Orphan Detection (AES506)
- **Description**: Check that surface files are reachable based on their group classification (Smart, Utility, Passive).
- **Input**: File path, root directory, reachable file paths set, optional layer definition.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - **Smart** (`_command`, `_controller`, `_page`, `_entry`): Must be imported by entry point or container.
  - **Utility** (`_hook`, `_store`, `_action`, `_screen`): Must be imported by a Smart surface.
  - **Passive** (all other surface files): Must be imported by Smart OR Utility surface.
  - Dependency chain: `Entry → Smart → Utility → Passive`.
  - Detection uses inbound import graph only (no identifier-based fallback).
- **Edge Cases**: A passive surface imported only by another passive surface is orphan. Smart surfaces bypass passive checks.
- **Error Handling**: Files with unclassifiable suffixes default to Passive group.

### FR-010: Barrel File Exception Handling
- **Description**: Skip known barrel/package marker files from orphan detection.
- **Input**: File path.
- **Output**: Skip signal (no violation produced).
- **Business Rules**:
  - `__init__.py` — Python package marker.
  - `mod.rs` — Rust module re-export.
  - `index.ts` / `index.js` — TypeScript/JavaScript barrel files.
  - These files are package markers or re-export files, not logic.
- **Edge Cases**: A file named `mod.rs` inside a deeply nested module is still skipped.
- **Error Handling**: N/A — simple filename suffix check.

### FR-011: Configuration-Driven Orphan Rules
- **Description**: Respect per-layer configuration for enabling/disabling orphan checks and providing exceptions.
- **Input**: Architecture configuration with layer definitions.
- **Output**: Filtered scan results.
- **Business Rules**:
  - `ignored_paths` — Path patterns to exclude from scanning entirely.
  - `orphan_entry_points` — Additional entry point filename patterns beyond defaults.
  - `layers.<layer>.orphan.check_orphan` — Enable/disable orphan checking per layer.
  - `layers.<layer>.exceptions` — Files to exclude from orphan detection per layer.
  - When configuration is disabled, the full orphan scan returns empty immediately.
- **Edge Cases**: Empty exceptions list means all files in the layer are checked. Nonexistent ignored paths are silently ignored.
- **Error Handling**: Malformed config values default to permissive (check enabled, no exceptions).

## Data Model / Entity Relationship

```
Graph Analysis Context
├── Import Graph (forward edges)
├── Inbound Link Map (reverse index)
├── File Definition Map (trait/class definitions)
└── Inheritance Map (implementation relationships)

Orphan Indicator Result
├── is_orphan: bool
├── reason: String
└── severity: Severity

Reachability Result
└── values: Vec<FilePath>

Orphan File List
└── values: Vec<String>

Orphan Entry Pattern List
└── values: Vec<String>

Lint Result (output)
├── file: FilePath
├── line: LineNumber
├── column: ColumnNumber
├── code: ErrorCode (AES501–AES506)
├── message: LintMessage
├── source: AdapterName ("architecture")
├── severity: Severity
├── enclosing_scope: Option<ScopeRef>
└── related_locations: LocationList
```

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| Build orphan graph context | File list, root directory | Graph analysis context | Build full import graph for the workspace |
| Identify orphan entry points | File list | Set of entry point paths | Discover all valid entry points |
| Full orphan scan | File list, root directory | Lint results | Full orphan scan with graph construction |
| Orphan scan with context | File list, root directory, pre-built graph | Lint results | Orphan scan with pre-built graph (avoids rebuild) |
| Check taxonomy orphan | File path, root directory, layer definition, reverse link map | Orphan indicator result | Check single taxonomy file for orphan status |
| Check contract orphan | File path, root directory, definition map, inheritance map, all files | Orphan indicator result | Check single contract file for orphan status |
| Check capabilities orphan | File path, root directory, reachable file set | Orphan indicator result | Check single capabilities file for orphan status |
| Check utility orphan | File path, root directory, all files, reverse link map | Orphan indicator result | Check single utility file for orphan status |
| Check agent orphan | File path, root directory, all files | Orphan indicator result | Check single agent file for orphan status |
| Check surface orphan | File path, root directory, reachable file set, layer definition | Orphan indicator result | Check single surface file for orphan status |
| Create default DI container | — | Orphan detection container | Default dependency injection container |
| Create DI container with config | Architecture configuration | Orphan detection container | DI container with custom config |
| Create DI from config orchestrator | Config orchestrator reference, root directory | Orphan detection container | Canonical DI from config orchestrator |

## Integration Points

- **Internal**:
  - The code analysis shared module — graph analysis context, import graph, reverse link map, orphan indicator result, and reachability result value objects.
  - The orphan detection aggregate contract — aggregate trait defining the public API surface.
  - The orphan detection protocol contracts — 6 layer-specific orphan indicator protocols.
  - The orphan detection file I/O utility — file reading, scanning, directory checks.
  - The orphan detection filename utility — filename parsing (stem, suffix, basename).
  - The orphan detection path utility — path resolution and ignore checking.
  - The common layer detection utility — layer detection from filename prefix.
  - The config system configuration value objects — architecture config for exceptions and rules.
  - The lint result value objects — lint result, severity, and location types.
  - The config system orchestrator aggregate — config loading from orchestrator.
- **External**: None — pure static analysis, no network or filesystem writes.

## Non-functional Requirements (Detailed)

- **Performance**: 1,000 files < 500ms; 5,000 files < 2s; 10,000 files < 5s.
- **Memory**: Graph construction should hold all edges in memory; for 10,000 files with average 10 imports each, peak memory < 50MB.
- **Accuracy**: Zero false positives on transitively reachable code. A file is valid if it is transitively reachable from an entry point.
- **Concurrency**: Thread-safe via async runtime. Protocol-based shared ownership for concurrent analysis.
- **Configurability**: All behavior overridable via the architecture configuration. No hardcoded assumptions about project structure beyond workspace directory conventions.

## Test Scenarios / QA Checklist

- [ ] Workspace with 100 files, 5 orphans across 3 layers — all 5 detected, 0 false positives.
- [ ] Python nested `__init__.py` packages — barrel files skipped, not flagged as orphan.
- [ ] TypeScript barrel `index.ts` re-exports — barrel files skipped.
- [ ] Rust `mod.rs` re-exports — barrel files skipped.
- [ ] Circular imports between two capabilities — both reachable, neither flagged.
- [ ] Contract protocol with implementation but zero callers — still flagged as orphan.
- [ ] Agent file with no aggregate implementation — flagged as HIGH severity orphan.
- [ ] Surface dependency chain: Smart → Utility → Passive — all alive. Remove Smart import — Utility + Passive flagged.
- [ ] Config with `check_orphan: false` for a layer — no violations for that layer.
- [ ] Config with exceptions list — excepted files produce no violations.
- [ ] Config with `ignored_paths` — excluded paths produce no violations.
- [ ] Workspace with zero entry points — all non-barrel files flagged as orphans.
- [ ] Cross-crate imports (crate A imports from crate B) — graph resolves correctly.
- [ ] 10,000 file workspace completes in under 5 seconds.
- [ ] Configuration disabled — full orphan scan returns empty immediately.

## Assumptions & Constraints

- Workspace follows AES convention with `crates/`, `packages/`, `modules/` directories.
- Naming convention validation is handled by the naming rules crate; orphan-detector assumes filenames are correctly named.
- Entry points are identified by filename patterns, not by content analysis.
- Import resolution is language-specific (Rust `mod`/`crate`, Python `import`/`from`, TypeScript `import`).
- No network calls are required; all analysis is local filesystem.
- Configuration is loaded once and reused across all checks in a scan.

## Glossary

| Term | Definition |
|------|------------|
| **Orphan** | A source file not transitively reachable from any entry point |
| **Entry point** | A file that anchors the reachability graph (main, lib, container, entry) |
| **Barrel file** | A package marker or re-export file (`__init__.py`, `mod.rs`, `index.ts`) |
| **Alive file** | A file reachable via BFS from any entry point through the import graph |
| **AES** | Architecture Enforcement Standard — the 7-layer coding convention |
| **DI** | Dependency Injection — wiring implementations to trait/interface contracts |
| **Inbound link** | A file that imports the target file (reverse import edge) |

## Reference

- PRD: [PRD.md](../../PRD.md)
