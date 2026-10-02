# FRD — Structure Rules (AES701–AES704)

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

## System Overview

The structure-rules crate enforces the 7-layer AES folder discipline by auditing the layout of every workspace. It checks that shared, feature, and surface folders obey their shape contracts — shared folders carry DATA.md + BACKLOG.md, feature folders carry FRD.md + BACKLOG.md, and surface folders carry DESIGN.md + BACKLOG.md — and that every feature folder carries a complete test suite. It returns findings as workspace-root-relative paths so a member-directory scan resolves them correctly.

## Functional Requirements

### FR-STR-001: Shared Folder Purity and Documentation (AES701)

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

- **Description**: Audit feature folders for completeness (capabilities + orchestrator), forbid foreign layer files, and require the doc pair.
- **Input**: An audit root path and the filesystem inventory of each candidate feature folder.
- **Output**: `Vec<LintResult>` carrying one finding per structural defect found in a feature folder.
- **Business Rules**:
  - A feature folder must hold at least one `capabilities_*` file AND at least one `agent_*_orchestrator` file. Missing capabilities produces `feature_missing_capability`; missing orchestrator produces `feature_missing_agent`.
  - A feature folder must not hold `utility_*`, `surface_*`, `taxonomy_*`, or `contract_*` files — those belong elsewhere. Each foreign file produces one `feature_has_forbidden_files` finding.
  - A feature folder must carry `FRD.md` and `BACKLOG.md` at its root. Missing either produces one `feature_missing_doc_pair` finding listing the missing files.
  - Reverse direction: any folder (including non-feature) that carries both `FRD.md` and `BACKLOG.md` must also hold at least one `*_orchestrator` file. Absence produces one `doc_pair_without_orchestrator` finding.
  - A folder with no capabilities and no orchestrator is not a feature and owes no document pair.
- **Edge Cases**: A folder with capabilities but no orchestrator and no docs reports both `feature_missing_agent` and `feature_missing_doc_pair`. A folder with only an orchestrator and no capabilities reports `feature_missing_capability`. A utility-only folder is skipped entirely.
- **Error Handling**: If the folder path cannot be read, the auditor skips it silently.

### FR-STR-003: Surface Folder Purity and Documentation (AES703)

- **Description**: Audit surface-dominated folders for misplaced files and require DESIGN.md + BACKLOG.md.
- **Input**: An audit root path and the filesystem inventory of each surface-dominated folder.
- **Output**: `Vec<LintResult>` carrying one finding per structural defect found in a surface folder.
- **Business Rules**:
  - A surface-dominated folder (where `surface_*` files outnumber all other classified files combined) must not hold `capabilities_*` or `agent_*` files. Each misplaced file produces one `surface_has_misplaced_files` finding.
  - Utility files, root wiring files, and barrel files are permitted alongside surface files.
  - A surface-dominated folder must carry `DESIGN.md` and `BACKLOG.md` at its root. Missing DESIGN.md produces `surface_missing_design_md`; missing BACKLOG.md produces `surface_missing_backlog_md`.
  - A folder that is not surface-dominated is not checked for AES703.
- **Edge Cases**: A feature-dominated folder that happens to contain one surface file is NOT surface-dominated and is checked under AES702 instead. A surface folder with only permitted support files, DESIGN.md, and BACKLOG.md is clean.
- **Error Handling**: If the folder path cannot be read, the auditor skips it silently.

### FR-STR-004: Test Suite Category Coverage (AES704)

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

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `audit_shared` | `StructureRequest::AuditAll { root }` | `StructureResponse::Findings { findings }` | I/O error on path read | — | AES701 seam: shared folder purity and its doc pair |
| `audit_feature` | `StructureRequest::AuditAll { root }` | `StructureResponse::Findings { findings }` | I/O error on path read | — | AES702 seam: feature folder health and its doc pair |
| `audit_surface` | `StructureRequest::AuditAll { root }` | `StructureResponse::Findings { findings }` | I/O error on path read | — | AES703 seam: surface folder purity and its design docs |
| `audit_test_suite` | `StructureRequest::AuditAll { root }` | `StructureResponse::Findings { findings }` | I/O error on path read | — | AES704 seam: per-category test-suite coverage |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `execute` | `StructureRequest` | `StructureResponse` | I/O error on path read | — | Public aggregate entry point; routes to the appropriate auditor |

## Integration Points

| System | Direction | Purpose | Failure mode |
|--------|-----------|---------|--------------|
| Filesystem crate | Inbound | Provides file inventory (paths, stems, extensions) for each folder | Missing files or unreadable directories → folder skipped |
| Shared VOs | Inbound | `FolderInventory`, `LayerFile`, `StructureFinding`, `StructureRequest`, `StructureResponse` | Type mismatch → compile error |
| CLI `check` / `scan` commands | Outbound | Receives findings and renders them to the user | Formatting error → finding dropped |

## Non-functional Requirements

| Metric | Target | Measurement method |
|--------|--------|-------------------|
| Audit latency per folder | < 10 ms | Benchmark on `workspaces-good` and `workspaces-bad` |
| Finding determinism | Identical output across runs on same tree | Run audit twice, compare sorted findings |
| Path resolution | Workspace-root-relative paths in all findings | Inspect `finding.file` field |

## Test Scenarios

- A shared folder holds a `capabilities_*` file → AES701 fires `shared_has_forbidden_files`.
- A shared folder holds both `capabilities_*` and `agent_*` files → AES701 fires once per forbidden file.
- A shared folder holds only `taxonomy_*`, `utility_*`, `contract_*` → AES701 silent on purity.
- A shared folder lacks `DATA.md` or `BACKLOG.md` → AES701 fires `shared_missing_doc_pair`.
- A shared folder carries `DATA.md` and `BACKLOG.md` → AES701 silent.
- A feature folder holds capabilities but no orchestrator → AES702 fires `feature_missing_agent`.
- A feature folder holds an orchestrator but no capabilities → AES702 fires `feature_missing_capability`.
- A feature folder holds both capabilities and orchestrator but no docs → AES702 fires `feature_missing_doc_pair`.
- A feature folder holds capabilities, orchestrator, and both docs → AES702 silent.
- A feature folder holds a `taxonomy_*` file alongside capabilities → AES702 fires `feature_has_forbidden_files`.
- A feature folder holds a `utility_*` file alongside capabilities → AES702 fires `feature_has_forbidden_files`.
- A folder carries `FRD.md` and `BACKLOG.md` but no orchestrator → AES702 fires `doc_pair_without_orchestrator`.
- A folder carries `FRD.md` and `BACKLOG.md` and an orchestrator → AES702 silent on reverse check.
- A utility-only folder (no capabilities, no orchestrator) → AES702 silent.
- A surface-dominated folder holds a `capabilities_*` file → AES703 fires `surface_has_misplaced_files`.
- A surface-dominated folder holds an `agent_*` file → AES703 fires `surface_has_misplaced_files`.
- A surface-dominated folder lacks `DESIGN.md` → AES703 fires `surface_missing_design_md`.
- A surface-dominated folder lacks `BACKLOG.md` → AES703 fires `surface_missing_backlog_md`.
- A surface-dominated folder holds utilities, barrels, `DESIGN.md`, and `BACKLOG.md` → AES703 silent.
- A feature-dominated folder that also holds one surface file is checked under AES702, not AES703.
- A conforming workspace (shared has DATA+BACKLOG, features have FRD+BACKLOG, surfaces have DESIGN+BACKLOG) → 0 structure violations on `lint-arwaky-cli check .`.

## Assumptions & Constraints

- The audit walks exactly three member directories: `crates/`, `modules/`, `packages/`.
- Directory depth is capped at 3 levels to avoid scanning test/bench/build artifacts.
- Skipped directories (`benches`, `tests`, `target`, `node_modules`, `__pycache__`) are never audited.
- A folder is classified as "surface-dominated" when `surface_count * 2 >= total_count`.
- The reverse doc-pair check applies to all non-shared folders, not only feature-dominated ones.
- Findings are deduplicated by `(code, violation_type, file, message)` and sorted stably.
- Shared folders require `DATA.md` (spec) + `BACKLOG.md` (tracking) — not `FRD.md`.

## Glossary

- **Shared / kernel**: The cross-cutting folder (`shared/`) holding taxonomy, utility, and contract files; carries `DATA.md` + `BACKLOG.md`.
- **Feature folder**: A subdirectory of a member dir that holds at least one `capabilities_*` or `agent_*_orchestrator` file; carries `FRD.md` + `BACKLOG.md`.
- **Surface-dominated folder**: A folder where `surface_*` files constitute more than half of all classified files; carries `DESIGN.md` + `BACKLOG.md`.
- **Doc pair**: The two-doc file pair required by each folder kind (FRD+BACKLOG for features, DESIGN+BACKLOG for surfaces, DATA+BACKLOG for shared).
- **Foreign file**: A file whose layer prefix does not belong in the current folder's allowed set.
