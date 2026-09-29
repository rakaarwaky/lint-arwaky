# Structure Rules — FRD

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)

## System Overview

The structure-rules crate enforces the 7-layer AES folder discipline by auditing the layout of every workspace. It checks that shared, feature, and surface folders obey their shape contracts — purity invariants on shared and surface, health checks on features, doc-pair requirements on feature folders, and DESIGN.md presence on surface folders — and returns findings as workspace-root-relative paths so the scan is correct regardless of which member directory the caller passes.

## Functional Requirements

### FR-STR-001: Structure Violations Report (AES701–AES705)

- **Description**: `scan` and `check` report AES701–AES705 findings for the audit target; findings carry workspace-root-relative paths so a member-directory scan resolves them correctly; `check .` reports 0 structure violations for this repository.
- **Input**: An audit root path, file entries from the filesystem aggregate, and layer map configuration.
- **Output**: `Vec<LintResult>` carrying one finding per structural defect found.
- **Business Rules**:
  - AES701 fires when any non-taxonomy file lives directly under the shared folder.
  - AES702 fires when a feature folder holds a valid doc pair but no orchestrator file.
  - AES703 fires when any file outside the taxonomy layer lives directly under the surface folder.
  - AES704 fires when a feature folder carries a doc pair but no orchestrator file.
  - AES705 fires when a surface folder lacks a DESIGN.md record.
- **Edge Cases**: Shared folders with zero files produce no AES701 finding. Feature folders without a doc pair are skipped.
- **Error Handling**: Unreadable directories are skipped; missing directories produce no findings.

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `audit` | `IAuditRequest` | `AuditResponse` | `StructureError` | — | Single entry point over the structure feature. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `execute` | `StructureRequest` | `StructureResponse` | — | — | Routes the request through the capability checker and returns findings. |

## Integration Points

| System | Direction | Purpose | Failure mode |
|--------|-----------|---------|--------------|
| `dispatcher` | in | Aggregates the structure audit and feeds it into the scan pipeline | A missing dispatcher hook drops the entire structure group silently |
| `filesystem` | in | Supplies file entries and resolved paths for every source file | Parse failures in filesystem produce empty entries, not violations |
| `shared::taxonomy` | in | Supplies `LintFinding`, `LintResult`, and structure error types | A missing type forces a compile failure at the aggregate boundary |

## Non-functional Requirements

| Metric | Target | Measurement method |
|--------|--------|---------------------|
| Walk cost | The audit reads directory entries only; no file is parsed or indexed | Run a scan over a workspace and confirm no AST parse occurs for the structure group |
| Path resolution | Every finding path resolves against the workspace root on disk | Scan each member directory on its own and confirm each reported path exists |
| Idempotency | A second scan over an unchanged workspace reports the same finding set | Run the scan twice and compare the two finding lists |
| De-duplication | One structural defect produces one finding | Introduce the same misplaced file twice via two paths and confirm a single finding |

## Test Scenarios

- A shared folder holds a capability file → AES701 fires naming the file.
- A feature folder holds a doc pair but no orchestrator → AES702 fires naming the folder.
- A surface folder holds a non-taxonomy file → AES703 fires naming the file.
- A feature folder lacks its doc pair → AES704 fires naming the missing documents.
- A surface folder lacks a DESIGN.md → AES705 fires naming the folder.
- A clean repository reports 0 structure violations on `check .` → exit code 0.

## Assumptions & Constraints

- The audit runs over `crates/`, `modules/`, and `packages/` subdirectories identified from the workspace root.
- Layer detection is based on the filename prefix convention (`taxonomy_*`, `contract_*`, etc.).
- Shared and surface folders are defined by their position directly under the workspace root member, not by deeper nesting.
- Test fixtures in `workspaces-good/` and `workspaces-bad/` are excluded from the default scan scope to prevent false positives.
- Barrel files and marker files inside shared/surface folders are ignored for purity checks.

## Glossary

- **Shared folder**: The `shared/` directory under a workspace member that holds only taxonomy-layer files.
- **Feature folder**: A subdirectory of `crates/`, `modules/`, or `packages/` that contains source code plus an orchestrator.
- **Surface folder**: The outermost directory that holds surface-level files (commands, pages, views).
- **Doc pair**: The co-located `FRD.md` and `BACKLOG.md` pair required for every feature folder.
- **Orchestrator**: An agent file named `agent_*_orchestrator` that composes the feature's capabilities.
- **DESIGN.md**: A design document required in every surface folder.
