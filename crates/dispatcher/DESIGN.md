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
| `orchestrator_naming_pipeline.rs` | AES101–102 |
| `orchestrator_import_pipeline.rs` | AES201–205 |
| `orchestrator_quality_pipeline.rs` | AES301–305 |
| `orchestrator_role_pipeline.rs` | AES401–406 |
| `orchestrator_orphan_pipeline.rs` | AES501–506 |
| `orchestrator_structure_pipeline.rs` | AES701–703 |
| `orchestrator_check_pipeline.rs` | `check` — the exit-code shaped entry over all groups |
| `orchestrator_ci_pipeline.rs` | `ci` — the same groups with thresholds |
| `orchestrator_fix_pipeline.rs` | `fix` — auto-fix over collected findings; also aggregates every collected `FixOutcome` into the run-level outcome the surface maps to an exit code |
| `orchestrator_docs_pipeline.rs` | `docs` — document invariants only; the target is the audit root, not a filter |
| `orchestrator_watch_pipeline.rs` | `watch` — repeated scans over changed files |
| `orchestrator_config_pipeline.rs` | `config` — effective configuration |
| `orchestrator_git_pipeline.rs` | `git` — hook installation and checks |
| `orchestrator_version_pipeline.rs`, `orchestrator_plugin_pipeline.rs`, `orchestrator_setup_pipeline.rs`, `orchestrator_maintenance_pipeline.rs`, `orchestrator_external_pipeline.rs` | Supporting surfaces |

Every collector has the same shape: `collect_<group>(target, aggregates) -> Vec<Finding>`.

## Request Shape

A collector receives the requested target as a string and the shared aggregates
as an immutable borrow. It never reads the filesystem to decide scope; the scan
pipeline already produced the aggregates. Where a collector must resolve a path
— the structure group does, because its findings are workspace-root-relative —
it resolves against the workspace root and filters for containment under the
target.

### Path scoping: `scan`/`check` vs. `docs`

Not every path-taking subcommand scopes the same way, and the difference is
deliberate:

| Collector | What the target means |
| --- | --- |
| `orchestrator_check_pipeline.rs`, and every `surface_<group>_action.rs` | A **filter**. The pipeline indexes the workspace and the collector keeps only the findings contained under the target. |
| `orchestrator_docs_pipeline.rs` | A **root**. The doc request carries the target as its audit root, so pointing `docs` at a sub-directory audits that sub-tree as if it were its own workspace. Crosslink checks (AES604) that expect root-level `PRD.md`/`ROADMAP.md` siblings therefore behave differently than they do for a whole-workspace run. |

The doc-rules contract owns this semantic; see `crates/doc-rules/FRD.md`
("Path scoping" under System Overview) and the `docs` row in
`crates/cli-commands/DESIGN.md`.

### Exit-code aggregation for `fix`

`orchestrator_fix_pipeline.rs` owns the aggregation of per-item `FixOutcome`s into one
run-level outcome: any `Failed(reason)` raises the report's failure flag, while
a run of only `Applied`/`Skipped(reason)` items does not. `cli-commands`'
`surface_fix_command.rs` renders that outcome and maps it onto the exit-code
contract (a `Failed` present → `RuntimeError` (2); otherwise `Ok` (0), or
`PolicyFail` (1) when violations remain after a non-dry run). The surface never
re-derives the aggregation — per the CLI's own invariant, "a surface file
computes nothing about the codebase."

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
- The rule-code range in the Entry Points table is a copy of the range
  `RULES_AES.md` publishes for that group. The two must agree; a rule
  renumbering or consolidation edits both in the same change.

## Change Checklist

A new group: create `surface_<group>_action.rs`, add its findings to the
aggregates in the scan pipeline, and register the group in the config. The CLI,
MCP, and TUI surfaces pick it up with no edit.

Renumbering, adding, or consolidating a rule code: update `RULES_AES.md` first,
then the matching row here. `tools/check_doc_consistency.py` (run in CI) fails
when a range in this table names a code `RULES_AES.md` no longer publishes.
