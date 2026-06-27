# Plan: Remove `report` Command

## Problem

- CLI `report` — stub, only prints "not yet implemented"
- MCP `report` — functional but redundant with `scan` (already has summary + violations)

## Changes

### 1. Remove CLI definition

| File                      | Remove                                             |
| ------------------------- | -------------------------------------------------- |
| `surface_core_command.rs` | `Commands::Report { path, output_format }` variant |

### 2. Remove handler

| File                        | Remove                                         |
| --------------------------- | ---------------------------------------------- |
| `surface_report_command.rs` | Delete entire file (`handle_report` is a stub) |

### 3. Remove root entry

| File                     | Remove                                                                     |
| ------------------------ | -------------------------------------------------------------------------- |
| `root_cli_main_entry.rs` | `use cli_commands::surface_report_command;` + match arm `Commands::Report` |

### 4. Remove lib re-export

| File                      | Remove                                                                               |
| ------------------------- | ------------------------------------------------------------------------------------ |
| `cli-commands/src/lib.rs` | `pub mod surface_report_command;` + `pub use surface_report_command::handle_report;` |

### 5. Remove TUI menu item

| File                     | Remove                           |
| ------------------------ | -------------------------------- |
| `surface_tui_command.rs` | `[report]` menu item + match arm |

### 6. Remove catalog entries

| File                             | Remove                 |
| -------------------------------- | ---------------------- |
| `taxonomy_catalog_constant.rs`   | `"report"` entry       |
| `taxonomy_command_catalog_vo.rs` | `report` catalog entry |

### 7. Remove from MCP `execute_command` action enum

| File                       | Remove                                 |
| -------------------------- | -------------------------------------- |
| `surface_tools_command.rs` | `"report"` from action enum (line 282) |

### 8. Remove MCP `report` action handler

| File                       | Remove                                          |
| -------------------------- | ----------------------------------------------- |
| `surface_tools_command.rs` | `"report" => { ... }` match arm (lines 57-130+) |

## Breaking Changes

| Change                      | Impact                                                   |
| --------------------------- | -------------------------------------------------------- |
| CLI `report` removed        | Scripts using `lint-arwaky report` break                 |
| MCP `report` action removed | AI agents using `execute_command(action="report")` break |

**Mitigation**: `scan` already provides the same data (violations + summary). MCP clients can use `execute_command(action="scan")` instead.

## File Changes Summary

| Action | File                                                            |
| ------ | --------------------------------------------------------------- |
| DELETE | `crates/cli-commands/src/surface_report_command.rs`             |
| EDIT   | `crates/cli-commands/src/surface_core_command.rs`               |
| EDIT   | `crates/cli-commands/src/surface_tui_command.rs`                |
| EDIT   | `crates/cli-commands/src/lib.rs`                                |
| EDIT   | `crates/root_cli_main_entry.rs`                                 |
| EDIT   | `crates/shared/src/cli-commands/taxonomy_catalog_constant.rs`   |
| EDIT   | `crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs` |
| EDIT   | `crates/mcp-server/src/surface_tools_command.rs`                |
