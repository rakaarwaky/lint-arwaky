# Project: Production Readiness Fixes

## Architecture
- Cargo workspace with multiple crates, 6-layer architecture.
- Source parsing, code analysis, plugin system, output report, cli-commands, etc.

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | Cycle Detection | Replace todo!() in contract_cycle_protocol | none | PLANNED |
| 2 | Source Parser Default | Replace todo!() in SourceParserOrchestrator default | none | PLANNED |
| 3 | Placeholder Analyzer | Replace panic!() in PlaceholderAnalyzer | none | PLANNED |
| 4 | Mutex Unwrap | Fix mutex unwrap() in root_mcp_main_entry | none | PLANNED |
| 5 | Duplicate Metrics | Remove duplicate metrics provider | none | PLANNED |
| 6 | Workspace Version Sync | Sync crate versions to 1.10.13 | none | PLANNED |
| 7 | Plugin Commands | Implement PluginCommandsOrchestrator real logic | none | PLANNED |
| 8 | Report Commands | Implement ReportCommandsOrchestrator real logic | none | PLANNED |
| 9 | MCP Server Placeholders | Clean up println! placeholders in MCP server surfaces | none | PLANNED |
| 10| CLI Setup Fallback | Use cross-platform HOME fallback in surface_setup_command | none | PLANNED |
| 11| Test Data Cleanup | Remove test data from infrastructure_rust_scanner | none | PLANNED |
| 12| Orphan Surfaces check | Fix check_surfaces_orphan to accept file list | none | PLANNED |

## Interface Contracts
- Default behavior of core traits should not crash or run stub code.
- Mutexes should recover from poison.
- Crates should have synchronized versions matching the workspace root.
