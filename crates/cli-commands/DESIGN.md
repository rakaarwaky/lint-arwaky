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
  aggregate returns and maps the outcome to an exit code.
- A surface may not import a capabilities file directly. It reaches behavior
  through the aggregate seam (`crates/dispatcher`).
- A surface holds no state across invocations; each subcommand is a fresh call.
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
| `ci`          | `surface_ci_command.rs`          | every group, with thresholds | pass / fail |
| `fix <path>`  | `surface_fix_command.rs`         | auto-fix aggregate | clean / violations / error                |
| `config`      | `surface_config_command.rs`      | config-system aggregate | rendered / error                       |
| `git`         | `surface_git_command.rs`         | git-hooks aggregate | rendered / error                         |
| `doctor`, `security`, `deps`, `self-update` | `surface_maintenance_command.rs` | maintenance aggregate | rendered / error |
| `init`, `install`, `mcp-config` | `surface_setup_command.rs`     | project-setup aggregate | rendered / error                         |
| `watch`       | `surface_watch_command.rs`       | file-watch aggregate | streaming / error                         |
| `plugin`      | `surface_plugin_command.rs`      | plugin registry     | rendered / error                          |
| `skill`       | `surface_skill_command.rs`       | skill registry      | rendered / error                          |
| Findings rendering | `utility_output_text_formatter.rs` | — | rendered                                |

`docs <path>` takes the **audit root, not a filter** — unlike every other
path-taking subcommand here, which scopes a workspace-wide run down to the
target. Pointing `docs` at a sub-directory audits that sub-tree as its own
document chain, so crosslink checks (AES604) that expect root-level
`PRD.md`/`ROADMAP.md` siblings behave differently than in a whole-workspace run.
See `crates/doc-rules/FRD.md`.

`utility_output_text_formatter.rs` renders findings for a terminal; it holds no
rule logic and reads nothing from the aggregates beyond their results.

### States

| State                   | Condition                                                     | Output                                     |
| ----------------------- | ------------------------------------------------------------- | ------------------------------------------ |
| `clean`                 | The scan returned zero violations                              | The summary line plus `Ok`                  |
| `violations`            | One or more groups returned findings                           | The rendered findings plus `PolicyFail`     |
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
