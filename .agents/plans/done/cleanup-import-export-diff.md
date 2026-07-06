# Plan: Remove `import`, `export`, `diff` Commands

See [README.md](../../README.md) for CLI overview and [TEST.md](../../../TEST.md) for verification criteria.

## Problem

- `import` — stub, only prints "Imported" without reading/parsing/writing
- `export` — duplicate of `report`, runs lint + prints json/sarif/junit
- `diff` — redundant, user can `check` twice manually

## Changes

### 1. Remove CLI definitions

| File                      | Remove                                                                                               |
| ------------------------- | ---------------------------------------------------------------------------------------------------- |
| `surface_core_command.rs` | `Commands::Diff { path1, path2 }`, `Commands::Import { config_file }`, `Commands::Export { format }` |

### 2. Remove handlers

| File                     | Remove                                                                             |
| ------------------------ | ---------------------------------------------------------------------------------- |
| `surface_map_command.rs` | `handle_diff()`, `handle_import()`, `handle_export()`                              |
| `surface_map_command.rs` | Unused import: `surface_output_controller::{print_json, print_junit, print_sarif}` |
| `surface_map_command.rs` | Update PURPOSE comment                                                             |

### 3. Remove root entry match arms

| File                     | Remove                                                               |
| ------------------------ | -------------------------------------------------------------------- |
| `root_cli_main_entry.rs` | Match arms: `Commands::Diff`, `Commands::Import`, `Commands::Export` |

### 4. Remove lib re-export

| File                      | Remove                                                     |
| ------------------------- | ---------------------------------------------------------- |
| `cli-commands/src/lib.rs` | Re-export: `handle_diff`, `handle_import`, `handle_export` |

### 5. Remove catalog entries

| File                             | Remove                                   |
| -------------------------------- | ---------------------------------------- |
| `taxonomy_catalog_constant.rs`   | `"diff"`, `"import"`, `"export"` entries |
| `taxonomy_command_catalog_vo.rs` | `diff`, `export` catalog entries         |

### 6. Remove MCP server entries

| File                       | Remove                                        |
| -------------------------- | --------------------------------------------- |
| `surface_tools_command.rs` | `diff`, `import`, `export` in dev domain JSON |

### 7. Clean unused `print_*` functions (only used by `export`)

| File                           | Remove                                           |
| ------------------------------ | ------------------------------------------------ |
| `surface_output_controller.rs` | `print_json()`, `print_sarif()`, `print_junit()` |

Keep `get_output_dir`, `write_output`, `tee_stdout` — used by `fix`.

### 8. Remove unused imports in `surface_output_controller.rs`

| Remove                                                     |
| ---------------------------------------------------------- |
| `use shared::cli_commands::taxonomy_result_vo::LintResult` |
| `use shared::cli_commands::taxonomy_severity_vo::Severity` |

## File Changes Summary

| Action | File                                                            |
| ------ | --------------------------------------------------------------- |
| EDIT   | `crates/cli-commands/src/surface_core_command.rs`               |
| EDIT   | `crates/cli-commands/src/surface_map_command.rs`                |
| EDIT   | `crates/cli-commands/src/surface_output_controller.rs`          |
| EDIT   | `crates/cli-commands/src/lib.rs`                                |
| EDIT   | `crates/root_cli_main_entry.rs`                                 |
| EDIT   | `crates/shared/src/cli-commands/taxonomy_catalog_constant.rs`   |
| EDIT   | `crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs` |
| EDIT   | `crates/mcp-server/src/surface_tools_command.rs`                |

## CLI Commands After (cleaned)

```
Core:       check, scan, fix, report, ci
Detection:  orphan, security, duplicates, dependencies
Setup:      setup init, setup doctor, setup install, setup mcp-config, setup hermes
Config:     config show
Git:        install-hook, uninstall-hook, git-diff
Dev:        watch, cancel
Info:       version, adapters, vscode-graph
```
