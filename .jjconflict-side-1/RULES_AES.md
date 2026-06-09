# AES (Agentic Engineering System) Rules — v2.0

Rules enforced by the `lint-arwaky` architecture checker on layer boundaries and code structure.

> **Renumbering (v2.0)**: AES codes have been reorganized into 4 logical groups with fresh numbering. See [Old-to-New Mapping](#old-to-new-mapping) at the bottom. Active codes: **27** across 4 groups.

---

## Group 1: Layer & Import Boundary (AES001–AES002)

Enforces strict import direction between architectural layers. Layer is identified by **filename prefix** (`taxonomy_`, `contract_`, etc.), not directory path.

### AES001 — Import Layer Violation (CRITICAL)

Satu rule dengan **13 sub-conditions** — masing-masing punya `allowed`, `mandatory`, `forbidden`.

| # | Scope | Allowed Imports | Mandatory Imports | Forbidden Imports |
|---|---|---|---|---|
| 1 | `taxonomy(vo)` | taxonomy | None | agent_, infrastructure_, surface_, contract_, capabilities_, root |
| 2 | `taxonomy(entity,error,event)` | taxonomy | taxonomy(vo\|constant) | agent_, infrastructure_, surface_, contract_, capabilities_, root |
| 3 | `taxonomy(constant)` | taxonomy | None | agent_, infrastructure_, surface_, contract_, capabilities_, root |
| 4 | `contract(port\|protocol)` | taxonomy, contract | taxonomy | agent_, infrastructure_, surface_, capabilities_, contract(aggregate), root |
| 5 | `contract(aggregate)` | taxonomy, contract | taxonomy, contract(port\|protocol\|aggregate) | agent_, infrastructure_, surface_, capabilities_, root |
| 6 | `capabilities` | taxonomy, contract | taxonomy, contract(protocol) | infrastructure_, surface_, agent_, capabilities_, root |
| 7 | `infrastructure` | taxonomy, contract | taxonomy, contract(port) | surface_, capabilities_, agent_, infrastructure_, root |
| 8 | `agent(container\|registry\|mixin)` | taxonomy, contract, infrastructure, capabilities | taxonomy, contract | surface_, root |
| 9 | `agent(orchestrator\|coordinator)` | taxonomy, contract | taxonomy, contract(aggregate) | surface_, agent(orchestrator\|coordinator\|manager\|handler\|state), agent(container\|registry\|mixin), infrastructure, capabilities, root |
| 10 | `agent(manager\|state)` | taxonomy, contract | taxonomy, contract(aggregate) | agent_, infrastructure_, capabilities_, surface_, root |
| 11 | `surfaces(command\|controller\|page\|entry)` | taxonomy, contract | taxonomy, contract(aggregate) | agent_, infrastructure_, capabilities_, contract(port), contract(protocol), root |
| 12 | `surfaces(hook\|store\|action\|screen\|router)` | taxonomy | None | agent_, infrastructure_, capabilities_, contract(port), contract(protocol), smart surfaces_, root |
| 13 | `surfaces(component\|view\|layout)` | taxonomy | taxonomy | agent_, contract_, infrastructure_, capabilities_, all surface_, root |

### AES002 — Mandatory Import Missing (HIGH)

File does not import required layers. Message: *"Layer '{layer}' must import from {layers}. WHY? Mandatory dependencies ensure contract enforcement. FIX: Import required taxonomy or contract modules."*

---

## Group 2: Naming & Structure (AES010–AES016)

Enforces file naming conventions, structural definitions, and type safety across layers.

| Code | Name | Severity | Message |
|------|------|----------|---------|
| AES010 | Naming Convention | MEDIUM | Filename does not follow [layer]_[concept(s)]_[suffix] pattern. WHY? Layer prefix identifies the architectural layer, suffix defines role. FIX: Must start with layer prefix (taxonomy_/contract_/etc.) and end with allowed suffix. Exceptions: main.rs, lib.rs, mod.rs. |
| AES011 | Mandatory Definition | HIGH | File is missing a struct, enum, or trait definition. WHY? Encapsulation in structs/traits is required. FIX: Group functions into a struct. |
| AES012 | Circular Dependency | CRITICAL | Circular dependency detected between layers ({source} -> {target}). WHY? Circular deps break the bottom-up layering. FIX: Extract shared logic into a lower layer. |
| AES013 | Forbidden Inheritance | CRITICAL | Contract Aggregate inherits from Port or Protocol. WHY? Aggregate is a composition contract, not an implementation. FIX: Use composition (fields) instead of inheritance. |
| AES014 | Mandatory Inheritance | HIGH | File imports contracts but no struct/class implements them. WHY? Contracts imported must be fulfilled. FIX: Add impl TraitName for YourStruct. |
| AES015 | Constant Purity | HIGH | _constant file contains non-constant declarations. WHY? _constant files must contain ONLY pub const/pub static. FIX: Move non-const decl to _vo/_entity/capability module. |
| AES016 | Primitive Usage | HIGH | Raw primitive types in domain entity/error/event/contract interface. WHY? Core domain integrity requires Value Objects. FIX: Replace raw types with _vo components. |

### Suffix Policy (AES010, AES011)

| Layer | Allowed Suffixes | Forbidden Suffixes |
|---|---|---|
| `root` | `_entry` | N/A |
| `taxonomy` | `_vo`, `_entity`, `_error`, `_event`, `_constant` | N/A |
| `contract` | `_port`, `_protocol`, `_aggregate` | N/A |
| `capabilities` | `_checker`, `_analyzer`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_aggregator`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_actions` | `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate` |
| `infrastructure` | `_adapter`, `_provider`, `_scanner`, `_client`, `_constants`, `_schemas`, `_lifespan`, `_wrapper`, `_tracer`, `_tracker`, `_variants`, `_detector`, `_patterns`, `_util`, `_system`, `_repository`, `_cache`, `_loader`, `_writer`, `_reader`, `_driver`, `_connector`, `_gateway`, `_serializer`, `_encoder`, `_decoder`, `_fetcher`, `_watcher`, `_indexer`, `_dispatcher`, `_recorder`, `_proxy`, `_publisher`, `_subscriber`, `_listener`, `_poller`, `_streamer` | `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate` |
| `surfaces` | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_entry`, `_hook`, `_store`, `_action`, `_screen` | N/A |
| `agent` | `_container`, `_manager`, `_orchestrator`, `_registry`, `_coordinator`, `_mixin`, `_state` | N/A |

### Mandatory Inheritance (AES014)

| Prefix | Must Implement |
|---|---|
| `infrastructure_` | `_port` contracts |
| `capabilities_` | `_protocol` contracts |
| `agent_` | `_aggregate` contracts |

---

## Group 3: File & Content Quality (AES020–AES024)

Enforces file-level quality standards and prohibits bypass mechanisms.

| Code | Name | Severity | Message |
|------|------|----------|---------|
| AES020 | File Size Limit | LOW | File exceeds maximum allowed line count. WHY? Large files violate SRP. FIX: Split into smaller focused files. |
| AES021 | File Minimum Size | LOW | File contains fewer than minimum required lines. WHY? Tiny files clutter structure. FIX: Merge into related module. |
| AES022 | Bypass Comment | CRITICAL | Forbidden bypass detected (#[allow], unwrap(), panic!, noqa, type: ignore). WHY? Suppressions bypass type safety. FIX: Use proper error handling. |
| AES023 | Unused Import | MEDIUM | Symbol imported but never used in scope. WHY? Unused imports indicate architectural bypass attempt. FIX: Remove unused import or use the symbol. |
| AES024 | Dead Inheritance | MEDIUM | Empty class, struct, or trait detected. WHY? Empty classes/traits/structs bypass architectural enforcement. FIX: Implement trait methods, class methods, or define struct attributes. |

---

## Group 4: Role Violations (AES030–AES038)

Suffix-specific behavioral mandates. A single code covers multiple roles with **conditional messages** depending on which suffix is violated.

| Code | Name | Severity | Role(s) | Condition / Message |
|------|------|----------|---------|---------------------|
| AES030 | Orphan Code | MEDIUM | All prefixes | File is unreachable/unused — not imported by any consumer and not an entry point. |
| AES031 | Surface Role | HIGH | Smart: `_command`/`_controller`/`_page`/`_entry` | Exceeds 15 fn or contains domain logic (>3 control flow). Must delegate via ServiceContainerAggregate. |
| AES031 | Surface Role | HIGH | Utility: `_hook`/`_store`/`_action`/`_screen`/`_router` | Contains domain logic or imports Smart surfaces. Must be stateless helpers. |
| AES031 | Surface Role | HIGH | Passive: `_component`/`_view`/`_layout` | Imports outside taxonomy. Must only import taxonomy types. |
| AES032 | Agent Role | HIGH | `_container`/`_registry`/`_mixin` | Contains domain logic or non-wiring code. Structural wiring only. |
| AES032 | Agent Role | HIGH | `_orchestrator`/`_coordinator` | Non-stateless or imports infra/capabilities directly. |
| AES032 | Agent Role | HIGH | `_manager`/`_state` | Contains domain logic or stores domain data. Lifecycle tracking only. |
| AES033 | Surface Hierarchy | HIGH | `_hook`/`_store`/`_action`/`_screen`/`_router` → Smart | Utility surface imports Smart surface. FIX: Move shared logic to Agent/Taxonomy. |
| AES034 | Passive Surface | HIGH | `_component`/`_view`/`_layout` | Imports forbidden layers (agent/contract/infrastructure/capabilities). |
| AES035 | Agent Any Bypass | HIGH | `agent_` prefix | `any` type annotation found in agent orchestrator layer. |
| AES036 | Capability Bottleneck | MEDIUM | `capabilities_` prefix | All dispatch routes go to a single capability class. |
| AES037 | Capability Method | HIGH | `capabilities_` prefix | Capability method referenced in dispatch catalog does not exist on target. |
| AES038 | Missing VO | MEDIUM | `capabilities_`/`infrastructure_` prefix | Capability method call missing required Value Object parameter. |

### Role Mandates Detail

| Role | Suffix | Layer | Mandate |
|---|---|---|---|
| Container | `_container` | agent | Structural DI wiring only. Implement ServiceContainerAggregate. No domain logic. |
| Orchestrator | `_orchestrator` | agent | Stateless conductor. Imports taxonomy+contract only. Coordinates capabilities/infra via contracts. |
| Coordinator | `_coordinator` | agent | High-level policy. Coordinates multiple orchestrators. No direct infra/cap imports. |
| Registry | `_registry` | agent | CRUD only. No decision logic. Thread-safe. |
| Manager | `_manager` | agent | Lifecycle tracking. No domain data storage. |
| Mixin | `_mixin` | agent | Assembler. May import capabilities+infrastructure for DI bundle. |
| State | `_state` | agent | State container. May be stateful. No domain logic. |
| Smart surface | `_command`/`_controller`/`_page`/`_entry` | surface | <15 fn. No domain logic. Delegates via ServiceContainerAggregate. |
| Utility surface | `_hook`/`_store`/`_action`/`_screen`/`_router` | surface | Stateless. No domain logic. No Smart surface imports. |
| Passive surface | `_component`/`_view`/`_layout` | surface | Taxonomy imports only. No logic or orchestration. |
| Capability | All `capabilities_` suffixes | capabilities | Single execution goal. One file, one responsibility. |
| Contract | `_port`/`_protocol`/`_aggregate` | contract | No inheritance across subtypes. Must be implemented by consumers. |
| Taxonomy | `_vo`/`_entity`/`_error`/`_event`/`_constant` | taxonomy | Primitive purity. Constant purity for _constant files. |

---

## Old-to-New Mapping

| Old Code | New Code | Name | Notes |
|----------|----------|------|-------|
| AES001 | AES001 | Import Layer Violation | Merged with AES010/011/023 |
| AES002 | AES002 | Mandatory Import Missing | |
| AES003 | AES010 | Naming Convention | |
| AES004 | AES020 | File Size Limit | |
| AES005 | AES021 | File Minimum Size | |
| AES006 | AES016 | Primitive Usage | |
| AES008 | AES011 | Contract/Mandatory Def | Merged with AES009 |
| AES009 | AES011 | Mandatory Definition | Merged with AES008 |
| AES010 | AES001 | Root Import | Sub-condition of AES001 |
| AES011 | AES011 | Suffix Policy | Part of naming checks |
| AES014 | AES022 | Bypass Comment | |
| AES015 | AES023 | Unused Import | |
| AES016 | AES024 | Dead Inheritance | |
| AES017 | AES030 | Orphan Code | |
| AES018 | AES033 | Surface Hierarchy | |
| AES019 | AES034 | Passive Surface | |
| AES020 | AES012 | Circular Dependency | |
| AES021 | AES032 | Agent Role | |
| AES022 | AES031 | Surface Role | |
| AES023 | AES001 | Surface Dependency | Sub-condition of AES001 |
| AES024 | AES035 | Agent Any Bypass | |
| AES026 | AES013 | Forbidden Inheritance | |
| AES027 | AES014 | Mandatory Inheritance | |
| AES030 | AES037 | Capability Method | |
| AES031 | AES036 | Capability Bottleneck | |
| AES032 | AES038 | Missing VO | |
| AES033 | AES015 | Constant Purity | |
| AES007 | — | Contract Barrel | **Removed** |
| AES012 | — | Barrel Completeness | **Removed** |
| AES013 | — | Internal All Forbidden | **Removed** |
| AES025 | — | MCP Schema | **Removed** |
