# AES (Agentic Engineering System) Rules — v3.0

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full 7-layer specification.

---

## Summary

**38 rules across 7 groups:** Naming (AES101–103) 3, Import (AES201–205) 5, Quality (AES301–305) 5, Role (AES401–406) 6, Orphan (AES501–506) 6, Doc (AES601–AES607) 7, Structure (AES701–AES706) 6 — 3·5·5·6·6·7·6 = 38. The `scan`/`check` command runs the five code linters (groups 1–5) plus structure and the doc-audit pass — all 7 groups — surfacing AES601–AES607 inside `scan`/`check`; the `docs` command covers group 6. External adapters (Clippy, Ruff, ESLint, …) run alongside via `external` and emit tool-native codes.

| Code   | Name                | Severity | Group  | Description                                                                               |
| -------- | --------------------- | ---------- | -------- | --------------------------------------------------------------------------------------------|
| AES101 | Naming Convention   | HIGH     | Naming | Filename must follow`prefix_concept_suffix` pattern — lowercase, underscore, min 3 words.|
| AES102 | Suffix Prefix Rules | HIGH     | Naming | Suffix must match layer definition — allowed, forbidden, mandatory strict.               |
| AES103 | Test File Prefix    | HIGH     | Naming | A file in `tests/` or `benches/` must carry a legal test-type prefix and neither directory may nest.|

| Code   | Name             | Severity | Group  | Description                                                                                   |
| -------- | ------------------ | ---------- | -------- | ------------------------------------------------------------------------------------------------|
| AES201 | Forbidden Import | CRITICAL | Import | Cross-layer imports must comply with allowed/mandatory/forbidden rules.                       |
| AES202 | Mandatory Import | HIGH     | Import | File is missing required imports defined by config.                                           |
| AES203 | Unused Import    | MEDIUM   | Import | Symbol is imported but never used in file scope.                                              |
| AES204 | Dummy Import     | HIGH     | Import | Import string matches a forbidden dummy pattern; symbol used only in dummy functions or stubs.|
| AES205 | Circular Import  | CRITICAL | Import | Circular dependency between layers — must be unidirectional bottom-up.                       |

| Code   | Name                 | Severity      | Group   | Description                                                                       |
| -------- | ---------------------- | --------------- | --------- | ------------------------------------------------------------------------------------|
| AES301 | File Maximum Limit   | HIGH          | Quality | File exceeds maximum allowed line count (default: 1000).                          |
| AES302 | File Minimum Limit   | HIGH          | Quality | File is below minimum required line count (default: 5).                           |
| AES303 | Mandatory Definition | HIGH / MEDIUM | Quality | File missing struct/enum/trait/class definition, or definition is empty.          |
| AES304 | Bypass Comment       | CRITICAL      | Quality | Forbidden bypass pattern detected (`#[allow]`, `unwrap()`, `panic!`, `noqa`, etc).|
| AES305 | Duplication Code     | MEDIUM        | Quality | Duplicate code blocks detected across files.                                      |

| Code   | Name              | Severity | Group | Description                                                                                    |
| -------- | ------------------- | ---------- | ------- | -------------------------------------------------------------------------------------------------|
| AES401 | Taxonomy Role     | HIGH     | Role  | Constant file contains non-constant declarations; primitives used in entity/error/event.       |
| AES402 | Contract Role     | HIGH     | Role  | Contract trait/method uses primitive types instead of taxonomy VO or constant types.           |
| AES403 | Capabilities Role | HIGH/MEDIUM/LOW | Role  | Capability exceeds 3 types, has no protocol implementor, reversed block order, missing or misordered block markers, local const, inline test, or public helper.|
| AES404 | Utility Role      | MEDIUM   | Role  | Utility contains struct/impl/trait/type-alias (Rust), class/interface/enum (Python/TS), or non-taxonomy imports|
| AES405 | Agent Role        | MEDIUM   | Role  | Orchestrator contains too many types, or has no aggregate implementor or uses`Any` annotations.|
| AES406 | Surface Role      | HIGH     | Role  | Passive surface contains active domain logic; file exceeds 15 functions.                       |

| Code   | Name                | Severity | Group  | Description                                                                                                                                      |
| -------- | --------------------- | ---------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------|
| AES501 | Taxonomy Orphan     | LOW      | Orphan | Taxonomy file has no inbound imports from any contract file.                                                                                     |
| AES502 | Contract Orphan     | MEDIUM   | Orphan | Contract protocol not implemented by capabilities or not called by agent; aggregate not called by surface.                                       |
| AES503 | Capabilities Orphan | MEDIUM   | Orphan | Capability not wired in any container AND unreachable in import graph.                                                                           |
| AES504 | Utility Orphan      | MEDIUM   | Orphan | Utility file not imported or consumed by any capability, agent, or surface layer.                                                                |
| AES505 | Agent Orphan        | HIGH     | Orphan | Agent orchestrator not called by any surface file or entry point.                                                                                |
| AES506 | Surface Orphan      | HIGH/MEDIUM/LOW | Orphan | Smart surface not imported by entry/router; utility surface not imported by smart surface; passive surface not imported by smart/utility surface.|

| Code   | Name                  | Severity | Group  | Description                                                                                    |
| -------- | --------------------- | -------- | ------ | -------------------------------------------------------------------------------------------------|
| AES601 | FR Format             | HIGH     | Doc    | Requirement IDs are `FR-<FEATURE>-NNN`; every requirement states all six fields; FRD requirement count equals protocol class count.                 |
| AES602 | Section Structure     | HIGH     | Doc    | FRD sections follow template order; required tables and subsections are present; API Contract holds exactly the Protocol API / Aggregate API pair, each with its own table.|
| AES603 | Spec Purity           | HIGH     | Doc    | Specs never name source files and never carry implementation state.                             |
| AES604 | Crosslinks            | HIGH     | Doc    | An FRD crosslinks its PRD and backlog; a feature backlog never restates master sections.        |
| AES605 | Doc Heading Structure | HIGH     | Doc    | Every root document carries one H1 and its own template-derived H2 set; off-template H2s are reported.|
| AES606 | Doc Template Conformance | HIGH     | Doc    | Mandated table-shaped sections carry exactly their template table: bullets or prose fire `section_not_table`; missing columns fire `table_missing_columns`; undeclared columns fire `table_extra_columns`; a header repeated as a data row fires `table_header_repeated`; a table where the HOW-TO shows none fires `table_not_in_template`; an unsanctioned `DS-` position H3 fires `h3_unexpected`. The six required root documents (`PRD.md`, `ROADMAP.md`, `README.md`, `AGENTS.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`) exist at a real workspace root; a missing one fires `missing_required_doc`.|
| AES607 | PR Folder Ownership      | HIGH     | Doc    | Every `PR-<PROJECT>-NNN` declared in the root PRD's `Project Requirements` must own a top-level module folder under `crates&#124;modules&#124;packages` named after its project identifier (`PR-LA-001` → `LA`). One identifier may own the whole non-empty layout; fired per unowned PR-ID anchored to its PRD heading line, code `AES607` / `pr_folder_not_owned`; surfaces via `scan`/`check` (DocRequest::audit_all) as well as `docs`.|

| Code   | Name               | Severity | Group     | Description                                                                               |
| -------- | -------------------- | ---------- | ----------- | ----------------------------------------------------------------------------------------------|
| AES701 | Shared Folder Purity | MEDIUM   | Structure | A `shared` folder holds a `capabilities_*`, `agent_*`, or `surface_*` file.                 |
| AES702 | Feature Folder Health | MEDIUM   | Structure | A feature folder lacks an `agent_*_orchestrator` or a `capabilities_*` file, holds foreign layer files, lacks its doc pair, or is not mirrored as a `shared/src/<feature>/` subfolder.                                 |
| AES703 | Surface Folder Purity | MEDIUM   | Structure | A surface folder holds a `capabilities_*`, `agent_*`, or `utility_*` file, or lacks DESIGN.md.             |
| AES704 | Test Suite Coverage | MEDIUM   | Structure | A feature folder lacks a `tests/` category or its `benches/` benchmark.                                     |
| AES705 | Member-Root Placement | MEDIUM   | Structure | A `crates/`, `modules/`, or `packages/` root holds a loose `capabilities_*`, `agent_*`, `surface_*`, or `utility_*` file.|
| AES706 | PR-LA Folder Ownership | MEDIUM | Structure | Every `PR-LA-001..N` id in PRD.md/ROADMAP.md maps positionally to a crate folder under `crates&#124;modules&#124;packages` (`PR-LA-001` → first folder, `filesystem`), and each crate folder's FRD carries `FR-` ids whose feature prefix matches its folder name (`crates/filesystem` → `FR-Filesystem-NNN`). Violation types: `pr_la_missing_frd`, `pr_la_fr_id_mismatch`, `pr_la_id_mismatch`.|

### AES705 — Member-Root Placement

**Severity:** MEDIUM

A member dir — `crates/`, `modules/`, or `packages/` — holds folders and
wiring. A layer file loose at its root belongs to a folder beneath it, and
without that folder it has no owning feature: AES702 cannot hold it to a
contract, AES704 cannot ask it for a test suite, and no FRD describes it.

| Prefix           | Belongs in             |
| ---------------- | -----------------------|
| `capabilities_`  | a feature folder       |
| `agent_`         | the feature that owns the orchestrator|
| `surface_`       | a surface folder       |
| `utility_`       | the shared folder      |

Only the top level is read. `root_*` wiring, a barrel module, and any file whose
stem carries no layer prefix — a README, a manifest — stay legal at a member
root, because the container composes them there.

---

### AES706 — PR-LA Folder Ownership

**Severity:** MEDIUM

Every `PR-LA-001..N` id declared in PRD.md/ROADMAP.md's `Project Requirements` maps
positionally to a crate folder under `crates`, `modules`, or `packages`
(`PR-LA-001` → the first folder). Each crate folder's FRD must carry `FR-`
ids whose feature prefix matches the folder name (`crates/filesystem` →
`FR-Filesystem-NNN`). Violation types: `pr_la_missing_frd`, `pr_la_fr_id_mismatch`,
`pr_la_id_mismatch`.

---

## Group 1: Naming

| Code | Name | Severity | Sub-checks | Violation names | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES101 | Naming Convention | HIGH | 1 | `NAMING_CONVENTION` | check/scan, naming |
| AES102 | Suffix/Prefix Rules | HIGH | 3 | `SUFFIX_PREFIX` | check/scan, naming |
| AES103 | Test File Prefix | HIGH | 3 | `TEST_FILE_PREFIX` | check/scan, naming |

### AES101 — Naming Convention

| Field | Value |
| --- | --- |
| Code / Severity | AES101 / HIGH |
| Fires when | File stem violates `prefix_concept[_concept…]_suffix`: lowercase, `_` separated, ≥3 words (`NamingConfig::new(Count::new(3))`) |
| Scope | Production source files; `tests/`+`benches/` handled by AES103 only |
| Fix | Rename to the layer pattern, e.g. `capabilities_user_checker.rs` |
| Exceptions | `DEFAULT_RULE_EXCEPTIONS` in `shared_common` |

#### Sub-checks

| # | Sub-check | Severity | Violation name | Fires when |
| --- | --- | --- | --- | --- |
| 1 | Naming convention | HIGH | `NAMING_CONVENTION` | Stem is not the `prefix_concept_suffix` shape (all-lowercase, ≥3 underscore-separated words) |

### AES102 — Suffix/Prefix Rules

| Field | Value |
| --- | --- |
| Code / Severity | AES102 / HIGH |
| Fires when | A file's layer suffix matches neither its layer's allowed set nor its forbidden list (forbidden-suffix, strict-policy, and flexible-policy sub-checks) |
| Scope | Production source files whose stem carries a recognized layer prefix (`taxonomy_`, `contract_`, `utility_`, `capabilities_`, `agent_`, `surface_`, `root_`) |
| Fix | Rename with an allowed suffix for the detected layer, or move the file to the layer whose suffix set it matches |
| Exceptions | Barrel/entry files (`main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`) plus any `exceptions` entries for `AES102` in config |

#### Layer suffix policy

| Layer | Policy | Allowed Suffixes | Forbidden Suffixes |
| --- | --- | --- | --- |
| `root` | strict | `entry`, `container` | — |
| `taxonomy` | strict | `vo`, `entity`, `error`, `event`, `constant`, `request`, `response` | — |
| `contract` | strict | `protocol`, `aggregate` | — |
| `utility` | flexible | any | `vo`, `entity`, `error`, `event`, `constant`, `protocol`, `aggregate`, `request`, `response` |
| `capabilities` | flexible | any | `vo`, `entity`, `error`, `event`, `constant`, `constants`, `protocol`, `aggregate`, `utility`, `request`, `response` |
| `agent` | strict | `orchestrator` | — |
| `surface` | strict | `command`, `controller`, `page`, `view`, `component`, `router`, `layout`, `hook`, `store`, `action`, `screen` | — |

#### Sub-checks

| # | Sub-check | Severity | Violation name | Fires when |
| --- | --- | --- | --- | --- |
| 1 | Forbidden suffix | HIGH | `SUFFIX_PREFIX` | Suffix is in this layer's `forbidden_suffix` list |
| 2 | Strict suffix policy | HIGH | `SUFFIX_PREFIX` | Layer policy is `strict` and suffix is not in `allowed_suffix` |
| 3 | Flexible suffix policy | HIGH | `SUFFIX_PREFIX` | Layer policy is `flexible` and suffix is in `forbidden_suffix` |

### AES103 — Test File Prefix

| Field | Value |
| --- | --- |
| Code / Severity | AES103 / HIGH |
| Fires when | A file in `tests/` or `benches/` is nested under a subdirectory, lacks a legal test-type prefix, or carries a prefix belonging to the other directory; barrel files are skipped |
| Scope | `tests/` and `benches/` files only; every other per-file auditor skips them; support prefixes like `regression_` satisfy AES103 but are not required categories for AES704 |
| Fix | Flatten the file into `tests/`/`benches/` and rename it `<type>_<subject>.<ext>` with a legal prefix |
| Exceptions | Barrel files `mod.rs`, `lib.rs`, `__init__.py`, `index.ts`, `index.js` — same exemption AES101 grants |

#### Legal prefixes

| Directory | Legal prefixes |
| --- | --- |
| `tests/` | `contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`, plus support `regression_`, `behavioral_`, `mock_`, `fixture_` |
| `benches/` | `bench_` |
| anywhere | any — a file outside both directories carries no test-type prefix and is not read by this rule |

#### Sub-checks

| # | Sub-check | Severity | Violation name | Fires when |
| --- | --- | --- | --- | --- |
| 1 | Flat layout | HIGH | `TEST_FILE_PREFIX` | File sits below `tests/` or `benches/` in a subdirectory |
| 2 | Legal prefix | HIGH | `TEST_FILE_PREFIX` | Stem starts with no legal prefix for its directory |
| 3 | Barrel exemption | HIGH | `TEST_FILE_PREFIX` | `mod.rs`, `lib.rs`, `__init__.py`, `index.ts`, `index.js` name no test and are skipped |

## Group 2: Layer & Import Boundary

| Code | Name | Severity | Sub-checks | Violation names | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES201 | Forbidden Import | CRITICAL | 13 scope rows | `FORBIDDEN_IMPORT` | check/scan, import |
| AES202 | Mandatory Import | HIGH | — | `MANDATORY_IMPORT` | check/scan, import |
| AES203 | Unused Import | MEDIUM | — | `UNUSED_IMPORT` | check/scan, import |
| AES204 | Dummy Import | HIGH | — | `DUMMY_IMPORT` | check/scan, import |
| AES205 | Circular Import | CRITICAL | — | `CIRCULAR_IMPORT` | check/scan, import |

### AES201 — Forbidden Import

| Field | Value |
| --- | --- |
| Code / Severity | AES201 / CRITICAL |
| Fires when | An import violates the allowed/mandatory/forbidden triple for the file's filename-prefix layer scope (13 rows) |
| Scope | Every scanned source file whose stem maps to one of the 13 layer scopes below |
| Fix | Remove or redirect the import to a layer the scope's Allowed cell permits |
| Config | `architecture.rules.AES201` (per-scope rows are not individually switchable) |
| Source | `config_architecture_rules.rs` `RULES` |

#### Import matrix (13 scopes)

| # | Scope | Allowed | Mandatory | Forbidden |
| --- | --- | --- | --- | --- |
| 1 | `taxonomy(vo)` | `taxonomy` | — | `agent,surface,contract,utility,capabilities,root` |
| 2 | `taxonomy(request,response)` | `taxonomy` | — | `agent,surface,contract,utility,capabilities,root` |
| 3 | `taxonomy(entity,error,event)` | `taxonomy` | `taxonomy(vo&#124;constant)` | `agent,surface,contract,utility,capabilities,root` |
| 4 | `taxonomy(constant)` | `taxonomy` | — | `agent,surface,contract,utility,capabilities,root` |
| 5 | `utility` | `taxonomy` | — | `agent,surface,contract,capabilities,root,utility` |
| 6 | `contract(protocol)` | `taxonomy,contract` | `taxonomy` | `agent,surface,capabilities,contract(aggregate),root` |
| 7 | `contract(aggregate)` | `taxonomy,contract` | `taxonomy` | `agent,surface,capabilities,root` |
| 8 | `capabilities` | `taxonomy,contract,utility` | `taxonomy,contract(protocol)` | `surface,agent,capabilities,root` |
| 9 | `agent(orchestrator)` | `taxonomy,contract(aggregate),contract(protocol),utility` | `taxonomy,contract(aggregate)` | `surface,capabilities,root` |
| 10 | `root` | `taxonomy,contract,capabilities,agent,surface` | — | — |
| 11 | `surface(command&#124;controller&#124;page)` | `taxonomy,contract(aggregate),utility` | — | `agent,capabilities,contract(protocol),root` |
| 12 | `surface(hook&#124;store&#124;action&#124;screen&#124;router)` | `taxonomy,contract(aggregate),utility` | — | `agent,capabilities,contract(protocol),surface(command&#124;controller&#124;page&#124;entry),root` |
| 13 | `surface(component&#124;view&#124;layout)` | `taxonomy,contract(aggregate),utility` | — | `agent,contract,capabilities,surface(command&#124;controller&#124;page&#124;entry&#124;hook&#124;store&#124;action&#124;screen&#124;router),root` |

### AES202 — Mandatory Import

| Field | Value |
| --- | --- |
| Code / Severity | AES202 / HIGH |
| Fires when | A file is missing a required import defined by its layer's `mandatory` list |
| Scope | Every scanned source file whose layer declares a non-empty `mandatory` list |
| Fix | Add the required import statement |
| Config | `architecture.rules.AES202` (per-scope rows not individually switchable) |
| Exceptions | `exceptions` entries for `AES202` in config; barrel files are skipped |

### AES203 — Unused Import

| Field | Value |
| --- | --- |
| Code / Severity | AES203 / MEDIUM |
| Fires when | A symbol is imported but never used in file scope (AST analysis across Rust, Python, JavaScript) |
| Scope | Every scanned source file |
| Fix | Remove the unused import or use the symbol |
| Config | `architecture.rules.AES203` |
| Exceptions | `exceptions` entries for `AES203` in config; barrel files are skipped |

### AES204 — Dummy Import

| Field | Value |
| --- | --- |
| Code / Severity | AES204 / HIGH |
| Fires when | An import string matches a forbidden dummy pattern — symbols used only inside `_use_mandatory_imports` dummy functions, dummy functions, or dummy trait impls |
| Scope | Every scanned source file |
| Fix | Use imported symbols in real logic, remove `_use_mandatory_imports` functions, implement contract methods with real behavior |
| Config | `architecture.rules.AES204` |
| Exceptions | `exceptions` entries for `AES204` in config; barrel files are skipped |

### AES205 — Circular Import

| Field | Value |
| --- | --- |
| Code / Severity | AES205 / CRITICAL |
| Fires when | A back-edge or cross-layer dependency cycle exists between layers |
| Scope | Every scanned source file |
| Fix | Extract shared types to taxonomy, define a contract protocol, or restructure dependency direction to bottom-up (`taxonomy → contract / utility → capabilities → agent → surface → root`) |
| Config | `architecture.rules.AES205` |
| Exceptions | `exceptions` entries for `AES205` in config |

## Group 3: File & Content Quality

| Code | Name | Severity | Sub-checks | Violation names | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES301 | File Maximum Limit | HIGH | — | `FILE_TOO_LARGE` | check/scan, quality |
| AES302 | File Minimum Limit | HIGH | — | `FILE_TOO_SHORT` | check/scan, quality |
| AES303 | Mandatory Definition | HIGH (missing) / MEDIUM (empty) | 2 | `MANDATORY_DEFINITION`, `DEAD_INHERITANCE` | check/scan, quality |
| AES304 | Bypass Comment | CRITICAL | — | `BYPASS_COMMENT`, `UNWRAP_EXPECT`, `PANIC`, `UNIMPLEMENTED`, `TODO` | check/scan, quality |
| AES305 | Code Duplication | MEDIUM | 1 | — | check/scan, quality |

### AES301 — File Maximum Limit

| Field | Value |
| --- | --- |
| Code / Severity | AES301 / HIGH |
| Fires when | File exceeds the maximum allowed line count (default: 1000) |
| Scope | Every scanned source file; barrel/entry files and `*_constant` files are excepted |
| Fix | Split into smaller files |
| Config | `architecture.rules.AES301` (`code_analysis.max_lines`) |
| Exceptions | `DEFAULT_RULE_EXCEPTIONS` plus per-layer `exceptions` |

### AES302 — File Minimum Limit

| Field | Value |
| --- | --- |
| Code / Severity | AES302 / HIGH |
| Fires when | File is below the minimum required line count (default: 5) |
| Scope | Every scanned source file; barrel/entry files and `*_constant` files are excepted |
| Fix | Merge into a related module or add more documentation |
| Config | `architecture.rules.AES302` (`code_analysis.min_lines`) |
| Exceptions | `DEFAULT_RULE_EXCEPTIONS` plus per-layer `exceptions` |

### AES303 — Mandatory Definition

| Field | Value |
| --- | --- |
| Code / Severity | AES303 / HIGH (missing) / MEDIUM (empty) |
| Fires when | A source file has no struct/enum/trait/class definition, or a definition is empty (unit struct without impl/derive, empty class/interface) |
| Scope | Every scanned source file; barrel/entry files, `*_constant.rs`, `*_constant.py`, `main.rs`, `main.py`, `py.typed` are excepted |
| Fix | Group functions into a struct or implement an interface; remove empty definitions |
| Config | `architecture.rules.AES303` (`code_analysis.mandatory_class_definition`) |
| Exceptions | `__init__.py`, `mod.rs`, `lib.rs`, `*_constant.rs`, `*_constant.py` |

#### Sub-checks

| # | Sub-check | Severity | Violation name | Fires when |
| --- | --- | --- | --- | --- |
| 1 | Missing definition | HIGH | `MANDATORY_DEFINITION` | File has no struct/enum/trait/class at all |
| 2 | Empty / dead definition | MEDIUM | `DEAD_INHERITANCE` | `struct Foo;`, `impl X for Y {}`, `class Foo: pass`, `class Foo {}` |

#### Checker source

| Checker | Method | Path |
| --- | --- | --- |
| `MandatoryDefinitionChecker` | `check_mandatory_class_definition()` | `quality-rules/capabilities_mandatory_definition_checker.rs` |
| `DeadInheritanceChecker` | `check_dead_inheritance()` | `quality-rules/capabilities_dead_inheritance_checker.rs` |

### AES304 — Bypass Comment

| Field | Value |
| --- | --- |
| Code / Severity | AES304 / CRITICAL |
| Fires when | A forbidden bypass pattern is detected: `#[allow(...)]`, `unwrap`/`expect`, `panic!`, `todo`/`unimplemented`/`unreachable`, `noqa`, `type: ignore`, `eslint-disable`, `ts-ignore`/`ts-expect-error`, `FIXME`/`HACK`/`XXX`, Python `raise NotImplementedError`/`assert False`, JS/TS `throw new Error(...)`/`TypeError`/`RangeError`/`ReferenceError`/`SyntaxError` |
| Scope | Every scanned source file |
| Fix | Use proper error handling |
| Config | `architecture.rules.AES304` (`code_analysis.forbidden_bypass` patterns; fallback default list when empty) |
| Exceptions | Test modules are skipped |

#### Forbidden bypass patterns

| Pattern | Violation kind | Fires when |
| --- | --- | --- |
| `#[allow(...)]` | `BYPASS_COMMENT` | Forbidden bypass attribute |
| `unwrap()` / `expect()` | `UNWRAP_EXPECT` | Forbidden unwrap or expect call |
| `panic!` | `PANIC` | Forbidden panic call |
| `todo` | `TODO` | Forbidden unfinished-implementation marker |
| `unimplemented` / `unreachable` | `UNIMPLEMENTED` | Forbidden unimplemented-body marker |
| `noqa` / `type: ignore` / `eslint-disable` / `ts-ignore` / `ts-expect-error` | `BYPASS_COMMENT` | Forbidden lint-stopper comment |
| `FIXME` / `HACK` / `XXX` | `BYPASS_COMMENT` | Forbidden TODO marker |
| `raise NotImplementedError` / `assert False` | `UNIMPLEMENTED` / `PANIC` | Python bypass patterns |
| `throw new Error(...)` / `throw new TypeError(...)` / `throw new RangeError(...)` / `throw new ReferenceError(...)` / `throw new SyntaxError(...)` | `PANIC` | JS/TS bypass pattern |

### AES305 — Duplication Code

| Field | Value |
| --- | --- |
| Code / Severity | AES305 / MEDIUM |
| Fires when | A file's share of shared normalized content windows exceeds the duplication threshold |
| Scope | Every scanned source file; import-only windows are excluded |
| Fix | Extract duplicated logic into shared utilities |
| Config | `architecture.rules.AES305` (`code_analysis.min_lines`, `duplication_threshold`) |

## Group 4: Role Violations

| Code | Name | Severity | Sub-checks | Violation names | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES401 | Taxonomy Role | HIGH | — | `TAXONOMY_ROLE` | check/scan, role |
| AES402 | Contract Role | HIGH | — | `CONTRACT_ROLE` | check/scan, role |
| AES403 | Capabilities Role | HIGH / MEDIUM / LOW per sub-check | 8 | `CAPABILITY_ROLE` | check/scan, role |
| AES404 | Utility Role | MEDIUM | — | `UTILITY_ROLE` | check/scan, role |
| AES405 | Agent Role | HIGH / MEDIUM / LOW per sub-check | 11 | `AGENT_ROLE` | check/scan, role |
| AES406 | Surface Role | HIGH (one sub-check MEDIUM) | 4 | `SURFACE_ROLE` | check/scan, role |

### AES401 — Taxonomy Role

| Field | Value |
| --- | --- |
| Code / Severity | AES401 / HIGH |
| Fires when | A `_constant` file carries a non-const declaration, or an `_entity`/`_error`/`_event` file uses a primitive type directly in a field declaration |
| Scope | Taxonomy files: `*_constant.rs`, `*_entity.rs`, `*_error.rs`, `*_event.rs` |
| Fix | Replace primitives with taxonomy value objects; keep `_constant` files const-only |
| Source | `capabilities_taxonomy_role_auditor.rs` |
| Exceptions | Barrel files are skipped |

#### Sub-checks

| # | Sub-check | Severity | Violation name | Fires when |
| --- | --- | --- | --- | --- |
| 1 | Constant purity | HIGH | `TAXONOMY_ROLE` | `_constant` file contains non-const declarations |
| 2 | Primitive in taxonomy | HIGH | `TAXONOMY_ROLE` | `_entity`/`_error`/`_event` file uses `String`, `i32`, `bool`, `int`, `float` etc. directly |

### AES402 — Contract Role

| Field | Value |
| --- | --- |
| Code / Severity | AES402 / HIGH |
| Fires when | A contract trait/method signature uses a primitive type instead of a taxonomy VO or constant |
| Scope | Contract files (`*_protocol.rs`, `*_aggregate.rs`); test projects are the primary target |
| Fix | Replace primitives with VO/constant types from the taxonomy layer |
| Source | `utility_contract_role_checker.rs` |

#### Sub-checks

| # | Sub-check | Severity | Violation name | Fires when |
| --- | --- | --- | --- | --- |
| 1 | Contract role | HIGH | `CONTRACT_ROLE` | Primitive type in a contract trait/method signature |

### AES403 — Capabilities Role

| Field | Value |
| --- | --- |
| Code / Severity | AES403 / HIGH / MEDIUM / LOW per sub-check |
| Fires when | Capability violates protocol enforcement or the 3-block structure; per-sub-check severity applies |
| Scope | Capability files (`*_capability*`); runs for Rust, Python, and TypeScript/JavaScript |
| Fix | Ensure capability implements its protocol; split routing across capabilities; reorder impl blocks so protocol implementation comes first |
| Source | `capabilities_capabilities_{rust,python,ts}_role_auditor.rs` |

#### Sub-checks

| # | Sub-check | Severity | Violation name | Fires when |
| --- | --- | --- | --- | --- |
| 1 | CapabilityTooManyTypes | HIGH | `CAPABILITY_ROLE` | File exceeds max 3 type declarations |
| 2 | CapabilityNoImplementor | MEDIUM | `CAPABILITY_ROLE` | No struct/class implements a `_protocol` contract trait |
| 3 | CapabilityMultiProtocol | MEDIUM | `CAPABILITY_ROLE` | File implements more than one protocol trait |
| 4 | CapabilityBlockOrder | HIGH | `CAPABILITY_ROLE` | Block 2 (protocol trait impl) does not precede Block 3 (inherent impl) |
| 5 | CapabilityBlockMarkers | MEDIUM | `CAPABILITY_ROLE` | `Block <n>:` banners missing, out of order, or above 3 |
| 6 | CapabilityLocalConstant | MEDIUM | `CAPABILITY_ROLE` | File-level `const` declared inline; belongs in `taxonomy_<domain>_constant.rs` |
| 7 | CapabilityEmbeddedTest | LOW | `CAPABILITY_ROLE` | `#[cfg(test)]` or `mod tests` declared inline |
| 8 | CapabilityPublicHelper | MEDIUM | `CAPABILITY_ROLE` | Block 3 helper with no production caller is `pub` |

#### Language coverage

| Language | Coverage |
| --- | --- |
| Rust | All eight sub-checks |
| Python | All eight sub-checks |
| TypeScript / JavaScript | All eight sub-checks |

### AES404 — Utility Role

| Field | Value |
| --- | --- |
| Code / Severity | AES404 / MEDIUM |
| Fires when | A utility file contains stateful objects, struct/class state, trait definitions, impl blocks, type aliases (Rust `pub type`), interface definitions (TS), enum definitions |
| Scope | Utility files (`*_utility*`) |
| Fix | Refactor to stateless standalone functions; remove non-taxonomy imports; move stateful logic into Capabilities |
| Source | `agent_role_orchestrator.rs:227` |

#### Sub-checks

| # | Sub-check | Severity | Violation name | Fires when |
| --- | --- | --- | --- | --- |
| 1 | Utility role | MEDIUM | `UTILITY_ROLE` | Utility contains stateful objects or type definitions |

### AES405 — Agent Role

| Field | Value |
| --- | --- |
| Code / Severity | AES405 / HIGH / MEDIUM / LOW per sub-check |
| Fires when | Agent orchestrator violates role discipline; per-sub-check severity applies; dispatch in `agent_role_orchestrator.rs:140-168` |
| Scope | Agent files (`*_orchestrator*`) |
| Fix | Coordinate at minimum 2 injected subsystems; no state mutation outside the constructor; delegate computation to capabilities |
| Source | `agent_role_orchestrator.rs:140-168`; severities in `contract_role_protocol.rs:173-267` |

#### Sub-checks

| # | Sub-check (dispatch order) | Severity | Violation name | Fires when |
| --- | --- | --- | --- | --- |
| 1 | `check_agent_routing` (implementor + type budget + annotation) | MEDIUM / HIGH / MEDIUM | `AGENT_ROLE` | No aggregate-trait implementor (MEDIUM); >3 type declarations (HIGH); `Any`/`any` annotation (MEDIUM) |
| 2 | `check_agent_block_order` | HIGH | `AGENT_ROLE` | Block 2 does not precede Block 3 |
| 3 | `check_agent_protocol_forbidden` | HIGH | `AGENT_ROLE` | Agent implements a contract protocol |
| 4 | `check_agent_block_markers` | MEDIUM | `AGENT_ROLE` | `Block <n>:` banner with n > 3 |
| 5 | `check_agent_io_forbidden` | MEDIUM | `AGENT_ROLE` | Forbidden I/O operations in the agent file |
| 6 | `check_agent_constant_placement` | MEDIUM | `AGENT_ROLE` | File-level constants; policy constants belong in taxonomy |
| 7 | `check_agent_computation` | MEDIUM | `AGENT_ROLE` | `.sum()` / `.fold()` / `.reduce()` / arithmetic in agent file |
| 8 | `check_agent_abstract_method` | MEDIUM | `AGENT_ROLE` | `@abstractmethod` in agent file |
| 9 | `check_agent_stateless` | MEDIUM | `AGENT_ROLE` | State mutation outside the constructor |
| 10 | `check_agent_free_fn` | LOW | `AGENT_ROLE` | Module-level free functions |
| 11 | `check_agent_subsystem_count` | LOW | `AGENT_ROLE` | Fewer than 2 injected protocol subsystems |

### AES406 — Surface Role

| Field | Value |
| --- | --- |
| Code / Severity | AES406 / HIGH (one sub-check MEDIUM) |
| Fires when | A surface file carries too many responsibilities, active domain logic in a passive surface, or direct surface-to-domain calls |
| Scope | Surface files (`*_command*`, `*_controller*`, `*_page*`, `*_router*`, `*_hook*`, `*_store*`, `*_action*`, `*_screen*`, `*_component*`, `*_view*`, `*_layout*`) |
| Fix | Split surface responsibilities; move business logic to Capabilities; route domain calls through contracts |
| Source | `capabilities_surface_role_auditor.rs:69-70` |

#### Sub-checks

| # | Sub-check | Severity | Violation name | Fires when |
| --- | --- | --- | --- | --- |
| 1 | Function-count ceiling (tier-specific) | HIGH | `SURFACE_ROLE` | Smart exempt; utility 25; passive 50 |
| 2 | Active domain logic in passive surface | HIGH | `SURFACE_ROLE` | `_component`/`_view`/`_layout` carries business logic |
| 3 | Surface logic bypass | MEDIUM | `SURFACE_ROLE` | Surface calls domain logic directly (`lint_path(`, `walk_rs_files(`) |
| 4 | Role boundary violation | HIGH | `SURFACE_ROLE` | Passive surface exceeds MAX_PUBLIC_METHODS (50) functions |

## Group 5: Orphan Code

| Code | Name | Severity | Sub-checks | Violation names | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES501 | Taxonomy Orphan | LOW | — | `TAXONOMY_ORPHAN` | check/scan, orphan |
| AES502 | Contract Orphan | MEDIUM | — | `CONTRACT_ORPHAN` | check/scan, orphan |
| AES503 | Capabilities Orphan | MEDIUM | — | `CAPABILITIES_ORPHAN` | check/scan, orphan |
| AES504 | Utility Orphan | MEDIUM | — | `UTILITY_ORPHAN` | check/scan, orphan |
| AES505 | Agent Orphan | HIGH | — | `AGENT_ORPHAN` | check/scan, orphan |
| AES506 | Surface Orphan | HIGH smart / MEDIUM utility / LOW passive | — | `SURFACE_ORPHAN` | check/scan, orphan |

### AES501 — Taxonomy Orphan

| Field | Value |
| --- | --- |
| Code / Severity | AES501 / LOW |
| Fires when | A taxonomy file (VO, entity, error, event, constant) has no inbound imports from any contract file |
| Scope | Taxonomy files |
| Fix | Wire the taxonomy type into a contract, or remove the dead taxonomy file |
| Source | `capabilities_orphan_taxonomy_analyzer.rs:162` |

### AES502 — Contract Orphan

| Field | Value |
| --- | --- |
| Code / Severity | AES502 / MEDIUM |
| Fires when | A contract protocol is not implemented by any capabilities file, or a contract aggregate is not implemented by any agent file; not imported by any container or entry point |
| Scope | Contract files (`*_protocol*`, `*_aggregate*`) |
| Fix | Implement the protocol/aggregate, or wire it into a container; barrel re-exports do not count |
| Source | `capabilities_orphan_contract_analyzer.rs:79` |

### AES503 — Capabilities Orphan

| Field | Value |
| --- | --- |
| Code / Severity | AES503 / MEDIUM |
| Fires when | A capability file is not wired in any `_container` |
| Scope | Capability files |
| Fix | Wire the capability into a container |
| Source | `capabilities_orphan_capabilities_analyzer.rs:85` |

### AES504 — Utility Orphan

| Field | Value |
| --- | --- |
| Code / Severity | AES504 / MEDIUM |
| Fires when | A utility file is not imported or consumed by any capability, agent, or surface layer, or is only imported by other utility files |
| Scope | Utility files |
| Fix | Import the utility from a capability/agent/surface, or remove it |
| Source | `capabilities_orphan_utility_analyzer.rs:158` |

### AES505 — Agent Orphan

| Field | Value |
| --- | --- |
| Code / Severity | AES505 / HIGH |
| Fires when | An agent orchestrator file (suffix `_orchestrator`) is not wired in any `_container` |
| Scope | Agent orchestrator files |
| Fix | Wire the orchestrator into a container |
| Source | `capabilities_orphan_agent_analyzer.rs:102` |

### AES506 — Surface Orphan

| Field | Value |
| --- | --- |
| Code / Severity | AES506 / HIGH smart / MEDIUM utility / LOW passive |
| Fires when | A surface file is not imported by its required consumer; severity depends on surface category |
| Scope | Surface files (`*_command*`, `*_controller*`, `*_page*`, `*_router*`, `*_hook*`, `*_store*`, `*_action*`, `*_screen*`, `*_component*`, `*_view*`, `*_layout*`) |
| Fix | Import the surface from its required consumer |
| Source | `capabilities_orphan_surfaces_analyzer.rs:43-45` |

| Category | Suffixes | Severity | Must be imported by |
| --- | --- | --- | --- |
| Smart | `_command`, `_controller`, `_page`, `_router` | HIGH | entry |
| Utility | `_hook`, `_store`, `_action`, `_screen` | MEDIUM | smart surface |
| Passive | `_component`, `_view`, `_layout` | LOW | smart or utility surface |
| `_entry` | — | never fires | — |

## Group 6: Document Invariants

| Code | Name | Severity | Sub-checks | Violation types | Ran by |
| --- | --- | --- | --- | --- | --- |
| AES601 | FR Format | HIGH | — | 14 | check/scan, docs |
| AES602 | Section Structure | HIGH | — | 1 | check/scan, docs |
| AES603 | Spec Purity | HIGH | — | 2 | check/scan, docs |
| AES604 | Crosslinks | HIGH | — | 11 | check/scan, docs |
| AES605 | Doc Heading Structure | HIGH | — | 5 | check/scan, docs |
| AES606 | Doc Template Conformance | HIGH | — | 11 | check/scan, docs |
| AES607 | PR Folder Ownership | HIGH | — | 1 | check/scan, docs |

### AES601 — FR Format

| Field | Value |
| --- | --- |
| Code / Severity | AES601 / HIGH |
| Fires when | FR-ID is bare (`FR-NNN`) without feature prefix; a requirement omits a field; FRD requirement count differs from contract `I*Protocol` count; BACKLOG/DESIGN/DATA row ID mismatches or missing rows |
| Scope | Recognized docs: `PRD.md`, `FRD.md`, `BACKLOG.md`, `DESIGN.md`, `DATA.md` |
| Fix | Merge/split requirements or protocol classes; align row IDs |
| Source | `capabilities_doc_fr_format_checker.rs` |

#### Violation types

| Violation type | Fires when |
| --- | --- |
| `id_missing_feature_prefix` | An ID reads `FR-NNN` with no feature prefix |
| `api_method_not_found` | A method named in the Protocol API / Aggregate API table has no matching declaration in the contract module |
| `backlog_id_not_fr` | A BACKLOG `## Backlog` row's ID cell is not a valid FRD FR-ID |
| `backlog_id_missing` | An FRD FR-ID has no corresponding BACKLOG `## Backlog` row |
| `ds_backlog_id_not_ds` | A BACKLOG `## Backlog` row's ID cell is not a valid DESIGN.md `DS-<DESIGNNAME>-NNN` ID |
| `ds_backlog_id_missing` | A DESIGN.md `DS-<DESIGNNAME>-NNN` ID has no corresponding BACKLOG `## Backlog` row |
| `dt_backlog_id_not_dt` | A BACKLOG `## Backlog` row's ID cell is not a valid DATA.md `DT-<DATANAME>-NNN` ID |
| `dt_backlog_id_missing` | A DATA.md `DT-<DATANAME>-NNN` ID has no corresponding BACKLOG `## Backlog` row |
| `pr_roadmap_id_missing` | A PRD `PR-<PROJECT>-NNN` item has no ROADMAP `Roadmap Status` entry |
| `roadmap_pr_id_not_pr` | A ROADMAP `Roadmap Status` item is not declared in the PRD's `Project Requirements` |
| `backlog_qualifier_mismatch` | A BACKLOG entry's H3 qualifier does not equal any ID declared by the paired spec |
| `pr_backlog_id_missing` | A BACKLOG `## Backlog Status` entry H3 whose qualifier has no matching PR-ID in the root ROADMAP/PRD pair |
| `backlog_id_not_pr` | A token in a BACKLOG entry's `Next`/`Done`/`Blocked` row that is not a valid PR-ID of the root ROADMAP/PRD pair |
| `protocol_count_mismatch` | The FRD requirement count and the module's `I*Protocol` class count differ; one protocol class is one capability seam; aggregate traits (`I*Aggregate`) are excluded |

### AES602 — Section Structure

| Field | Value |
| --- | --- |
| Code / Severity | AES602 / HIGH |
| Fires when | The FRD's level-2 sections are out of template order, or the API Contract subsections do not match |
| Scope | `FRD.md` |
| Fix | Reorder sections to `Feature Overview → Functional Requirements → Public Interface → Reference`; accept exactly `### Protocol API` then `### Aggregate API` |
| Source | `capabilities_doc_section_structure_checker.rs` |

#### Violation types

| Violation type | Fires when |
| --- | --- |
| `order_violation` | Level-2 sections appear out of template order, or API Contract subsections are out of order |

### AES603 — Spec Purity

| Field | Value |
| --- | --- |
| Code / Severity | AES603 / HIGH |
| Fires when | A promise-bearing spec names a concrete source file or carries implementation state |
| Scope | Exactly four promise-bearing documents: `PRD.md`, `FRD.md`, `DATA.md`, `ROADMAP.md`; `ARCHITECTURE.md`, `CONTRIBUTING.md`, `DESIGN.md` excluded (`is_spec` never includes them) |
| Fix | Remove source-file names and status/checkbox/progress state from the spec |
| Source | `capabilities_doc_spec_purity_checker.rs` |

#### Violation types

| Violation type | Fires when |
| --- | --- |
| `source_file_named` | The spec names a concrete source file (`.py`, `.rs`, `.ts`, `.tsx`) |
| `status_leak` | The spec carries checkboxes, a Status field, implementation state, release state, a status marker, or a progress percentage |

### AES604 — Crosslinks

| Field | Value |
| --- | --- |
| Code / Severity | AES604 / HIGH |
| Fires when | A document's Reference section lacks a mandatory link, or a feature backlog restates a root-master state-vocabulary section |
| Scope | Recognized docs with Reference sections; `MASTER_ONLY_SECTIONS = ["Roadmap Status"]` |
| Fix | Add the missing link; remove restated master sections from the backlog |
| Source | `capabilities_doc_crosslink_checker.rs` |

#### Violation types

| Violation type | Fires when |
| --- | --- |
| `no_backlog_link` | An FRD's Reference section does not link its BACKLOG.md |
| `no_prd_link` | An FRD's Reference section does not link its PRD.md |
| `no_roadmap_link` | Reference section lacks a ROADMAP.md link |
| `no_architecture_link` | Reference section lacks an ARCHITECTURE.md link |
| `no_readme_link` | Reference section lacks a README.md link |
| `no_contributing_link` | Reference section lacks a CONTRIBUTING.md link |
| `no_frd_link` | Reference section lacks an FRD.md link |
| `no_data_link` | Reference section lacks a DATA.md link |
| `no_design_link` | Reference section lacks a DESIGN.md link |
| `no_required_link` | Fallback link violation for unmapped targets |
| `state_vocab_restated` | A feature backlog restates a root-master state-vocabulary section |

### AES605 — Doc Heading Structure

| Field | Value |
| --- | --- |
| Code / Severity | AES605 / HIGH |
| Fires when | Zero or >1 H1; a required H2 is absent; an H2 is outside required ∪ allowed; a non-placeholder line drifts from its template; YAML frontmatter is missing |
| Scope | Ten recognized docs: `PRD, ROADMAP, FRD, BACKLOG, DATA, DESIGN, README, AGENTS, ARCHITECTURE, CONTRIBUTING` |
| Fix | Restore the missing H2, demote/remove off-template H2s, match template lines verbatim, add frontmatter |
| Source | `capabilities_doc_heading_structure_checker.rs` |

#### Required and allowed H2s (per `DOC_HEADING_CONTRACTS`)

| Document | Required H2s | Allowed extra |
| --- | --- | --- |
| `AGENTS.md` | Agent Overview · Runtime · Project Structure · Commands · Reference | — |
| `ARCHITECTURE.md` | Purpose · Workspace Organization · Naming Convention · Vertical Slicing Layout · Taxonomy Layer · Contract Layer · Utility Layer · Capabilities Layer · Agent Layer · Surface Layer · Root Layer | — |
| `CONTRIBUTING.md` | Contribution Overview · Contribution Guidelines · Workflow · Commit Convention · Reference | — |
| `PRD.md` | Problem Statement · Project Requirements · Reference | — |
| `README.md` | Project Overview · Prerequisites · Quick Start · Reference | — |
| `ROADMAP.md` | Progress Overview · Roadmap Status · Reference | — |
| `FRD.md` | Feature Overview · Functional Requirements · Public Interface · Reference | `API Contract` (legacy) |
| `BACKLOG.md` | Progress Overview · Backlog Status · Reference | — |
| `DATA.md` | Data Overview · Data Requirements · Reference | `DT-` item H2s |
| `DESIGN.md` | Design Overview · Design Requirements · Reference | — |

#### Violation types

| Violation type | Fires when |
| --- | --- |
| `h1_count` | Zero or >1 H1 |
| `h2_missing` | A required H2 is absent |
| `h2_unexpected` | H2 outside required ∪ allowed |
| `line_drift` | A non-placeholder scaffolded line was edited away from its template |
| `frontmatter_missing` | YAML frontmatter block absent |

### AES606 — Doc Template Conformance

| Field | Value |
| --- | --- |
| Code / Severity | AES606 / HIGH |
| Fires when | A recognized document's body, entry table, or root/member document presence drifts from the shipped `HOW-TO-MAKE-*` template |
| Scope | Ten recognized docs: `PRD, ROADMAP, FRD, BACKLOG, DATA, DESIGN, README, AGENTS, ARCHITECTURE, CONTRIBUTING` |
| Fix | Re-shape the section to its template row (`docs scaffold` regenerates missing docs) |
| Source | `capabilities_doc_template_conformance_checker.rs` |

#### Violation types

| Violation type | Fires when |
| --- | --- |
| `section_not_table` | A mandated section carries bullets/prose instead of a table |
| `table_missing_columns` | A mandated table lacks a required column |
| `table_extra_columns` | Header carries a column the template never declares |
| `table_header_repeated` | A data row repeats the header row |
| `table_not_in_template` | A section whose template prescribes no table still carries one |
| `entry_missing_row` | Entry table's first column misses a required row label (`FRD_ROW_LABELS` family) |
| `h3_unexpected` | `###` in a position only `DS-<NAME>-NNN` may occupy |
| `missing_required_doc` | A required root doc absent at a real workspace root |
| `missing_member_doc` | Member folder missing its required doc pair (`FRD+BACKLOG`/`DATA+BACKLOG`/`DESIGN+BACKLOG`) |
| `unfilled_placeholder` | Template placeholder token survived into a non-prose line |
| `section_deleted` | A non-deletable template section is entirely absent |

#### Required table shapes

| Document | Section / role | Required shape |
| --- | --- | --- |
| `PRD.md` | `Project Requirements` entry (Aspect/Detail) | rows: `Description, User Story, In Scope, Out of Scope, Success Metric` |
| `ROADMAP.md` | `Roadmap Status` entry | rows: `Priority, State, Dependencies, Updated` |
| `BACKLOG.md` | `Backlog Status` entry | rows: `State, Done, In Progress, Blocked, Next, Risk, Updated` |
| `FRD.md` | `Functional Requirements` entry | rows: the 12 `FRD_ROW_LABELS` |
| `AGENTS.md` | `Runtime` entry; `Path/Purpose`; `Category/Command/Scope/CI Gate` | `AGENTS_RUNTIME_ROW_LABELS` / column sets |
| all | `Reference` | `Artifact, Purpose, Link` |

### AES607 — PR Folder Ownership

| Field | Value |
| --- | --- |
| Code / Severity | AES607 / HIGH |
| Fires when | A PR-ID declared in the root PRD's `Project Requirements` owns no matching top-level module folder under `crates|modules|packages` |
| Scope | Root PRD's `Project Requirements` entries |
| Fix | Create the module folder named after the PR-ID's project identifier |
| Source | `capabilities_doc_pr_folder_ownership_checker.rs:93` |

#### Violation types

| Violation type | Fires when |
| --- | --- |
| `pr_folder_not_owned` | A PRD-declared PR-ID's project identifier owns no matching module folder |

## Group 7: Folder Structure

### AES701 — Shared Folder Purity

**Severity:** MEDIUM (report-normalized from the structure audit)

A workspace holds a locked `shared` folder that contains only taxonomy, utility, and contract files. Any `capabilities_*`, `agent_*`, or `surface_*` file sitting there is misplaced and must be moved to a feature or surface folder.

| Violation type                  | Fires when                                                      |
| --------------------------------- | ------------------------------------------------------------------|
| `shared_has_forbidden_files` | A shared folder holds a `capabilities_*`, `agent_*`, or `surface_*` file.|

---

### AES702 — Feature Folder Health

**Severity:** MEDIUM

A feature folder is a member subdirectory (under `crates/`, `modules/`, or `packages/`) that carries business logic. It must hold at least one `agent_*_orchestrator` file AND at least one `capabilities_*` file. A folder with only one side of that pair is incomplete and should be split or merged into the correct home.

It must also hold its composition wiring: a `root_<concept>_container` file, or a `root_<concept>_entry` file when the feature's root is an entry point. The root layer is the only layer that may instantiate and wire the feature's components, so a feature without that file has no home for its wiring and nothing below it can reach it. The wiring may sit anywhere under the feature folder, in the feature's `src/` subfolder, or directly at the member root — one composition file at the member root composes every feature folder in the member and answers for them all.

The orchestrator check is per folder. A member-level orchestrator — one sitting directly under `crates/`, `modules/`, or `packages/` rather than inside a feature — does **not** answer for the folders beneath it. Each feature owns its own orchestration, so a member-level orchestrator is a routing aggregate over the features, and a feature folder missing its own agent is still a split feature.

**Shared kernel mirror.** A feature folder is also mirrored in its member's kernel: the subfolder `shared/src/<feature>/` must exist, where `<feature>` is the folder name with `-` replaced by `_` (the module name). The kernel holds the feature's contract and taxonomy modules, so a feature without that subfolder has no home for them. The mirror is required only for feature folders; a surface folder's shared subfolder is optional and never fires this check.

| Violation type                    | Fires when                                                                   |
| ------------------------------------ | -------------------------------------------------------------------------------|
| `feature_missing_agent`       | The folder holds capabilities but no `agent_*_orchestrator` file of its own.|
| `feature_missing_capability` | The folder holds an agent orchestrator but no `capabilities_*` file.       |
| `feature_missing_shared_subfolder` | The feature has no `shared/src/<feature>/` mirror subfolder in its member's kernel.|
| `feature_missing_root_container` | The feature holds no `root_*_container` or `root_*_entry` file under it, in its `src/` subfolder, or at the member root.|

---

### AES703 — Surface Folder Purity

**Severity:** MEDIUM

A surface folder is a folder where surface files dominate — it has more surface files than all other layers combined. Such a folder must contain surface files only. Any `capabilities_*`, `agent_*`, or `utility_*` file in a surface folder is misplaced: capability and agent files move to a feature folder, utility files move to the `shared/` kernel folder. Barrel files and entry-point wrappers are permitted alongside surfaces.

| Violation type                       | Fires when                                                            |
| -------------------------------------- | ------------------------------------------------------------------------|
| `surface_has_misplaced_files` | A surface-dominated folder holds a `capabilities_*`, `agent_*`, or `utility_*` file.|

---

### AES704 — Test Suite Coverage

**Severity:** MEDIUM

The `aes-testing-suite` skill fixes one test layout: seven test types in `tests/`
plus benchmarks in `benches/`, each named by a flat prefix. A feature folder that
owns source owes all eight, so a missing category is a structural gap rather
than a naming slip — AES103 asks whether a name is legal, AES704 asks whether the
category is present.

A folder is in scope when it holds at least one `capabilities_*` or one
`agent_*_orchestrator` file — the same gate AES702 uses to decide what counts as
a feature. A utility-only folder or a surface owes no suite.

| Prefix         | Directory  | Proves                                 |
| --------------- | ----------- | ---------------------------------------|
| `contract_`     | `tests/`    | Protocol/trait/interface impl exists   |
| `unit_`         | `tests/`    | One public function                    |
| `integration_`  | `tests/`    | Module / crate / package + DI wiring   |
| `dogfood_`      | `tests/`    | CLI against a live service; skip if unavailable|
| `smoke_`        | `tests/`    | App boots + responds, under 5 s        |
| `e2e_`          | `tests/`    | Full request lifecycle                 |
| `acceptance_`   | `tests/`    | Business requirement met               |
| `bench_`        | `benches/`  | Performance regression                 |

Support prefixes (`regression_`, `behavioral_`, `mock_`, `fixture_`) are legal
file names and satisfy **no** category: a folder carrying only regression guards
still owes the full seven.

| Violation type                     | Fires when                                                                |
| ----------------------------------- | --------------------------------------------------------------------------|
| `test_suite_missing_tests_dir`     | A feature folder owns source and has no `tests/` directory.               |
| `test_suite_missing_bench_dir`     | A feature folder owns source and has no `benches/` directory.             |
| `test_suite_missing_category`      | A feature folder owns source and no file carries one required test prefix. |

Each missing category is its own finding, so a reader sees exactly which seam is
untested rather than a single "your suite is incomplete".

---
