# AES (Agentic Engineering System) Rules — v2.0

---

## Group 1: Layer & Import Boundary (AES001–AES002)

Enforces strict import direction between architectural layers. Layer is identified by **filename prefix** (`taxonomy_`, `contract_`, etc.), not directory path.

### AES001 — Import Layer Violation (CRITICAL)

Satu rule dengan **13 sub-conditions** — masing-masing punya `allowed`, `mandatory`, `forbidden`.

| #  | Scope                                         | Allowed Imports                                  | Mandatory Imports                             | Forbidden Imports                                                                                                                          |
| -- | --------------------------------------------- | ------------------------------------------------ | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| 1  | `taxonomy(vo)`                              | taxonomy                                         | None                                          | agent_, infrastructure_, surface_, contract_, capabilities_, root                                                                          |
| 2  | `taxonomy(entity,error,event)`              | taxonomy                                         | taxonomy(vo\|constant)                        | agent_, infrastructure_, surface_, contract_, capabilities_, root                                                                          |
| 3  | `taxonomy(constant)`                        | taxonomy                                         | None                                          | agent_, infrastructure_, surface_, contract_, capabilities_, root                                                                          |
| 4  | `contract(port\|protocol)`                   | taxonomy, contract                               | taxonomy                                      | agent_, infrastructure_, surface_, capabilities_, contract(aggregate), root                                                                |
| 5  | `contract(aggregate)`                       | taxonomy, contract                               | taxonomy, contract(port\|protocol\|aggregate) | agent_, infrastructure_, surface_, capabilities_, root                                                                                     |
| 6  | `capabilities`                              | taxonomy, contract                               | taxonomy, contract(protocol)                  | infrastructure_, surface_, agent_, capabilities_, root                                                                                     |
| 7  | `infrastructure`                            | taxonomy, contract                               | taxonomy, contract(port)                      | surface_, capabilities_, agent_, infrastructure_, root                                                                                     |
| 8  | `agent(container)`           | taxonomy, contract, infrastructure, capabilities | taxonomy, contract                            | surface_, root                                                                                                                             |
| 9  | `agent(orchestrator)`           | taxonomy, contract                               | taxonomy, contract(aggregate)                 | surface_, agent(lifecycle), agent(container), infrastructure, capabilities, root |
| 10 | `agent(lifecycle)`                      | taxonomy, contract                               | taxonomy, contract(aggregate)                 | agent_, infrastructure_, capabilities_, surface_, root                                                                                     |
| 11 | `surfaces(command\|controller\|page\|entry)`   | taxonomy, contract                               | taxonomy, contract(aggregate)                 | agent_, infrastructure_, capabilities_, contract(port), contract(protocol), root                                                           |
| 12 | `surfaces(hook\|store\|action\|screen\|router)` | taxonomy                                         | None                                          | agent_, infrastructure_, capabilities_, contract(port), contract(protocol), smart surfaces_, root                                          |
| 13 | `surfaces(component\|view\|layout)`           | taxonomy                                         | taxonomy                                      | agent_, contract_, infrastructure_, capabilities_, all surface_, root                                                                      |

### AES002 — Mandatory Import Missing (HIGH)

File does not import required layers. Message: *"Layer '{layer}' must import from {layers}. WHY? Mandatory dependencies ensure contract enforcement. FIX: Import required taxonomy or contract modules."*

---

## Group 2: Naming & Structure (AES010–AES014)

Enforces file naming conventions, structural definitions, and type safety across layers.

| Code   | Name                  | Severity | Message                                                                                                                                                                                                                                                                     |
| ------ | --------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AES010 | Naming Convention     | MEDIUM   | Filename does not follow [layer]_[concept(s)]_[suffix] pattern. WHY? Layer prefix identifies the architectural layer, suffix defines role. FIX: Must start with layer prefix (taxonomy_/contract_/etc.) and end with allowed suffix. Exceptions: main.rs, lib.rs, mod.rs. |
| AES011 | Mandatory Definition  | HIGH     | File is missing a struct, enum, or trait definition. WHY? Encapsulation in structs/traits is required. FIX: Group functions into a struct.                                                                                                                                  |
| AES012 | Circular Dependency   | CRITICAL | Circular dependency detected between layers ({source} -> {target}). WHY? Circular deps break the bottom-up layering. FIX: Extract shared logic into a lower layer.                                                                                                          |
| AES013 | Forbidden Inheritance | CRITICAL | Contract Aggregate inherits from Port or Protocol. WHY? Aggregate is a composition contract, not an implementation. FIX: Use composition (fields) instead of inheritance.                                                                                                   |
| AES014 | Mandatory Inheritance | HIGH     | File imports contracts but no struct/class implements them. WHY? Contracts imported must be fulfilled. FIX: Add impl TraitName for YourStruct.                                                                                                                              |

### Suffix Policy (AES010, AES011)

| Layer                    | Allowed Suffixes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Forbidden Suffixes                                                                     |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `root`                 | `_entry`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | N/A                                                                                    |
| `taxonomy (VO)`        | `_vo`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | N/A                                                                                    |
| `taxonomy (Entity)`    | `_entity`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | N/A                                                                                    |
| `taxonomy (Error)`     | `_error`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | N/A                                                                                    |
| `taxonomy (Event)`     | `_event`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | N/A                                                                                    |
| `taxonomy (Constant)`  | `_constant`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | N/A                                                                                    |
| `contract (Port)`      | `_port`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | N/A                                                                                    |
| `contract (Protocol)`  | `_protocol`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | N/A                                                                                    |
| `contract (Aggregate)` | `_aggregate`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | N/A                                                                                    |
| `capabilities`         | `_checker`, `_analyzer`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_aggregator`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_actions`                                                                                                                                                  | `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate` |
| `infrastructure`       | `_adapter`, `_provider`, `_scanner`, `_client`, `_constants`, `_schemas`, `_lifespan`, `_wrapper`, `_tracer`, `_tracker`, `_variants`, `_detector`, `_patterns`, `_util`, `_system`, `_repository`, `_cache`, `_loader`, `_writer`, `_reader`, `_driver`, `_connector`, `_gateway`, `_serializer`, `_encoder`, `_decoder`, `_fetcher`, `_watcher`, `_indexer`, `_dispatcher`, `_recorder`, `_proxy`, `_publisher`, `_subscriber`, `_listener`, `_poller`, `_streamer` | `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate` |
| `surfaces`             | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_entry`, `_hook`, `_store`, `_action`, `_screen`                                                                                                                                                                                                                                                                                                                                                                                      | N/A                                                                                    |
| `agent`                | `_container`, `_orchestrator`, `_lifecycle`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | N/A                                                                                    |

### Mandatory Inheritance (AES014)

| Prefix              | Must Implement           |
| ------------------- | ------------------------ |
| `infrastructure_` | `_port` contracts      |
| `capabilities_`   | `_protocol` contracts  |
| `agent_`          | `_aggregate` contracts |

---

## Group 3: File & Content Quality (AES020–AES024)

Enforces file-level quality standards and prohibits bypass mechanisms.

| Code   | Name              | Severity | Message                                                                                                                                                                               |
| ------ | ----------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AES020 | File Size Limit   | LOW      | File exceeds maximum allowed line count. WHY? Large files violate SRP. FIX: Split into smaller focused files.                                                                         |
| AES021 | File Minimum Size | LOW      | File contains fewer than minimum required lines. WHY? Tiny files clutter structure. FIX: Merge into related module.                                                                   |
| AES022 | Bypass Comment    | CRITICAL | Forbidden bypass detected (#[allow], unwrap(), panic!, noqa, type: ignore). WHY? Suppressions bypass type safety. FIX: Use proper error handling.                                     |
| AES023 | Unused Import     | MEDIUM   | Symbol imported but never used in scope. WHY? Unused imports indicate architectural bypass attempt. FIX: Remove unused import or use the symbol.                                      |
| AES024 | Dead Inheritance  | MEDIUM   | Empty class, struct, or trait detected. WHY? Empty classes/traits/structs bypass architectural enforcement. FIX: Implement trait methods, class methods, or define struct attributes. |

---

## Group 4: Role Violations (AES030–AES0306)

Suffix-specific behavioral mandates. A single code covers multiple roles with **conditional messages** depending on which suffix is violated.

| Code    | Name                | Severity | Role(s)                                                              | Condition / Message                                                                                                                       |
| ------- | ------------------- | -------- | -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| AES030  | Orphan Code         | MEDIUM   | All prefixes                                                         | File is unreachable/unused — not imported by any consumer and not an entry point.                                                        |
| AES0301 | Taxonomy Role       | HIGH     | `_constant` / `_vo` / `_entity`                                | Constant purity violations or primitive usage in domain models.                                                                           |
| AES0302 | Contract Role       | HIGH     | `_port` / `_protocol` / `_aggregate`                           | Inheritance issues or incorrect structural implementation contracts.                                                                      |
| AES0303 | Capability Role     | MEDIUM   | `capabilities_` suufix                                             | Capability method not found in dispatch, all dispatch routes go to a single capability, or missing VO parameter.                          |
| AES0304 | Infrastructure Role | MEDIUM   | `infrastructure_` suffix                                           | Infrastructure method called without required request VO parameter.                                                                       |
| AES0305 | Agent Role          | HIGH     | `_container` / `_orchestrator` / `_lifecycle` | Agent file too large (>300 lines), non-stateless execution, low-level policy imports, or `any` type annotations.                        |
| AES0306 | Surface Role        | HIGH     | `_command` / `_controller` / `_view` / `_component`          | Surface file exceeds 15 functions, contains active domain logic, or violates surface hierarchy (not in barrel, imports forbidden layers). |

### Role Mandates Detail

| Role            | Suffix                                                   | Layer          | Mandate                                                                                                                                 |
| --------------- | -------------------------------------------------------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| Container       | `_container`                                           | agent          | Structural DI wiring + registry + mixin. Implement ServiceContainerAggregate or compose traits. No domain logic.                        |
| Orchestrator    | `_orchestrator`                                        | agent          | Stateless conductor. Imports taxonomy+contract only. Coordinates capabilities/infra via contracts.                                      |
| Lifecycle       | `_lifecycle`                                           | agent          | Lifecycle tracking + state container. May be stateful. No domain logic. No direct infra/cap imports.                                    |
| Smart surface   | `_command`/`_controller`/`_page`/`_entry`        | surface        | <15 fn. No domain logic. Delegates via ServiceContainerAggregate.                                                                       |
| Utility surface | `_hook`/`_store`/`_action`/`_screen`/`_router` | surface        | Stateless. No domain logic. No Smart surface imports.                                                                                   |
| Passive surface | `_component`/`_view`/`_layout`                     | surface        | Taxonomy imports only. No logic or orchestration.                                                                                       |
| Capability      | All `capabilities_` suffixes                           | capabilities   | Single execution goal. One file, one responsibility.                                                                                    |
| Adapter         | `_adapter`                                             | infrastructure | Concrete implementation of a contract port. Interacts with external tools/libraries.                                                    |
| Scanner         | `_scanner`                                             | infrastructure | Interfaces with raw system APIs, file system, or AST structure parsers.                                                                 |
| Provider        | `_provider`                                            | infrastructure | Delivers configurations, constants, or technical resources.                                                                             |
| Client          | `_client`                                              | infrastructure | Manages external integrations, protocols, or network/standard I/O.                                                                      |
| Port            | `_port`                                                | contract       | Interface for infrastructure adapters. Defines technical boundaries. No inheritance across subtypes. Must be implemented by consumers.  |
| Protocol        | `_protocol`                                            | contract       | Interface for capability use-cases. Defines logical boundaries. No inheritance across subtypes. Must be implemented by consumers.       |
| Aggregate       | `_aggregate`                                           | contract       | Facade contract for service containers, aggregates, or orchestrators. No inheritance across subtypes. Must be implemented by consumers. |
| Value Object    | `_vo`                                                  | taxonomy       | Primitive attributes allowed, no business logic except formatting or self-contained validation. Used as type wrappers.                  |
| Entity          | `_entity`                                              | taxonomy       | Strict Value Object usage (no primitives). Represents unique domain elements with identity.                                             |
| Error           | `_error`                                               | taxonomy       | Strict Value Object usage (no primitives). Represents domain exceptions.                                                                |
| Event           | `_event`                                               | taxonomy       | Strict Value Object usage (no primitives). Represents domain state changes.                                                             |
| Constant        | `_constant`                                            | taxonomy       | Constant purity. Contains only `pub const` and `pub static` declarations.                                                           |

---

## Old-to-New Mapping

| Old Code | New Code        | Name                     | Notes                         |
| -------- | --------------- | ------------------------ | ----------------------------- |
| AES001   | AES001          | Import Layer Violation   | Merged with AES010/011/023    |
| AES002   | AES002          | Mandatory Import Missing |                               |
| AES003   | AES010          | Naming Convention        |                               |
| AES004   | AES020          | File Size Limit          |                               |
| AES005   | AES021          | File Minimum Size        |                               |
| AES006   | AES0301/AES0302 | Primitive Usage          | Merged into AES0301/AES0302   |
| AES008   | AES011          | Contract/Mandatory Def   | Merged with AES009            |
| AES009   | AES011          | Mandatory Definition     | Merged with AES008            |
| AES010   | AES001          | Root Import              | Sub-condition of AES001       |
| AES011   | AES011          | Suffix Policy            | Part of naming checks         |
| AES014   | AES022          | Bypass Comment           |                               |
| AES015   | AES023          | Unused Import            |                               |
| AES016   | AES024          | Dead Inheritance         |                               |
| AES017   | AES030          | Orphan Code              |                               |
| AES018   | AES0306         | Surface Hierarchy        | Merged into AES0306           |
| AES019   | AES0306         | Passive Surface          | Merged into AES0306           |
| AES020   | AES012          | Circular Dependency      |                               |
| AES021   | AES0305         | Agent Role               | Merged into AES0305           |
| AES022   | AES0306         | Surface Role             | Merged into AES0306           |
| AES023   | AES001          | Surface Dependency       | Sub-condition of AES001       |
| AES024   | AES0305         | Agent Any Bypass         | Merged into AES0305           |
| AES026   | AES013          | Forbidden Inheritance    |                               |
| AES027   | AES014          | Mandatory Inheritance    |                               |
| AES030   | AES0303         | Capability Method        | Merged into AES0303           |
| AES031   | AES0303         | Capability Bottleneck    | Merged into AES0303           |
| AES032   | AES0303/AES0304 | Missing VO               | Split between AES0303/AES0304 |
| AES033   | AES0301         | Constant Purity          | Merged into AES0301           |
| AES007   | —              | Contract Barrel          | **Removed**             |
| AES012   | —              | Barrel Completeness      | **Removed**             |
| AES013   | —              | Internal All Forbidden   | **Removed**             |
| AES025   | —              | MCP Schema               | **Removed**             |
