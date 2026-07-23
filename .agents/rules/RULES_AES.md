opencode


# AES (Agentic Engineering System) Rules — v3.0

See [ARCHITECTURE.md](../ARCHITECTURE.md) for the full 7-layer specification.

---

## Summary


| Code   | Name                | Severity | Group  | Description                                                                                |
| -------- | --------------------- | ---------- | -------- | -------------------------------------------------------------------------------------------- |
| AES101 | Naming Convention   | HIGH     | Naming | Filename must follow`prefix_concept_suffix` pattern — lowercase, underscore, min 2 words. |
| AES102 | Suffix Prefix Rules | HIGH     | Naming | Suffix must match layer definition — allowed, forbidden, mandatory strict.                |


| Code   | Name             | Severity | Group  | Description                                                                  |
| -------- | ------------------ | ---------- | -------- | ------------------------------------------------------------------------------ |
| AES201 | Forbidden Import | CRITICAL | Import | Cross-layer imports must comply with allowed/mandatory/forbidden rules.      |
| AES202 | Mandatory Import | HIGH     | Import | File is missing required imports defined by config.                          |
| AES203 | Unused Import    | MEDIUM   | Import | Symbol is imported but never used in file scope.                             |
| AES204 | Dummy Import     | MEDIUM   | Import | Import string matches a forbidden dummy pattern (e.g. orphan detector test). |
| AES205 | Circular Import  | HIGH     | Import | Circular dependency between layers — must be unidirectional bottom-up.      |


| Code   | Name                 | Severity | Group   | Description                                                                        |
| -------- | ---------------------- | ---------- | --------- | ------------------------------------------------------------------------------------ |
| AES301 | File Maximum Limit   | LOW      | Quality | File exceeds maximum allowed line count (default: 1000).                           |
| AES302 | File Minimum Limit   | LOW      | Quality | File is below minimum required line count (default: 5).                            |
| AES303 | Mandatory Definition | HIGH     | Quality | File missing struct/enum/trait/class definition, or definition is empty.           |
| AES304 | Bypass Comment       | CRITICAL | Quality | Forbidden bypass pattern detected (`#[allow]`, `unwrap()`, `panic!`, `noqa`, etc). |
| AES305 | Duplication Code     | MEDIUM   | Quality | Duplicate code blocks detected across files.                                       |


| Code   | Name                | Severity | Group | Description                                                                               |
| -------- | --------------------- | ---------- | ------- | ------------------------------------------------------------------------------------------- |
| AES401 | Taxonomy Role       | HIGH     | Role  | Constant file contains non-constant declarations; primitives used in /entity/error/event. |
| AES402 | Contract Role       | HIGH     | Role  | Contract trait/method uses primitive types instead of taxonomy VO or constant types.      |
| AES403 | Capabilities Role   | HIGH     | Role  | Capability has no protocol implementation                                                 |
| AES404 | Infrastructure Role | HIGH     | Role  | Infrastructure has no  port implementation                                              |
| AES405 | Agent Role          | MEDIUM   | Role  | orchestrator do not call  any port  or protocol                                         |
| AES406 | Surface Role        | MEDIUM   | Role  | Passive surface contains active domain logic; file exceeds 25 functions.                  |


| Code   | Name                  | Severity | Group  | Description                                                                          |
| -------- | ----------------------- | ---------- | -------- | -------------------------------------------------------------------------------------- |
| AES501 | Taxonomy Orphan       | LOW      | Orphan | Taxonomy file has no inbound imports from any contract file.                         |
| AES502 | Contract Orphan       | LOW      | Orphan | Contract trait not implemented by expected layer; port/protocol not called by agent. |
| AES503 | Capabilities Orphan   | MEDIUM   | Orphan | Capability not wired in any container AND unreachable in import graph.               |
| AES504 | Infrastructure Orphan | MEDIUM   | Orphan | Infrastructure not wired in any container AND unreachable in import graph.           |
| AES505 | Agent Orphan          | HIGH     | Orphan | Aggregate contracts  that implement by agents are not call by the surface           |
| AES506 | Surface Orphan        | HIGH     | Orphan | Smart surface not imported by entry/router; utility not imported by smart surface.   |

---

## Group 1: Naming

### AES101 — Naming Convention

**Severity:** HIGH

Filename must follow pattern: `prefix_concept_suffix` or `prefix_concept1_concept2_suffix`

- All **lowercase**
- Separator: **underscore** (`_`)
- Minimum **2 words** (prefix + suffix)
- Maximum: Unlimited
- Examples: `capabilities_user_checker.rs`, `infrastructure_db_adapter.py`

**Exceptions:** `main.rs`, `lib.rs`, `mod.rs`, `root_cli_main_entry.rs`, `root_mcp_main_entry.rs`, `root_tui_main_entry.rs`, `root_composition_container.rs`, `__init__.py`, `index.ts`, `index.js`, barrel/entry files.

---

### AES102 — Suffix/Prefix Rules

**Severity:** HIGH

Suffix must match the layer definition. Three sub-checks:

1. **Forbidden suffix** — suffix must not be in the `forbidden_suffix` list
2. **Strict suffix policy** — suffix must be in the `allowed_suffix` list
3. **Flexible suffix policy** — suffix can be anything except `forbidden` ones

#### Suffix Policy per Layer


| Layer            | Policy   | Allowed Suffixes                                                                                                                                                                                                                                                                                                                                                                                                                               | Forbidden Suffixes                                                                                                         |
| ------------------ | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| `root`           | strict   | `_entry`, `_container`                                                                                                                                                                                                                                                                                                                                                                                                                         | N/A                                                                                                                        |
| `taxonomy`       | strict   | `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`, `_helper`                                                                                                                                                                                                                                                                                                                                                                       | N/A                                                                                                                        |
| `contract`       | strict   | `_port`, `_protocol`, `_aggregate`                                                                                                                                                                                                                                                                                                                                                                                                             | N/A                                                                                                                        |
| `capabilities`   | flexible | `_checker`, `_analyzer`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_aggregator`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_auditor`,                                                                                                  | `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_constants`, `_utility`, `_helper`, `_port`, `_protocol`, `_aggregate` |
| `infrastructure` | flexible | `_adapter`, `_provider`, `_scanner`, `_client`, `_schemas`, `_lifespan`, `_wrapper`, `_tracer`, `_tracker`, `_variants`, `_detector`, `_patterns`, `_system`, `_repository`, `_cache`, `_loader`, `_writer`, `_reader`, `_driver`, `_connector`, `_gateway`, `_serializer`, `_encoder`, `_decoder`, `_fetcher`, `_watcher`, `_indexer`, `_dispatcher`, `_recorder`, `_proxy`, `_publisher`, `_subscriber`, `_listener`, `_poller`, `_streamer` | `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_constants`, `_utility`, `_helper`, `_port`, `_protocol`, `_aggregate` |
| `surfaces`       | strict   | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen`                                                                                                                                                                                                                                                                                                                       | N/A                                                                                                                        |
| `agent`          | strict   | `_orchestrator`                                                                                                                                                                                                                                                                                                                                                                                                                                | N/A                                                                                                                        |

---

## Group 2: Layer & Import Boundary

### AES201 — Forbidden Import

**Severity:** CRITICAL

A single rule with **13 sub-conditions** — each has `allowed`, `mandatory`, and `forbidden` fields. Layers are identified by **filename prefix** (`taxonomy_`, `contract_`, etc.), not directory path.


| #  | Scope                                                           | Allowed Imports                                                   | Mandatory Imports                                     | Forbidden Imports                                                                                 |
| ---- | ----------------------------------------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| 1  | `taxonomy(vo)`                                                  | taxonomy                                                          | None                                                  | agent*, infrastructure*, surface*, contract*, capabilities\_, root                                |
| 2  | `taxonomy(entity,error,event)`                                  | taxonomy                                                          | taxonomy(vo&#124;constant)                            | agent*, infrastructure*, surface*, contract*, capabilities\_, root                                |
| 3  | `taxonomy(constant)`                                            | taxonomy                                                          | None                                                  | agent*, infrastructure*, surface*, contract*, capabilities\_, root                                |
| 4  | `contract(port&#124;protocol)`                                  | taxonomy, contract                                                | taxonomy                                              | agent*, infrastructure*, surface*, capabilities*, contract(aggregate), root                       |
| 5  | `contract(aggregate)`                                           | taxonomy, contract                                                | taxonomy, contract(port&#124;protocol&#124;aggregate) | agent*, infrastructure*, surface*, capabilities*, root                                            |
| 6  | `capabilities`                                                  | taxonomy, contract                                                | taxonomy, contract(protocol)                          | infrastructure*, surface*, agent*, capabilities*, root                                            |
| 7  | `infrastructure`                                                | taxonomy, contract                                                | taxonomy, contract(port)                              | surface*, capabilities*, agent*, infrastructure*, root                                            |
| 8  | `agent(orchestrator)`                                           | taxonomy, contract(aggregate), contract(port), contract(protocol) | taxonomy, contract(aggregate)                         | surfaces, infrastructure, capabilities, root                                                      |
| 9  | `surfaces(command&#124;controller&#124;page&#124;entry)`        | taxonomy, contract                                                | taxonomy, contract(aggregate)                         | agent*, infrastructure*, capabilities\_, contract(port), contract(protocol), root                 |
| 10 | `surfaces(hook&#124;store&#124;action&#124;screen&#124;router)` | taxonomy                                                          | None                                                  | agent*, infrastructure*, capabilities*, contract(port), contract(protocol), smart surfaces*, root |
| 11 | `surfaces(component&#124;view&#124;layout)`                     | taxonomy                                                          | taxonomy                                              | agent*, contract*, infrastructure*, capabilities*, all surface\_, root                            |

---

### AES202 — Mandatory Import

**Severity:** HIGH

File is missing required imports defined by the configuration. Each layer has specific mandatory import expectations to ensure dependencies are properly structured.

**FIX:** Add the required import statement to the file.

---

### AES203 — Unused Import

**Severity:** MEDIUM

Symbol is imported but never used in file scope. Detected via AST analysis across Rust, Python, and JavaScript.

**FIX:** Remove the unused import or use the symbol.

---

### AES204 — Dummy Import

**Severity:** MEDIUM

Import statement matches a forbidden dummy pattern. Used to detect fake/redundant imports that exist only to satisfy the linter but serve no real purpose.

**Example:** `use output_report::taxonomy_*;` in non-output-report files (orphan detection test material).

---

### AES205 — Circular Import

**Severity:** HIGH

Circular dependency detected between layers. Layer dependencies must be unidirectional (bottom-up).
Allowed direction: `taxonomy → contract → capabilities/infrastructure → agent → surface`.
Any back-edge or cross-layer cycle is a violation.

---

## Group 3: File & Content Quality

### AES301 — File Maximum Limit

**Severity:** LOW

File exceeds maximum allowed line count (default: 1000).

**FIX:** Split into smaller files.

---

### AES302 — File Minimum Limit

**Severity:** LOW

File is below minimum required line count (default: 5).

**FIX:** Merge into a related module or add more documentation.

---

### AES303 — Mandatory Definition

**Severity:** HIGH

File must have at least one struct/enum/trait/class definition, and definitions must not be empty.

Two sub-checks:

1. **Missing definition** — file has no struct/enum/trait/class at all
2. **Empty definition** — `struct Foo;`, `impl X for Y {}`, `class Foo: pass`, `class Foo {}`


| Checker                  | Method                               | Path                                                     |
| -------------------------- | -------------------------------------- | ---------------------------------------------------------- |
| `ArchClassChecker`       | `check_mandatory_class_definition()` | `code-analysis/capabilities_class_checker.rs`            |
| `DeadInheritanceChecker` | `check_dead_inheritance()`           | `code-analysis/capabilities_dead_inheritance_checker.rs` |

**Exceptions:** `__init__.py`, `mod.rs`, `lib.rs`, `*_constant.rs`, `*_constant.py`.

---

### AES304 — Bypass Comment

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

### AES305 — Duplication Code

**Severity:** MEDIUM

Duplicate code blocks detected across files within the project scope.

**FIX:** Extract duplicated logic into shared utilities.

---

## Group 4: Role Violations

### AES401 — Taxonomy Role

**Severity:** HIGH

Constant purity violation or primitive usage in domain models. Two sub-checks:

1. **Constant purity** — `_constant` files must only contain `pub const` / `pub static` declarations
2. **Primitive in taxonomy** — `_entity`, `_error`, `_event` files must not use direct primitive types (e.g. `String`, `i32`, `int`) in field declarations. `_vo`, `_helper`, `_utility` files ARE allowed to use primitives directly.

**FIX:** Replace primitives with taxonomy value objects.

---

### AES402 — Contract Role

**Severity:** HIGH

Contract trait/method must use taxonomy VO/constant types, not primitive types.

Checks for primitive types (`String`, `i32`, `bool`, `int`, `float`, etc.) in contract trait method signatures. Test projects are the primary target.

**FIX:** Replace primitives with VO/constant from the taxonomy layer.

---

### AES403 — Capabilities Role

**Severity:** HIGH

Capability routing and protocol enforcement. Two sub-checks:

1. **Missing protocol implementation** — capability file must implement at least one `_protocol` contract
2. **Single routing bottleneck** — orchestrator dispatch must not route all calls to a single capability

**FIX:** Ensure capability implements its protocol; split routing across multiple capabilities.

---

### AES404 — Infrastructure Role

**Severity:** HIGH

Infrastructure role boundary violation. Infrastructure files must reference at least one `_port` contract — missing a port import means the file is broken/useless (no external dependency to adapt). Additionally, infrastructure files must only implement `_port` contracts and must not contain business logic, routing, or orchestration.

---

### AES405 — Agent Role

**Severity:** MEDIUM

Checks:

- **Non-stateless execution** — state assignment outside `__init__` / constructor
- **Low-level infrastructure imports** — agent must not import infrastructure or capabilities directly
- **Infrastructure re-implementation** — agent must not re-implement infrastructure logic inline (e.g., filesystem I/O, source parsing, directory walking, network calls). All infrastructure operations must go through contract ports defined in the contract layer.
- **`any` type annotations** — forbidden in agent layer
- **Single execution goal** — orchestrator must coordinate at minimum 2 subsystems
- **Container initialization** — complex domain logic in container module

**Note:** File size limits for agent files are governed by **AES301** (max 1000 lines), same as all other layers.

---

### AES406 — Surface Role

**Severity:** MEDIUM

Checks:

- **File > 15 functions** — surface file has too many responsibilities
- **Active domain logic in passive surface** — passive surfaces (`_component`, `_view`, `_layout`) must not contain business logic
- **Role boundary violation** — surface enters forbidden territory (e.g. importing infrastructure directly)

---

## Group 5: Orphan Code

### AES501 — Taxonomy Orphan

**Severity:** LOW

Taxonomy file (VO, entity, error, event, constant) has no inbound imports from any contract file. If no contract references a taxonomy type, it may be dead code.

---

### AES502 — Contract Orphan

**Severity:** LOW

Contract trait not implemented by the expected layer:

- `_port` → not implemented by any `infrastructure_` & not called by any agent\_
- `_protocol` → not implemented by any `capabilities_` & not called by any agent\_
- `_aggregate` → not called by any agent* file not called by any surfaces*

---

### AES503 — Capabilities Orphan

**Severity:** MEDIUM

Capability file not wired in any `_container` AND unreachable in the import graph.

---

### AES504 — Infrastructure Orphan

**Severity:** MEDIUM

Infrastructure file not wired in any `_container` AND unreachable in the import graph.

---

### AES505 — Agent Orphan

**Severity:** HIGH

Aggregate contracts that are implemented by agents are not called by the surface. Agent orchestrator not called by any `surface_` file or entry point. Suffix `_orchestrator` is checked.

---

### AES506 — Surface Orphan

**Severity:** HIGH

Orphan detection per category:

- **Smart** (`_command` / `_controller` / `_page` / `_entry`) — must be imported by entry or router
- **Utility** (`_hook` / `_store` / `_action` / `_screen` / `_router`) — must be imported by smart surface
- **Passive** (`_component` / `_view` / `_layout`) — must be imported by smart or utility surface
