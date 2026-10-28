# Dispatcher — DESIGN

> Routing contract between the rule groups and every surface. Renderers read
> the outcome vocabulary below to decide how to present a run.
> Condition: [ROADMAP.md](../../ROADMAP.md) (workspace) +
> [BACKLOG.md](BACKLOG.md).

## Design Overview

This surface draws nothing, so it has no palette. What it owns instead is the
**outcome vocabulary** every renderer must honour: a collector never decides
that a run is acceptable, it only reports what the groups found and lets the
surface map that onto the process exit-code contract.

## Design Requirements

| Component            | File                     | Serves    | States                 |
| -------------------- | ------------------------ | --------- | -----------------------|
| Naming collector     | `surface_naming_action.rs`  | AES101–102 | collected / scoped |
| Import collector     | `surface_import_action.rs`  | AES201–205 | collected / scoped |
| Quality collector    | `surface_quality_action.rs` | AES301–305 | collected / scoped |
| Role collector       | `surface_role_action.rs`   | AES401–406 | collected / scoped |
| Orphan collector     | `surface_orphan_action.rs` | AES501–506 | collected / scoped |
| Structure collector  | `surface_structure_action.rs` | AES701–704 | collected / scoped |
| Check entry          | `surface_check_action.rs`  | `check`   | exit-code shaped over all groups|
| Layer entry          | `surface_layer_scan_action.rs` | `taxonomy`, `contract`, `capabilities`, `utility`, `agents`, `surface` | every group, findings narrowed to one layer|
| CI entry             | `surface_ci_action.rs`     | `ci`      | the same groups with thresholds  |
| Fix entry            | `surface_fix_action.rs`    | `fix`     | aggregates every `FixOutcome` into the run-level outcome|
| Docs entry           | `surface_docs_action.rs`   | `docs`    | audit root, not a filter         |
| Watch entry          | `surface_watch_action.rs`  | `watch`   | repeated scans over changed files|
| Config entry         | `surface_config_action.rs` | `config`  | effective configuration          |
| Git entry            | `surface_git_action.rs`    | `git`     | hook installation and checks     |
| Supporting surfaces  | `surface_version_action.rs`, `surface_plugin_action.rs`, `surface_setup_action.rs`, `surface_maintenance_action.rs`, `surface_external_action.rs` | `version`, `plugin`, `setup`, `doctor`/`security`/`deps`, `external` | pass-through|

Every collector has the same shape: `collect_<group>(target, aggregates) -> Vec<Finding>`.

### DS-DISP-001: Outcome Vocabulary

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-002: Kind

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-003: Request Shape

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-004: Invariants

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-005: Change Checklist

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-006: States

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-007: Error States

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
