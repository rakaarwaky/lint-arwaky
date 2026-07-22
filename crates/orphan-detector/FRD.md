# FRD — orphan-detector

## Feature Goal

The orphan-detector crate identifies dead, unused, or unreachable code components across the 7-layer architecture. By building an import reachability graph starting from valid entry points (containers, binary entries, main files), it flags any source file that has been orphaned, preventing codebase bloat and keeping the system maintainable.

**Scope:** All `.rs`, `.py`, `.ts`, `.js` source files in the workspace. Naming convention validation is handled by the `naming-rules` crate — orphan-detector assumes naming is already correct and focuses solely on reachability.

## Workspace Assumptions

The orphan-detector assumes the workspace follows AES convention with these directories:

- `crates/` — Rust workspace members
- `packages/` — TypeScript/JavaScript packages
- `modules/` — Python modules or sub-projects

Entry points are identified by filename patterns: `main.*`, `lib.rs`, `index.*`, `*_entry.*`, `*_container.*`, `root_*`.

## Barrel File Exceptions

The following files are explicitly skipped from orphan detection (they are package markers or re-export files, not logic):

- `__init__.py` — Python package marker
- `mod.rs` — Rust module re-export
- `index.ts` / `index.js` — TypeScript/JavaScript barrel files

## Requirements & Scope

### AES501 — Taxonomy Orphan Checker

- **Requirement:** Taxonomy layer files (e.g. `taxonomy_*`) must be imported by at least one file from any other layer (contract, capabilities, agent, utility, or surface).
- **Rationale:** Taxonomy defines the domain foundation. If no other layer uses a taxonomy type, it may be dead code. Some taxonomy types are used internally by capabilities (not exposed in contract signatures), which is valid.
- **Detection:** Check inbound imports — taxonomy file is orphan if no `contract_*`, `capabilities_*`, `agent_*`, `utility_*`, or `surface_*` file imports it.

### AES502 — Contract Orphan Checker

- **Requirement:** Contract files must have at least one implementation or consumer.
- **Sub-rules:**
  - `contract_*_protocol` — Must be implemented by a `capabilities_*` file AND called by an agent or other capability via dependency injection.
  - `contract_*_aggregate` — Must be implemented by an `agent_*` file AND called by a `surface_*` file via dependency injection.
  - `contract_*_port` — **Legacy.** Marked for removal. Currently checked but will be deprecated.
- **Detection:** Extract trait/interface names from contract file, search for `impl Trait for Type` (Rust), `class Foo(Trait)` (Python), `class Foo implements Trait` (TS) across workspace.

### AES503 — Capabilities Orphan Checker

- **Requirement:** Capability files (e.g. `capabilities_*`) must be wired in a root container.
- **Rationale:** Capabilities use dependency injection (Rust: `Arc<T>`, Python/TS: DI containers). They should NOT directly import agent or other capability files.
- **Detection:** Check if capability's struct/trait names appear in any `*_container.*` file. Also check reachability from entry points.

### AES504 — Utility Orphan Checker

- **Requirement:** Utility files (e.g. `utility_*`) must be imported and consumed by at least one of: agent, capability, or surface layer.
- **Detection:** Check inbound imports from consumer layers (`capabilities_*`, `agent_*`, `surface_*`). Utility-only import chains are flagged as dead code.

### AES505 — Agent Orphan Checker

- **Requirement:** Agent orchestrator files (e.g. `agent_*_orchestrator`) must be called by surface layer files or binary entry points.
- **Detection:** Extract aggregate trait names from agent file (e.g. `impl IOrphanAggregate for ArchOrphanAnalyzer`). Check if any `surface_*`, `*_entry.*`, `main.*`, `index.*`, or `*_container.*` file references these aggregate names.
- **Severity:** HIGH — orphaned agent means entire feature behavior is unreachable.

### AES506 — Surface Orphan Checker

- **Requirement:** Surface files must be reachable based on their group classification.
- **Detection method:** Inbound import graph only (no identifier-based fallback).

| Group | Roles | Orphan Rule | Detection |
|-------|-------|-------------|-----------|
| **Smart** | `_command`, `_controller`, `_page`, `_router` | Must be imported by entry point or container | Check inbound imports from `main.*`, `index.*`, `*_entry.*`, `*_container.*` |
| **Utility** | `_hook`, `_store`, `_action`, `_screen` | Must be imported by a Smart surface | Check inbound imports from `surface_*_command`, `surface_*_controller`, `surface_*_page`, `surface_*_router` |
| **Passive** | `_component`, `_view`, `_layout` | Must be imported by Smart OR Utility surface | Check inbound imports from any `surface_*` file |

**Dependency chain:** `Entry → Smart → Utility → Passive`

- **Severity:** HIGH — orphaned surface means user-facing behavior is unreachable.

## Configuration

- `ignored_paths` — List of path patterns to exclude from scanning.
- `orphan_entry_points` — Additional entry point filename patterns beyond the defaults.
- `layers.<layer>.orphan.check_orphan` — Enable/disable orphan checking per layer.
- `layers.<layer>.exceptions` — Files to exclude from orphan detection per layer.

## Success Indicators

- [ ] Dead code identification — 100% detection of unused or unreachable source files.
- [ ] Zero false warnings on valid code — a file is valid if it is transitively reachable from an entry point.
- [ ] Configuration flexibility — correctly respects rule-specific exceptions and ignored path patterns.
- [ ] Performance targets:
  - 1,000 files: < 500ms
  - 5,000 files: < 2s
  - 10,000 files: < 5s
- [ ] Workspace cleanliness — keeps the production binary lightweight and clean of deprecated/unused components.

## Shared Module Dependencies

Critical shared modules used by orphan-detector:

| Module | Purpose | Criticality |
|--------|---------|-------------|
| `shared::code_analysis::taxonomy_analysis_vo` | Graph analysis VOs (ImportGraph, InboundLinkMap, etc.) | Critical |
| `shared::orphan_detector::contract_orphan_protocol` | Protocol traits for all 6 orphan analyzers | Critical |
| `shared::orphan_detector::contract_orphan_aggregate` | IOrphanAggregate trait | Critical |
| `shared::orphan_detector::utility_orphan_io` | File I/O utilities (read, scan, is_file) | Critical |
| `shared::orphan_detector::utility_orphan_filename` | Filename parsing (stem, suffix, basename) | Critical |
| `shared::orphan_detector::utility_orphan_path` | Path resolution and ignore checking | Critical |
| `shared::common::utility_layer_detector` | Layer detection from filename prefix | Critical |
| `shared::config_system::taxonomy_config_vo` | ArchitectureConfig for exceptions/rules | Critical |
| `shared::taxonomy_lint_vo` | LintResult, Severity, Location types | Critical |
| `shared::orphan_detector::utility_orphan` | Module normalization and token extraction | Optional |
| `shared::orphan_detector::utility_workspace` | Workspace root detection, container wiring check | Optional |
| `shared::orphan_detector::utility_file_cache` | Cached file reading | Optional |

## Multi-Language Test Scenarios

Acceptance tests must cover:

1. **Python:** Nested `__init__.py` packages, `from module import Class` patterns
2. **TypeScript:** Barrel `index.ts` re-exports, `import { X } from './module'` patterns
3. **Rust:** `mod.rs` re-exports, `crate::` and `super::` import paths, `#[path]` attributes
4. **Cross-language:** Workspace with mixed Rust + Python + TypeScript crates

## Acceptance Criteria for Config Exceptions

- Files listed in `ignored_paths` must produce zero violations.
- Files matching `orphan_entry_points` patterns must be treated as valid entry points.
- Layer-specific `exceptions` lists must suppress orphan detection for those files.
- `check_orphan: false` for a layer must skip all files in that layer.
