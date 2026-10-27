# FRD — Structure Rules (AES701–AES705)

## Feature Overview

The structure-rules crate enforces the 7-layer AES folder discipline by auditing the layout of every workspace. It checks that shared, feature, and surface folders obey their shape contracts — shared folders carry DATA.md + BACKLOG.md, feature folders carry FRD.md + BACKLOG.md, and surface folders carry DESIGN.md + BACKLOG.md — and that every feature folder carries a complete test suite. It returns findings as workspace-root-relative paths so a member-directory scan resolves them correctly.

## Functional Requirements

### FR-STR-001: Shared Folder Purity and Documentation (AES701)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-STR-001 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: Audit shared/kernel folders for forbidden file types and require the doc pair.
- **Input**: An audit root path and the filesystem inventory of each shared folder.
- **Output**: `Vec<LintResult>` carrying one finding per structural defect found in shared.
- **Business Rules**:
  - `capabilities_*`, `agent_*`, and `surface_*` files are forbidden in shared — they belong in feature folders. Each forbidden file produces one `shared_has_forbidden_files` finding.
  - A shared (kernel) folder must carry `DATA.md` and `BACKLOG.md`. Missing either produces one `shared_missing_doc_pair` finding listing the missing files.
  - Taxonomy, utility, and contract files are the only allowed layer prefixes in shared.
- **Edge Cases**: A shared folder with mixed permitted and forbidden files reports one finding per forbidden file. A shared folder with only taxonomy/utility/contract files and both DATA.md and BACKLOG.md is clean.
- **Error Handling**: If the shared folder path cannot be read, the auditor skips it silently.

### FR-STR-002: Feature Folder Health and Documentation (AES702)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-STR-002 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: Audit feature folders for completeness (capabilities + orchestrator + root composition wiring), forbid foreign layer files, and require the doc pair.
- **Input**: An audit root path and the filesystem inventory of each candidate feature folder.
- **Output**: `Vec<LintResult>` carrying one finding per structural defect found in a feature folder.
- **Business Rules**:
  - A feature folder must hold at least one `capabilities_*` file AND at least one `agent_*_orchestrator` file. Missing capabilities produces `feature_missing_capability`; missing orchestrator produces `feature_missing_agent`.
  - A feature folder must hold its composition wiring: a `root_<concept>_container` file, or a `root_<concept>_entry` file when the feature's root is an entry point. The root layer is the only layer that may instantiate and wire the feature's components. Absence produces one `feature_missing_root_container` finding.
  - A feature folder must not hold `utility_*`, `surface_*`, `taxonomy_*`, or `contract_*` files — those belong elsewhere. Each foreign file produces one `feature_has_forbidden_files` finding.
  - A feature folder must carry `FRD.md` and `BACKLOG.md` at its root. Missing either produces one `feature_missing_doc_pair` finding listing the missing files.
  - Reverse direction: any folder (including non-feature) that carries both `FRD.md` and `BACKLOG.md` must also hold at least one `*_orchestrator` file. Absence produces one `doc_pair_without_orchestrator` finding.
  - A folder with no capabilities and no orchestrator is not a feature and owes no document pair.
  - A feature folder is mirrored in its member's kernel: the subfolder `shared/src/<feature>/` (folder name, `-` → `_`) must exist. Its absence produces one `feature_missing_shared_subfolder` finding. The mirror is member-scoped — a feature under `crates/` mirrors into `crates/shared/src/`, one under `modules/` into `modules/shared/src/` — and is required only for feature folders. A surface folder's shared subfolder is optional and never fires this check.
- **Edge Cases**: A folder with capabilities but no orchestrator and no docs reports both `feature_missing_agent` and `feature_missing_doc_pair`. A folder with only an orchestrator and no capabilities reports `feature_missing_capability`. A utility-only folder is skipped entirely. A folder with a complete pair but no `root_` file reports `feature_missing_root_container` alongside any missing-side findings.
- **Error Handling**: If the folder path cannot be read, the auditor skips it silently.

### FR-STR-003: Surface Folder Purity and Documentation (AES703)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-STR-003 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: Audit surface-dominated folders for misplaced files and require DESIGN.md + BACKLOG.md.
- **Input**: An audit root path and the filesystem inventory of each surface-dominated folder.
- **Output**: `Vec<LintResult>` carrying one finding per structural defect found in a surface folder.
- **Business Rules**:
  - A surface-dominated folder must hold ONLY `surface_*` prefixed files (plus permitted support files like barrel files and package manifests).
  - Files with any other layer prefix (`capabilities_*`, `agent_*`, `utility_*`, `taxonomy_*`, `contract_*`, `root_*`) are misplaced and produce one `surface_has_misplaced_files` finding per file.
  - Utility files are **NOT allowed** in surface folders — they belong in feature folders.
  - A surface-dominated folder must carry `DESIGN.md` and `BACKLOG.md` at its root. Missing DESIGN.md produces `surface_missing_design_md`; missing BACKLOG.md produces `surface_missing_backlog_md`.
  - A folder that is not surface-dominated is not checked for AES703.
- **Edge Cases**: A feature-dominated folder that happens to contain one surface file is NOT surface-dominated and is checked under AES702 instead. A surface folder with only `surface_*` files, permitted support files, DESIGN.md, and BACKLOG.md is clean.
- **Error Handling**: If the folder path cannot be read, the auditor skips it silently.

### FR-STR-004: Test Suite Category Coverage (AES704)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-STR-004 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: Require every feature folder that owns source to carry one file per test type in `tests/` and at least one benchmark in `benches/`.
- **Input**: An audit root path and the filesystem inventory of each candidate feature folder.
- **Output**: `Vec<LintResult>` carrying one finding per missing test category, plus one finding per missing directory.
- **Business Rules**:
  - Scope uses the same gate as AES702: a folder holding at least one `capabilities_*` or one `agent_*_orchestrator` file is a feature and owes a suite. A utility-only folder, a surface, and the `shared` kernel are out of scope.
  - `tests/` must hold at least one file for each of the seven test types: `contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`.
  - `benches/` must hold at least one `bench_` file.
  - A missing `tests/` directory produces one `test_suite_missing_tests_dir` finding rather than seven per-category findings; a missing `benches/` produces one `test_suite_missing_bench_dir`.
  - Each absent category prefix produces its own `test_suite_missing_category` finding naming the prefix and what that test type proves, so a reader sees which seam is untested rather than a single "incomplete suite".
  - The prefix vocabulary is read from `shared-naming-rules`, the same list AES103 enforces, so the two rules cannot drift apart on what counts as a category.
  - Support prefixes — `regression_`, `behavioral_`, `mock_`, `fixture_` — are legal file names but satisfy no category. A folder carrying only regression guards still owes all seven.
- **Edge Cases**: A `tests/` directory holding every category except one reports exactly one `test_suite_missing_category`. A feature folder with a complete suite is clean. A folder with no `tests/` directory at all reports the directory finding once. A utility-only folder reports nothing.
- **Error Handling**: If a test directory cannot be read, it is treated as empty — every category it owed is reported — so an unreadable directory can never read as a satisfied one.

### FR-STR-005: Member-Root File Placement (AES705)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-STR-005 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: Require `crates/`, `modules/`, and `packages/` to hold folders and wiring only; a layer file loose at a member root belongs inside the folder that owns it.
- **Input**: An audit root path and the top-level entries of every member dir.
- **Output**: `Vec<LintResult>` carrying one finding per misplaced file.
- **Business Rules**:
  - Only the top level of a member dir is read. A file inside a folder beneath it belongs to that folder, which is the shape this rule preserves.
  - `capabilities_*`, `agent_*`, `surface_*`, and `utility_*` files are forbidden at a member root. Each produces one `member_root_has_layer_file` finding naming the prefix and the folder kind it belongs in.
  - The finding names the destination by layer: capabilities and agents move into a feature folder, surfaces into a surface folder, utilities into the shared folder.
  - `root_*` and barrel-module files stay legal at a member root — they are the wiring the container composes. So does any file whose stem carries no classified layer prefix, such as `README.md` or `Cargo.toml`.
- **Edge Cases**: A member dir holding every forbidden prefix reports one finding per file. A member dir holding only `root_*` wiring and a barrel module is clean. A member dir with no files at all is clean.
- **Error Handling**: If a member dir cannot be read, it reports nothing rather than failing the audit.

## Public Interface

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `audit_shared` | `StructureRequest::AuditAll { root }` | `StructureResponse::Findings { findings }` | I/O error on path read | — | Walk the kernel folders and report shared-folder purity and documentation findings (AES701). |
| `audit_feature` | `StructureRequest::AuditAll { root }` | `StructureResponse::Findings { findings }` | I/O error on path read | — | Classify each feature folder and report folder health and documentation findings (AES702). |
| `audit_surface` | `StructureRequest::AuditAll { root }` | `StructureResponse::Findings { findings }` | I/O error on path read | — | Report surface-folder purity and documentation findings, sorted and deduplicated (AES703). |
| `audit_test_suite` | `StructureRequest::AuditAll { root }` | `StructureResponse::Findings { findings }` | I/O error on path read | — | Report the test category each feature folder still owes — one finding per absent type, plus any missing test or benchmark directory (AES704). |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `execute` | `StructureRequest` | `StructureResponse` | I/O error on path read | — | Public aggregate entry point; routes to the appropriate auditor |

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

| Roadmap | {Link to ROADMAP.md} | [{link}](ROADMAP.md) |
| Architecture | {Link to ARCHITECTURE.md} | [{link}](ARCHITECTURE.md) |
