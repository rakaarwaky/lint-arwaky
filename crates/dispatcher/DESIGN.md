# Dispatcher — DESIGN

## Kind

`api` — the single boundary where every rule group meets the surfaces. One
`surface_<group>_action.rs` file per group, each collecting that group's
findings from the shared `ScanAggregates` and narrowing them to the requested
target. The CLI, the MCP server, and the TUI all reach rule behavior through
here and nowhere else.

## Entry Points

| File | Serves |
| --- | --- |
| `surface_naming_action.rs` | AES101–102 |
| `surface_import_action.rs` | AES201–205 |
| `surface_quality_action.rs` | AES301–305 |
| `surface_role_action.rs` | AES401–406 |
| `surface_orphan_action.rs` | AES501–506 |
| `surface_structure_action.rs` | AES701–705 |
| `surface_check_action.rs` | `check` — the exit-code shaped entry over all groups |
| `surface_ci_action.rs` | `ci` — the same groups with thresholds |
| `surface_fix_action.rs` | `fix` — auto-fix over collected findings |
| `surface_docs_action.rs` | `docs` — document invariants only |
| `surface_watch_action.rs` | `watch` — repeated scans over changed files |
| `surface_config_action.rs` | `config` — effective configuration |
| `surface_git_action.rs` | `git` — hook installation and checks |
| `surface_version_action.rs`, `surface_plugin_action.rs`, `surface_setup_action.rs`, `surface_maintenance_action.rs`, `surface_external_action.rs` | Supporting surfaces |

Every collector has the same shape: `collect_<group>(target, aggregates) -> Vec<Finding>`.

## Request Shape

A collector receives the requested target as a string and the shared aggregates
as an immutable borrow. It never reads the filesystem to decide scope; the scan
pipeline already produced the aggregates. Where a collector must resolve a path
— the structure group does, because its findings are workspace-root-relative —
it resolves against the workspace root and filters for containment under the
target.

## States

| State | Condition |
| --- | --- |
| `clean` | Every group returned zero findings |
| `violations` | One or more groups returned findings |
| `error` | A group failed to run; the dispatcher reports it rather than reporting a clean scan |

A group that fails is never silently treated as clean. A scan that could not
complete reports the failure.

## Error States

| Condition | Behaviour |
| --- | --- |
| Target path missing | The caller returns `RuntimeError` before dispatching |
| A group returns an internal error | The failure is surfaced; other groups still contribute findings |
| A finding's path cannot be resolved | The finding is dropped from a scoped scan rather than reported against a path that does not exist |

## Invariants

- A collector filters, it does not re-derive. Rule decisions belong to the rule
  crates; this layer only scopes and shapes their output.
- Collectors are pure with respect to the filesystem — they read `ScanAggregates`,
  which the pipeline built.
- Adding a rule group means adding one collector here and registering it in
  `ScanAggregates`; no surface needs to change.

## Change Checklist

A new group: create `surface_<group>_action.rs`, add its findings to the
aggregates in the scan pipeline, and register the group in the config. The CLI,
MCP, and TUI surfaces pick it up with no edit.
