# AES (Agentic Engineering System) Rules — v3.0

See [ARCHITECTURE.md](../../ARCHITECTURE.md) for the full 7-layer specification.

---

## Summary


| Code   | Name                | Severity | Group  | Description                                                                                |
| -------- | --------------------- | ---------- | -------- | -------------------------------------------------------------------------------------------- |
| AES101 | Naming Convention   | HIGH     | Naming | Filename must follow`prefix_concept_suffix` pattern — lowercase, underscore, min 3 words. |
| AES102 | Suffix Prefix Rules | HIGH     | Naming | Suffix must match layer definition — allowed, forbidden, mandatory strict.                |


| Code   | Name             | Severity | Group  | Description                                                                                    |
| -------- | ------------------ | ---------- | -------- | ------------------------------------------------------------------------------------------------ |
| AES201 | Forbidden Import | CRITICAL | Import | Cross-layer imports must comply with allowed/mandatory/forbidden rules.                        |
| AES202 | Mandatory Import | HIGH     | Import | File is missing required imports defined by config.                                            |
| AES203 | Unused Import    | MEDIUM   | Import | Symbol is imported but never used in file scope.                                               |
| AES204 | Dummy Import     | HIGH     | Import | Import string matches a forbidden dummy pattern; symbol used only in dummy functions or stubs. |
| AES205 | Circular Import  | CRITICAL | Import | Circular dependency between layers — must be unidirectional bottom-up.                        |


| Code   | Name                 | Severity      | Group   | Description                                                                        |
| -------- | ---------------------- | --------------- | --------- | ------------------------------------------------------------------------------------ |
| AES301 | File Maximum Limit   | HIGH          | Quality | File exceeds maximum allowed line count (default: 1000).                           |
| AES302 | File Minimum Limit   | HIGH          | Quality | File is below minimum required line count (default: 5).                            |
| AES303 | Mandatory Definition | HIGH / MEDIUM | Quality | File missing struct/enum/trait/class definition, or definition is empty.           |
| AES304 | Bypass Comment       | CRITICAL      | Quality | Forbidden bypass pattern detected (`#[allow]`, `unwrap()`, `panic!`, `noqa`, etc). |
| AES305 | Duplication Code     | MEDIUM        | Quality | Duplicate code blocks detected across files.                                       |


| Code   | Name              | Severity | Group | Description                                                                                     |
| -------- | ------------------- | ---------- | ------- | ------------------------------------------------------------------------------------------------- |
| AES401 | Taxonomy Role     | HIGH     | Role  | Constant file contains non-constant declarations; primitives used in entity/error/event.        |
| AES402 | Contract Role     | HIGH     | Role  | Contract trait/method uses primitive types instead of taxonomy VO or constant types.            |
| AES403 | Capabilities Role | HIGH/MEDIUM/LOW | Role  | Capability exceeds 3 types, has no protocol implementor, reversed block order, local const, inline test, or public helper. |
| AES404 | Utility Role      | MEDIUM   | Role  | Utility contains struct/impl/trait/type-alias (Rust), class/interface/enum (Python/TS), or non-taxonomy imports |
| AES405 | Agent Role        | MEDIUM   | Role  | Orchestrator contains too many types, or has no aggregate implementor or uses`Any` annotations. |
| AES406 | Surface Role      | HIGH     | Role  | Passive surface contains active domain logic; file exceeds 15 functions.                        |


| Code   | Name                | Severity | Group  | Description                                                                                                                                       |
| -------- | --------------------- | ---------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| AES501 | Taxonomy Orphan     | LOW      | Orphan | Taxonomy file has no inbound imports from any contract file.                                                                                      |
| AES502 | Contract Orphan     | MEDIUM   | Orphan | Contract protocol not implemented by capabilities or not called by agent; aggregate not called by surface.                                        |
| AES503 | Capabilities Orphan | MEDIUM   | Orphan | Capability not wired in any container AND unreachable in import graph.                                                                            |
| AES504 | Utility Orphan      | MEDIUM   | Orphan | Utility file not imported or consumed by any capability, agent, or surface layer.                                                                 |
| AES505 | Agent Orphan        | HIGH     | Orphan | Agent orchestrator not called by any surface file or entry point.                                                                                 |
| AES506 | Surface Orphan      | HIGH     | Orphan | Smart surface not imported by entry/router; utility surface not imported by smart surface; passive surface not imported by smart/utility surface. |

| Code   | Name                  | Severity | Group  | Description                                                                                     |
| -------- | --------------------- | -------- | ------ | ------------------------------------------------------------------------------------------------- |
| AES601 | FR Format             | HIGH     | Doc    | Requirement IDs are `FR-<FEATURE>-NNN`; every requirement states all six fields.                  |
| AES602 | Section Structure     | HIGH     | Doc    | FRD sections follow template order; required tables and subsections are present.                  |
| AES603 | Spec Purity           | HIGH     | Doc    | Specs never name source files and never carry implementation state.                              |
| AES604 | Crosslinks            | HIGH     | Doc    | An FRD crosslinks its PRD and backlog; a feature backlog never restates master sections.         |
| AES605 | Feature Folder Health | MEDIUM   | Doc    | Every folder with a doc pair holds an orchestrator; kernel folders carry no docs.                |
| AES606 | Doc Heading Structure | HIGH     | Doc    | Every root document carries one H1 and its own template-derived H2 set; off-template H2s are reported. |

| Code   | Name               | Severity | Group     | Description                                                                                |
| -------- | -------------------- | ---------- | ----------- | ---------------------------------------------------------------------------------------------- |
| AES701 | Shared Folder Purity | HIGH     | Structure | A `shared` folder holds a `capabilities_*`, `agent_*`, or `surface_*` file.                  |
| AES702 | Feature Folder Health | MEDIUM   | Structure | A feature folder lacks an `agent_*_orchestrator` or a `capabilities_*` file.                 |
| AES703 | Surface Folder Purity | MEDIUM   | Structure | A surface folder holds a `capabilities_*` or `agent_*` file.                                |
| AES704 | Feature Folder Docs  | MEDIUM   | Structure | A feature folder lacks its `FRD.md` + `BACKLOG.md` doc pair.                               |
| AES705 | Surface Folder Docs  | MEDIUM   | Structure | A surface folder lacks its `DESIGN.md`.                                                    |

---

## Group 1: Naming

### AES101 — Naming Convention

**Severity:** HIGH

Filename must follow pattern: `prefix_concept_suffix` or `prefix_concept1_concept2_suffix`

- All **lowercase**
- Separator: **underscore** (`_`)
- Minimum **3 words** (prefix + suffix)
- Maximum: Unlimited
- Examples: `capabilities_user_checker.rs`, `utility_path_resolver.rs`, `capabilities_db_adapter.py`

**Exceptions:** `main.rs`, `lib.rs`, `mod.rs`, `root_cli_main_entry.rs`, `root_mcp_main_entry.rs`, `root_tui_main_entry.rs`, `root_composition_container.rs`, `__init__.py`, `index.ts`, `index.js`, barrel/entry files.

---

### AES102 — Suffix/Prefix Rules

**Severity:** HIGH

Suffix must match the layer definition. Three sub-checks:

1. **Forbidden suffix** — suffix must not be in the `forbidden_suffix` list
2. **Strict suffix policy** — suffix must be in the `allowed_suffix` list
3. **Flexible suffix policy** — suffix can be anything except `forbidden` ones

#### Suffix Policy per Layer


| Layer          | Policy   | Allowed Suffixes                                                                                                         | Forbidden Suffixes                                                                                     |
| ---------------- | ---------- | -------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `root`         | strict   | `_entry`, `_container`                                                                                                   | N/A                                                                                                    |
| `taxonomy`     | strict   | `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_request`, `_response`                                                                        | N/A                                                                                                    |
| `contract`     | strict   | `_protocol`, `_aggregate`                                                                                                | N/A                                                                                                    |
| `utility`      | flexible | based on config                                                                                                          | `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_protocol`, `_aggregate`, `_request`, `_response`                           |
| `capabilities` | flexible | based on config                                                                                                          | `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_constants`, `_protocol`, `_aggregate`, `_utility`, `_request`, `_response` |
| `agent`        | strict   | `_orchestrator`                                                                                                          | N/A                                                                                                    |
| `surfaces`     | strict   | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen` | N/A                                                                                                    |

---

## Group 2: Layer & Import Boundary

### AES201 — Forbidden Import

**Severity:** CRITICAL

A single rule with **13 sub-conditions** — each has `allowed`, `mandatory`, and `forbidden` fields. Layers are identified by **filename prefix** (`taxonomy_`, `utility_`, `contract_`, `capabilities_`, `agent_`, `surface_`, `root_`), not directory path.


| #  | Scope                                                           | Allowed Imports                                            | Mandatory Imports             | Forbidden Imports                                                |
| ---- | ----------------------------------------------------------------- | ------------------------------------------------------------ | ------------------------------- | ------------------------------------------------------------------ |
| 1  | `taxonomy(vo)`                                                  | taxonomy                                                   | None                          | agent*, surface*, contract*, utility*, capabilities*, root       |
| 1a | `taxonomy(request,response)`                                    | taxonomy                                                   | None                          | agent*, surface*, contract*, utility*, capabilities*, root       |
| 2  | `taxonomy(entity,error,event)`                                  | taxonomy                                                   | taxonomy(vo&#124;constant)    | agent*, surface*, contract*, utility*, capabilities*, root       |
| 3  | `taxonomy(constant)`                                            | taxonomy                                                   | None                          | agent*, surface*, contract*, utility*, capabilities*, root       |
| 4  | `utility`                                                       | taxonomy                                                   | None                          | agent*, surface*, contract*, capabilities*, root                 |
| 5  | `contract(protocol)`                                            | taxonomy, contract                                         | taxonomy                      | agent*, surface*, capabilities*, contract(aggregate), root       |
| 6  | `contract(aggregate)`                                           | taxonomy, contract                                         | taxonomy                      | agent*, surface*, capabilities*, root                            |
| 7  | `capabilities`                                                  | taxonomy, contract(protocol), utility                      | taxonomy, contract(protocol)  | surface*, agent*, capabilities*, root                            |
| 8  | `agent(orchestrator)`                                           | taxonomy, contract(aggregate), contract(protocol), utility | taxonomy, contract(aggregate) | surface*, capabilities*, root                                    |
| 9  | `surfaces(command&#124;controller&#124;page)`                   | taxonomy, contract(aggregate), utility                     | None                          | agent*, capabilities*, contract(protocol), root                  |
| 10 | `surfaces(hook&#124;store&#124;action&#124;screen&#124;router)` | taxonomy                                                   | None                          | agent*, capabilities*, contract(protocol), smart surfaces*, root |
| 11 | `surfaces(component&#124;view&#124;layout)`                     | taxonomy                                                   | None                          | agent*, contract*, capabilities*, all surface*, root             |
| 12 | `root`                                                          | taxonomy, contract, capabilities, agent, surface           | None                          | None                                                             |

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

**Severity:** HIGH

Import statement matches a forbidden dummy pattern. Used to detect fake/redundant imports that exist only to satisfy the linter but serve no real purpose. Includes four sub-checks:

1. **Dummy imports** — imported symbols only used inside `_use_mandatory_imports` dummy functions (dead code to silence import warnings)
2. **Dummy functions** — `_use_mandatory_imports` function ranges flagged as dead code
3. **Dummy trait impls** — trait implementations with empty/todo bodies that violate contract abstraction
4. **Surface logic bypass** — surface-layer code calling domain logic directly (`lint_path(`, `compute_score(`, `has_critical(`, `walk_rs_files(`) — `Severity: MEDIUM`

**FIX:** Use imported symbols in real logic, remove `_use_mandatory_imports` functions, implement contract methods with real behavior.

---

### AES205 — Circular Import

**Severity:** CRITICAL

Circular dependency detected between layers. Layer dependencies must be unidirectional (bottom-up).
Allowed direction: `taxonomy → contract / utility → capabilities → agent → surface → root`.
Any back-edge or cross-layer cycle is a violation.

---

## Group 3: File & Content Quality

### AES301 — File Maximum Limit

**Severity:** HIGH

File exceeds maximum allowed line count (default: 1000).

**FIX:** Split into smaller files.

---

### AES302 — File Minimum Limit

**Severity:** HIGH

File is below minimum required line count (default: 5).

**FIX:** Merge into a related module or add more documentation.

---

### AES303 — Mandatory Definition

**Severity:** HIGH (sub-check 1) / MEDIUM (sub-check 2)

File must have at least one struct/enum/trait/class definition, and definitions must not be empty.

Two sub-checks:

1. **Missing definition** (`Severity: HIGH`) — file has no struct/enum/trait/class at all
2. **Empty / dead definition** (`Severity: MEDIUM`) — `struct Foo;`, `impl X for Y {}`, `class Foo: pass`, `class Foo {}`


| Checker                  | Method                               | Path                                                     |
| -------------------------- | -------------------------------------- | ---------------------------------------------------------- |
| `MandatoryDefinitionChecker` | `check_mandatory_class_definition()` | `quality-rules/capabilities_mandatory_definition_checker.rs` |
| `MandatoryDefinitionChecker` | `check_dead_inheritance()`           | `quality-rules/capabilities_mandatory_definition_checker.rs` |

**Exceptions:** `__init__.py`, `mod.rs`, `lib.rs`, `*_constant.rs`, `*_constant.py`.

---

### AES304 — Bypass Comment

**Severity:** CRITICAL

Forbidden bypass patterns detected:

- `#[allow(...)]`
- `unwrap()` / `expect()`
- `panic!`
- `todo`
- `unimplemented`
- `unreachable`
- `noqa`
- `type: ignore`
- `eslint-disable`
- `ts-ignore`
- `ts-expect-error`
- `FIXME`
- `HACK`
- `XXX`
- `raise NotImplementedError` (Python)
- `assert False` (Python)
- `throw new Error(...)` (JS/TS)

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

1. **Constant purity** — `_constant` files must only contain const  declarations
2. **Primitive in taxonomy** — `_entity`, `_error`, `_event` files must not use direct primitive types (e.g. `String`, `i32`, `int`) in field declarations. `_vo` _constant files are allowed to use primitives directly.

**FIX:** Replace primitives with taxonomy value objects.

---

### AES402 — Contract Role

**Severity:** HIGH

Contract trait/method must use taxonomy VO/constant types, not primitive types.

Checks for primitive types (`String`, `i32`, `bool`, `int`, `float`, etc.) in contract trait method signatures. Test projects are the primary target.

**FIX:** Replace primitives with VO/constant from the taxonomy layer.

---

### AES403 — Capabilities Role

**Severity:** HIGH / MEDIUM / LOW

Capability routing, protocol enforcement, and 3-block structure. Six sub-checks — each with its own severity:

| Sub-check                        | Severity   | Description                                                                                          |
| -------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------- |
| **CapabilityTooManyTypes**       | **HIGH**   | File exceeds max 3 type declarations.                                                                 |
| **CapabilityNoImplementor**      | **MEDIUM** | No struct/class in the capability file implements a `_protocol` contract trait.                        |
| **CapabilityBlockOrder**         | **HIGH**   | Block 2 (protocol trait impl) does not precede Block 3 (inherent impl: ctors, std traits, helpers).    |
| **CapabilityLocalConstant**      | **MEDIUM** | File-level `const` declared inline; belongs in `taxonomy_<domain>_constant.rs`.                        |
| **CapabilityEmbeddedTest**       | **LOW**    | `#[cfg(test)]` or `mod tests` declared inline; test code belongs in `tests/`.                          |
| **CapabilityPublicHelper**       | **MEDIUM** | Block 3 helper with no production caller (own-module or external) is `pub`; change to `fn` or `pub(crate)` (tests in `tests/` compile as separate crate and require `pub`). |

**Structure rule (3-block):** a capability file reads Block 1 (type + constructor) → Block 2 (protocol methods only) → Block 3 (factories, std traits, helpers). The `CapabilityBlockOrder` sub-check enforces the 2-before-3 half of that order.

**Language coverage:** all six sub-checks run for Rust, Python, and TypeScript/JavaScript. Each language has its own implementation of the block-order, constant-placement, test-placement, and helper-visibility sub-checks, since the syntactic markers differ (`impl X for` vs `class X(Base)` vs `class X implements I`).

**FIX:** Ensure capability implements its protocol; split routing across multiple capabilities; reorder impl blocks so the protocol implementation comes first.

---

### AES404 — Utility Role

**Severity:** MEDIUM

Utility role boundary violation. Utility files must contain stateless standalone functions only. They must not contain stateful objects, struct/class state, trait definitions, impl blocks, type aliases (Rust `pub type`), interface definitions (TS), or enum definitions. Furthermore, Utility files may only depend on Taxonomy, and must not import any other layer (`contract`, `capabilities`, `agent`, `surface`, `root`) or other `utility_*` files.

**FIX:** Refactor Utility to stateless functions and remove non-taxonomy imports or move stateful logic into Capabilities.

---

### AES405 — Agent Role

**Severity:** MEDIUM / HIGH

Checks — each with its own severity:


| Sub-check              | Severity   | Description                                                                       |
| ------------------------ | ------------ | ----------------------------------------------------------------------------------- |
| **AgentTooManyTypes**  | **HIGH**   | File exceeds max 3 type declarations (struct/enum/class/interface).               |
| **AgentNoImplementor** | **MEDIUM** | No struct/class implements an aggregate trait.                                    |
| **AnyType annotation** | **MEDIUM** | `: Any`, `Any<`, `Any[` patterns detected in agent code; must use concrete types. |

Additional checks:

- **Non-stateless execution** — state assignment outside `__init__` / constructor
- **Direct capabilities imports** — agent must not import capabilities directly; must communicate via contract protocols/aggregates
- **Direct capability implementation** — agent must delegate execution to capabilities via protocols
- **Single execution goal** — orchestrator must coordinate at minimum 2 subsystems
- **Container initialization** — complex domain logic in container module

**Note:** File size limits for agent files are governed by **AES301** (max 1000 lines), same as all other layers.

---

### AES406 — Surface Role

**Severity:** HIGH

Checks:

- **File > 15 functions** — surface file has too many responsibilities
- **Active domain logic in passive surface** — passive surfaces (`_component`, `_view`, `_layout`) must not contain business logic
- **Role boundary violation** — surface enters forbidden territory (e.g. importing capabilities or non-aggregate contracts directly)

---

## Group 5: Orphan Code

### AES501 — Taxonomy Orphan

**Severity:** LOW

Taxonomy file (VO, entity, error, event, constant) has no inbound imports from any contract file. If no contract references a taxonomy type, it may be dead code.

---

### AES502 — Contract Orphan

**Severity:** MEDIUM

Contract trait not implemented by the expected layer:

- `_protocol` → not implemented by any `capabilities_` & not called by any `agent_`
- `_aggregate` → not implemented by any `agent_` & not called by any `surface_`

---

### AES503 — Capabilities Orphan

**Severity:** MEDIUM

Capability file not wired in any `_container`

---

### AES504 — Utility Orphan

**Severity:** MEDIUM

Utility file is not imported or consumed by any capability, agent, or surface layer or is only imported by other utility files.

---

### AES505 — Agent Orphan

**Severity:** HIGH

Agent orchestrator file not wired in any _container

**Suffix checked:** `_orchestrator`

---

### AES506 — Surface Orphan

**Severity:** HIGH

Orphan detection per category:

- **Smart** (`_command` / `_controller` / `_page` / `_entry`) — must be imported by entry
- **Utility** (`_hook` / `_store` / `_action` / `_screen` / `_router`) — must be imported by smart surface
- **Passive** (`_component` / `_view` / `_layout`) — must be imported by smart or utility surface

---

## Group 6: Document Invariants

These rules audit the AES document chain — `PRD.md`, `ROADMAP.md`, `README.md`, `AGENTS.md` at the workspace root, plus the `FRD.md` / `BACKLOG.md` pair in each feature folder. Unlike groups 1–5 they read Markdown, not source code, so they run through the `docs` command rather than `scan`.

**Run:** `lint-arwaky-cli docs .`

**Finding shape:** each code groups one category. A finding carries `code`, a `violation_type` naming the exact sub-invariant that failed, and a `message` giving the line and the concrete drift. The `violation_type` is stable and machine-readable; the message is for a human.

**Documented by:** the HOW-TO-MAKE-FRD / HOW-TO-MAKE-ROADMAP templates in the `aes-docs` skill.

---

### AES601 — FR Format

**Severity:** HIGH

Requirement IDs follow `FR-<FEATURE>-NNN: <imperative name>`, and every requirement states all six fields: Description, Input, Output, Business Rules, Edge Cases, Error Handling.

| Violation type        | Fires when                                                              |
| ----------------------- | ------------------------------------------------------------------------ |
| `id_missing_feature_prefix` | An ID reads `FR-NNN` with no feature prefix.                        |
| `field_missing`            | A requirement omits one or more of the six fields; the message names each. |

---

### AES602 — Section Structure

**Severity:** HIGH

Sections follow the template order (Reference, System Overview, Functional Requirements, API Contract, Integration Points, Non-functional, Test Scenarios, Assumptions, Glossary), and each carries the shape its type requires.

| Violation type               | Fires when                                                            |
| ------------------------------ | ---------------------------------------------------------------------- |
| `order_violation`             | The level-2 sections appear out of template order.                    |
| `api_no_subsection`           | API Contract is missing its Protocol API or Aggregate API subsection.  |
| `integration_not_table`       | Integration Points is not a table with System, Direction, Purpose, Failure mode. |
| `nfr_not_table`               | Non-functional Requirements is not a table with Metric, Target, Measurement method. |
| `scenarios_empty`             | Test Scenarios carries no bullet items.                               |
| `glossary_empty`              | Glossary carries no `- **Term**: definition` bullets.                  |

---

### AES603 — Spec Purity

**Severity:** HIGH

A spec promises; it never reports state. Applies to every promise-bearing document: PRD, FRD, ROADMAP, ARCHITECTURE, CONTRIBUTING.

| Violation type         | Fires when                                                              |
| ------------------------ | ------------------------------------------------------------------------ |
| `source_file_named`     | The spec names a concrete source file (`.py`, `.rs`, `.ts`, `.tsx`); the message names the file. |
| `status_leak`           | The spec carries checkboxes, a Status field, implementation state, release state, a status marker, or a progress percentage. |

---

### AES604 — Crosslinks

**Severity:** HIGH

Documents reference each other, and the state vocabulary lives in exactly one place.

| Violation type             | Fires when                                                              |
| --------------------------- | ------------------------------------------------------------------------ |
| `no_backlog_link`           | An FRD's Reference section does not link its BACKLOG.md.                |
| `no_prd_link`               | An FRD's Reference section does not link its PRD.md.                    |
| `state_vocab_restated`      | A feature backlog restates a root-master section (State Definitions, Status Policy, Feature Roll-up, Branches in Flight, Risk Register). |

---

### AES605 — Feature Folder Health

**Severity:** MEDIUM

A doc pair marks a feature, and only a feature holds one.

| Violation type     | Fires when                                                              |
| -------------------- | ------------------------------------------------------------------------ |
| `no_orchestrator`   | A folder carries a doc pair but holds no `*_orchestrator` file.        |
| `shared_has_docs`   | A kernel folder (`shared`) carries a doc pair.                          |

---

### AES606 — Doc Heading Structure

**Severity:** HIGH

Every root document carries exactly one level-1 heading and a template-derived set of required and allowed level-2 sections. Fenced code-block contents are stripped before scanning so they never read as headings.

The H2 set is closed per document: a required H2 is mandatory; a project-specific H2 is allowed only when it appears in that document's agreed optional list; any other H2 is reported so it can be demoted to a level-3 heading or removed. Level-3 headings and deeper are free-form.

Matching is by leading words after lowercasing and stripping punctuation, so `## 4. Vertical Slicing Layout` satisfies `Vertical Slicing Layout`.

**Required and allowed H2s per document:**

| Document | Required H2s |
| --- | --- |
| `AGENTS.md` | Precedence, Security, Architecture, Contributing, License, Commands, Git Workflow, Definition of Done, Related Documents |
| `ARCHITECTURE.md` | Purpose, Workspace Organization, Naming Convention, Vertical Slicing Layout, Taxonomy Layer, Contract Layer, Utility Layer, Capabilities Layer, Agent Layer, Surface Layer, Root Layer |
| `CONTRIBUTING.md` | Principles, Development Setup, Feature Change, Documentation Change, Quality Verification & PR Process |
| `PRD.md` | Problem Statement, Goals & Success Metrics, User Personas, Scope, Feature Requirements, Non-functional Requirements, Open Questions / Risks |
| `README.md` | Prerequisites, Quick Start, Architecture, Project Structure, Available Scripts/Commands, Configuration, Testing, Contributing, License |
| `ROADMAP.md` | Current Condition, State Definitions, Status Policy, Feature Roll-up, Branches in Flight, Risk Register |

Each document also carries an agreed set of project-specific H2s that are accepted without violation; see the constants in `taxonomy_doc_constant.rs` for the full per-document lists.

| Violation type     | Fires when                                                                            |
| ------------------- | ------------------------------------------------------------------------------------- |
| `h1_count`          | The document has zero or more than one level-1 heading.                              |
| `h2_missing`        | One or more of the required H2 sections is absent; the message names each one.         |
| `h2_unexpected`     | A level-2 heading is outside the required and allowed sets; the message asks for demotion to H3 or removal. |


---

## Group 7: Folder Structure

### AES701 — Shared Folder Purity

**Severity:** HIGH

A workspace holds a locked `shared` folder that contains only taxonomy, utility, and contract files. Any `capabilities_*`, `agent_*`, or `surface_*` file sitting there is misplaced and must be moved to a feature or surface folder.

| Violation type                  | Fires when                                                       |
| --------------------------------- | ------------------------------------------------------------------ |
| `shared_has_forbidden_files` | A shared folder holds a `capabilities_*`, `agent_*`, or `surface_*` file. |

---

### AES702 — Feature Folder Health

**Severity:** MEDIUM

A feature folder is a member subdirectory (under `crates/`, `modules/`, or `packages/`) that carries business logic. It must hold at least one `agent_*_orchestrator` file AND at least one `capabilities_*` file. A folder with only one side of that pair is incomplete and should be split or merged into the correct home.

| Violation type                    | Fires when                                                                    |
| ------------------------------------ | ------------------------------------------------------------------------------- |
| `feature_missing_agent`       | The folder holds capabilities but no `agent_*_orchestrator` file.           |
| `feature_missing_capability` | The folder holds an agent orchestrator but no `capabilities_*` file.        |

---

### AES703 — Surface Folder Purity

**Severity:** MEDIUM

A surface folder is a folder where surface files dominate — it has more surface files than all other layers combined. Such a folder must contain surface files only. Any `capabilities_*` or `agent_*` file in a surface folder is misplaced and should move to a feature folder. Utility files, barrels, and entry-point wrappers are permitted alongside surfaces.

| Violation type                       | Fires when                                                             |
| -------------------------------------- | ------------------------------------------------------------------------ |
| `surface_has_misplaced_files` | A surface-dominated folder holds a `capabilities_*` or `agent_*` file. |


---

### AES704 — Feature Folder Docs

**Severity:** MEDIUM

A feature folder documents itself. A folder carrying the layer files of a feature — capabilities, an orchestrator, or both — also carries the two documents beside its source: `FRD.md` saying what the feature does, and `BACKLOG.md` saying where its work stands. A folder holding neither is a feature nobody can read before changing. A folder carrying no capabilities and no orchestrator is not a feature and owes no doc pair.

| Violation type            | Fires when                                                                       |
| --------------------------- | ---------------------------------------------------------------------------------- |
| `feature_missing_doc_pair` | A feature folder holds capabilities or an orchestrator but lacks `FRD.md` or `BACKLOG.md`. |

---

### AES705 — Surface Folder Docs

**Severity:** MEDIUM

A surface folder documents itself. The source of a surface says what the surface does; `DESIGN.md` says what it looks like, which entry points reach it, and which states a user sees. A folder that is surface-dominated but carries no `DESIGN.md` leaves the next reader inferring the surface's shape from its handlers.

| Violation type            | Fires when                                                              |
| --------------------------- | ------------------------------------------------------------------------- |
| `surface_missing_design_md` | A surface-dominated folder carries no `DESIGN.md`.                      |
