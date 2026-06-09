# 📄 Feature Requirements Document (FRD)
**Feature Name:** Circular Dependency Cycle Analyzer (AES020)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.0

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 09/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the circular dependency cycle analyzer (AES020) that detects cycles in the layer dependency graph. Layers must form a DAG following the strict bottom-up ordering: taxonomy → contract → capabilities/infrastructure → agent → surfaces. Uses DFS cycle detection on all `use`/`import` statements collected across files.

### 2.2 Scope
**In-Scope:**
- DFS-based cycle detection on the import dependency graph
- Layer assignment by file prefix (taxonomy_, contract_, capabilities_, infrastructure_, agent_, surface_)
- Cycle path reporting (source → target → ... → source)
- CRITICAL severity for every detected cycle
- Integration with `LintCheckingCoordinator` for inclusion in full `check` runs

**Out-of-Scope:**
- Intra-file dependency analysis (handled by AES001/AES010)
- External crate dependency analysis
- Fixing cycles (manual refactoring required)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **DependencyGraph** | Directed graph where nodes are layers, edges are import directions |
| **DFS cycle detection** | Depth-first search with back-edge detection to find cycles |
| **Cycle path** | Ordered list of layers forming the cycle (e.g., surfaces → agent → capabilities → surfaces) |
| **DAG** | Directed Acyclic Graph — the required structure for layer dependencies |
| **CircularDependencyCheckResult** | Struct containing all detected cycles with paths and severity |

## 3. Feature Overview
### 3.1 Background & Problem
Circular dependencies between layers violate the core AES architecture principle. Before this feature, there was no automated check for cycles — developers could accidentally introduce a `surface → agent → capabilities → surface` cycle without detection. The import graph spans 26 feature folders and thousands of files, making manual cycle detection infeasible.

### 3.2 Business Goals
- Enforce strict DAG layering at all times
- Provide clear cycle paths for debugging
- Fail the build on any circular dependency (CRITICAL)
- Integrate into the existing `check` pipeline

### 3.3 Target Users
- **Developers**: Detect cycles before committing
- **Architecture Engineers**: Review and break cycles in the dependency graph
- **CI/CD Pipelines**: Block PRs that introduce cycles

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want the linter to detect cycles when I run `check .`, so I know if my imports violate the layer DAG.
- **US-002:** As an architect, I want a clear cycle path (A → B → C → A) in the output, so I can identify which dependency to break.
- **US-003:** As a CI pipeline, I want any cycle to fail the run with CRITICAL severity, so cycles never reach production.

### 4.2 Use Cases & Workflow
**Cycle Detection Pipeline:**
```
Input: DependencyGraph (all layers × import edges)

1. Build graph nodes:
   ├── taxonomy_    → level 0
   ├── contract_    → level 1
   ├── capabilities_ → level 2
   ├── infrastructure_ → level 2
   ├── agent_       → level 3
   └── surface_     → level 4

2. DFS from each node:
   ├── For each outgoing edge (source → target):
   │     ├── If target already in current path → CYCLE DETECTED
   │     └── Record cycle path
   └── Mark node as visited

3. Report:
   ├── Each cycle → CRITICAL violation
   └── "AES020: Circular dependency: surface_ → agent_ → capabilities_ → surface_"
```

**Example Detection:**
```
File: src-rust/cli-commands/surface_command_handler.rs
  use crate::code_analysis::agent_checking_coordinator;  // allowed: surface → agent

File: src-rust/code-analysis/agent_checking_coordinator.rs
  use crate::file_system::infrastructure_file_watcher;    // allowed: agent → infra

File: src-rust/file-system/infrastructure_file_watcher.rs
  use crate::cli_commands::surface_main_entry;            // CYCLE: infra → surface!

Result: AES020 CRITICAL
  Cycle: surface → agent → infrastructure → surface
```

### 4.3 Business Rules
- Cycle detection runs across ALL layers (taxonomy through surfaces)
- Every cycle is CRITICAL severity regardless of cycle length
- Self-cycles (layer importing itself) are flagged as cycles
- Only `use` and `import` statements are considered edges
- Build all edges first, then run DFS — no incremental graph building

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Full dependency graph build + cycle detection (1000 files) | < 500ms |
| NFR-002 | Cycle path reporting accuracy | 100% |
| NFR-003 | False positive rate | 0% |

## 6. UI/UX Requirements
CLI output:
```
AES020 CRITICAL: Circular dependency detected
  Cycle: surface → agent → infrastructure → surface
  Path:
    surface_command_handler.rs:12 → agent_checking_coordinator.rs:45
    agent_checking_coordinator.rs:45 → infrastructure_file_watcher.rs:78
    infrastructure_file_watcher.rs:78 → surface_main_entry.rs:23
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Import graph has valid DAG structure | `run_cycle_detection()` | No cycles reported | Pending Review |
| AC-002 | Import graph has cycle (A → B → C → A) | `run_cycle_detection()` | Cycle path reported, severity CRITICAL | Pending Review |
| AC-003 | Self-cycle (layer imports same layer) | `run_cycle_detection()` | Self-cycle detected, reported | Pending Review |
| AC-004 | Empty graph (no imports) | `run_cycle_detection()` | No cycles, clean result | Pending Review |
| AC-005 | 3-node cycle in 1000 file graph | Full detection | Cycle found in < 500ms | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| Cycle analyzer | `layer-rules/capabilities_cycle_analyzer.rs` | 221 | **FULLY IMPLEMENTED** — DFS cycle detection, path reporting |
| Import statement parsing | `source-parsing/` (composite orchestrator) | — | **FULLY IMPLEMENTED** — 3-language import extraction |
| Dependency graph build | `layer-rules/capabilities_import_checker.rs` | — | **FULLY IMPLEMENTED** — builds layer-level edges from file imports |
| Integration in coordinator | `code-analysis/agent_checking_coordinator.rs` | — | **FULLY IMPLEMENTED** — wired into `run_all_checks()` |

### 8.2 Bugs Found

1. **Graph construction uses full file paths, not layer prefixes** (`capabilities_cycle_analyzer.rs`)
   - The analyzer may group by directory instead of layer prefix
   - With 26 feature folders, two files in different folders but same layer are treated as separate nodes
   - **Impact**: False cycles reported when files in same layer are in different feature folders
   - **Fix**: Normalize nodes by layer prefix before cycle detection

2. **No cycle deduplication** — if cycle A→B→C→A is detected from both A and B start nodes, it's reported twice
   - **Impact**: Duplicate violations in output
   - **Fix**: Track reported cycles as sorted tuple sets, skip duplicates

### 8.3 What Needs to Be Added

- **Layer normalization**: Group all files by layer prefix before building graph
- **Cycle deduplication**: Report each unique cycle once regardless of DFS start node
- **Integration test**: Test with known cycle in test-project-rust/

### 8.4 What to Keep

- **DFS algorithm** ✅ — correct back-edge detection
- **Path reporting** ✅ — complete cycle path with file locations
- **CRITICAL severity** ✅ — correctly marks all cycles as CRITICAL

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli check .` runs cycle detection against own codebase
- Test projects contain intentional cycles for verification
- Pending Review: All acceptance criteria after layer-normalization fix

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Import extraction from source files | Regex misses some imports | All major import patterns covered |
| FR-001 (Architecture) | Layer definitions for node grouping | Architecture changes break normalization | Config-driven layer definitions |
| CapabilitiesImportChecker | Builds the initial edge list | Missing edges = undetected cycles | Edge list cross-validated with raw imports |

## 10. Appendices
- `src-rust/layer-rules/capabilities_cycle_analyzer.rs` — DFS cycle detection (221 lines, working)
- `src-rust/layer-rules/capabilities_import_checker.rs` — Import extraction + edge building
- `src-rust/code-analysis/agent_checking_coordinator.rs` — Orchestration integration
- `docs/RULES_AES.md` — AES020 rule definition
