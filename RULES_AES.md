# AES (Agentic Engineering System) Rules — v3.0

---

## Summary

| Code    | Name                   | Severity | Group              | Description                                                                                   |
| ------- | ---------------------- | -------- | ------------------ | --------------------------------------------------------------------------------------------- |
| AES001  | Import Layer Violation | CRITICAL | Layer & Import     | Cross-layer imports must comply with allowed/mandatory/forbidden rules.                       |
| AES002  | Mandatory Import       | HIGH     | Layer & Import     | File is missing required imports defined by config.                                           |
| AES011  | Naming Convention      | HIGH     | Naming & Structure | Filename must follow `prefix_concept_suffix` pattern — lowercase, underscore, min 2 words. |
| AES012  | Suffix/Prefix Rules    | HIGH     | Naming & Structure | Suffix must match layer definition — allowed, forbidden, mandatory strict.                   |
| AES013  | Forbidden Inheritance  | CRITICAL | Naming & Structure | Contract Aggregate must not inherit/implement from Port/Protocol.                             |
| AES014  | Mandatory Inheritance  | HIGH     | Naming & Structure | File that imports a contract must implement it.                                               |
| AES015  | Circular Import        | CRITICAL | Naming & Structure | Circular dependency detected between layers.                                                  |
| AES020  | File Maximum Limit     | LOW      | File & Content     | File exceeds maximum allowed line count.                                                      |
| AES021  | File Minimum Limit     | LOW      | File & Content     | File is below minimum required line count.                                                    |
| AES022  | Bypass Comment         | CRITICAL | File & Content     | Forbidden bypass pattern detected (`#[allow]`, `unwrap()`, `panic!`, `noqa`).         |
| AES023  | Unused Import          | MEDIUM   | File & Content     | Symbol is imported but never used.                                                            |
| AES024  | Mandatory Definition   | HIGH     | File & Content     | File must have at least one struct/enum/trait, and definitions must not be empty.             |
| AES030  | Orphan Code            | MEDIUM   | Role Violations    | File is not imported by anyone and is not an entry point.                                     |
| AES0301 | Taxonomy Role          | HIGH     | Role Violations    | Constant purity violation or primitive usage in domain models.                                |
| AES0302 | Contract Role          | HIGH     | Role Violations    | Contract trait/method must use taxonomy VO/constant types, not primitives.                    |
| AES0303 | Capability Role        | MEDIUM   | Role Violations    | Capability method must have single responsibility; checks missing VO parameters.              |
| AES0304 | Infrastructure Role    | MEDIUM   | Role Violations    | Infrastructure method must use required request VO parameter.                                 |
| AES0305 | Agent Role             | HIGH     | Role Violations    | Agent file >300 lines, non-stateless, low-level imports, or `any` type.                     |
| AES0306 | Surface Role           | HIGH     | Role Violations    | Surface file >15 functions, contains domain logic, or violates hierarchy.                     |

---

## Group 1: Layer & Import Boundary

### AES001 — Import Layer Violation

**Severity:** CRITICAL

A single rule with **13 sub-conditions** — each has `allowed`, `mandatory`, and `forbidden` fields. Layers are identified by **filename prefix** (`taxonomy_`, `contract_`, etc.), not directory path.

| #  | Scope                                         | Allowed Imports                                  | Mandatory Imports                             | Forbidden Imports                                                                                 |
| -- | --------------------------------------------- | ------------------------------------------------ | --------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| 1  | `taxonomy(vo)`                              | taxonomy                                         | None                                          | agent_, infrastructure_, surface_, contract_, capabilities_, root                                 |
| 2  | `taxonomy(entity,error,event)`              | taxonomy                                         | taxonomy(vo\|constant)                        | agent_, infrastructure_, surface_, contract_, capabilities_, root                                 |
| 3  | `taxonomy(constant)`                        | taxonomy                                         | None                                          | agent_, infrastructure_, surface_, contract_, capabilities_, root                                 |
| 4  | `contract(port\|protocol)`                   | taxonomy, contract                               | taxonomy                                      | agent_, infrastructure_, surface_, capabilities_, contract(aggregate), root                       |
| 5  | `contract(aggregate)`                       | taxonomy, contract                               | taxonomy, contract(port\|protocol\|aggregate) | agent_, infrastructure_, surface_, capabilities_, root                                            |
| 6  | `capabilities`                              | taxonomy, contract                               | taxonomy, contract(protocol)                  | infrastructure_, surface_, agent_, capabilities_, root                                            |
| 7  | `infrastructure`                            | taxonomy, contract                               | taxonomy, contract(port)                      | surface_, capabilities_, agent_, infrastructure_, root                                            |
| 8  | `agent(container)`                          | taxonomy, contract, infrastructure, capabilities | taxonomy, contract                            | surface_, root                                                                                    |
| 9  | `agent(orchestrator)`                       | taxonomy, contract                               | taxonomy, contract(aggregate)                 | surface_, agent(lifecycle), agent(container), infrastructure, capabilities, root                  |
| 10 | `agent(lifecycle)`                          | taxonomy, contract                               | taxonomy, contract(aggregate)                 | agent_, infrastructure_, capabilities_, surface_, root                                            |
| 11 | `surfaces(command\|controller\|page\|entry)`   | taxonomy, contract                               | taxonomy, contract(aggregate)                 | agent_, infrastructure_, capabilities_, contract(port), contract(protocol), root                  |
| 12 | `surfaces(hook\|store\|action\|screen\|router)` | taxonomy                                         | None                                          | agent_, infrastructure_, capabilities_, contract(port), contract(protocol), smart surfaces_, root |
| 13 | `surfaces(component\|view\|layout)`           | taxonomy                                         | taxonomy                                      | agent_, contract_, infrastructure_, capabilities_, all surface_, root                             |

## Group 2: Naming & Structure

### AES011 — Naming Convention

**Severity:** HIGH

Filename must follow pattern:`prefix_concept_suffix` or  `prefix_concept1_concept2_suffix `

- All **lowercase**
- Separator: **underscore** (`_`)
- Minimum **2 words** (prefix + suffix)
- Maximum : Unlimited
- Examples: `capabilities_user_checker.rs`, `infrastructure_db_adapter.py`

**Exceptions:** `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`, barrel/entry files.

---

### AES012 — Suffix/Prefix Rules

**Severity:** HIGH

Suffix must match the layer definition. Three sub-checks:

1. **Forbidden suffix** — suffix must not be in the `forbidden_suffix` list
2. **Strict suffix policy** — suffix must be in the `allowed_suffix` list
3. **Flexible suffix policy** — suffix can be anything except `forbidden` ones

| Checker               | Method                      | Path                                           |
| --------------------- | --------------------------- | ---------------------------------------------- |
| `ArchNamingChecker` | `check_domain_suffixes()` | `layer-rules/capabilities_naming_checker.rs` |

#### Suffix Policy per Layer

| Layer              | Policy   | Allowed Suffixes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Forbidden Suffixes                                                                     |
| ------------------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `root`           | strict   | `_entry`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | N/A                                                                                    |
| `taxonomy`       | strict   | `_vo`, `_entity`, `_error`, `_event`, `_constant`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | N/A                                                                                    |
| `contract`       | strict   | `_port`, `_protocol`, `_aggregate`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | N/A                                                                                    |
| `capabilities`   | flexible | `_checker`, `_analyzer`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_aggregator`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_actions`                                                                                                                                                  | `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate` |
| `infrastructure` | flexible | `_adapter`, `_provider`, `_scanner`, `_client`, `_constants`, `_schemas`, `_lifespan`, `_wrapper`, `_tracer`, `_tracker`, `_variants`, `_detector`, `_patterns`, `_util`, `_system`, `_repository`, `_cache`, `_loader`, `_writer`, `_reader`, `_driver`, `_connector`, `_gateway`, `_serializer`, `_encoder`, `_decoder`, `_fetcher`, `_watcher`, `_indexer`, `_dispatcher`, `_recorder`, `_proxy`, `_publisher`, `_subscriber`, `_listener`, `_poller`, `_streamer` | `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate` |
| `surfaces`       | strict   | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_entry`, `_hook`, `_store`, `_action`, `_screen`                                                                                                                                                                                                                                                                                                                                                                                      | N/A                                                                                    |
| `agent`          | strict   | `_container`, `_orchestrator`, `_lifecycle`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | N/A                                                                                    |

---

### AES013 — Forbidden Inheritance

**Severity:** CRITICAL

Contract Aggregate must not inherit from Port or Protocol. Aggregate is a composition contract, not an implementation.

**FIX:** Use composition (fields) instead of inheritance.

---

### AES014 — Mandatory Inheritance

**Severity:** HIGH

File that imports a contract **must implement** it as an implementor.

| Prefix              | Must Implement           |
| ------------------- | ------------------------ |
| `infrastructure_` | `_port` contracts      |
| `capabilities_`   | `_protocol` contracts  |
| `agent_`          | `_aggregate` contracts |

Caller patterns (`Box<dyn ITrait>`, `Arc<dyn ITrait>`, parameter type injection) are considered **OK** — not a violation.

| Checker                         | Method                            | Path                                                            |
| ------------------------------- | --------------------------------- | --------------------------------------------------------------- |
| `MandatoryInheritanceChecker` | `check_mandatory_inheritance()` | `code-analysis/capabilities_mandatory_inheritance_checker.rs` |

---

### AES015 — Circular Import

**Severity:** CRITICAL

Circular dependency detected between layers. Layer dependencies must be unidirectional (bottom-up).

**FIX:** Extract shared logic into a lower layer.

| Checker                     | Method                   | Path                                           |
| --------------------------- | ------------------------ | ---------------------------------------------- |
| `DependencyCycleAnalyzer` | `detect_cycle_edges()` | `layer-rules/capabilities_cycle_analyzer.rs` |

---

## Group 3: File & Content Quality

### AES020 — File Maximum Limit

**Severity:** LOW

File exceeds maximum allowed line count (default: 1000).

**FIX:** Split into smaller files.

---

### AES021 — File Minimum Limit

**Severity:** LOW

File is below minimum required line count (default: 5).

**FIX:** Merge into a related module.

---

### AES022 — Bypass Comment

**Severity:** CRITICAL

Forbidden bypass patterns detected:

- `#[allow(...)]`
- `unwrap()` / `expect()`
- `panic!`
- `noqa`
- `type: ignore`
- `eslint-disable`

**FIX:** Use proper error handling.

---

### AES023 — Unused Import

**Severity:** MEDIUM

Symbol is imported but never used in scope.

**FIX:** Remove the unused import or use the symbol.

---

### AES024 — Mandatory Definition

**Severity:** HIGH

File must have at least one struct/enum/trait/class definition, and definitions must not be empty.

Two sub-checks:

1. **Missing definition** — file has no struct/enum/trait at all
2. **Empty definition** — `struct Foo;`, `impl X for Y {}`, `class Foo: pass`, `class Foo {}`

| Checker                    | Method                                 | Path                                                       |
| -------------------------- | -------------------------------------- | ---------------------------------------------------------- |
| `ArchClassChecker`       | `check_mandatory_class_definition()` | `code-analysis/capabilities_class_checker.rs`            |
| `DeadInheritanceChecker` | `check_dead_inheritance()`           | `code-analysis/capabilities_dead_inheritance_checker.rs` |

**Exceptions:** `__init__.py`, `mod.rs`, `lib.rs`, `*_constant.rs`, `*_constant.py`.

---

## Group 4: Role Violations

### AES030 — Orphan Code

**Severity:** MEDIUM

File is not imported by anyone and is not an entry point.

---

### AES0301 — Taxonomy Role

**Severity:** HIGH

Constant purity violation or primitive usage in domain models (`_constant`, `_vo`, `_entity`).

---

### AES0302 — Contract Role

**Severity:** HIGH

Contract trait/method must use taxonomy VO/constant types, not primitive types.

**FIX:** Replace primitives with VO/constant from the taxonomy layer.

---

### AES0303 — Capability Role

**Severity:** MEDIUM

Capability method must have single responsibility. Also checks for missing VO parameters.

---

### AES0304 — Infrastructure Role

**Severity:** MEDIUM

Infrastructure method must use the required request VO parameter.

---

### AES0305 — Agent Role

**Severity:** HIGH

Checks:

- File > 300 lines
- Non-stateless execution (state assignment outside `__init__`)
- Low-level infrastructure imports
- `any` type annotations
- Single execution goal

---

### AES0306 — Surface Role

**Severity:** HIGH

Checks:

- File > 15 functions
- Active domain logic in passive surface
- Surface hierarchy violation
