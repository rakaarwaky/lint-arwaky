# Plan: Cleanup Setup & Maintenance — Remove `cancel`, Move `doctor`

See [README.md](../../../README.md) for CLI commands and [DEPLOY.md](../../../DEPLOY.md) for maintenance context.

## Problem

1. `cancel` command is useless — lint process is fast, no long-running jobs to cancel
2. `doctor` exists in **both** `setup` and `maintenance` with different implementations — confusing
3. `setup` should focus on one-time init; `doctor` is a periodic health check

## Changes

### 1. Remove `cancel` from everywhere

| File                                                 | What to remove                                        |
| ---------------------------------------------------- | ----------------------------------------------------- |
| `surface_core_command.rs`                            | `Commands::Cancel { job_id }` variant                 |
| `root_cli_main_entry.rs`                             | Match arm `Commands::Cancel` + import `handle_cancel` |
| `surface_map_command.rs`                             | `handle_cancel()` function                            |
| `cli-commands/src/lib.rs`                            | Re-export `handle_cancel`                             |
| `shared/cli-commands/taxonomy_catalog_constant.rs`   | `"cancel"` entry                                      |
| `shared/cli-commands/taxonomy_command_catalog_vo.rs` | `cancel` catalog entry                                |
| `mcp-server/src/surface_tools_command.rs`            | `cancel` in dev domain JSON                           |

### 2. Move `doctor` from setup to maintenance

**Remove from setup:**

| File                       | What to remove                                    |
| -------------------------- | ------------------------------------------------- |
| `surface_core_command.rs`  | `SetupCommands::Doctor` variant                   |
| `surface_setup_command.rs` | `SetupCommands::Doctor` match arm (lines 360-508) |
| `surface_tui_command.rs`   | `[setup doctor]` menu item + match arm            |

**Add to maintenance:**

| File                             | What to update                                                     |
| -------------------------------- | ------------------------------------------------------------------ |
| `surface_core_command.rs`        | Add `Commands::Doctor` top-level variant                           |
| `root_cli_main_entry.rs`         | Add match arm calling maintenance `doctor`                         |
| `surface_maintenance_command.rs` | Replace stub `doctor()` with real impl (from setup, lines 360-508) |
| `surface_tui_command.rs`         | Replace `[setup doctor]` with `[doctor]` under Info section        |

### 3. Update TUI menu

**Before:**

```
Setup & config:
  [setup doctor]     Diagnose environment
  [setup install]    Install all adapters
  [setup init]       Create default config
  [config show]      Show active configuration
```

**After:**

```
Setup:
  [setup install]    Install all adapters
  [setup init]       Create default config
  [config show]      Show active configuration

Info:
  [doctor]           Diagnose environment
  [version]          Show version
  [adapters]         List active adapters
  [vscode-graph]     Export graph JSON for VS Code
```

## CLI Commands Before → After

| Before             | After                          |
| ------------------ | ------------------------------ |
| `setup doctor`     | `doctor` (top-level)           |
| `setup install`    | `setup install` (unchanged)    |
| `setup init`       | `setup init` (unchanged)       |
| `setup mcp-config` | `setup mcp-config` (unchanged) |
| `cancel <job_id>`  | **removed**                    |

## File Changes Summary

| Action | File                                                            |
| ------ | --------------------------------------------------------------- |
| EDIT   | `crates/cli-commands/src/surface_core_command.rs`               |
| EDIT   | `crates/cli-commands/src/surface_setup_command.rs`              |
| EDIT   | `crates/cli-commands/src/surface_maintenance_command.rs`        |
| EDIT   | `crates/cli-commands/src/surface_map_command.rs`                |
| EDIT   | `crates/cli-commands/src/surface_tui_command.rs`                |
| EDIT   | `crates/cli-commands/src/lib.rs`                                |
| EDIT   | `crates/root_cli_main_entry.rs`                                 |
| EDIT   | `crates/shared/src/cli-commands/taxonomy_catalog_constant.rs`   |
| EDIT   | `crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs` |
| EDIT   | `crates/mcp-server/src/surface_tools_command.rs`                |

## Risk

- Low risk — removing unused feature + consolidating duplicate
- `setup doctor` → `doctor` is a breaking change for users who have scripts calling `setup doctor`
- Mitigate: keep `setup doctor` as alias that prints deprecation warning → calls `doctor`
