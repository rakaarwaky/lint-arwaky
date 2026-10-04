# CLI Commands — DESIGN

> Output contract for the process surface. Renderers and CI consume the
> outcome vocabulary below to decide how to present a run and whether it passed.
> Condition: [ROADMAP.md](../../ROADMAP.md) (workspace) +
> [BACKLOG.md](BACKLOG.md).

## Brand & Style

This surface writes text, not pixels, so it has no palette. What it owns is the
**outcome vocabulary** a caller branches on — the exit code, and nothing else.
Findings are rendered by `utility_output_text_formatter.rs`; the exit code is
the machine signal, and it is the only one.

### Outcome Vocabulary

| Outcome               | Condition                                     | Exit code                     |
| --------------------- | --------------------------------------------- | ----------------------------- |
| `clean`               | The scan returned zero violations              | `Ok` (0)                      |
| `violations`          | One or more groups returned findings           | `PolicyFail` (1)              |
| `prerequisite_missing`| A required external tool is absent             | `PrerequisiteMissing` (3)     |
| `error`               | The path is missing, arguments are invalid, or I/O failed | `RuntimeError` (2) |

Every failure path returns an `ExitCode`, so the process ends through one place.
A caller must branch on the exit code; the rendered text is for a human.

### Path Semantics

A path decides what the report shows, not what the scan covers. The
architecture rules read sibling files, so the scan always walks the whole
member dir. The target classifies into four scopes:

- Workspace root: scan every member, report everything.
- A member dir (`crates`, `packages`, `modules`): scan that dir alone, report all of it.
- A subfolder inside a member: scan the whole member, report only the violations under the subfolder.
- A single file inside a member: scan the whole member, report only that file.

- `report [path]` runs the same scan as `scan <path>`; it changes only the
  report shape, so it narrows the result the same way a subfolder or file target does.

### Output Shapes

Three report shapes, one field set. Every violation carries `code`,
`violation_name`, `file`, `line`, `column`, `severity`, `why`, and `fix`.
`violation_name`, `why`, and `fix` come from the `LintResult` fields a
capability fills; when a capability leaves them empty the legacy message
format parses the same three fields out of the `msg` text, so an
unconverted capability still renders complete.

The text report is a folder tree: `{top}` braces the member dir, `[member]`
braces the member, `(file:line:col)` braces the source position, and each
violation prints as four lines.

```text
{crates}

[shared_common]

(src/agent_aes304_bypass.rs:7)
AES304:UNWRAP_EXPECT
WHY: Found forbidden bypass token: 'unwrap'
FIX: Replace the unwrap/expect call with structured error handling.
```

The text report ends with a `Hint` block of four lines: `lint-arwaky-cli skill
list` and the three ways to narrow a scan: one member dir, one folder under a
member dir, one file. The machine formats carry the same fields:

- JSON: one object per violation with `code`, `violation_name`, `file`, `line`,
  `column`, `message`, `severity`, `why`, `fix`, `member`, wrapped in
  `{target, results}`. No summary block: a scan lists what it found, not
  counts of what it found.
- SARIF: the reason goes in `message.text` and the rest in `properties`.
- JUnit: the fields travel as `failure` attributes.

A violation's `msg` argument states the fact and the subject only. It carries
no `AES### NAME:` prefix and no embedded `WHY:` or `FIX:` clauses, because
those now travel in the three dedicated fields.

### Report

`report [path]` runs the same scan as `scan`, counts violations per member,
and prints the count and its change against the last run for the target,
rather than the violations themselves. Counts are keyed by the same member
path the scan report groups by, so a baseline compares identical definitions
whether the scan covered one member or the whole workspace. Snapshots live in
the XDG data directory, one entry per target, so a scan of one target never
becomes the baseline of another. The command exits success whenever it prints
a report: a first run with no baseline prints absolute counts and a note that
the next run will show the delta.

### Kind

`cli` — the process surface a user types into. Each `surface_*_command.rs` file
parses one subcommand's arguments, calls the aggregate behind it, and turns the
result into output plus an exit code. No rule logic lives here.

### Request Shape

A subcommand receives its arguments as parsed values, never as raw `argv`. The
surface resolves a path to an absolute path before any aggregate sees it, so a
group never has to know the process's working directory.

### Invariants

- A surface file computes nothing about the codebase. It renders what an
  aggregate returns and maps the outcome to an exit code. The exception is
  `report`: it counts violations per member and reads a snapshot, because a
  progress table is data, not rendering.

- A surface may not import a capabilities file directly. It reaches behavior
  through the aggregate seam (`crates/dispatcher`).
- A surface holds no state across invocations; each subcommand is a fresh call.
  `report` reads a snapshot file written by an earlier run of itself — the
  exception is the stored counts, not process state.
- Every subcommand path ends in exactly one `ExitCode`. There is no second
  place the process can terminate.

### Change Checklist

Changing a subcommand's arguments touches the surface file and its contract.
Adding a new rule group touches `crates/dispatcher/src/surface_<group>_action.rs`
so the existing subcommands pick it up. Adding a new subcommand touches one
surface file, the CLI wiring, and the Entry Points table below.

## Components

| Subcommand | File                              | Aggregates reached | States                                    |
| ---------- | --------------------------------- | ----------------- | ----------------------------------------- |
| `scan <path>` | `surface_scan_command.rs`      | naming, import, quality, role, orphan, structure | clean / violations / error |
| `docs <path>` | `surface_scan_command.rs`      | doc-rules aggregate | clean / violations / error                |
| `check <path>` | `surface_check_action.rs` (dispatcher) | the same group as `scan` | exit-code shaped |
| `taxonomy` / `contract` / `capabilities` / `utility` / `agents` / `surface` | `surface_scan_command.rs` | every group, narrowed to one layer by `surface_layer_scan_action.rs` (dispatcher) | clean / violations / error |
| `ci`          | `surface_ci_command.rs`          | every group, with thresholds | pass / fail |
| `fix <path>`  | `surface_fix_command.rs`         | auto-fix aggregate | clean / violations / error                |
| `config`      | `surface_config_command.rs`      | config-system aggregate | rendered / error                       |
| `git`         | `surface_git_command.rs`         | git-hooks aggregate | rendered / error                         |
| `doctor`, `security`, `deps`, `self-update` | `surface_maintenance_command.rs` | maintenance aggregate | rendered / error |
| `init`, `install`, `mcp-config` | `surface_setup_command.rs`     | project-setup aggregate | rendered / error                         |
| `watch`       | `surface_watch_command.rs`       | file-watch aggregate | streaming / error                         |
| `plugin`      | `surface_plugin_command.rs`      | plugin registry     | rendered / error                          |
| `report [path]` | `surface_report_command.rs` | the same groups as `scan`; prints counts, not violations | clean / error |
| `skill`       | `surface_skill_command.rs`       | skill registry      | rendered / error                          |
| Findings rendering | `utility_output_text_formatter.rs` | — | rendered                                |

`docs <path>` takes the **audit root, not a filter** — unlike every other
path-taking subcommand here, which scopes a workspace-wide run down to the
target. Pointing `docs` at a sub-directory audits that sub-tree as its own
document chain, so crosslink checks (AES604) that expect root-level
`PRD.md`/`ROADMAP.md` siblings behave differently than in a whole-workspace run.
See `crates/doc-rules/FRD.md`.

The `report` command never prints findings. It counts the scan's violations
per member and shows each count's change against the last run for that target.
A first run has no baseline, so it shows absolute counts and a note that the
next run will show the delta.

### States

| State                   | Condition                                                     | Output                                     |
| ----------------------- | ------------------------------------------------------------- | ------------------------------------------ |
| `clean`                 | The scan returned zero violations                              | The summary line plus `Ok`                  |
| `violations`            | One or more groups returned findings                           | The rendered findings plus `PolicyFail`     |
| `progress`              | The `report` command printed a table; it exits success whether or not the scan found violations | The progress table plus `Ok` |
| `prerequisite_missing`  | A required external tool is absent                             | The tool name and an install hint plus `PrerequisiteMissing` |
| `error`                 | The path is missing, arguments are invalid, or I/O failed      | The cause plus `RuntimeError`               |

### Error States

| Condition                                    | Surface                                       | Exit code                     |
| -------------------------------------------- | --------------------------------------------- | ----------------------------- |
| Path does not exist                           | every subcommand taking a path                 | `RuntimeError` (2)            |
| External linter not on `PATH`                 | `scan` with adapters enabled, `doctor`        | `PrerequisiteMissing` (3)    |
| Config unreadable or malformed                | `config`                                      | `RuntimeError` (2)            |
| Violations above the configured threshold     | `ci`                                          | `PolicyFail` (1)              |

Surfaces report; they never raise.

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- Rules: [RULES_AES.md](../../RULES_AES.md)
