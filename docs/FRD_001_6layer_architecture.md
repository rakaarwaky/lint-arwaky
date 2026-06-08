# 📄 Feature Requirements Document (FRD)
**Feature Name:** 6-Layer AES Architecture (Core Platform)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.1  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v1.1 | 09/06/2026 | Raka | Updated to prefix-based architecture: layers are file prefixes, not directories; code organized into 26 vertical feature folders; removed layer-gated Cargo features | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the 6-layer AES (Agentic Engineering System) architecture that serves as the foundational structural framework for Lint Arwaky. It specifies layer hierarchy, dependency direction, naming conventions, and import rules that every module in the codebase must follow. Layers are identified by **filename prefix** (not directory path), and code is organized into **26 vertical feature folders** rather than 6 layer directories.

### 2.2 Scope
**In-Scope:**
- Definition of 6 architectural layers: taxonomy, contract, capabilities, infrastructure, agent, surfaces
- Layer identification via filename prefix (`taxonomy_`, `contract_`, `capabilities_`, `infrastructure_`, `agent_`, `surface_`)
- 26 feature folders for vertical slicing (e.g., `layer-rules/`, `naming-rules/`, `cli-commands/`, `mcp-server/`)
- Allowed and forbidden imports per layer
- DI container wiring contract (ServiceContainerAggregate)
- Barrel file requirements per feature folder
- Suffix naming conventions per layer

**Out-of-Scope:**
- Specific lint rules (covered in FR-010 to FR-017)
- External tool adapter implementations
- CLI command definitions

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES** | Agentic Engineering System — the architectural pattern used |
| **Taxonomy** | Bottom layer — domain value objects, entities, events, errors, constants. Files prefixed `taxonomy_` |
| **Contract** | Abstract interfaces — ports, protocols, aggregates. Files prefixed `contract_` |
| **Capabilities** | Business logic — checkers, analyzers, processors. Files prefixed `capabilities_` |
| **Infrastructure** | Technical implementations — adapters, providers, scanners. Files prefixed `infrastructure_` |
| **Agent** | Orchestration — DI containers, orchestrators, coordinators. Files prefixed `agent_` |
| **Surfaces** | Entry points — CLI commands, MCP handlers, views. Files prefixed `surface_` |
| **ServiceContainerAggregate** | Primary DI trait in contract layer — surfaces access infra/capabilities through this |
| **Barrel file** | `mod.rs` that re-exports all modules in a feature folder |
| **Vertical slicing** | Code organized by feature (26 folders), not by layer. Layer is inferred from filename prefix |
| **AES001–AES033** | Rule codes enforced by the architecture checker |

## 3. Feature Overview
### 3.1 Background & Problem
Before the 6-layer AES architecture, Lint Arwaky had no structural boundaries: all code lived in flat directories, circular dependencies were common (infrastructure importing surfaces, capabilities importing infrastructure), filenames had no conventions, and there was no way to audit architectural compliance automatically.

After the initial 6-directory implementation, developers struggled with discoverability — features were scattered across 6 large directories, causing duplicate files and confusion. The solution: **vertical slicing** (26 feature folders) while keeping the 6 layers as file prefixes.

### 3.2 Business Goals
- Eliminate circular dependencies between layers
- Enforce unidirectional dependency flow (upper layers → lower layers)
- Enable self-audit: the tool checks its own architecture compliance
- Standardize prefix+suffix naming so file name communicates architectural role
- Improve code discoverability via vertical feature folders

### 3.3 Target Users
- **Architecture Engineers**: Enforce clean architecture and DDD
- **Developers**: Understand where to place new code based on layer rules and feature folder
- **AI Agents (MCP)**: Navigate and modify the codebase without violating architecture

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to know which layer a file belongs to by looking at its directory and suffix, so I can follow architecture rules without memorizing documentation.
- **US-002:** As an architect, I want the system to detect forbidden cross-layer imports automatically, so violations are caught at lint time instead of code review.
- **US-003:** As a CI pipeline, I want to run `cargo check --features check_taxonomy` to compile only the taxonomy layer, so I can verify layer isolation.

### 4.2 Use Cases & Workflow
**Layer Detection Workflow (Prefix-Based):**
```
Input: FilePath("/project/src-rust/layer-rules/capabilities_import_checker.rs")
                     ├── feature folder: layer-rules/
                     └── filename: capabilities_import_checker.rs

Step 1: extract_layer_from_prefix()
  └── Parse filename prefix before first underscore:
        "taxonomy_"       → LAYER_TAXONOMY
        "contract_"       → LAYER_CONTRACT
        "capabilities_"   → LAYER_CAPABILITIES  ← MATCH
        "infrastructure_" → LAYER_INFRASTRUCTURE
        "agent_"          → LAYER_AGENT
        "surface_"        → LAYER_SURFACES
        (none)            → LAYER_ROOT

Step 2: resolve_sub_layer_from_suffix()
  └── Match file suffix after last underscore:
        "_vo"       → taxonomy(vo)
        "_entity"   → taxonomy(entity)
        "_error"    → taxonomy(error)
        "_event"    → taxonomy(event)
        "_constant" → taxonomy(constant)
        "_port"     → contract(port)
        "_protocol" → contract(protocol)
        "_aggregate" → contract(aggregate)
        "_checker"  → capabilities(checker)   ← MATCH
        "_analyzer" → capabilities(analyzer)
        ...
        (fallback → general layer)

Result: LAYER_CAPABILITIES + sub-layer "checker"
```

**Import Validation Workflow (Prefix-Based):**
```
File: layer-rules/capabilities_import_checker.rs
Layer (from prefix): capabilities

For each "use ..." or "import ..." in the file:

1. check_forbidden_imports() → AES001
   ├── Determine the TARGET layer from the import symbol prefix
   │     "use crate::infrastructure_fs_scanner::..." → target = infrastructure
   │
   └── Check rules:
         capabilities MAY import: taxonomy, contract(protocol)
         capabilities MUST NOT import: infrastructure, surfaces, agent, capabilities(sibling)
         └── infrastructure is in the forbidden list → FLAG VIOLATION (AES001)

2. check_mandatory_imports() → AES002
   └── capabilities MUST import: taxonomy, contract(protocol)
         └── If taxonomy is not imported → FLAG VIOLATION (AES002)

3. AES023 (surfaces only):
   └── Surface may only access infra/cap via ServiceContainerAggregate
         └── If there is "use crate::infrastructure_..." directly → FLAG
```

**Layer Enforcement (Lint-Time, Not Compile-Time):**
With prefix-based architecture, layer enforcement happens at lint time (AES rules) rather than compile time. The layered compilation via Cargo features was removed because code is no longer grouped by layer directory — files from all layers coexist within each of the 26 feature folders. AES rules (AES001/AES023) enforce import boundaries at lint time instead.

### 4.3 Business Rules
**Dependency Direction:**
```
surfaces → agent → capabilities + infrastructure → contract → taxonomy
```

**Layer Prefix + Suffix Rules:**
| Layer | Prefix | Allowed Suffixes (from config) |
|-------|--------|-------------------------------|
| taxonomy | `taxonomy_` | `_vo`, `_entity`, `_event`, `_error`, `_constant` |
| contract | `contract_` | `_port`, `_protocol`, `_aggregate` |
| capabilities | `capabilities_` | `analyzer`, `checker`, `processor`, `evaluator`, `resolver`, `validator`, `formatter`, `executor`, `transformer`, `calculator`, `builder`, `compiler`, `aggregator`, `classifier`, `extractor`, `reporter`, `mapper`, `filter`, `collector`, `comparator`, `scorer`, `inspector`, `reviewer`, `assessor`, `actions` |
| infrastructure | `infrastructure_` | `adapter`, `provider`, `scanner`, `client`, `constants`, `schemas`, `lifespan`, `wrapper`, `tracer`, `tracker`, `variants`, `detector`, `patterns`, `util`, `system`, `repository`, `cache`, `store`, `loader`, `writer`, `reader`, `driver`, `connector`, `gateway`, `serializer`, `encoder`, `decoder`, `fetcher`, `watcher`, `indexer`, `dispatcher`, `recorder`, `proxy`, `publisher`, `subscriber`, `listener`, `poller`, `streamer` |
| agent | `agent_` | `container`, `manager`, `orchestrator`, `registry`, `coordinator` (Primary); `mixin`, `result`, `state` (Support) |
| surfaces | `surface_` | `command`, `handler`, `controller`, `page`, `view`, `component`, `router`, `layout`, `entry`, `hook`, `store`, `provider` |

Suffix lists are defined in `lint_arwaky.config.rust.yaml` and enforced at lint time (AES010/AES011). Each suffix is unique per layer — no suffix may appear in multiple layers (e.g., `_handler` exists only in surfaces, not capabilities or agent).

**DI Container Rule (AES023):** Surfaces (`surface_` prefix) must NOT import infrastructure or capabilities directly. Access goes through `ServiceContainerAggregate` in the contract layer.

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Layer detection latency per file | < 10ms |
| NFR-002 | Prefix parsing accuracy | 100% (deterministic, no I/O) |
| NFR-003 | DI container initialization time | < 500ms |
| NFR-004 | All contract traits must be `Send + Sync` | Required |

## 6. UI/UX Requirements
This feature has no direct UI — it is an architectural constraint enforced at lint time. Developer feedback comes through:
- **Lint violations**: When `lint-arwaky-cli check .` detects layer prefix violations (AES001/023)
- **Naming violations**: When filename does not follow `[layer]_[concept]_[suffix].rs` pattern (AES003)

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | A file prefixed `capabilities_` imports a type prefixed `infrastructure_` | `check_forbidden_imports()` runs | AES001 CRITICAL is flagged | Pending Review |
| AC-002 | A file prefixed `surface_` imports a type prefixed `infrastructure_` | `check_surface_dependency()` runs | AES023 CRITICAL is flagged | Pending Review |
| AC-003 | Taxonomy-prefixed file does not import any outer layer | `check_forbidden_imports()` runs | No violation | Pending Review |
| AC-004 | A file is named without proper `[layer]_[concept]_[suffix]` pattern | `check_naming()` runs | AES003 flagged | Pending Review |
| AC-005 | All `mod.rs` files in each feature folder | Barrel completeness check runs | All modules re-exported (AES012) | Pending Review |
| AC-006 | A file uses suffix not in layer's allowed list | `check_suffix()` runs | AES010/AES011 flagged | Pending Review |
| AC-007 | Self-lint runs on own codebase | `lint-arwaky-cli check .` | Detects violations across AES codes | Pending Review |
| AC-008 | No `unwrap()`/`panic!()` in implementation | AES014 scan runs | ~50 violations remaining — 110+ already fixed across 66 files | Pending Review Known issue — Lazy/static Regex unwraps kept (compile-time only); auto-fix replaces unwrap() with expect() |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

The 6-layer AES architecture is implemented through a distributed set of checker modules coordinated by `agent_compliance_orchestrator.rs`:

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| Layer detection & config resolution | `layer-rules/capabilities_compliance_analyzer.rs` | 390 | **FULLY IMPLEMENTED** |
| Import rules (AES001/AES002/AES023) | `layer-rules/capabilities_import_checker.rs` | 557 | **FULLY IMPLEMENTED** |
| Import processor | `layer-rules/capabilities_import_processor.rs` | 527 | **FULLY IMPLEMENTED** |
| Surface hierarchy (AES018/AES019) | `layer-rules/capabilities_hierarchy_checker.rs` | 371 | **FULLY IMPLEMENTED** |
| Barrel/internal (AES012/AES013) | `layer-rules/capabilities_internal_checker.rs` | 154 | **FULLY IMPLEMENTED** |
| Inheritance (AES026/AES027) | `layer-rules/capabilities_inheritance_checker.rs` | 212 | **FULLY IMPLEMENTED** |
| Cycle detection (AES020) | `layer-rules/capabilities_cycle_analyzer.rs` | 221 | **FULLY IMPLEMENTED** |
| Unused check (AES015) | `layer-rules/capabilities_unused_checker.rs` | 139 | **FULLY IMPLEMENTED** |
| Compliance orchestration | `layer-rules/agent_compliance_orchestrator.rs` | 485 | **FULLY IMPLEMENTED** |
| Layer definitions | `layer-rules/taxonomy_definition_vo.rs` | 323 | **FULLY IMPLEMENTED** |
| Rule definitions | `layer-rules/taxonomy_rule_vo.rs` | 302 | **FULLY IMPLEMENTED** |
| Suffix definitions | `layer-rules/taxonomy_suffix_vo.rs` | 155 | **FULLY IMPLEMENTED** |
| Naming checker (AES003) | `naming-rules/capabilities_naming_checker.rs` | — | **FULLY IMPLEMENTED** |
| Role checker (AES021/AES022/AES024) | `role-rules/capabilities_role_checker.rs` | — | **DEAD CODE** (unwired) |
| Orphan detector (AES017) | `orphan-detector/capabilities_orphan_analyzer.rs` | — | **FULLY IMPLEMENTED** |
| Primitive checker (AES006) | `primitive-checker/capabilities_type_checker.rs` | — | **ORPHAN** (unused inline checker active instead) |

- Invoked from `agent_compliance_orchestrator.rs:455` (`check_compliance`)
- Prefix-based layer detection implemented in `ArchComplianceAnalyzer::detect_module_layer()` (uses filename prefix, not directory path)

### 8.2 Bugs Found — Status

| # | Bug | Status | Fixed In |
|---|---|---|---|
| 1 | **Project-level config file parsed but thrown away** — `agent_loading_orchestrator.rs:55-62` always returns `default_config_for_language()` | **✅ FIXED** — now reads file content and parses through `parse_config_yaml()` | `agent_loading_orchestrator.rs` |
| 2 | **Path-based detection in inline checkers** — `check_agent_role`, `check_surface_role`, `check_surface_imports`, `check_capability_routing`, `check_single_bottleneck`, `check_missing_vo` used `file.contains("/agent/")` etc. | **✅ FIXED** — now use prefix-based layer parameter (`"agent"`, `"surfaces"`, etc.) | `agent_checking_coordinator.rs` |
| 3 | **ArchRoleChecker unwired** — 658 lines, 7 sub-role checks, zero callers | **⏳ PENDING** — needs DI wiring via ServiceContainerAggregate | — |
| 4 | **Inline checks before layer detection** — 13 checkers run on ALL files, including non-Rust/barrel | **✅ FIXED** — layer-dependent checks moved after `detect_layer()` | `agent_checking_coordinator.rs` |
| 5 | **Legacy import checker path-based** — `detect_module_layer()` didn't use prefix detection | **✅ FIXED** — added prefix-matching + Rust `::` separator support | `capabilities_import_checker.rs` |
| 6 | **AES003 naming doesn't validate prefix-layer** — only word count | **⏳ PENDING** — minor, prefix determines layer (by design) | — |

### 8.3 What Needs to Be Added (Remaining)

- **Wire ArchRoleChecker**: Integrate into `run_all_checks()` via DI container
- **Logging for undetected files**: warn when `detect_layer()` returns `None`
- **Test fixtures**: verify fixed bugs (config loading, prefix detection) with automated tests

### 8.4 What to Keep (Already Implemented)

- **Prefix-based layer detection** in `ArchComplianceAnalyzer` — `extract_layer_from_prefix()` with PREFIX_MAP
- **Path-based fallback** for root entry files (`cli_main_entry.rs`, `mcp_main_entry.rs`)
- **Inline bypass/unused/barrel checks** — AES014/AES015/AES007 (layer-independent)
- **ArchRoleChecker implementation** — 658 lines, 7 sub-role checks (just needs wiring)
- **YAML config structure** — per-layer violation messages, suffix policies, import rules
- **Barrel-aware suffix resolution** in import checker — type→suffix map via barrel files

### 8.5 Empirical Evidence from Test Projects

| File | What it Tests | Active Check Result | Dead Code Would Catch |
|------|--------------|--------------------|-----------------------|
| `test-project-rust/src-rust/agent/large_orchestrator.rs` | 358-line orchestrator | **Flagged** AES021 (line count > 300) | Also flagged (stateless execution) |
| `test-project-rust/src-rust/agent/stateful_orchestrator.rs` | Mutable `counter: u32` field | **NOT flagged** (line count ~40 < 300) | Flagged AES021 (mutable state in orchestrator) |
| `test-project-python/src-python/agent/stateful.py` | `self.state`, `self.executed` | **NOT flagged** (no line count check) | Flagged AES021 (state persistence) |
| `test-project-rust/src-rust/surfaces/passive_bad_view.rs` | Passive surface imports contract | **DEPENDS** on prefix detection | Flagged AES019 |
| `test-project-rust/src-rust/capabilities/forbidden_suffix_vo.rs` | `_vo` suffix in capabilities | **NOT flagged** | Flagged AES011 |

**Summary**: 4 of 6 bugs fixed. Active checks now use prefix-based layer detection (FRD v1.1 compliant). Stateful orchestrators still pass undetected — ArchRoleChecker remains unwired.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Source Parsing) | Layer detection requires `ISourceParserPort` to parse imports | If parser is unreliable, layer detection fails | Fallback to path-based detection |
| Cargo feature chain | Feature gating requires correct `Cargo.toml` configuration | Wrong feature chain could allow layer leakage | Enforced by `lib.rs` conditional compilation |
| YAML config | Layer definitions loaded from `lint_arwaky.config.rust.yaml` | Missing or malformed YAML causes runtime failure | Built-in `default_aes_config()` fallback embedded at compile time |

## 10. Appendices
- `docs/RULES_AES.md` — Full rule catalog (AES001–AES033)
- `lint_arwaky.config.rust.yaml` — Layer definitions in configuration
- `src-rust/lib.rs` — Module declarations with `#[path]` for all 26 feature folders
- `src-rust/di-containers/agent_injection_container.rs` — DI container wiring
- `src-rust/di-containers/contract_service_aggregate.rs` — ServiceContainerAggregate trait
- `src-rust/layer-rules/` — Layer import/compliance/cycle checker implementations
