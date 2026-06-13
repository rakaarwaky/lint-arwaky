# AES (Agentic Engineering System) Rules — v3.0

---

## Summary

| Code    | Name                   | Severity   | Group              | Description                                                                                |
| ------- | ---------------------- | ---------- | ------------------ | ------------------------------------------------------------------------------------------ |
| AES001  | Import Layer Violation | CRITICAL   | Layer & Import     | Cross-layer imports must comply with allowed/mandatory/forbidden rules.                    |
| AES002  | Mandatory Import       | HIGH       | Layer & Import     | File is missing required imports defined by config.                                        |
| AES011  | Naming Convention      | HIGH       | Naming & Structure | Filename must follow `prefix_concept_suffix` pattern — lowercase, underscore, min 2 words. |
| AES012  | Suffix/Prefix Rules    | HIGH       | Naming & Structure | Suffix must match layer definition — allowed, forbidden, mandatory strict.                 |
| AES013  | Forbidden Inheritance  | HIGH       | Naming & Structure | Contract Aggregate must not inherit/implement from Port/Protocol.                          |
| AES014  | Mandatory Inheritance  | HIGH       | Naming & Structure | File that imports a contract must implement it.                                            |
| AES015  | Circular Import        | CRITICAL   | Naming & Structure | Circular dependency detected between layers.                                               |
| AES020  | File Maximum Limit     | LOW        | File & Content     | File exceeds maximum allowed line count.                                                   |
| AES021  | File Minimum Limit     | LOW        | File & Content     | File is below minimum required line count.                                                 |
| AES022  | Bypass Comment         | CRITICAL   | File & Content     | Forbidden bypass pattern detected (`#[allow]`, `unwrap()`, `panic!`, `noqa`).              |
| AES023  | Unused Import          | MEDIUM     | File & Content     | Symbol is imported but never used.                                                         |
| AES024  | Mandatory Definition   | HIGH       | File & Content     | File must have at least one struct/enum/trait, and definitions must not be empty.          |
| AES030  | Orphan Code            | LOW - HIGH | Role Violations    | File is not imported by anyone and is not an entry point.                                  |
| AES0301 | Taxonomy Role          | HIGH       | Role Violations    | Constant purity violation or primitive usage in domain models.                             |
| AES0302 | Contract Role          | HIGH       | Role Violations    | Contract trait/method must use taxonomy VO/constant types, not primitives.                 |
| AES0305 | Agent Role             | HIGH       | Role Violations    | non-stateless, low-level imports, or `any` type.                                           |
| AES0306 | Surface Role           | HIGH       | Role Violations    | Surface file >15 functions or contains domain logic.                                       |

---

## Group 1: Layer & Import Boundary

### AES001 — Import Layer Violation

**Severity:** CRITICAL

A single rule with **13 sub-conditions** — each has `allowed`, `mandatory`, and `forbidden` fields. Layers are identified by **filename prefix** (`taxonomy_`, `contract_`, etc.), not directory path.

| #   | Scope                                           | Allowed Imports                                  | Mandatory Imports                                     | Forbidden Imports                                                                                 |
| --- | ----------------------------------------------- | ------------------------------------------------ | ----------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| 1   | `taxonomy(vo)`                                  | taxonomy                                         | None                                                  | agent*, infrastructure*, surface*, contract*, capabilities\_, root                                |
| 2   | `taxonomy(entity,error,event)`                  | taxonomy                                         | taxonomy(vo&#124;constant)                            | agent*, infrastructure*, surface*, contract*, capabilities\_, root                                |
| 3   | `taxonomy(constant)`                            | taxonomy                                         | None                                                  | agent*, infrastructure*, surface*, contract*, capabilities\_, root                                |
| 4   | `contract(port&#124;protocol)`                  | taxonomy, contract                               | taxonomy                                              | agent*, infrastructure*, surface*, capabilities*, contract(aggregate), root                       |
| 5   | `contract(aggregate)`                           | taxonomy, contract                               | taxonomy, contract(port&#124;protocol&#124;aggregate) | agent*, infrastructure*, surface*, capabilities*, root                                            |
| 6   | `capabilities`                                  | taxonomy, contract                               | taxonomy, contract(protocol)                          | infrastructure*, surface*, agent*, capabilities*, root                                            |
| 7   | `infrastructure`                                | taxonomy, contract                               | taxonomy, contract(port)                              | surface*, capabilities*, agent*, infrastructure*, root                                            |
| 8   | `agent(orchestrator)`                           | taxonomy, contract(aggregate), contract(port), contract(protocol) | taxonomy, contract(aggregate)              | surfaces, infrastructure, capabilities, root                                                       |
| 9   | `surfaces(command&#124;controller&#124;page&#124;entry)` | taxonomy, contract                       | taxonomy, contract(aggregate)                         | agent*, infrastructure*, capabilities\_, contract(port), contract(protocol), root                 |
| 10  | `surfaces(hook&#124;store&#124;action&#124;screen&#124;router)` | taxonomy                       | None                                                  | agent*, infrastructure*, capabilities*, contract(port), contract(protocol), smart surfaces*, root |
| 11  | `surfaces(component&#124;view&#124;layout)`   | taxonomy                                         | taxonomy                                              | agent*, contract*, infrastructure*, capabilities*, all surface\_, root                            |

### AES002 — Mandatory Import

**Severity:** HIGH

File is missing required imports defined by the configuration. Each layer has specific mandatory import expectations to ensure dependencies are properly structured.

**FIX:** Add the required import statement to the file.

---

## Group 2: Naming & Structure

### AES011 — Naming Convention

**Severity:** HIGH

Filename must follow pattern:`prefix_concept_suffix` or `prefix_concept1_concept2_suffix `

- All **lowercase**
- Separator: **underscore** (`_`)
- Minimum **2 words** (prefix + suffix)
- Maximum : Unlimited
- Examples: `capabilities_user_checker.rs`, `infrastructure_db_adapter.py`

**Exceptions:** `main.rs`, `lib.rs`, `mod.rs`, `root_cli_main_entry.rs`, `root_mcp_main_entry.rs`, `root_tui_main_entry.rs`, `root_composition_container.rs`, `__init__.py`, `index.ts`, `index.js`, barrel/entry files.

---

### AES012 — Suffix/Prefix Rules

**Severity:** HIGH

Suffix must match the layer definition. Three sub-checks:

1. **Forbidden suffix** — suffix must not be in the `forbidden_suffix` list
2. **Strict suffix policy** — suffix must be in the `allowed_suffix` list
3. **Flexible suffix policy** — suffix can be anything except `forbidden` ones

**Taxonomy Relaxed Suffixes (`_utility`, `_helper`):**
Files with `_utility` or `_helper` suffix are taxonomy files with relaxed rules:
- **Boleh punya logic** — tidak seperti `_vo` yang harus pure data
- **Boleh di-import oleh layer manapun** — capabilities, infrastructure, agent boleh pakai
- **Tidak wajib implement contract trait** — berbeda dengan capabilities yang harus implement protocol
- **Contoh use case:** path utilities, format helpers, common validation functions

| Checker             | Method                    | Path                                         |
| ------------------- | ------------------------- | -------------------------------------------- |
| `ArchNamingChecker` | `check_domain_suffixes()` | `layer-rules/capabilities_naming_checker.rs` |

#### Suffix Policy per Layer

| Layer            | Policy   | Allowed Suffixes                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Forbidden Suffixes                                                       |
| ---------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `root`           | strict   | `_entry`, `_container`                                                                                                                                                                                                                                                                                                                                                                                                                | N/A                                                                      |
| `taxonomy`       | strict   | `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`, `_helper`                                                                                                                                                                                                                                                                                                                                                                                                | N/A                                                                      |
| `contract`       | strict   | `_port`, `_protocol`, `_aggregate`                                                                                                                                                                                                                                                                                                                                                                                                                                    | N/A                                                                      |
| `capabilities`   | flexible | `_checker`, `_analyzer`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_aggregator`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_actions`                                                                                                                          | `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate` |
| `infrastructure` | flexible | `_adapter`, `_provider`, `_scanner`, `_client`, `_constants`, `_schemas`, `_lifespan`, `_wrapper`, `_tracer`, `_tracker`, `_variants`, `_detector`, `_patterns`, `_util`, `_system`, `_repository`, `_cache`, `_loader`, `_writer`, `_reader`, `_driver`, `_connector`, `_gateway`, `_serializer`, `_encoder`, `_decoder`, `_fetcher`, `_watcher`, `_indexer`, `_dispatcher`, `_recorder`, `_proxy`, `_publisher`, `_subscriber`, `_listener`, `_poller`, `_streamer` | `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate` |
| `surfaces`       | strict   | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_entry`, `_hook`, `_store`, `_action`, `_screen`                                                                                                                                                                                                                                                                                                                                    | N/A                                                                      |
| `agent`          | strict   | `_orchestrator`                                                                                                                                                                                                                                                                                                                                                                                                                                                       | N/A                                                                      |

---

### AES013 — Forbidden Inheritance

**Severity:** HIGH

Contract Aggregate must not inherit from Port or Protocol. Aggregate is a composition contract, not an implementation.

**FIX:** Use composition (fields) instead of inheritance.

---

### AES014 — Mandatory Inheritance

**Severity:** HIGH

File that imports a contract **must implement** it as an implementor.

| Prefix            | Must Implement         |
| ----------------- | ---------------------- |
| `infrastructure_` | `_port` contracts      |
| `capabilities_`   | `_protocol` contracts  |
| `agent_`          | `_aggregate` contracts |

Caller patterns (`Box<dyn ITrait>`, `Arc<dyn ITrait>`, parameter type injection) are considered **OK** — not a violation.

| Checker                       | Method                          | Path                                                          |
| ----------------------------- | ------------------------------- | ------------------------------------------------------------- |
| `MandatoryInheritanceChecker` | `check_mandatory_inheritance()` | `code-analysis/capabilities_mandatory_inheritance_checker.rs` |

---

### AES015 — Circular Import

**Severity:** CRITICAL

Circular dependency detected between layers. Layer dependencies must be unidirectional (bottom-up).

**FIX:** Extract shared logic into a lower layer.

| Checker                   | Method                 | Path                                         |
| ------------------------- | ---------------------- | -------------------------------------------- |
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

| Checker                  | Method                               | Path                                                     |
| ------------------------ | ------------------------------------ | -------------------------------------------------------- |
| `ArchClassChecker`       | `check_mandatory_class_definition()` | `code-analysis/capabilities_class_checker.rs`            |
| `DeadInheritanceChecker` | `check_dead_inheritance()`           | `code-analysis/capabilities_dead_inheritance_checker.rs` |

**Exceptions:** `__init__.py`, `mod.rs`, `lib.rs`, `*_constant.rs`, `*_constant.py`.

---

## Group 4: Role Violations

### AES030 — Orphan Code

**Severity:** MEDIUM (taxonomy, surfaces), HIGH (contract, capabilities, infrastructure, agent)

File is not imported by anyone and is not an entry point. Detection is **per-layer** with different strategies:

| Layer             | Detection Logic                                                                                                                                                                                                                                                                                                     | Severity |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| `taxonomy_`       | No inbound imports from any contract file.                                                                                                                                                                                                                                                                          | LOW      |
| `contract_`       | (1) Trait not implemented by expected layer (`_port` → `infrastructure_`, `_protocol` → `capabilities_`, `_aggregate` → `agent_` / `root_`). (2) Port/protocol not called by any `agent_*_orchestrator`. (3) Aggregate not called by any `surface_*`.                                                                         | HIGH     |
| `capabilities_`   | Not wired in any `_container` AND unreachable in import graph.                                                                                                                                                                                                                                                      | HIGH     |
| `infrastructure_` | Not wired in any `_container` AND unreachable in import graph.                                                                                                                                                                                                                                                      | HIGH     |
| `agent_`          | Agent implements an orchestrator or aggregate, but is not called by any `surface_*` or entry file. Suffix `_orchestrator` is checked.                                                                                                                                                                               | HIGH     |
| `surfaces_`       | Orphan detection per category:**Smart** (`_command`/`_controller`/`_page`/`_entry`) must be imported by entry or router. **Utility** (`_hook`/`_store`/`_action`/`_screen`/`_router`) must be imported by smart surface. **Passive** (`_component`/`_view`/`_layout`) must be imported by smart or utility surface. | MEDIUM   |

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

### AES0305 — Agent Role

**Severity:** HIGH

Checks:

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
