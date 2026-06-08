# 📄 Feature Requirements Document (FRD)
**Feature Name:** 6-Layer AES Architecture (Core Platform)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 08/06/2026  
**Version:** v1.0  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the 6-layer AES (Agentic Engineering System) architecture that serves as the foundational structural framework for Lint Arwaky. It specifies layer hierarchy, dependency direction, naming conventions, and import rules that every module in the codebase must follow.

### 2.2 Scope
**In-Scope:**
- Definition of 6 architectural layers: taxonomy, contract, capabilities, infrastructure, agent, surfaces
- Allowed and forbidden imports per layer
- Layer-gated compilation via Cargo features
- DI container wiring contract (ServiceContainerAggregate)
- Barrel file requirements per layer
- Suffix naming conventions per layer

**Out-of-Scope:**
- Specific lint rules (covered in FR-010 to FR-017)
- External tool adapter implementations
- CLI command definitions

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES** | Agentic Engineering System — the architectural pattern used |
| **Taxonomy** | Bottom layer — domain value objects, entities, events, errors, constants |
| **Contract** | Abstract interfaces — ports, protocols, aggregates |
| **Capabilities** | Business logic — checkers, analyzers, processors |
| **Infrastructure** | Technical implementations — adapters, providers, scanners |
| **Agent** | Orchestration — DI containers, orchestrators, coordinators |
| **Surfaces** | Entry points — CLI commands, MCP handlers, views |
| **ServiceContainerAggregate** | Primary DI trait in contract layer — surfaces access infra/capabilities through this |
| **Barrel file** | `mod.rs` that re-exports all modules in a layer |
| **AES001–AES033** | Rule codes enforced by the architecture checker |

## 3. Feature Overview
### 3.1 Background & Problem
Before the 6-layer AES architecture, Lint Arwaky had no structural boundaries: all code lived in flat directories, circular dependencies were common (infrastructure importing surfaces, capabilities importing infrastructure), filenames had no conventions, and there was no way to audit architectural compliance automatically.

### 3.2 Business Goals
- Eliminate circular dependencies between layers
- Enforce unidirectional dependency flow (upper layers → lower layers)
- Provide compile-time protection via Cargo feature gating
- Enable self-audit: the tool checks its own architecture compliance
- Standardize naming so file location and suffix communicate architectural role

### 3.3 Target Users
- **Architecture Engineers**: Enforce clean architecture and DDD
- **Developers**: Understand where to place new code based on layer rules
- **AI Agents (MCP)**: Navigate and modify the codebase without violating architecture

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to know which layer a file belongs to by looking at its directory and suffix, so I can follow architecture rules without memorizing documentation.
- **US-002:** As an architect, I want the system to detect forbidden cross-layer imports automatically, so violations are caught at lint time instead of code review.
- **US-003:** As a CI pipeline, I want to run `cargo check --features check_taxonomy` to compile only the taxonomy layer, so I can verify layer isolation.

### 4.2 Use Cases & Workflow
**Layer Detection Workflow:**
```
Input: FilePath("/project/src-rust/capabilities/architecture_import_checker.rs")

Step 1: detect_layer()
  ├── Check path prefix:
  │     "src-rust/taxonomy/"  → LAYER_TAXONOMY
  │     "src-rust/contract/"  → LAYER_CONTRACT
  │     "src-rust/capabilities/" → LAYER_CAPABILITIES  ← MATCH
  │     "src-rust/infrastructure/" → LAYER_INFRASTRUCTURE
  │     "src-rust/agent/"     → LAYER_AGENT
  │     "src-rust/surfaces/"  → LAYER_SURFACES
  │     "src/" atau root → LAYER_ROOT
  │
Step 2: resolve_specialized_layer()
  └── Match file suffix:
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

**Import Validation Workflow:**
```
File: capabilities/architecture_import_checker.rs
Layer: capabilities

For each "use ..." or "import ..." in the file:

1. check_forbidden_imports() → AES001
   ├── Determine the TARGET layer from the import path
   │     "use crate::infrastructure::..." → target = infrastructure
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
         └── If there is "use crate::infrastructure::..." directly → FLAG
```

**Layer-Gated Compilation:**
```bash
# Only compile taxonomy (bottom layer)
cargo check --lib --no-default-features --features check_taxonomy

# Compile taxonomy + contract
cargo check --lib --no-default-features --features check_contract

# Compile all layers (default)
cargo check --lib
```

Feature dependency diagram in `Cargo.toml`:

```toml
[features]
check_taxonomy = []
check_contract = ["check_taxonomy"]
check_infrastructure = ["check_contract"]
check_capabilities = ["check_contract"]
check_agent = ["check_infrastructure", "check_capabilities"]
check_surfaces = ["check_agent"]
default = ["check_surfaces"]
```

If capabilities tries to import surfaces:
- Compile `--features check_capabilities` → OK (surfaces is not included)
- But at runtime `check surfaces` → ERROR because `check_capabilities` does not include surfaces
- **Compile-time protection**: surfaces is not available in capabilities scope

### 4.3 Business Rules
**Dependency Direction:**
```
surfaces → agent → capabilities + infrastructure → contract → taxonomy
```

**Layer Suffix Rules:**
| Layer | Allowed Suffixes |
|-------|-----------------|
| taxonomy | `_vo`, `_entity`, `_event`, `_error`, `_constant` |
| contract | `_port`, `_protocol`, `_aggregate` |
| capabilities | 26 suffixes (analyzer, checker, processor, ...) |
| infrastructure | 38 suffixes (adapter, provider, scanner, ...) |
| agent | 10 suffixes (container, orchestrator, coordinator, ...) |
| surfaces | 12 suffixes (command, handler, controller, ...) |

**DI Container Rule (AES023):** Surfaces must NOT import infrastructure or capabilities directly. Access goes through `ServiceContainerAggregate` in the contract layer.

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Layer detection latency per file | < 10ms |
| NFR-002 | Compile-time gating must prevent cross-layer compilation | Guaranteed by Cargo feature chain |
| NFR-003 | DI container initialization time | < 500ms |
| NFR-004 | All contract traits must be `Send + Sync` | Required |

## 6. UI/UX Requirements
This feature has no direct UI — it is an architectural constraint enforced at compile time and lint time. Developer feedback comes through:
- **Compiler errors**: When compiling with wrong feature flags
- **Lint violations**: When `lint-arwaky-cli check .` detects layer violations

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | A file in `capabilities/` imports from `infrastructure/` | `detect_layer()` runs | AES001 CRITICAL is flagged | ✅ |
| AC-002 | A surface file imports from `infrastructure/` | `check_legacy_import_rules()` runs | AES023 CRITICAL is flagged | ✅ |
| AC-003 | Taxonomy file does not import any outer layer | `check_forbidden_imports()` runs | No violation | ✅ |
| AC-004 | `cargo check --features check_taxonomy` | Compilation runs | Only taxonomy modules compile | ✅ |
| AC-005 | All `mod.rs` files in each layer | Barrel completeness check runs | All modules re-exported (AES012) | ✅ |
| AC-006 | A file uses suffix not in layer's allowed list | `check_domain_suffixes()` runs | AES010/AES008 flagged | ✅ |
| AC-007 | Self-lint runs on own codebase | `lint-arwaky-cli check .` | Detects 153+ violations across 15 AES codes | ✅ |
| AC-008 | No `unwrap()`/`panic!()` in implementation | AES014 scan runs | ~50 violations remaining — 110+ already fixed across 66 files | ⚠️ Known issue — Lazy/static Regex unwraps kept (compile-time only); auto-fix replaces unwrap() with expect() |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Source Parsing) | Layer detection requires `ISourceParserPort` to parse imports | If parser is unreliable, layer detection fails | Fallback to path-based detection |
| Cargo feature chain | Feature gating requires correct `Cargo.toml` configuration | Wrong feature chain could allow layer leakage | Enforced by `lib.rs` conditional compilation |
| YAML config | Layer definitions loaded from `lint_arwaky.config.rust.yaml` | Missing or malformed YAML causes runtime failure | Built-in `default_aes_config()` fallback embedded at compile time |

## 9. Appendices
- `docs/ARCHITECTURE.md` — Detailed AES architecture specification with Mermaid diagrams
- `docs/RULES_AES.md` — Full rule catalog (AES001–AES033)
- `lint_arwaky.config.rust.yaml` — Layer definitions in configuration
- `src-rust/lib.rs` — Feature-gated module compilation
- `src-rust/agent/dependency_injection_container.rs` — DI container wiring
- `src-rust/contract/service_container_aggregate.rs` — ServiceContainerAggregate trait
