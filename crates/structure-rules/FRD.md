# FRD — Structure Rules (AES701–AES703)

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

## System Overview

The structure-rules crate enforces the 7-layer AES folder discipline by auditing the layout of every workspace. It checks that shared, feature, and surface folders obey their shape contracts — purity invariants on shared and surface, health checks on features, doc-pair requirements on feature folders, and DESIGN.md presence on surface folders — and returns findings as workspace-root-relative paths so a member-directory scan resolves them correctly.

## Functional Requirements

### FR-STR-001: Shared Folder Purity Check (AES701)

- **Description**: Audit shared/kernel folders for forbidden file types and reject doc pairs.
- **Input**: An audit root path and the filesystem inventory of each shared folder.
- **Output**: `Vec<LintResult>` carrying one finding per structural defect found in shared.
- **Business Rules**:
  - `capabilities_*`, `agent_*`, and `surface_*` files are forbidden in shared — they belong in feature folders. Each forbidden file produces one `shared_has_forbidden_files` finding.
  - A shared (kernel) folder must not carry a doc pair (`FRD.md` + `BACKLOG.md`). A doc pair in shared produces one `shared_has_docs` finding.
  - Taxonomy, utility, and contract files are the only allowed layer prefixes in shared.
- **Edge Cases**: A shared folder with mixed permitted and forbidden files reports one finding per forbidden file. A shared folder with only taxonomy/utility/contract files and no docs is clean.
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

- **Description**: Audit surface-dominated folders for misplaced files and require DESIGN.md.
- **Input**: An audit root path and the filesystem inventory of each surface-dominated folder.
- **Output**: `Vec<LintResult>` carrying one finding per structural defect found in a surface folder.
- **Business Rules**:
  - A surface-dominated folder (where `surface_*` files outnumber all other classified files combined) must not hold `capabilities_*` or `agent_*` files. Each misplaced file produces one `surface_has_misplaced_files` finding.
  - Utility files, root wiring files, and barrel files are permitted alongside surface files.
  - A surface-dominated folder must carry `DESIGN.md` at its root. Missing produces one `surface_missing_design_md` finding.
  - A folder that is not surface-dominated is not checked for AES703.
- **Edge Cases**: A feature-dominated folder that happens to contain one surface file is NOT surface-dominated and is checked under AES702 instead. A surface folder with only permitted support files and DESIGN.md is clean.
- **Error Handling**: If the folder path cannot be read, the auditor skips it silently.

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `audit` | `StructureRequest::AuditAll { root }` | `StructureResponse::Findings { findings }` | I/O error on path read | — | Walk all member directories, classify each folder, run all applicable checks, return sorted deduplicated findings |

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
- A shared folder holds only `taxonomy_*`, `utility_*`, `contract_*` → AES701 silent.
- A shared folder carries `FRD.md` and `BACKLOG.md` → AES701 fires `shared_has_docs`.
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
- A surface-dominated folder holds utilities, barrels, and `DESIGN.md` → AES703 silent.
- A surface-dominated folder lacks `DESIGN.md` → AES703 fires `surface_missing_design_md`.
- A feature-dominated folder that also holds one surface file is checked under AES702, not AES703.
- A conforming workspace (shared clean, features complete, surfaces documented) → 0 structure violations on `lint-arwaky-cli check .`.

## Assumptions & Constraints

- The audit walks exactly three member directories: `crates/`, `modules/`, `packages/`.
- Directory depth is capped at 3 levels to avoid scanning test/bench/build artifacts.
- Skipped directories (`benches`, `tests`, `target`, `node_modules`, `__pycache__`) are never audited.
- A folder is classified as "surface-dominated" when `surface_count * 2 >= total_count`.
- The reverse doc-pair check applies to all non-shared folders, not only feature-dominated ones.
- Findings are deduplicated by `(code, violation_type, file, message)` and sorted stably.

## Glossary

- **Shared / kernel**: The cross-cutting folder (`shared/`) holding taxonomy, utility, and contract files only; never a feature folder.
- **Feature folder**: A subdirectory of a member dir that holds at least one `capabilities_*` or `agent_*_orchestrator` file.
- **Surface-dominated folder**: A folder where `surface_*` files constitute more than half of all classified files.
- **Doc pair**: The `FRD.md` + `BACKLOG.md` file pair that marks a feature folder's documentation.
- **Foreign file**: A file whose layer prefix does not belong in the current folder's allowed set.
