# FRD — 6-Layer AES Architecture (Core Platform)

> **PRD Reference**: [FR-001](PRD.md) — 6-layer AES architecture enforcement
> **Dependency**: — (Foundation, no prior dependency)
> **Status**: ✅ **PRODUCTION-READY** — All 10 checkers real, DI container real, feature gating real. 269 Rust files across 6 layers. NOTE: ~170 `unwrap()` calls across codebase cause AES014 self-violations.
> **Self-lint**: `lint-arwaky-cli check .` — project audits itself under this architecture

## 1. Problem Statement

Before the 6-layer AES architecture, Lint Arwaky had:

| Issue | Description |
|-------|-------------|
| **No layer boundaries** | All code lived in flat directories without separation of concerns |
| **Circular dependencies** | Infrastructure could import surfaces, capabilities could import infrastructure — no compile-time guard |
| **No naming conventions** | Filenames were inconsistent — no suffix-based layer identification |
| **No import rules** | Any module could import any other module, creating spaghetti dependencies |
| **No self-audit** | Architecture violations were undetectable without manual review |
| **No DI contract** | Surfaces bypassed the DI container and imported infrastructure/capabilities directly |

## 2. Basic Concepts

AES divides code into 6 vertical layers with strict rules:

```
Semakin ke atas → semakin dekat ke user (CLI, MCP)
Semakin ke bawah → semakin dekat ke domain murni (VO, Entity)
```

**Golden rule**: Upper layers can use lower layers, but lower layers MUST NOT know about upper layers.

```
Surfaces ──► Contract ──► Taxonomy
Agent ──► Capabilities + Infrastructure ──► Contract ──► Taxonomy
```

### Sibling Equivalence
Capabilities and Infrastructure are **peers**. They must not import each other. Both only communicate through Contract (Ports/Protocols).

### Dependency Inversion
Surfaces never import concrete implementations. They only hold `ServiceContainerAggregate` (trait in contract). Implementations are injected through the DI container.

## 3. How It Works — Step by Step

### 3.1 Layer Detection (`architecture_compliance_analyzer.rs`)

Each file is scanned, its layer is detected with this algorithm:

```
FilePath: "src-rust/capabilities/architecture_import_checker.rs"

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

### 3.2 Import Validation (`architecture_import_checker.rs`)

After the layer is known, the imports in the file are validated:

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

### 3.3 Layer-Gated Compilation (`src-rust/lib.rs`)

Compile-time guard via Cargo features:

```rust
// lib.rs
#[cfg(feature = "check_taxonomy")]
pub mod taxonomy;

#[cfg(feature = "check_contract")]
pub mod contract;   // depends on taxonomy

#[cfg(feature = "check_infrastructure")]
pub mod infrastructure;  // depends on contract

#[cfg(feature = "check_capabilities")]
pub mod capabilities;    // depends on contract

#[cfg(feature = "check_agent")]
pub mod agent;           // depends on capabilities + infrastructure + contract

#[cfg(feature = "check_surfaces")]
pub mod surfaces;        // depends on agent + contract
```

Feature chain: `check_taxonomy` → `check_contract` → `check_infrastructure` / `check_capabilities` → `check_agent` → `check_surfaces` (default).

If someone compiles `--features check_agent` without `check_capabilities`, compilation **fails** because agent needs capabilities. This prevents circular dependencies at compile time.

### 3.4 DI Container Wiring (`agent/dependency_injection_container.rs`)

```
Entry point (cli_main_entry.rs / mcp_main_entry.rs)
    │
    ▼
Create all concrete implementations (infrastructure):
    ├── OSFileSystemAdapter
    ├── ASTRustParserAdapter
    ├── ASTPythonParserAdapter
    ├── ASTJSParserAdapter
    ├── ConfigYamlReader
    ├── LanguageDetectorProvider
    ├── PythonRuffAdapter
    ├── ...
    │
    ▼
Wrap in Arc<dyn TraitContract>:
    ├── Arc<dyn IFileSystemPort>
    ├── Arc<dyn ISourceParserPort>
    ├── Arc<dyn IConfigReaderPort>
    ├── Arc<dyn ILinterAdapterPort>
    ├── ...
    │
    ▼
Create capability (logic) with injected dependencies:
    ├── ArchitectureComplianceAnalyzer(IFileSystemPort)
    ├── ArchitectureImportChecker(ISourceParserPort)
    ├── ConfigOrchestrationProcessor(IConfigReaderPort, ILanguageDetectorPort)
    ├── ...
    │
    ▼
Create ServiceContainerAggregate (contract) → concrete implementation:
    └── DependencyInjectionContainer { all Arc }
        │
        ▼
Inject into surfaces (CLI, MCP) via Arc<dyn ServiceContainerAggregate>

Surface CALLS:   container.get_compliance_analyzer().detect_layer(path)
Surface DOES NOT: ArchitectureComplianceAnalyzer::new()  ← FORBIDDEN (AES023)
```

### 3.5 Self-Lint Flow

```
lint-arwaky-cli check .
    │
    ├─► ArchLintHandler.find_source_dir(".")
    │     └─► Search: src-rust/? src-python/? src-javascript/? src/?
    │           → found src-rust/ → language = Rust
    │
    ├─► ConfigLoaderOrchestrator.load_project_config(".")
    │     └─► Read lint_arwaky.config.rust.yaml → ArchitectureConfig
    │
    ├─► LintCheckingCoordinator.run_all_checks("src-rust/")
    │     │
    │     ├─► Walk all .rs files in src-rust/
    │     │
    │     ├─► Per file:
    │     │     ├─► detect_layer() → know which layer this file is in
    │     │     ├─► extract_imports() → know what imports this file has
    │     │     ├─► check naming (AES003) → 3-word snake_case?
    │     │     ├─► check suffix (AES011) → suffix matches layer?
    │     │     ├─► check imports (AES001, AES002) → imports allowed/not?
    │     │     ├─► check bypass (AES014) → any #[allow(...)]?
    │     │     ├─► check inheritance (AES026, AES027) → implements contract?
    │     │     ├─► check primitives (AES006) → any String/i32 in entity?
    │     │     ├─► check file size (AES004, AES005) → 10-500 lines?
    │     │     └─► ...
    │     │
    │     └─► Cross file:
    │           ├─► barrel completeness (AES012) → mod.rs exports everything?
    │           ├─► circular dependencies (AES020) → any cycles?
    │           └─► orphan detection (AES017) → file not referenced?
    │
    └─► Collect all violations → ArchitectureGovernanceEntity
          ├─► Score: 100 - (LOW*1 + MEDIUM*2 + HIGH*3 + CRITICAL*5)
          ├─► If there is CRITICAL → auto fail
          └─► Print violations grouped by severity/rule
```

## 4. Layer Specification

| Layer | Directory | Suffixes | May Import | Must Not Import |
|-------|-----------|----------|--------------|-------------------|
| **Surface (Smart)** | `surfaces/` | `_command`, `_handler`, `_controller`, `_entry` | taxonomy, contract(aggregate) | agent, capabilities, infrastructure, contract(port), contract(protocol) |
| **Surface (Utility)** | `surfaces/` | `_hook`, `_store`, `_provider`, `_router` | taxonomy, contract(aggregate) | agent, capabilities, infrastructure, Smart surfaces |
| **Surface (Passive)** | `surfaces/` | `_component`, `_layout`, `_view` | taxonomy | agent, contract, capabilities, infrastructure, all other surfaces |
| **Agent (Container)** | `agent/` | `_container`, `_registry`, `_mixin` | taxonomy, contract | surfaces |
| **Agent (Orchestrator)** | `agent/` | `_orchestrator`, `_coordinator`, `_dispatcher` | taxonomy, contract(aggregate) | surfaces, agent siblings |
| **Agent (Support)** | `agent/` | `_manager`, `_handler`, `_result`, `_state` | taxonomy | other agents, infrastructure, capabilities, surfaces |
| **Capabilities** | `capabilities/` | `_checker`, `_analyzer`, `_processor`, etc. | taxonomy, contract(protocol) | infrastructure, surfaces, agent, capabilities(sibling) |
| **Infrastructure** | `infrastructure/` | `_adapter`, `_provider`, `_scanner`, etc. | taxonomy, contract(port) | surfaces, capabilities, agent, infrastructure(sibling) |
| **Contract** | `contract/` | `_port`, `_protocol`, `_aggregate` | taxonomy, contract(sibling) | agent, infrastructure, surfaces, capabilities |
| **Taxonomy** | `taxonomy/` | `_vo`, `_entity`, `_event`, `_error`, `_constant` | taxonomy only | all other layers |

## 5. Key Files

### Taxonomy (4 files)
| File | Content |
|------|-----|
| `taxonomy/layer_names_constant.rs` | `LAYER_TAXONOMY = "taxonomy"`, `LAYER_CONTRACT = "contract"`, etc. |
| `taxonomy/layer_names_vo.rs` | Factory function: `all_core_layers()` → vec of all layers |
| `taxonomy/layer_definition_vo.rs` | `LayerDefinition { path, allowed_suffixes, allowed_imports, forbidden_imports, mandatory_imports, min_lines, max_lines }` |
| `taxonomy/layer_content_vo.rs` | `LayerNameVO`, `FileContentVO`, `LineContentVO` — data containers for parse results |

### Contract (5 traits)
| File | Trait | Method |
|------|-------|--------|
| `contract/service_container_aggregate.rs` | `ServiceContainerAggregate` | `get_compliance_analyzer()`, `get_import_checker()`, `get_lint_orchestrator()`, etc. |
| `contract/infrastructure_container_aggregate.rs` | `InfrastructureContainerAggregate` | Init all adapters |
| `contract/capability_container_aggregate.rs` | `CapabilityContainerAggregate` | Init all checkers |
| `contract/orchestrator_container_aggregate.rs` | `OrchestratorContainerAggregate` | Init orchestrator |
| `contract/adapter_container_aggregate.rs` | `AdapterContainerAggregate` | Init linter adapters |

### Capabilities (10 checker files)
| File | Function |
|------|--------|
| `architecture_compliance_analyzer.rs` | `detect_layer(path) → LayerNameVO` — core layer detection |
| `architecture_naming_checker.rs` | `check_naming(file) → Vec<Violation>` — AES003, AES011 |
| `architecture_import_checker.rs` | `check_imports(file, layer) → Vec<Violation>` — AES001, AES002, AES023 |
| `architecture_role_checker.rs` | `check_role(file, layer) → Vec<Violation>` — AES021 |
| `architecture_metric_checker.rs` | `check_metrics(file) → Vec<Violation>` — AES004, AES005, AES006 |
| `architecture_internal_checker.rs` | `check_internal(file) → Vec<Violation>` — AES012, AES013 |
| `architecture_inheritance_checker.rs` | `check_inheritance(file) → Vec<Violation>` — AES026, AES027 |
| `architecture_orphan_analyzer.rs` | `find_orphans(project) → Vec<Violation>` — AES017 |
| `architecture_cycle_analyzer.rs` | `find_cycles(project) → Vec<Violation>` — AES020 |
| `surface_hierarchy_checker.rs` | `check_surfaces(file) → Vec<Violation>` — AES018, AES019, AES022 |

## 6. Import Validation Flow (Detail)

```
Sebuah file surfaces/cli_check_command.rs di-check:

1. detect_layer("surfaces/cli_check_command.rs")
   → LAYER_SURFACES, sub-layer "command" (Smart surface)

2. extract_imports() → dapat daftar import:
   ├── "use crate::contract::service_container_aggregate::ServiceContainerAggregate"
   ├── "use crate::infrastructure::rust_linter_adapter::RustLinterAdapter"
   └── "use crate::taxonomy::FilePath"

3. check_forbidden_imports():
   ├── contract → surfaces BOLEH import contract(aggregate) ✅
   ├── infrastructure → surfaces TIDAK BOLEH import infrastructure ❌
   │     └── VIOLATION: AES001 — Layer 'surface' cannot import from 'infrastructure'
   └── taxonomy → surfaces BOLEH import taxonomy ✅

4. check_mandatory_imports():
   └── Surfaces WAJIB import: taxonomy, contract(aggregate)
         └── taxonomy ✅, contract ✅ → OK

5. AES023 check:
   └── Apakah ada import langsung infrastructure/capabilities tanpa lewat container?
         └── Ada: "use crate::infrastructure::..." → VIOLATION (AES023)
```

## 7. Barrel File Mechanism

```
Setiap layer punya mod.rs yang bertindak sebagai "pintu gerbang":

taxonomy/mod.rs:
  pub mod layer_names_constant;
  pub mod layer_names_vo;
  pub mod layer_definition_vo;
  pub mod layer_content_vo;
  // ... semua file di taxonomy/

AES012: Baris di atas WAJIB ada semua. Kalau ada file taxonomy baru
  yang tidak ditambahkan ke mod.rs → VIOLATION "barrel incompleteness"

AES013: File selain mod.rs TIDAK BOLEH punya `pub mod` atau `pub use`
  yang mengekspos modul lain. Hanya mod.rs yang boleh jadi pintu keluar.
```

## 8. Layer-Gated Compilation — How It Works

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

## 9. Files Summary

| Category | Count | File |
|----------|--------|------|
| Layer VOs/Constants | 4 | `layer_names_constant.rs`, `layer_names_vo.rs`, `layer_definition_vo.rs`, `layer_content_vo.rs` |
| Barrel files | 6 | `taxonomy/mod.rs`, `contract/mod.rs`, `capabilities/mod.rs`, `infrastructure/mod.rs`, `agent/mod.rs`, `surfaces/mod.rs` |
| DI Contract traits | 5 | `service_container_aggregate.rs`, `infrastructure_container_aggregate.rs`, `capability_container_aggregate.rs`, `orchestrator_container_aggregate.rs`, `adapter_container_aggregate.rs` |
| Layer enforcement | 10 | All files in `capabilities/` for layer logic |
| YAML configs | 3 | `lint_arwaky.config.rust.yaml`, `.python.yaml`, `.javascript.yaml` |
| Entry points | 3 | `lib.rs`, `cli_main_entry.rs`, `mcp_main_entry.rs` |

## 10. AES Compliance

| Rule | Status | Mechanism |
|------|--------|-----------|
| AES001 | ✅ | `check_forbidden_imports()` — match import target against forbidden list per layer |
| AES002 | ✅ | `check_mandatory_imports()` — ensure mandatory imports are present |
| AES003 | ✅ | `check_naming()` — regex `^\w+_\w+_\w+\.rs$` |
| AES006 | ✅ | `find_primitive_violations()` — scan type annotation: `String`, `i32`, `int`, `str` |
| AES008 | ✅ | `check_suffix()` — contract must be `_port`, `_protocol`, or `_aggregate` |
| AES011 | ✅ | `check_suffix()` — suffix must match allowed list per layer |
| AES012 | ✅ | `check_barrel()` — mod.rs must re-export all files in the layer |
| AES013 | ✅ | `check_internal()` — non-mod.rs files must not have `pub mod` / `pub use` |
| AES018 | ✅ | Utility surfaces must not import Smart surfaces |
| AES019 | ✅ | Passive surfaces may only import taxonomy |
| AES022 | ✅ | Smart surfaces must delegate via `ServiceContainerAggregate` |
| AES023 | ✅ | Surface must not directly import infra/capabilities |
| AES026 | ✅ | Contract Aggregate must not `impl PortTrait` — must use composition |
| AES027 | ✅ | Every logic file must implement at least one contract trait |
| AES033 | ✅ | `_constant` file only contains `pub const` / `pub static` |

## 11. Implementation Order

1. **Taxonomy**: Define layer names, definitions, content VOs
2. **Contract**: Define `ServiceContainerAggregate` + DI traits
3. **Infrastructure**: Implement `OSFileSystemAdapter`, config providers, parsers
4. **Capabilities**: Implement compliance analyzer, naming checker, import checker
5. **Agent**: Wire DI container
6. **Surfaces**: Wire CLI check command

## 12. Acceptance Criteria

| # | Criteria | Verification Mechanism | Status |
|---|----------|---------------------|--------|
| AC001 | Each file has the correct layer suffix | `check_suffix()` on all files in `src-rust/` | ✅ |
| AC002 | Taxonomy does not import upper layers | `check_forbidden_imports()` on taxonomy | ✅ |
| AC003 | Surface does not directly import infra/cap | `check_legacy_import_rules()` → AES023 | ✅ |
| AC004 | Barrel is complete (AES012) | `check_barrel()` on all mod.rs | ✅ |
| AC005 | `features check_taxonomy` only compiles taxonomy | `cargo check --no-default-features --features check_taxonomy` | ✅ |
| AC006 | `features check_surfaces` compiles everything | `cargo check --features check_surfaces` | ✅ |
| AC007 | Self-lint detects violations | `lint-arwaky-cli check .` → violations detected | ✅ |
| AC008 | Circular dependency detected | `architecture_cycle_analyzer` | ✅ |
| AC009 | Every logic file implements contract trait | `check_inheritance()` → AES027 | ✅ |
| AC010 | Configuration via YAML without code changes | Read `lint_arwaky.config.rust.yaml` | ✅ |
