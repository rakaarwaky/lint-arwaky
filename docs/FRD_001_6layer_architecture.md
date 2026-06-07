# FRD — 6-Layer AES Architecture (Core Platform)

> **PRD Reference**: [FR-001](PRD.md) — 6-layer AES architecture enforcement
> **Dependency**: — (Foundation, no prior dependency)
> **Status**: ✅ COMPLETE — Implemented as the core architecture of the project
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

## 2. Solution Overview

The 6-layer AES architecture defines strict vertical layering with one-way dependency flow:

| Layer | Directory | Suffixes (per ARCHITECTURE.md) | Responsibility |
|-------|-----------|----------|----------------|
| **Surface** | `src-rust/surfaces/` | `_command`, `_handler`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_entry`, `_hook`, `_store`, `_provider` | User-facing entry points — CLI, MCP |
| **Agent** | `src-rust/agent/` | `_container`, `_orchestrator`, `_coordinator`, `_registry`, `_manager`, `_mixin`, `_dispatcher`, `_handler`, `_result`, `_state` | Composition & DI wiring |
| **Capability** | `src-rust/capabilities/` | `_analyzer`, `_checker`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_handler`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_aggregator`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_actions` | Business logic, rule checking |
| **Infrastructure** | `src-rust/infrastructure/` | `_adapter`, `_provider`, `_scanner`, `_client`, `_constants`, `_schemas`, `_lifespan`, `_wrapper`, `_tracer`, `_tracker`, `_variants`, `_detector`, `_patterns`, `_util`, `_system`, `_repository`, `_cache`, `_store`, `_loader`, `_writer`, `_reader`, `_driver`, `_connector`, `_gateway`, `_serializer`, `_encoder`, `_decoder`, `_fetcher`, `_watcher`, `_indexer`, `_dispatcher`, `_recorder`, `_proxy`, `_publisher`, `_subscriber`, `_listener`, `_poller`, `_streamer` | Concrete I/O implementations |
| **Contract** | `src-rust/contract/` | `_port`, `_protocol`, `_aggregate` | Abstract interfaces (traits) |
| **Taxonomy** | `src-rust/taxonomy/` | `_vo`, `_entity`, `_event`, `_error`, `_constant` | Domain value objects & foundation |

### Dependency Flow

```
surfaces → contract → taxonomy
agent → capabilities + infrastructure → contract → taxonomy
capabilities → contract → taxonomy
infrastructure → contract → taxonomy
contract → taxonomy
taxonomy → taxonomy
```

Surfaces must NOT import `agent`, `capabilities`, or `infrastructure` directly — they access capabilities and infrastructure only through `ServiceContainerAggregate` in the contract layer (AES023, AES022). Per ARCHITECTURE.md, agent imports are also forbidden for surfaces.

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│  SURFACES    (25 files — CLI, MCP, controllers, views)  │
│  _command _handler _controller _page _view _component    │
│  _router _layout _entry _hook _store _provider           │
├─────────────────────────────────────────────────────────┤
│  AGENT       (22 files — orchestration, DI, lifecycle)  │
│  _container _orchestrator _coordinator _registry          │
│  _manager _mixin _dispatcher _handler _result _state      │
├──────────────────────────┬──────────────────────────────┤
│  CAPABILITIES  (29)      │  INFRASTRUCTURE  (41)        │
│  _checker _analyzer      │  _adapter _provider _scanner  │
│  _processor _evaluator   │  _client _fetcher _watcher    │
│  _resolver _builder ...  │  _loader _driver _gateway ... │
├──────────────────────────┴──────────────────────────────┤
│  CONTRACT    (71 files — ports, protocols, aggregates)  │
│  _port _protocol _aggregate                              │
├─────────────────────────────────────────────────────────┤
│  TAXONOMY    (70 files — domain foundation)             │
│  _vo _entity _error _event _constant                     │
└─────────────────────────────────────────────────────────┘
```

### Layer-Gated Compilation

`src-rust/lib.rs` conditionally compiles modules by Cargo feature, enforcing the dependency chain at compile time:

```
check_taxonomy     → cargo check --lib --features check_taxonomy
check_contract     → cargo check --lib --features check_contract
check_infrastructure → cargo check --lib --features check_infrastructure
check_capabilities → cargo check --lib --features check_capabilities
check_agent        → cargo check --lib --features check_agent
check_surfaces     → cargo check --lib --features check_surfaces
```

Feature chain: `check_taxonomy` → `check_contract` → `check_infrastructure` / `check_capabilities` → `check_agent` → `check_surfaces` (default).

## 3. Taxonomy Changes

### Layer Definition Files

| File | Contents |
|------|----------|
| `taxonomy/layer_names_constant.rs` | Layer name constants: `LAYER_TAXONOMY`, `LAYER_CONTRACT`, etc. |
| `taxonomy/layer_names_vo.rs` | Factory functions: `layer_taxonomy()`, `layer_contract()`, `all_core_layers()` |
| `taxonomy/layer_definition_vo.rs` | `LayerDefinition` struct — path, suffix policies, import rules, bypass rules, line limits |
| `taxonomy/layer_content_vo.rs` | `LayerNameVO`, `FileContentVO`, `Identity`, `LineContentVO` |

### Existing VOs to Keep

- `ArchitectureConfig` — unchanged
- `LayerNameVO` — unchanged
- `LayerDefinition` — unchanged
- `ArchitectureRule` — unchanged
- `NamingConfig` — unchanged

## 4. Contract Changes

### New Traits (Already Defined)

| Trait | File | Purpose |
|-------|------|---------|
| `ServiceContainerAggregate` | `contract/service_container_aggregate.rs` | **Primary DI trait** — surfaces go through this to access capabilities/infrastructure (AES023) |
| `InfrastructureContainerAggregate` | `contract/infrastructure_container_aggregate.rs` | Infrastructure initialization |
| `CapabilityContainerAggregate` | `contract/capability_container_aggregate.rs` | Capability initialization |
| `OrchestratorContainerAggregate` | `contract/orchestrator_container_aggregate.rs` | Orchestrator initialization |
| `AdapterContainerAggregate` | `contract/adapter_container_aggregate.rs` | Adapter initialization |

### Barrel Files (One Per Layer)

| Layer | Barrel File | Responsibility |
|-------|-------------|----------------|
| taxonomy | `taxonomy/mod.rs` | Re-exports all VOs, entities, errors, events, constants |
| contract | `contract/mod.rs` | Re-exports all ports, protocols, aggregates |
| capabilities | `capabilities/mod.rs` | Re-exports all checkers, analyzers, processors |
| infrastructure | `infrastructure/mod.rs` | Re-exports all adapters, providers, scanners |
| agent | `agent/mod.rs` | Re-exports all orchestrators, containers, registries |
| surfaces | `surfaces/mod.rs` | Re-exports all commands, handlers, controllers |

Each barrel must comply with AES012 (barrel completeness) and AES013 (no re-exports in sub-modules).

## 5. Infrastructure Changes

### YAML Config Files

| File | Language | Description |
|------|----------|-------------|
| `lint_arwaky.config.rust.yaml` | Rust | Primary config — 6 layers + root with 40+ architecture rules |
| `lint_arwaky.config.python.yaml` | Python | Same architecture adapted for Python projects |
| `lint_arwaky.config.javascript.yaml` | JavaScript | Same architecture adapted for JS/TS projects |

## 6. Capability Changes

### Layer Enforcement Engine

| File | Responsibility |
|------|----------------|
| `capabilities/architecture_compliance_analyzer.rs` | Core layer detection engine: `detect_layer()`, `detect_module_layer()`, `resolve_specialized_layer()` |
| `capabilities/architecture_naming_checker.rs` | Layer-specific naming/suffix enforcement (AES003, AES011) |
| `capabilities/architecture_import_checker.rs` | Layer import rule enforcement (AES001, AES023) |
| `capabilities/architecture_role_checker.rs` | Role-based layer checking (AES021) |
| `capabilities/architecture_metric_checker.rs` | Layer-aware metric collection |
| `capabilities/architecture_internal_checker.rs` | Layer-internal structure checks (AES013) |
| `capabilities/architecture_inheritance_checker.rs` | Layer inheritance checks (AES026) |
| `capabilities/architecture_orphan_analyzer.rs` | Orphan detection per layer (AES017) |
| `capabilities/architecture_cycle_analyzer.rs` | Layer dependency cycle detection |
| `capabilities/surface_hierarchy_checker.rs` | Surface layer hierarchy rules (AES018, AES019) |

### Layer Detection Algorithm

```
Input: FilePath
  │
  ├─► detect_layer() → match file path against registered layer paths
  │     - Check if path starts with known layer root (src-rust/taxonomy/, etc.)
  │     - Check recursive flag (taxonomy, contract, etc. are recursive; root is not)
  │     - Refine to sub-layer if applicable (taxonomy(vo), taxonomy(entity), etc.)
  │
  ├─► detect_module_layer() → extract layer from module path
  │
  └─► resolve_specialized_layer() → match suffix pattern to sub-layer
        - _vo → taxonomy(vo)
        - _entity → taxonomy(entity)
        - _port → contract(port)
        - _protocol → contract(protocol)
        - etc.
```

### Import Validation Flow

```
FileContentVO (parsed source)
  │
  ├─► check_forbidden_imports() → AES001
  │     - Does this layer allow importing from the target layer?
  │     - If forbidden, flag violation
  │
  ├─► check_mandatory_imports() → AES002
  │     - Does this layer REQUIRE certain imports?
  │     - If missing, flag violation
  │
  └─► check_legacy_import_rules() → AES023
        - Surface-specific: no direct infra/cap imports
        - Orchestrator-specific: only contract(aggregate)
```

## 7. Agent Changes

### Updated: `DependencyInjectionContainer`

File: `agent/dependency_injection_container.rs`

Wires all layers through `ServiceContainerAggregate`:
- File system, command executor, path normalization, source parser
- Architecture linter (via `IArchLintProtocol`)
- Linter adapters (Ruff, Bandit, MyPy, ESLint, etc.)

## 8. Surface Changes

No changes needed — surfaces already delegate to capabilities/infrastructure through `ServiceContainerAggregate` (a contract aggregate), not through direct imports. The DI container is created at the root entry point (`cli_main_entry.rs`) and injected into surfaces.

## 9. Files Summary

### Existing Files (Architecture Core — No New Files Needed)

FR-001 is already **fully implemented**. No new files need to be created.

| Category | Count | Files |
|----------|-------|-------|
| Layer VOs/Constants | 4 | `layer_names_constant.rs`, `layer_names_vo.rs`, `layer_definition_vo.rs`, `layer_content_vo.rs` |
| Barrel files | 6 | `taxonomy/mod.rs`, `contract/mod.rs`, `capabilities/mod.rs`, `infrastructure/mod.rs`, `agent/mod.rs`, `surfaces/mod.rs` |
| DI Contract traits | 5 | `service_container_aggregate.rs`, `infrastructure_container_aggregate.rs`, `capability_container_aggregate.rs`, `orchestrator_container_aggregate.rs`, `adapter_container_aggregate.rs` |
| Layer enforcement engine | 10 | All files in `capabilities/` that reference layer logic |
| YAML configs | 3 | `lint_arwaky.config.rust.yaml`, `.python.yaml`, `.javascript.yaml` |
| Root entry points | 3 | `lib.rs`, `cli_main_entry.rs`, `mcp_main_entry.rs` |

## 10. AES Compliance

| Rule | Status | Notes |
|------|--------|-------|
| AES001 | ✅ | Layer import violations detected and flagged; surfaces forbidden from importing agent/infra/capabilities |
| AES002 | ✅ | Mandatory imports per layer enforced (taxonomy for contract, contract(protocol) for capabilities, etc.) |
| AES003 | ✅ | 3-word filename convention enforced |
| AES006 | ✅ | Primitive usage restricted in taxonomy(entity\|error\|event) and contract; exempt in _vo and _constant |
| AES008 | ✅ | Contract suffixes enforced (_port, _protocol, _aggregate) |
| AES011 | ✅ | Layer-specific suffix rules enforced per allowed list in ARCHITECTURE.md |
| AES012 | ✅ | Barrel completeness enforced (mod.rs must export all) |
| AES013 | ✅ | No re-exports in non-barrel sub-modules |
| AES018 | ✅ | Utility surfaces must NOT import Smart surfaces |
| AES019 | ✅ | Passive surfaces must import taxonomy only |
| AES022 | ✅ | Surface layer rules enforced — Smart surfaces delegate via ServiceContainerAggregate; Passive surfaces are I/O only |
| AES023 | ✅ | Surfaces go through ServiceContainerAggregate (contract), NOT direct agent/infra/cap imports |
| AES026 | ✅ | Contract Aggregate must not inherit from Port/Protocol — use composition |
| AES027 | ✅ | Every logic file implements a contract trait |
| AES033 | ✅ | _constant files must contain only pub const/pub static |

## 11. Implementation Order

1. **Taxonomy**: Define layer names, definitions, and content VOs
2. **Contract**: Define `ServiceContainerAggregate` and DI container traits
3. **Infrastructure**: Implement `OSFileSystemAdapter` and config providers
4. **Capabilities**: Implement compliance analyzer, naming checker, import checker
5. **Agent**: Wire `DependencyInjectionContainer` with all layers
6. **Surfaces**: Wire CLI check command to use container

## 12. Acceptance Criteria

| # | Kriteria | Status |
|---|----------|--------|
| AC001 | Setiap file di `src-rust/` memiliki suffix layer yang sesuai (`_vo`, `_port`, `_checker`, dll) | ✅ |
| AC002 | Layer bawah (taxonomy) tidak boleh import layer atas (surfaces, agent) | ✅ |
| AC003 | Surface tidak boleh langsung import infrastructure/capabilities — harus melalui `ServiceContainerAggregate` | ✅ |
| AC004 | Semua barrel (`mod.rs`) lengkap mengekspos semua modul di layer-nya (AES012) | ✅ |
| AC005 | `cargo check --features check_taxonomy` hanya mengcompile taxonomy | ✅ |
| AC006 | `cargo check --features check_surfaces` mengcompile semua layer | ✅ |
| AC007 | `lint-arwaky-cli check .` berhasil mendeteksi pelanggaran arsitektur pada dirinya sendiri | ✅ |
| AC008 | Dependency cycle antar layer terdeteksi dan dilaporkan sebagai violation | ✅ |
| AC009 | Setiap logic file (bukan VO/constant) mengimplementasikan minimal satu contract trait (AES027) | ✅ |
| AC010 | Konfigurasi layer (suffix, import rules) dapat diubah melalui YAML tanpa perubahan kode | ✅ |
