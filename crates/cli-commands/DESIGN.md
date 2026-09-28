# CLI Commands — DESIGN

## Kind

`cli` — the process surface a user types into. Each `surface_*_command.rs` file
parses one subcommand's arguments, calls the aggregate behind it, and turns the
result into output plus an exit code. No rule logic lives here.

## Entry Points

| File | Subcommand | Aggregates reached |
| --- | --- | --- |
| `surface_scan_command.rs` | `scan <path>` | naming, import, quality, role, orphan, structure |
| `surface_check_action.rs` (dispatcher) | `check <path>` | the same group as `scan`, exit-code shaped |
| `surface_ci_command.rs` | `ci` | every group, with thresholds |
| `surface_fix_command.rs` | `fix <path>` | auto-fix aggregate |
| `surface_config_command.rs` | `config` | config-system aggregate |
| `surface_git_command.rs` | `git` | git-hooks aggregate |
| `surface_maintenance_command.rs` | `doctor`, `security`, `deps`, `self-update` | maintenance aggregate |
| `surface_setup_command.rs` | `init`, `install`, `mcp-config` | project-setup aggregate |
| `surface_watch_command.rs` | `watch` | file-watch aggregate |
| `surface_plugin_command.rs` | `plugin` | plugin registry |
| `surface_skill_command.rs` | `skill` | skill registry |

`utility_output_text_formatter.rs` renders findings for a terminal; it holds no
rule logic and reads nothing from the aggregates beyond their results.

## Request Shape

A subcommand receives its arguments as parsed values, never as raw `argv`. The
surface resolves a path to an absolute path before any aggregate sees it, so a
group never has to know the process's working directory.

## States

| State | Condition | Output |
| --- | --- | --- |
| `clean` | The scan returned zero violations | The summary line plus `Ok` |
| `violations` | One or more groups returned findings | The rendered findings plus `PolicyFail` |
| `prerequisite_missing` | A required external tool is absent | The tool name and an install hint plus `PrerequisiteMissing` |
| `error` | The path is missing, arguments are invalid, or I/O failed | The cause plus `RuntimeError` |

## Error States

| Condition | Surface | Exit code |
| --- | --- | --- |
| Path does not exist | every subcommand taking a path | `RuntimeError` (2) |
| External linter not on `PATH` | `scan` with adapters enabled, `doctor` | `PrerequisiteMissing` (3) |
| Config unreadable or malformed | `config` | `RuntimeError` (2) |
| Violations above the configured threshold | `ci` | `PolicyFail` (1) |

Surfaces report; they never raise. Every failure path returns an `ExitCode`, so
the process ends through one place.

## Invariants

- A surface file computes nothing about the codebase. It renders what an
  aggregate returns and maps the outcome to an exit code.
- A surface may not import a capabilities file directly. It reaches behavior
  through the aggregate seam (`crates/dispatcher`).
- A surface holds no state across invocations; each subcommand is a fresh call.

## Change Checklist

Changing a subcommand's arguments touches the surface file and its contract. Adding
a new rule group touches `crates/dispatcher/src/surface_<group>_action.rs` so the
existing subcommands pick it up. Adding a new subcommand touches one surface file,
the CLI wiring, and this table.
