# FRD — orphan-detector

## System Overview

The orphan-detector crate identifies dead, unused, or unreachable code components across the 7-layer AES architecture. It builds an import reachability graph starting from valid entry points (containers, binary entries, main files), then flags any source file that has been orphaned. The orchestrator dispatches to 6 layer-specific analyzers (taxonomy, contract, capabilities, utility, agent, surfaces) and produces `LintResult` violations.

```
Entry Points (main.*, lib.rs, *_entry.*, *_container.*)
        │
        ▼
┌─────────────────────┐
│ ArchOrphanAnalyzer  │  ← IOrphanAggregate trait
│ (orchestrator)      │
└────┬────┬────┬──────┘
     │    │    │
     ▼    ▼    ▼
  Taxonomy  Contract  Capabilities  Utility  Agent  Surfaces
  Analyzer  Analyzer  Analyzer      Analyzer Analyzer Analyzer
```

**Scope:** All `.rs`, `.py`, `.ts`, `.js` source files in the workspace. Naming convention validation is handled by the `naming-rules` crate — orphan-detector assumes naming is already correct and focuses solely on reachability.

## Functional Requirements

### FR-001: Import Graph Construction
- **Description**: Build a bidirectional import graph from all workspace source files, resolving cross-crate and cross-language imports.
- **Input**: List of source file paths (`Vec<String>`) and workspace root directory.
- **Output**: `GraphAnalysisContext` containing `ImportGraph` (forward edges), `InboundLinkMap` (reverse index), `FileDefinitionMap` (trait/class definitions), and `InheritanceMap` (implementation relationships).
- **Business Rules**:
  - Scan all `crates/`, `packages/`, `modules/` subdirectories recursively for source files.
  - Resolve `crate::`, `super::`, `mod` imports (Rust); `from module import` (Python); `import { X } from` (TypeScript).
  - Expand file list to include all workspace source files for cross-crate import resolution.
- **Edge Cases**: Files with circular imports form cycles in the graph but do not cause infinite loops (BFS with visited set). Files outside supported extensions are silently skipped.
- **Error Handling**: Unreadable files are skipped with no error. Invalid paths produce no entry in the graph.

### FR-002: Entry Point Discovery
- **Description**: Identify valid entry points that anchor the reachability graph.
- **Input**: List of file paths and optional configured entry point patterns from `ArchitectureConfig`.
- **Output**: `HashSet<String>` of entry point file paths.
- **Business Rules**:
  - Default entry point patterns: `main.rs`, `lib.rs`, `main.py`, `__main__.py`, `main.ts`, `main.js`, `index.ts`, `index.js`, `*_container.*`, `*_entry.*`.
  - Merges configured `orphan_entry_points` from each layer definition in `ArchitectureConfig`.
  - Deduplicates and sorts the final list.
- **Edge Cases**: Workspace with zero entry points results in all files flagged as orphans. Workspace with entry points in non-standard locations requires config override.
- **Error Handling**: Missing or inaccessible entry point files are excluded from the set.

### FR-003: Reachability Tracing
- **Description**: Perform BFS from all entry points through the import graph to determine which files are transitively reachable ("alive").
- **Input**: Entry point set (`Vec<String>`) and `ImportGraph`.
- **Output**: `Vec<String>` of all reachable file paths.
- **Business Rules**:
  - Uses breadth-first search with a `HashSet` visited tracker to avoid revisiting nodes.
  - A file is "alive" if it is transitively reachable from any entry point.
  - The `alive` set is used by capabilities, agent, and surfaces orphan analyzers.
- **Edge Cases**: Isolated files with no imports from any entry point are flagged. Entry points that import nothing are valid (they are roots).
- **Error Handling**: Cycles in the graph are handled by the visited set — no infinite loops.

### FR-004: Taxonomy Orphan Detection (AES501)
- **Description**: Check that taxonomy layer files (`taxonomy_*`) are imported by at least one file from any other layer.
- **Input**: File path, root directory, `InboundLinkMap`.
- **Output**: `OrphanIndicatorResult` with `is_orphan`, `reason`, and `severity`.
- **Business Rules**:
  - A taxonomy file is orphan if no `contract_*`, `capabilities_*`, `agent_*`, `utility_*`, or `surface_*` file imports it.
  - Internal taxonomy imports (from capabilities) are valid — taxonomy types used within capabilities but not exposed in contract signatures are not orphans.
- **Edge Cases**: Taxonomy files imported only by other taxonomy files are flagged (no consumer outside taxonomy).
- **Error Handling**: Files with no detectable imports are treated as orphan candidates.

### FR-005: Contract Orphan Detection (AES502)
- **Description**: Check that contract files have at least one implementation or consumer.
- **Input**: File path, root directory, `FileDefinitionMap`, `InheritanceMap`, all workspace files.
- **Output**: `OrphanIndicatorResult` with `is_orphan`, `reason`, and `severity`.
- **Business Rules**:
  - `contract_*_protocol` — Must be implemented by a `capabilities_*` file AND called by an agent or capability via DI.
  - `contract_*_aggregate` — Must be implemented by an `agent_*` file AND called by a `surface_*` file via DI.
  - `contract_*_port` — Legacy, marked for removal.
  - Detects `impl Trait for Type` (Rust), `class Foo(Trait)` (Python), `class Foo implements Trait` (TS).
- **Edge Cases**: Protocol with an implementation but zero callers is still reachable (implementation exists). Port contracts are checked but deprecated.
- **Error Handling**: Files with unparseable signatures are not flagged as orphan (fail-safe).

### FR-006: Capabilities Orphan Detection (AES503)
- **Description**: Check that capability files are wired in a root container or reachable from entry points.
- **Input**: File path, root directory, `ReachabilityResult` (alive files set).
- **Output**: `OrphanIndicatorResult` with `is_orphan`, `reason`, and `severity`.
- **Business Rules**:
  - Capabilities use DI (`Arc<T>` in Rust, DI containers in Python/TS).
  - A capability is orphan if its struct/trait names do not appear in any `*_container.*` file AND the file is not transitively reachable.
  - Capabilities should NOT directly import agent or other capability files (enforced by role-rules, not here).
- **Edge Cases**: A capability imported only by other capabilities in a chain is alive if any link in the chain reaches a container.
- **Error Handling**: Files with no struct/trait names detectable are treated as potential orphans.

### FR-007: Utility Orphan Detection (AES504)
- **Description**: Check that utility files are imported by at least one consumer layer (agent, capability, or surface).
- **Input**: File path, root directory, all workspace files, `InboundLinkMap`.
- **Output**: `OrphanIndicatorResult` with `is_orphan`, `reason`, and `severity`.
- **Business Rules**:
  - Utility-only import chains are flagged as dead code (utility importing utility does not count).
  - Must have inbound imports from `capabilities_*`, `agent_*`, or `surface_*` files.
- **Edge Cases**: Utility imported by another utility that is itself orphaned — the chain is dead.
- **Error Handling**: Unparseable import statements cause the utility to be treated as a candidate orphan.

### FR-008: Agent Orphan Detection (AES505)
- **Description**: Check that agent orchestrator files are called by surface layer files or binary entry points.
- **Input**: File path, root directory, all workspace files.
- **Output**: `OrphanIndicatorResult` with `is_orphan`, `reason`, and `severity`.
- **Business Rules**:
  - Extract aggregate trait names from the agent file (e.g., `impl IOrphanAggregate for ArchOrphanAnalyzer`).
  - Check if any `surface_*`, `*_entry.*`, `main.*`, `index.*`, or `*_container.*` file references these aggregate names.
  - Severity: HIGH — orphaned agent means entire feature behavior is unreachable.
- **Edge Cases**: Agent file with no aggregate implementation is flagged immediately.
- **Error Handling**: Files that cannot be parsed for trait implementations are flagged as HIGH severity orphans.

### FR-009: Surface Orphan Detection (AES506)
- **Description**: Check that surface files are reachable based on their group classification (Smart, Utility, Passive).
- **Input**: File path, root directory, `ReachabilityResult`, optional `LayerDefinition`.
- **Output**: `OrphanIndicatorResult` with `is_orphan`, `reason`, and `severity`.
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
- **Input**: `ArchitectureConfig` with layer definitions.
- **Output**: Filtered scan results.
- **Business Rules**:
  - `ignored_paths` — Path patterns to exclude from scanning entirely.
  - `orphan_entry_points` — Additional entry point filename patterns beyond defaults.
  - `layers.<layer>.orphan.check_orphan` — Enable/disable orphan checking per layer.
  - `layers.<layer>.exceptions` — Files to exclude from orphan detection per layer.
  - When `config.enabled` is false, `check_orphans` returns empty immediately.
- **Edge Cases**: Empty exceptions list means all files in the layer are checked. Nonexistent ignored paths are silently ignored.
- **Error Handling**: Malformed config values default to permissive (check enabled, no exceptions).

## Data Model / Entity Relationship

```
GraphAnalysisContext
├── ImportGraph { mapping: HashMap<String, Vec<String>> }
├── InboundLinkMap { mapping: HashMap<String, Vec<String>> }
├── FileDefinitionMap { mapping: HashMap<String, Vec<String>> }
└── InheritanceMap { mapping: HashMap<String, Vec<String>> }

OrphanIndicatorResult
├── is_orphan: bool
├── reason: String
└── severity: Severity

ReachabilityResult
└── values: Vec<FilePath>

OrphanFileListVO
└── values: Vec<String>

OrphanEntryPatternListVO
└── values: Vec<String>

LintResult (output)
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
| `IOrphanAggregate::build_orphan_graph_context` | `&[String]` files, `&str` root_dir | `GraphAnalysisContext` | Build full import graph for the workspace |
| `IOrphanAggregate::identify_orphan_entry_points` | `&[String]` files | `HashSet<String>` | Discover all valid entry points |
| `IOrphanAggregate::check_orphans` | `&[String]` files, `&str` root_dir | `Vec<LintResult>` | Full orphan scan with graph construction |
| `IOrphanAggregate::check_orphans_with_context` | `&[String]` files, `&str` root_dir, `&GraphAnalysisContext` | `Vec<LintResult>` | Orphan scan with pre-built graph (avoids rebuild) |
| `ITaxonomyOrphanProtocol::is_taxonomy_orphan` | `FilePath`, `FilePath`, `Option<LayerDefinition>`, `InboundLinkMap` | `OrphanIndicatorResult` | Check single taxonomy file for orphan status |
| `IContractOrphanProtocol::is_contract_orphan` | `FilePath`, `FilePath`, `FileDefinitionMap`, `InheritanceMap`, `&[String]` | `OrphanIndicatorResult` | Check single contract file for orphan status |
| `ICapabilitiesOrphanProtocol::is_capabilities_orphan` | `FilePath`, `FilePath`, `ReachabilityResult` | `OrphanIndicatorResult` | Check single capabilities file for orphan status |
| `IUtilityOrphanProtocol::is_utility_orphan` | `FilePath`, `FilePath`, `&[String]`, `InboundLinkMap` | `OrphanIndicatorResult` | Check single utility file for orphan status |
| `IAgentOrphanProtocol::is_agent_orphan` | `FilePath`, `FilePath`, `&[String]` | `OrphanIndicatorResult` | Check single agent file for orphan status |
| `ISurfacesOrphanProtocol::is_surface_orphan` | `FilePath`, `FilePath`, `ReachabilityResult`, `Option<LayerDefinition>` | `OrphanIndicatorResult` | Check single surface file for orphan status |
| `OrphanContainer::new` | — | `OrphanContainer` | Default DI container |
| `OrphanContainer::new_with_config` | `ArchitectureConfig` | `OrphanContainer` | DI container with custom config |
| `OrphanContainer::from_orchestrator` | `&Arc<dyn IConfigOrchestratorAggregate>`, `&str` | `OrphanContainer` | Canonical DI from config orchestrator |

## Integration Points

- **Internal**:
  - `shared::code_analysis::taxonomy_analysis_vo` — `GraphAnalysisContext`, `ImportGraph`, `InboundLinkMap`, `OrphanIndicatorResult`, `ReachabilityResult` VOs.
  - `shared::orphan_detector::contract_orphan_aggregate` — `IOrphanAggregate` trait (aggregate contract).
  - `shared::orphan_detector::contract_orphan_protocol` — 6 layer-specific orphan indicator protocols.
  - `shared::orphan_detector::utility_orphan_io` — File I/O (read, scan, is_dir).
  - `shared::orphan_detector::utility_orphan_filename` — Filename parsing (stem, suffix, basename).
  - `shared::orphan_detector::utility_orphan_path` — Path resolution and ignore checking.
  - `shared::common::utility_layer_detector` — Layer detection from filename prefix.
  - `shared::config_system::taxonomy_config_vo` — `ArchitectureConfig` for exceptions/rules.
  - `shared::taxonomy_lint_vo` — `LintResult`, `Severity`, `Location` types.
  - `shared::config_system::contract_config_orchestrator_aggregate` — Config loading from orchestrator.
- **External**: None — pure static analysis, no network or filesystem writes.

## Non-functional Requirements (Detailed)

- **Performance**: 1,000 files < 500ms; 5,000 files < 2s; 10,000 files < 5s.
- **Memory**: Graph construction should hold all edges in memory; for 10,000 files with average 10 imports each, peak memory < 50MB.
- **Accuracy**: Zero false positives on transitively reachable code. A file is valid if it is transitively reachable from an entry point.
- **Concurrency**: Thread-safe via `Send + Sync` on all protocol traits. Container uses `Arc<dyn ...>` for shared ownership.
- **Configurability**: All behavior overridable via `ArchitectureConfig`. No hardcoded assumptions about project structure beyond workspace directory conventions.

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
- [ ] `config.enabled: false` — `check_orphans` returns empty immediately.

## Assumptions & Constraints

- Workspace follows AES convention with `crates/`, `packages/`, `modules/` directories.
- Naming convention validation is handled by the `naming-rules` crate; orphan-detector assumes filenames are correctly named.
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
