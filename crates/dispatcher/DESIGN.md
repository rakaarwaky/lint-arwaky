# Dispatcher — DESIGN

> Routing contract between the rule groups and every surface. Renderers read
> the outcome vocabulary below to decide how to present a run.
> Condition: [ROADMAP.md](../../ROADMAP.md) (workspace) +
> [BACKLOG.md](BACKLOG.md).

## Brand & Style

This surface draws nothing, so it has no palette. What it owns instead is the
**outcome vocabulary** every renderer must honour: a collector never decides
that a run is acceptable, it only reports what the groups found and lets the
surface map that onto the process exit-code contract.

### Outcome Vocabulary

| Outcome      | Condition                                        | Who grades it   |
| ------------ | ------------------------------------------------ | --------------- |
| `clean`      | Every group returned zero findings                | the surface     |
| `violations` | One or more groups returned findings              | the surface     |
| `error`      | A group failed to run                             | the surface     |

The three are not interchangeable. A group that fails is never silently treated
as clean — a scan that could not complete reports the failure, because a clean
result the user did not earn is worse than a visible error.

### Kind

`api` — the single boundary where every rule group meets the surfaces. One
`surface_<group>_action.rs` file per group, each collecting that group's
findings from the shared `ScanAggregates` and narrowing them to the requested
target. The CLI, the MCP server, and the TUI all reach rule behavior through
here and nowhere else.

### Request Shape

A collector receives the requested target as a string and the shared aggregates
as an immutable borrow. It never reads the filesystem to decide scope; the scan
pipeline already produced the aggregates. Where a collector must resolve a path
— the structure group does, because its findings are workspace-root-relative —
it resolves against the workspace root and filters for containment under the
target.

#### Path scoping: `scan`/`check` vs. `docs`

Not every path-taking subcommand scopes the same way, and the difference is
deliberate:

| Collector | What the target means |
| --- | --- |
| `surface_check_action.rs`, and every `surface_<group>_action.rs` | A **filter**. The pipeline indexes the workspace and the collector keeps only the findings contained under the target. |
| `surface_docs_action.rs` | A **root**. The doc request carries the target as its audit root, so pointing `docs` at a sub-directory audits that sub-tree as if it were its own workspace. Crosslink checks (AES604) that expect root-level `PRD.md`/`ROADMAP.md` siblings therefore behave differently than they do for a whole-workspace run. |

The doc-rules contract owns this semantic; see `crates/doc-rules/FRD.md`
("Path scoping" under System Overview) and the `docs` row in
`crates/cli-commands/DESIGN.md`.

#### Layer scoping: the six layer subcommands

`surface_layer_scan_action.rs` is a second filter, applied after the scan rather
than instead of it. A layer subcommand still runs every rule group over the
whole workspace — an import cycle or an orphan is a property of the workspace,
not of one layer — and then keeps only the violations whose file name carries
that layer's AES prefix. Narrowing the analysis instead of the output would make
each layer command blind to findings that only become visible across layers.

The layer of a file is its name prefix (`taxonomy_`, `contract_`,
`capabilities_`, `utility_`, `agent_`, `surface_`, `root_`), the same signal
`detect_layer_from_prefix` gives every other rule. Two consequences follow, and
both are deliberate: `root` has no subcommand, because root files are wiring and
`check` already reports them; and a finding that names no layer-prefixed file —
a folder-level structure finding, a Markdown doc invariant — belongs to no
single layer, so it is dropped here and stays with `structure` and `docs`.

#### Exit-code aggregation for `fix`

`surface_fix_action.rs` owns the aggregation of per-item `FixOutcome`s into one
run-level outcome: any `Failed(reason)` raises the report's failure flag, while
a run of only `Applied`/`Skipped(reason)` items does not. `cli-commands`'
`surface_fix_command.rs` renders that outcome and maps it onto the exit-code
contract (a `Failed` present → `RuntimeError` (2); otherwise `Ok` (0), or
`PolicyFail` (1) when violations remain after a non-dry run). The surface never
re-derives the aggregation — per the CLI's own invariant, "a surface file
computes nothing about the codebase."

### Invariants

- A collector filters, it does not re-derive. Rule decisions belong to the rule
  crates; this layer only scopes and shapes their output.
- Collectors are pure with respect to the filesystem — they read `ScanAggregates`,
  which the pipeline built.
- Adding a rule group means adding one collector here and registering it in
  `ScanAggregates`; no surface needs to change.
- The rule-code range in the Entry Points table is a copy of the range
  `RULES_AES.md` publishes for that group. The two must agree; a rule
  renumbering or consolidation edits both in the same change.

### Change Checklist

A new group: create `surface_<group>_action.rs`, add its findings to the
aggregates in the scan pipeline, and register the group in the config. The CLI,
MCP, and TUI surfaces pick it up with no edit.

Renumbering, adding, or consolidating a rule code: update `RULES_AES.md` first,
then the matching row here. `tools/check_doc_consistency.py` (run in CI) fails
when a range in this table names a code `RULES_AES.md` no longer publishes.

## Components

| Component            | File                     | Serves    | States                  |
| -------------------- | ------------------------ | --------- | ----------------------- |
| Naming collector     | `surface_naming_action.rs`  | AES101–102 | collected / scoped  |
| Import collector     | `surface_import_action.rs`  | AES201–205 | collected / scoped  |
| Quality collector    | `surface_quality_action.rs` | AES301–305 | collected / scoped  |
| Role collector       | `surface_role_action.rs`   | AES401–406 | collected / scoped  |
| Orphan collector     | `surface_orphan_action.rs` | AES501–506 | collected / scoped  |
| Structure collector  | `surface_structure_action.rs` | AES701–704 | collected / scoped  |
| Check entry          | `surface_check_action.rs`  | `check`   | exit-code shaped over all groups |
| Layer entry          | `surface_layer_scan_action.rs` | `taxonomy`, `contract`, `capabilities`, `utility`, `agents`, `surface` | every group, findings narrowed to one layer |
| CI entry             | `surface_ci_action.rs`     | `ci`      | the same groups with thresholds   |
| Fix entry            | `surface_fix_action.rs`    | `fix`     | aggregates every `FixOutcome` into the run-level outcome |
| Docs entry           | `surface_docs_action.rs`   | `docs`    | audit root, not a filter          |
| Watch entry          | `surface_watch_action.rs`  | `watch`   | repeated scans over changed files |
| Config entry         | `surface_config_action.rs` | `config`  | effective configuration           |
| Git entry            | `surface_git_action.rs`    | `git`     | hook installation and checks      |
| Supporting surfaces  | `surface_version_action.rs`, `surface_plugin_action.rs`, `surface_setup_action.rs`, `surface_maintenance_action.rs`, `surface_external_action.rs` | `version`, `plugin`, `setup`, `doctor`/`security`/`deps`, `external` | pass-through |

Every collector has the same shape: `collect_<group>(target, aggregates) -> Vec<Finding>`.

### States

| State        | Condition                                                    |
| ------------ | ------------------------------------------------------------ |
| `clean`      | Every group returned zero findings                            |
| `violations` | One or more groups returned findings                          |
| `error`      | A group failed to run; the dispatcher reports it rather than reporting a clean scan |

### Error States

| Condition                        | Behaviour                                                                                   |
| -------------------------------- | ------------------------------------------------------------------------------------------- |
| Target path missing              | The caller returns `RuntimeError` before dispatching                                        |
| A group returns an internal error| The failure is surfaced; other groups still contribute findings                             |
| A finding's path cannot be resolved | The finding is dropped from a scoped scan rather than reported against a path that does not exist |

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- Rules: [RULES_AES.md](../../RULES_AES.md)
