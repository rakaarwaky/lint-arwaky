# FRD — dispatcher

## Feature Overview

The dispatcher crate is the shared source of truth for workspace action
business logic: the CLI, MCP server, and TUI surfaces call its action modules
and format the returned results themselves. Each surface action module owns one
workspace verb (check, scan, ci, fix, config, git, setup, watch, and the
per-rule-group scans), and findings flow to the caller as violation items
through shared entry records and output components.

## Functional Requirements

### FR-Dispatcher-001: Workspace Action Dispatch

| Aspect | Detail|
|--------|-------|
| Description | Expose every workspace verb as a surface action the CLI, MCP, and TUI surfaces call, so all three share one behavior. |
| Business Rules | One action module per verb; an action takes a workspace plus its request and returns a typed response; actions depend on feature aggregates and never on surface-side output formatting; scope and discovery arguments narrow the action's target. |
| Edge Cases | An action on an unreadable workspace fails with its own error variant instead of panicking; repeated invocation with the same workspace and request is deterministic; a panicking action is isolated rather than taking down the process. |
| Capability | Surface action modules for check, scan, ci, fix, config, git, setup, watch, docs, import, external, maintenance, naming, orphan, plugin, quality, role, structure, layer scan, version, and suppression. |
| Protocol | —; this crate has no capability seam in the shared kernel, it dispatches plain action calls over feature aggregates. |
| Method | one execute-style action per verb (check, scan, ci, fix, config, git, setup, watch, and the rule-group scans). |
| Input | workspace path plus the per-verb request (scope, options); source is the calling surface command. |
| Output | typed per-verb response carrying violation items, counts, or action status for the caller to format. |
| Error | per-verb error variants surfaced to the caller; panic isolation reports failure as an error, not a process abort. |
| Event | None; actions are synchronous calls. |
| Test | acceptance_check_action_on_current_project, check_action_module_exists, and the panic-isolation regression suite assert dispatch wiring and isolation. |
| Metric | Every seeded workspace verb maps to exactly one action module; each seeded failure surfaces as one error result. |

### FR-Dispatcher-002: Scan Modes, External Filtering, and Cancellation

| Aspect | Detail|
|--------|-------|
| Description | Run scans in every supported mode — single file, subfolder, and whole workspace — while filtering external tool output and honoring cancellation. |
| Business Rules | Each scan mode walks its target and reports violations per language; external adapter output keeps member files and drops files outside the workspace root; a cancel request stops the scan before its completion milestone; results from all enabled rule groups are merged into one report. |
| Edge Cases | A single-file scan outside a workspace root behaves like a plain file check; external output naming no member file is dropped whole; a cancel raised before the scan starts prevents the run; a cancel mid-scan stops short with no completion milestone. |
| Capability | Layer scan and external scan actions with cancellation flags. |
| Protocol | —; the modes are plain action calls over the feature aggregates. |
| Method | scan variants per target shape plus the cancel flag checked at scan milestones. |
| Input | target path or workspace plus scan options; cancel is a caller-held flag. |
| Output | merged violation items for the scanned scope; completion status distinguishes finished from canceled runs. |
| Error | target errors are reported per entry; a canceled run reports cancellation, not failure. |
| Event | per-rule-group findings stream into the merged result as each group completes. |
| Test | regression_good_rust_workspace, regression_external_filter_keeps_member_files_drops_root_files, and cancel_before_scan_stops_short_with_no_completion_milestone assert modes, filtering, and cancellation. |
| Metric | Every seeded language fixture yields findings only for files inside its scope; each canceled scan ends before the completion milestone. |

### FR-Dispatcher-003: Violation Records and Output Components

| Aspect | Detail|
|--------|-------|
| Description | Turn raw lint results into stable violation items and format them through shared output components. |
| Business Rules | A violation item preserves code, file, line, column, severity, and message from its lint result; JSON construction fills defaults on missing optional fields; per-run entries are stored so output components can render results consistently for every surface. |
| Edge Cases | A violation item built from JSON with a missing field falls back to its default; severity ordering is total across levels; the entries store keeps one record per scanned target. |
| Capability | Entries store and output component over the violation-item type. |
| Protocol | —; the record shaping lives in plain functions beside the actions. |
| Method | violation-item construction from lint results and JSON, plus component formatting of a list of items. |
| Input | lint results or JSON objects for construction; stored entries for rendering. |
| Output | violation items with stable fields; formatted text for the caller. |
| Error | JSON construction returns None on a structurally invalid object instead of panicking. |
| Event | None. |
| Test | violation_item_from_lint_result_preserves_fields, acceptance_output_component_formats_results, and output_component_module_exists assert record stability and formatting. |
| Metric | Every seeded lint result round-trips to a violation item with its fields preserved. |

## Public Interface

| Aspect | Detail|
|--------|-------|
| Agent | dispatcher surface actions — the entry modules the CLI, MCP, and TUI call for each workspace verb. |
| Aggregate | feature aggregates reached by the actions (structure, doc, import, quality, naming, orphan, role, and the external adapters). |
| Method | one execute-style action per verb, plus violation-item and output-component helpers. |
| Request | per-verb request structs carrying workspace, scope, and options. |
| Response | per-verb response structs carrying violation items, counts, and status. |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
