# Analysis: shared/src — Common vs Feature Folder Classification

## Rule
- **Move to common/**: Types imported by **3+ other** feature crates (cross-cutting contracts/taxonomy)
- **Stay in feature folder**: Types used by **≤2** other crates (feature-specific)

---

## Classification

### MOVE TO common/ (cross-cutting contracts)

| Source Folder | File | Type | Used By | Action |
|--------------|------|------|---------|--------|
| **config-system** | `contract_config_orchestrator_aggregate.rs` | `IConfigOrchestratorAggregate` | 8 crates | → `common/contract_config_orchestrator_aggregate.rs` |
| **code-analysis** | `contract_code_analysis_aggregate.rs` | `ICodeAnalysisAggregate` | 6 crates | → `common/contract_code_analysis_aggregate.rs` |
| **cli-commands** | `taxonomy_result_vo.rs` | `LintResult` | 11 crates | → `common/taxonomy_lint_result_vo.rs` |
| **cli-commands** | `taxonomy_scan_report_vo.rs` | `LintResultList` | 6 crates | → `common/taxonomy_lint_result_list_vo.rs` |
| **cli-commands** | `taxonomy_format_vo.rs` | `Format` | 5 crates | → `common/taxonomy_format_vo.rs` |
| **import-rules** | `contract_import_runner_aggregate.rs` | `IImportRunnerAggregate` | 4 crates | → `common/contract_import_runner_aggregate.rs` |
| **import-rules** | `taxonomy_violation_import_vo.rs` | violation types | 5 crates | → `common/taxonomy_import_violation_vo.rs` |
| **import-rules** | `taxonomy_import_error.rs` | `ImportError` | 5 crates | → `common/taxonomy_import_error.rs` |
| **orphan-detector** | `contract_orphan_aggregate.rs` | `IOrphanAggregate` | 4 crates | → `common/contract_orphan_aggregate.rs` |
| **orphan-detector** | `taxonomy_orphan_parse_result_vo.rs` | parse result types | 5 crates | → `common/taxonomy_orphan_parse_result_vo.rs` |
| **naming-rules** | `contract_naming_runner_aggregate.rs` | `INamingRunnerAggregate` | 4 crates | → `common/contract_naming_runner_aggregate.rs` |
| **role-rules** | `contract_role_runner_aggregate.rs` | `IRoleRunnerAggregate` | 4 crates | → `common/contract_role_runner_aggregate.rs` |
| **external-lint** | `contract_external_lint_aggregate.rs` | `IExternalLintAggregate` | 4 crates | → `common/contract_external_lint_aggregate.rs` |
| **git-hooks** | `contract_git_hooks_aggregate.rs` | `GitHooksAggregate` | 3 crates | → `common/contract_git_hooks_aggregate.rs` |
| **git-hooks** | `contract_manager_protocol.rs` | `IHookManagerProtocol` | 2 crates | → `common/contract_hook_manager_protocol.rs` |
| **project-setup** | `contract_setup_aggregate.rs` | `SetupManagementAggregate` | 4 crates | → `common/contract_setup_aggregate.rs` |
| **maintenance** | `contract_maintenance_aggregate.rs` | `MaintenanceCommandsAggregate` | 4 crates | → `common/contract_maintenance_aggregate.rs` |
| **auto-fix** | `contract_fix_aggregate.rs` | `LintFixOrchestratorAggregate` | 2 crates | → `common/contract_fix_aggregate.rs` |
| **auto-fix** | `taxonomy_fix_vo.rs` | `FixResult` | 2 crates | → `common/taxonomy_fix_result_vo.rs` |
| **file-watch** | `contract_provider_protocol.rs` | `IWatchProviderProtocol` | 2 crates | → `common/contract_watch_provider_protocol.rs` |
| **file-watch** | `contract_change_analyzer_protocol.rs` | `IChangeAnalyzerProtocol` | 2 crates | → `common/contract_change_analyzer_protocol.rs` |
| **report-formatter** | `contract_report_formatter_aggregate.rs` | `IReportFormatterAggregate` | 2 crates | → `common/contract_report_formatter_aggregate.rs` |

### STAY in feature folder (feature-specific)

#### auto-fix/
| File | Type | Reason |
|------|------|--------|
| `contract_fix_protocol.rs` | `IFixProtocol` | Only used by auto-fix |
| `contract_file_adapter_protocol.rs` | `IFileAdapterProtocol` | Only used by auto-fix |
| `taxonomy_fix_applied_event.rs` | `FixApplied` | Only used by auto-fix |
| `taxonomy_fix_outcome_vo.rs` | `FixOutcome` | Only used by auto-fix |
| `utility_symbol_renamer.rs` | symbol rename logic | Only used by auto-fix |

#### cli-commands/
| File | Type | Reason |
|------|------|--------|
| `taxonomy_cli_vo.rs` | CLI-specific VOs | Only used by cli-commands |
| `taxonomy_command_catalog_vo.rs` | command catalog | Only used by cli-commands |
| `taxonomy_protocol_vo.rs` | protocol VOs | Only used by cli-commands |
| `taxonomy_scan_request_vo.rs` | scan request | Only used by cli-commands |
| `utility_path_resolver.rs` | path resolution | Only used by cli-commands |

#### code-analysis/
| File | Type | Reason |
|------|------|--------|
| `contract_adapter_protocol.rs` | `ILinterAdapterProtocol` | Only used by external-lint (1 other) |
| `contract_bypass_checker_protocol.rs` | bypass checker | Only used by code-analysis |
| `contract_class_protocol.rs` | class protocol | Only used by code-analysis |
| `contract_code_metric_analyzer_protocol.rs` | metric analyzer | Only used by code-analysis |
| `contract_dead_inheritance_protocol.rs` | dead inheritance | Only used by code-analysis |
| `contract_line_protocol.rs` | line protocol | Only used by code-analysis |
| `taxonomy_analysis_vo.rs` | analysis VOs | Only used by code-analysis |
| `taxonomy_code_analysis_rule_vo.rs` | rule VOs | Only used by code-analysis |
| `taxonomy_operation_error.rs` | operation error | Only used by code-analysis |
| `taxonomy_violation_code_analysis_vo.rs` | violation VOs | Only used by code-analysis |
| `utility_bypass_detector.rs` | bypass detection | Only used by code-analysis |
| `utility_code_duplication_detector.rs` | duplication | Only used by code-analysis |
| `utility_column_index.rs` | column index | Only used by code-analysis |
| `utility_file_reader.rs` | file reader | Only used by code-analysis |
| `utility_language_mapper.rs` | language mapping | Only used by code-analysis |
| `utility_mandatory_checker.rs` | mandatory check | Only used by code-analysis |
| `utility_target_resolver.rs` | target resolution | Only used by code-analysis |

#### config-system/
| File | Type | Reason |
|------|------|--------|
| `contract_parser_protocol.rs` | `IConfigParserProtocol` | Only used by config-system |
| `contract_reader_protocol.rs` | `IConfigReaderProtocol` | Only used by config-system |
| `contract_validator_protocol.rs` | `IConfigValidatorProtocol` | Only used by config-system |
| `contract_workspace_detector_protocol.rs` | workspace detector | Only used by config-system |
| `taxonomy_config_error.rs` | config error | Only used by config-system |
| `taxonomy_config_language_vo.rs` | language VO | Only used by config-system |
| `taxonomy_config_vo.rs` | config VO | Only used by config-system |
| `taxonomy_identifier_vo.rs` | identifier VO | Only used by config-system |
| `taxonomy_multi_project_workspace_info_vo.rs` | workspace info | Only used by config-system |
| `taxonomy_setting_vo.rs` | setting VO | Only used by config-system |
| `taxonomy_source_vo.rs` | source VO | Only used by config-system |
| `taxonomy_validation_vo.rs` | validation VO | Only used by config-system |
| `utility_config_defaults.rs` | config defaults | Only used by config-system |
| `utility_config_io.rs` | config I/O | Only used by config-system |
| `utility_config_merger.rs` | config merger | Only used by config-system |
| `utility_config_parser.rs` | config parser | Only used by config-system |

#### external-lint/
| File | Type | Reason |
|------|------|--------|
| `contract_external_lint_executor_protocol.rs` | executor protocol | Only used by external-lint |
| `contract_external_lint_selector_protocol.rs` | selector protocol | Only used by external-lint |
| `contract_external_lint_utility_protocol.rs` | utility protocol | Only used by external-lint |
| `utility_external_lint.rs` | external lint utils | Only used by external-lint |
| `utility_external_lint_io.rs` | external lint I/O | Only used by external-lint |

#### git-hooks/
| File | Type | Reason |
|------|------|--------|
| `contract_diff_protocol.rs` | `IDiffProtocol` | Only used by git-hooks |
| `contract_hook_protocol.rs` | `IHookProtocol` | Only used by git-hooks |
| `contract_orchestrator_aggregate.rs` | orchestrator aggregate | Only used by git-hooks |
| `taxonomy_git_diff_data_vo.rs` | diff data VO | Only used by git-hooks |
| `taxonomy_hook_error.rs` | hook error | Only used by git-hooks |
| `utility_git_io.rs` | git I/O | Only used by git-hooks |

#### import-rules/
| File | Type | Reason |
|------|------|--------|
| `contract_cycle_import_protocol.rs` | cycle import | Only used by import-rules |
| `contract_dummy_import_protocol.rs` | dummy import | Only used by import-rules |
| `contract_import_forbidden_protocol.rs` | forbidden import | Only used by import-rules |
| `contract_import_mandatory_protocol.rs` | mandatory import | Only used by import-rules |
| `contract_unused_import_protocol.rs` | unused import | Only used by import-rules |
| `taxonomy_dependency_edge_vo.rs` | dependency edge | Only used by import-rules |
| `taxonomy_graph_color_vo.rs` | graph color | Only used by import-rules |
| `taxonomy_import_constant.rs` | import constants | Only used by import-rules |
| `taxonomy_resolved_import_vo.rs` | resolved import | Only used by import-rules |
| `utility_cycle_detector.rs` | cycle detection | Only used by import-rules |
| `utility_dummy_detector.rs` | dummy detection | Only used by import-rules |
| `utility_import_module_parser.rs` | module parser | Only used by import-rules |
| `utility_import_resolver.rs` | import resolver | Only used by import-rules |
| `utility_import_symbol_extractor.rs` | symbol extractor | Only used by import-rules |
| `utility_path_normalizer.rs` | path normalizer | Only used by import-rules |

#### maintenance/
| File | Type | Reason |
|------|------|--------|
| `contract_maintenance_protocol.rs` | maintenance protocol | Only used by maintenance |
| `taxonomy_doctor_vo.rs` | doctor VO | Only used by maintenance |
| `taxonomy_stats_vo.rs` | stats VO | Only used by maintenance |
| `utility_dependency_io.rs` | dependency I/O | Only used by maintenance |

#### mcp-server/
| File | Type | Reason |
|------|------|--------|
| `contract_mcp_server_aggregate.rs` | MCP aggregate | Only used by mcp-server |
| `taxonomy_mcp_tool_args_vo.rs` | tool args VO | Only used by mcp-server |

#### naming-rules/
| File | Type | Reason |
|------|------|--------|
| `contract_naming_checker_protocol.rs` | naming checker | Only used by naming-rules |
| `taxonomy_naming_constant.rs` | naming constants | Only used by naming-rules |
| `taxonomy_naming_violation_vo.rs` | naming violation | Only used by naming-rules |
| `utility_naming_checker.rs` | naming checker | Only used by naming-rules |

#### orphan-detector/
| File | Type | Reason |
|------|------|--------|
| `contract_orphan_graph_resolver_protocol.rs` | graph resolver | Only used by orphan-detector |
| `contract_orphan_parser_protocol.rs` | parser protocol | Only used by orphan-detector |
| `contract_orphan_protocol.rs` | orphan protocol | Only used by orphan-detector |
| `taxonomy_orphan_contract_vo.rs` | orphan contract VO | Only used by orphan-detector |
| `taxonomy_violation_orphan_vo.rs` | violation VO | Only used by orphan-detector |
| `utility_file_cache.rs` | file cache | Only used by orphan-detector |
| `utility_orphan_detector.rs` | orphan detector | Only used by orphan-detector |
| `utility_orphan_filename.rs` | filename utils | Only used by orphan-detector |
| `utility_orphan_graph_resolver.rs` | graph resolver | Only used by orphan-detector |
| `utility_orphan_io.rs` | orphan I/O | Only used by orphan-detector |
| `utility_orphan_path.rs` | orphan path | Only used by orphan-detector |
| `utility_orphan_python_parser.rs` | Python parser | Only used by orphan-detector |
| `utility_orphan_rust_parser.rs` | Rust parser | Only used by orphan-detector |
| `utility_orphan_ts_parser.rs` | TS parser | Only used by orphan-detector |
| `utility_workspace_scanner.rs` | workspace scanner | Only used by orphan-detector |

#### project-setup/
| File | Type | Reason |
|------|------|--------|
| `contract_setup_protocol.rs` | setup protocol | Only used by project-setup |
| `contract_tool_executor_protocol.rs` | tool executor | Only used by project-setup |
| `taxonomy_setup_contract_vo.rs` | setup VO | Only used by project-setup |
| `utility_filesystem_checker.rs` | filesystem check | Only used by project-setup |
| `utility_setup_io.rs` | setup I/O | Only used by project-setup |

#### report-formatter/
| File | Type | Reason |
|------|------|--------|
| `contract_report_formatter_protocol.rs` | formatter protocol | Only used by report-formatter |
| `taxonomy_json_dto_vo.rs` | JSON DTO | Only used by report-formatter |
| `taxonomy_sarif_*.rs` (5 files) | SARIF VOs | Only used by report-formatter |
| `utility_report_format.rs` | report format | Only used by report-formatter |

#### role-rules/
| File | Type | Reason |
|------|------|--------|
| `contract_agent_role_protocol.rs` | agent role | Only used by role-rules |
| `contract_capabilities_role_protocol.rs` | capabilities role | Only used by role-rules |
| `contract_role_contract_protocol.rs` | role contract | Only used by role-rules |
| `contract_surface_role_protocol.rs` | surface role | Only used by role-rules |
| `contract_taxonomy_role_protocol.rs` | taxonomy role | Only used by role-rules |
| `contract_utility_role_protocol.rs` | utility role | Only used by role-rules |
| `taxonomy_layer_names_constant.rs` | layer constants | Only used by role-rules |
| `taxonomy_layer_names_vo.rs` | layer names | Only used by role-rules |
| `taxonomy_violation_role_vo.rs` | violation VO | Only used by role-rules |

#### tui/
| File | Type | Reason |
|------|------|--------|
| ALL files | TUI-specific | Only used by tui |

#### file-watch/
| File | Type | Reason |
|------|------|--------|
| `contract_watch_aggregate.rs` | watch aggregate | Only used by file-watch |
| `taxonomy_diff_result_vo.rs` | diff result | Only used by file-watch |
| `taxonomy_service_error.rs` | service error | Only used by file-watch |
| `taxonomy_watch_config_vo.rs` | watch config | Only used by file-watch |
| `taxonomy_watch_event_vo.rs` | watch event | Only used by file-watch |

---

## Summary

### Move to common/ (22 files)
1. `config-system/contract_config_orchestrator_aggregate.rs`
2. `code-analysis/contract_code_analysis_aggregate.rs`
3. `cli-commands/taxonomy_result_vo.rs`
4. `cli-commands/taxonomy_scan_report_vo.rs`
5. `cli-commands/taxonomy_format_vo.rs`
6. `import-rules/contract_import_runner_aggregate.rs`
7. `import-rules/taxonomy_violation_import_vo.rs`
8. `import-rules/taxonomy_import_error.rs`
9. `orphan-detector/contract_orphan_aggregate.rs`
10. `orphan-detector/taxonomy_orphan_parse_result_vo.rs`
11. `naming-rules/contract_naming_runner_aggregate.rs`
12. `role-rules/contract_role_runner_aggregate.rs`
13. `external-lint/contract_external_lint_aggregate.rs`
14. `git-hooks/contract_git_hooks_aggregate.rs`
15. `git-hooks/contract_manager_protocol.rs`
16. `project-setup/contract_setup_aggregate.rs`
17. `maintenance/contract_maintenance_aggregate.rs`
18. `auto-fix/contract_fix_aggregate.rs`
19. `auto-fix/taxonomy_fix_vo.rs`
20. `file-watch/contract_provider_protocol.rs`
21. `file-watch/contract_change_analyzer_protocol.rs`
22. `report-formatter/contract_report_formatter_aggregate.rs`

### Stay in feature folders (remaining ~120 files)
All other files stay in their respective feature folders.
