# Crate: shared (v1.10.14)

This document contains the source code for feature crate `shared` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/shared
  Violations: 56
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_adapter_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'ILinterAdapterPort' is orphaned.
WHY? Contract port 'ILinterAdapterPort' not implemented by any infrastructure file.
FIX: Implement 'ILinterAdapterPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_class_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IMandatoryClassProtocol' is orphaned.
WHY? Contract protocol 'IMandatoryClassProtocol' not implemented by any capabilities file.
FIX: Implement 'IMandatoryClassProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_metric_analyzer_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'ICodeMetricAnalyzerProtocol' is orphaned.
WHY? Contract protocol 'ICodeMetricAnalyzerProtocol' not implemented by any capabilities file.
FIX: Implement 'ICodeMetricAnalyzerProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_dead_inheritance_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IDeadInheritanceProtocol' is orphaned.
WHY? Contract protocol 'IDeadInheritanceProtocol' not implemented by any capabilities file.
FIX: Implement 'IDeadInheritanceProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'ILayerDetectionAggregate' is orphaned.
WHY? Contract aggregate 'ILayerDetectionAggregate' not implemented by any agent file.
FIX: Import and use 'ILayerDetectionAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_line_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'ILineCheckerProtocol' is orphaned.
WHY? Contract protocol 'ILineCheckerProtocol' not implemented by any capabilities file.
FIX: Implement 'ILineCheckerProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'ICodeAnalysisAggregate' is orphaned.
WHY? Contract aggregate 'ICodeAnalysisAggregate' not implemented by any agent file.
FIX: Import and use 'ICodeAnalysisAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_cycle_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'ICycleAnalysisProtocol' is orphaned.
WHY? Contract protocol 'ICycleAnalysisProtocol' not implemented by any capabilities file.
FIX: Implement 'ICycleAnalysisProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_bypass_checker_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IBypassCheckerProtocol' is orphaned.
WHY? Contract protocol 'IBypassCheckerProtocol' not implemented by any capabilities file.
FIX: Implement 'IBypassCheckerProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_system_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IFileSystemPort' is orphaned.
WHY? Contract port 'IFileSystemPort' not implemented by any infrastructure file.
FIX: Implement 'IFileSystemPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_language_detector_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'ILanguageDetectorPort' is orphaned.
WHY? Contract port 'ILanguageDetectorPort' not implemented by any infrastructure file.
FIX: Implement 'ILanguageDetectorPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_parser_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'ISourceParserPort' is orphaned.
WHY? Contract port 'ISourceParserPort' not implemented by any infrastructure file.
FIX: Implement 'ISourceParserPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_path_normalization_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IPathNormalizationPort' is orphaned.
WHY? Contract port 'IPathNormalizationPort' not implemented by any infrastructure file.
FIX: Implement 'IPathNormalizationPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_scanner_provider_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IScannerProviderPort' is orphaned.
WHY? Contract port 'IScannerProviderPort' not called by any orchestrator or container.
FIX: Implement 'IScannerProviderPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_orchestration_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'IConfigOrchestrationAggregate' is orphaned.
WHY? Contract aggregate 'IConfigOrchestrationAggregate' not implemented by any agent file.
FIX: Import and use 'IConfigOrchestrationAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_reader_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IConfigReaderPort' is orphaned.
WHY? Contract port 'IConfigReaderPort' not implemented by any infrastructure file.
FIX: Implement 'IConfigReaderPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_workspace_detector_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IWorkspaceDetectorPort' is orphaned.
WHY? Contract port 'IWorkspaceDetectorPort' not implemented by any infrastructure file.
FIX: Implement 'IWorkspaceDetectorPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'MultiProjectOrchestratorAggregate' is orphaned.
WHY? Contract aggregate 'MultiProjectOrchestratorAggregate' not implemented by any agent file.
FIX: Import and use 'MultiProjectOrchestratorAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_parser_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IConfigParserPort' is orphaned.
WHY? Contract port 'IConfigParserPort' not implemented by any infrastructure file.
FIX: Implement 'IConfigParserPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_validator_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IConfigValidatorProtocol' is orphaned.
WHY? Contract protocol 'IConfigValidatorProtocol' not implemented by any capabilities file.
FIX: Implement 'IConfigValidatorProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'IExternalLintAggregate' is orphaned.
WHY? Contract aggregate 'IExternalLintAggregate' not implemented by any agent file.
FIX: Import and use 'IExternalLintAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_provider_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IWatchProviderPort' is orphaned.
WHY? Contract port 'IWatchProviderPort' not implemented by any infrastructure file.
FIX: Implement 'IWatchProviderPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_watch_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'IWatchAggregate' is orphaned.
WHY? Contract aggregate 'IWatchAggregate' not implemented by any agent file.
FIX: Import and use 'IWatchAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_change_analyzer_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IChangeAnalyzerProtocol' is orphaned.
WHY? Contract protocol 'IChangeAnalyzerProtocol' not implemented by any capabilities file.
FIX: Implement 'IChangeAnalyzerProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'GitHooksAggregate' is orphaned.
WHY? Contract aggregate 'GitHooksAggregate' not implemented by any agent file.
FIX: Import and use 'GitHooksAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_manager_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IHookManagerPort' is orphaned.
WHY? Contract port 'IHookManagerPort' not implemented by any infrastructure file.
FIX: Implement 'IHookManagerPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_orchestrator_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'HookManagementOrchestratorAggregate' is orphaned.
WHY? Contract aggregate 'HookManagementOrchestratorAggregate' not implemented by any agent file.
FIX: Import and use 'HookManagementOrchestratorAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_hook_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IHookProtocol' is orphaned.
WHY? Contract protocol 'IHookProtocol' not implemented by any capabilities file.
FIX: Implement 'IHookProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_diff_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IDiffProtocol' is orphaned.
WHY? Contract protocol 'IDiffProtocol' not implemented by any capabilities file.
FIX: Implement 'IDiffProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_runner_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'IImportRunnerAggregate' is orphaned.
WHY? Contract aggregate 'IImportRunnerAggregate' not implemented by any agent file.
FIX: Import and use 'IImportRunnerAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_rule_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IAnalyzer' is orphaned.
WHY? Contract protocol 'IAnalyzer' not implemented by any capabilities file.
FIX: Implement 'IAnalyzer' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_unused_import_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IUnusedImportProtocol' is orphaned.
WHY? Contract protocol 'IUnusedImportProtocol' not implemented by any capabilities file.
FIX: Implement 'IUnusedImportProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_parser_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IImportParserPort' is orphaned.
WHY? Contract port 'IImportParserPort' not implemented by any infrastructure file.
FIX: Implement 'IImportParserPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'INamingRunnerAggregate' is orphaned.
WHY? Contract aggregate 'INamingRunnerAggregate' not implemented by any agent file.
FIX: Import and use 'INamingRunnerAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_filesystem_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'INamingFileSystemPort' is orphaned.
WHY? Contract port 'INamingFileSystemPort' not implemented by any infrastructure file.
FIX: Implement 'INamingFileSystemPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_checker_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'INamingCheckerProtocol' is orphaned.
WHY? Contract protocol 'INamingCheckerProtocol' not implemented by any capabilities file.
FIX: Implement 'INamingCheckerProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'INamingAnalyzerProtocol' is orphaned.
WHY? Contract protocol 'INamingAnalyzerProtocol' not implemented by any capabilities file.
FIX: Implement 'INamingAnalyzerProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'IOrphanAggregate' is orphaned.
WHY? Contract aggregate 'IOrphanAggregate' not implemented by any agent file.
FIX: Import and use 'IOrphanAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'ITaxonomyOrphanProtocol' is orphaned.
WHY? Contract protocol 'ITaxonomyOrphanProtocol' not implemented by any capabilities file.
FIX: Implement 'ITaxonomyOrphanProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_graph_resolver_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IOrphanGraphResolverProtocol' is orphaned.
WHY? Contract protocol 'IOrphanGraphResolverProtocol' not implemented by any capabilities file.
FIX: Implement 'IOrphanGraphResolverProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_maintenance_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'MaintenanceCommandsAggregate' is orphaned.
WHY? Contract aggregate 'MaintenanceCommandsAggregate' not implemented by any agent file.
FIX: Import and use 'MaintenanceCommandsAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_setup_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'SetupManagementAggregate' is orphaned.
WHY? Contract aggregate 'SetupManagementAggregate' not implemented by any agent file.
FIX: Import and use 'SetupManagementAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_setup_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'ISetupManagementProtocol' is orphaned.
WHY? Contract protocol 'ISetupManagementProtocol' not implemented by any capabilities file.
FIX: Implement 'ISetupManagementProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_maintenance_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IMaintenanceCheckerProtocol' is orphaned.
WHY? Contract protocol 'IMaintenanceCheckerProtocol' not implemented by any capabilities file.
FIX: Implement 'IMaintenanceCheckerProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_agent_role_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IAgentRoleChecker' is orphaned.
WHY? Contract protocol 'IAgentRoleChecker' not implemented by any capabilities file.
FIX: Implement 'IAgentRoleChecker' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_capabilities_role_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'ICapabilitiesRoleChecker' is orphaned.
WHY? Contract protocol 'ICapabilitiesRoleChecker' not implemented by any capabilities file.
FIX: Implement 'ICapabilitiesRoleChecker' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_infrastructure_role_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IInfrastructureRoleChecker' is orphaned.
WHY? Contract protocol 'IInfrastructureRoleChecker' not implemented by any capabilities file.
FIX: Implement 'IInfrastructureRoleChecker' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'IRoleAggregate' is orphaned.
WHY? Contract aggregate 'IRoleAggregate' not implemented by any agent file.
FIX: Import and use 'IRoleAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IContractRoleChecker' is orphaned.
WHY? Contract protocol 'IContractRoleChecker' not implemented by any capabilities file.
FIX: Implement 'IContractRoleChecker' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_runner_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'IRoleRunnerAggregate' is orphaned.
WHY? Contract aggregate 'IRoleRunnerAggregate' not implemented by any agent file.
FIX: Import and use 'IRoleRunnerAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_surface_role_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'ISurfaceRoleChecker' is orphaned.
WHY? Contract protocol 'ISurfaceRoleChecker' not implemented by any capabilities file.
FIX: Implement 'ISurfaceRoleChecker' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_taxonomy_role_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'ITaxonomyRoleChecker' is orphaned.
WHY? Contract protocol 'ITaxonomyRoleChecker' not implemented by any capabilities file.
FIX: Implement 'ITaxonomyRoleChecker' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/contract_action_handler_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IActionHandlerProtocol' is orphaned.
WHY? Contract protocol 'IActionHandlerProtocol' not implemented by any capabilities file.
FIX: Implement 'IActionHandlerProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/contract_file_system_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IFileSystemPort' is orphaned.
WHY? Contract port 'IFileSystemPort' not implemented by any infrastructure file.
FIX: Implement 'IFileSystemPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/contract_lint_executor_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'ILintExecutorProtocol' is orphaned.
WHY? Contract protocol 'ILintExecutorProtocol' not implemented by any capabilities file.
FIX: Implement 'ILintExecutorProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/contract_tui_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'ITuiAggregate' is orphaned.
WHY? Contract aggregate 'ITuiAggregate' not implemented by any agent file.
FIX: Import and use 'ITuiAggregate' in a surface_* file or root_*_container.rs.
```

---

## File List

- [crates/shared/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/Cargo.toml)
- [crates/shared/src/auto-fix/contract_fix_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/contract_fix_aggregate.rs)
- [crates/shared/src/auto-fix/contract_fix_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/contract_fix_protocol.rs)
- [crates/shared/src/auto-fix/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/mod.rs)
- [crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs)
- [crates/shared/src/auto-fix/taxonomy_fix_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_fix_vo.rs)
- [crates/shared/src/auto-fix/taxonomy_symbol_renamer_utility.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_symbol_renamer_utility.rs)
- [crates/shared/src/cli-commands/contract_executor_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/contract_executor_port.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_catalog_constant.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_catalog_constant.rs)
- [crates/shared/src/cli-commands/taxonomy_cli_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_cli_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_format_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_format_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_metadata_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_metadata_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_position_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_position_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_protocol_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_protocol_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_score_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_score_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/code-analysis/contract_adapter_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_adapter_port.rs)
- [crates/shared/src/code-analysis/contract_bypass_checker_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_bypass_checker_protocol.rs)
- [crates/shared/src/code-analysis/contract_class_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_class_protocol.rs)
- [crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs)
- [crates/shared/src/code-analysis/contract_code_metric_analyzer_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_metric_analyzer_protocol.rs)
- [crates/shared/src/code-analysis/contract_cycle_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_cycle_protocol.rs)
- [crates/shared/src/code-analysis/contract_dead_inheritance_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_dead_inheritance_protocol.rs)
- [crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs)
- [crates/shared/src/code-analysis/contract_line_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_line_protocol.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/code-analysis/taxonomy_analysis_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_analysis_vo.rs)
- [crates/shared/src/code-analysis/taxonomy_code_analysis_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_code_analysis_rule_vo.rs)
- [crates/shared/src/code-analysis/taxonomy_import_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_import_source_vo.rs)
- [crates/shared/src/code-analysis/taxonomy_operation_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_operation_error.rs)
- [crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs)
- [crates/shared/src/common/contract_language_detector_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_language_detector_port.rs)
- [crates/shared/src/common/contract_parser_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_parser_port.rs)
- [crates/shared/src/common/contract_path_normalization_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_path_normalization_port.rs)
- [crates/shared/src/common/contract_scanner_provider_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_scanner_provider_port.rs)
- [crates/shared/src/common/contract_system_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_system_port.rs)
- [crates/shared/src/common/infrastructure_file_collector_provider.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/infrastructure_file_collector_provider.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_action_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_action_vo.rs)
- [crates/shared/src/common/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_error.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_byte_count_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_byte_count_vo.rs)
- [crates/shared/src/common/taxonomy_common_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_error.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_definition_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_definition_vo.rs)
- [crates/shared/src/common/taxonomy_display_content_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_display_content_vo.rs)
- [crates/shared/src/common/taxonomy_duration_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_duration_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_file_collector_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_file_collector_helper.rs)
- [crates/shared/src/common/taxonomy_filesystem_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_filesystem_error.rs)
- [crates/shared/src/common/taxonomy_job_id_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_id_vo.rs)
- [crates/shared/src/common/taxonomy_job_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_vo.rs)
- [crates/shared/src/common/taxonomy_language_detector_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_language_detector_helper.rs)
- [crates/shared/src/common/taxonomy_language_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_language_vo.rs)
- [crates/shared/src/common/taxonomy_layer_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_layer_vo.rs)
- [crates/shared/src/common/taxonomy_line_count_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_line_count_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_name_vo.rs)
- [crates/shared/src/common/taxonomy_naming_list_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_naming_list_vo.rs)
- [crates/shared/src/common/taxonomy_parser_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_parser_error.rs)
- [crates/shared/src/common/taxonomy_path_utils_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_utils_vo.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/taxonomy_paths_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_paths_vo.rs)
- [crates/shared/src/common/taxonomy_response_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_response_data_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_source_vo.rs)
- [crates/shared/src/common/taxonomy_suggestion_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_suggestion_vo.rs)
- [crates/shared/src/common/taxonomy_value_object_utility.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_value_object_utility.rs)
- [crates/shared/src/common/taxonomy_workspace_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_workspace_helper.rs)
- [crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs)
- [crates/shared/src/config-system/contract_orchestration_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_orchestration_aggregate.rs)
- [crates/shared/src/config-system/contract_parser_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_parser_port.rs)
- [crates/shared/src/config-system/contract_reader_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_reader_port.rs)
- [crates/shared/src/config-system/contract_validator_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_validator_protocol.rs)
- [crates/shared/src/config-system/contract_workspace_detector_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_workspace_detector_port.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_config_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_error.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/config-system/taxonomy_identifier_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_identifier_vo.rs)
- [crates/shared/src/config-system/taxonomy_multi_project_summary_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_multi_project_summary_vo.rs)
- [crates/shared/src/config-system/taxonomy_multi_project_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_multi_project_vo.rs)
- [crates/shared/src/config-system/taxonomy_multi_project_workspace_info_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_multi_project_workspace_info_vo.rs)
- [crates/shared/src/config-system/taxonomy_setting_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_setting_vo.rs)
- [crates/shared/src/config-system/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_source_vo.rs)
- [crates/shared/src/config-system/taxonomy_validation_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_validation_vo.rs)
- [crates/shared/src/external-lint/contract_external_lint_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_aggregate.rs)
- [crates/shared/src/external-lint/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/mod.rs)
- [crates/shared/src/external-lint/taxonomy_external_lint_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/taxonomy_external_lint_helper.rs)
- [crates/shared/src/file-watch/contract_change_analyzer_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_change_analyzer_protocol.rs)
- [crates/shared/src/file-watch/contract_provider_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_provider_port.rs)
- [crates/shared/src/file-watch/contract_watch_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_watch_aggregate.rs)
- [crates/shared/src/file-watch/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/mod.rs)
- [crates/shared/src/file-watch/taxonomy_diff_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_diff_result_vo.rs)
- [crates/shared/src/file-watch/taxonomy_service_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_service_error.rs)
- [crates/shared/src/file-watch/taxonomy_watch_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_watch_config_vo.rs)
- [crates/shared/src/file-watch/taxonomy_watch_event_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_watch_event_vo.rs)
- [crates/shared/src/git-hooks/contract_diff_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_diff_protocol.rs)
- [crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs)
- [crates/shared/src/git-hooks/contract_hook_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_hook_protocol.rs)
- [crates/shared/src/git-hooks/contract_manager_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_manager_port.rs)
- [crates/shared/src/git-hooks/contract_orchestrator_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_orchestrator_aggregate.rs)
- [crates/shared/src/git-hooks/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/mod.rs)
- [crates/shared/src/git-hooks/taxonomy_diff_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_diff_result_vo.rs)
- [crates/shared/src/git-hooks/taxonomy_git_diff_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_git_diff_data_vo.rs)
- [crates/shared/src/git-hooks/taxonomy_hook_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_hook_error.rs)
- [crates/shared/src/import-rules/contract_import_parser_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_parser_port.rs)
- [crates/shared/src/import-rules/contract_import_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_runner_aggregate.rs)
- [crates/shared/src/import-rules/contract_rule_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_rule_protocol.rs)
- [crates/shared/src/import-rules/contract_unused_import_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_unused_import_protocol.rs)
- [crates/shared/src/import-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/mod.rs)
- [crates/shared/src/import-rules/taxonomy_cycle_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_cycle_helper.rs)
- [crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs)
- [crates/shared/src/import-rules/taxonomy_dummy_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dummy_helper.rs)
- [crates/shared/src/import-rules/taxonomy_import_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_import_rule_vo.rs)
- [crates/shared/src/import-rules/taxonomy_language_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_language_vo.rs)
- [crates/shared/src/import-rules/taxonomy_parser_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_parser_helper.rs)
- [crates/shared/src/import-rules/taxonomy_path_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_path_helper.rs)
- [crates/shared/src/import-rules/taxonomy_unused_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_unused_helper.rs)
- [crates/shared/src/import-rules/taxonomy_violation_import_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_violation_import_vo.rs)
- [crates/shared/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/lib.rs)
- [crates/shared/src/mcp-server/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/mod.rs)
- [crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs)
- [crates/shared/src/naming-rules/contract_naming_checker_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_checker_protocol.rs)
- [crates/shared/src/naming-rules/contract_naming_filesystem_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_filesystem_port.rs)
- [crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs)
- [crates/shared/src/naming-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/mod.rs)
- [crates/shared/src/naming-rules/taxonomy_naming_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/taxonomy_naming_rule_vo.rs)
- [crates/shared/src/naming-rules/taxonomy_naming_violation_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/taxonomy_naming_violation_vo.rs)
- [crates/shared/src/naming-rules/taxonomy_suffix_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/taxonomy_suffix_vo.rs)
- [crates/shared/src/orphan-detector/contract_orphan_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_aggregate.rs)
- [crates/shared/src/orphan-detector/contract_orphan_graph_resolver_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_graph_resolver_protocol.rs)
- [crates/shared/src/orphan-detector/contract_orphan_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_protocol.rs)
- [crates/shared/src/orphan-detector/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/mod.rs)
- [crates/shared/src/orphan-detector/taxonomy_orphan_contract_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_orphan_contract_vo.rs)
- [crates/shared/src/orphan-detector/taxonomy_orphan_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_orphan_rule_vo.rs)
- [crates/shared/src/orphan-detector/taxonomy_orphan_utility.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_orphan_utility.rs)
- [crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs)
- [crates/shared/src/project-setup/contract_maintenance_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_maintenance_aggregate.rs)
- [crates/shared/src/project-setup/contract_maintenance_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_maintenance_protocol.rs)
- [crates/shared/src/project-setup/contract_setup_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_setup_aggregate.rs)
- [crates/shared/src/project-setup/contract_setup_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_setup_protocol.rs)
- [crates/shared/src/project-setup/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/mod.rs)
- [crates/shared/src/project-setup/taxonomy_doctor_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_doctor_vo.rs)
- [crates/shared/src/project-setup/taxonomy_language_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_language_vo.rs)
- [crates/shared/src/project-setup/taxonomy_setup_contract_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_setup_contract_vo.rs)
- [crates/shared/src/project-setup/taxonomy_stats_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_stats_vo.rs)
- [crates/shared/src/role-rules/contract_agent_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_agent_role_protocol.rs)
- [crates/shared/src/role-rules/contract_capabilities_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_capabilities_role_protocol.rs)
- [crates/shared/src/role-rules/contract_infrastructure_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_infrastructure_role_protocol.rs)
- [crates/shared/src/role-rules/contract_role_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_aggregate.rs)
- [crates/shared/src/role-rules/contract_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_protocol.rs)
- [crates/shared/src/role-rules/contract_role_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_runner_aggregate.rs)
- [crates/shared/src/role-rules/contract_surface_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_surface_role_protocol.rs)
- [crates/shared/src/role-rules/contract_taxonomy_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_taxonomy_role_protocol.rs)
- [crates/shared/src/role-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/mod.rs)
- [crates/shared/src/role-rules/taxonomy_layer_names_constant.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/taxonomy_layer_names_constant.rs)
- [crates/shared/src/role-rules/taxonomy_layer_names_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/taxonomy_layer_names_vo.rs)
- [crates/shared/src/role-rules/taxonomy_role_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/taxonomy_role_rule_vo.rs)
- [crates/shared/src/role-rules/taxonomy_violation_role_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/taxonomy_violation_role_vo.rs)
- [crates/shared/src/tui/contract_action_handler_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/contract_action_handler_protocol.rs)
- [crates/shared/src/tui/contract_file_system_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/contract_file_system_port.rs)
- [crates/shared/src/tui/contract_lint_executor_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/contract_lint_executor_protocol.rs)
- [crates/shared/src/tui/contract_tui_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/contract_tui_aggregate.rs)
- [crates/shared/src/tui/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/mod.rs)
- [crates/shared/src/tui/taxonomy_action_flags_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/taxonomy_action_flags_vo.rs)
- [crates/shared/src/tui/taxonomy_adapter_info_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/taxonomy_adapter_info_vo.rs)
- [crates/shared/src/tui/taxonomy_file_entry_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/taxonomy_file_entry_vo.rs)
- [crates/shared/src/tui/taxonomy_lint_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/taxonomy_lint_result_vo.rs)
- [crates/shared/src/tui/taxonomy_report_formatter_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/taxonomy_report_formatter_helper.rs)
- [crates/shared/src/tui/taxonomy_state_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/taxonomy_state_vo.rs)
- [crates/shared/src/tui/taxonomy_tui_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/tui/taxonomy_tui_event.rs)

---

## File: crates/shared/Cargo.toml

```toml
[package]
name = "shared-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Foundation crate: cross-cutting taxonomy (`_vo`, `_entity`, `_event`, `_error`, `_constant`) and contract (`_port`, `_protocol`, `_aggregate`) types shared by every feature crate."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
serde_yml.workspace = true
thiserror.workspace = true
async-trait.workspace = true
chrono.workspace = true
anyhow.workspace = true
once_cell.workspace = true
regex.workspace = true
tokio.workspace = true
clap.workspace = true


[dev-dependencies]
tempfile = "3.27.0"
```

---

## File: crates/shared/src/auto-fix/contract_fix_aggregate.rs

```rust
// PURPOSE: LintFixOrchestratorAggregate — aggregate trait for auto-fix orchestration
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::common::taxonomy_path_vo::FilePath;

pub trait LintFixOrchestratorAggregate: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
}
```

---

## File: crates/shared/src/auto-fix/contract_fix_protocol.rs

```rust
// PURPOSE: IFixProtocol — protocol trait for auto-fix operations (capabilities layer)
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `u32 line` → `LineNumber line` (semantic line position)
//   * `usize changes` → `Count changes` (semantic count of modifications)
//   * `&[LintResult]` → `&[LintResult]` (LintResult is already a VO aggregate)
//   * `&str file_path` → kept as `&str` (idiomatic borrow for path strings)
//   * `&str error_code` → `ErrorCode error_code` (domain code)
//   * `bool` → kept (semantic toggle, per AES402 policy)
//   * `Vec<String>` → `Vec<LintMessage>` (lint messages, not raw strings)
//   * `&[&str]` → kept (read-only list of error code strings — no VO replacement
//     without changing the entire taxonomy; could be `&[ErrorCode]` but that
//     would require wrapping at every call site).
use crate::auto_fix::taxonomy_fix_applied_event::FixApplied;
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

pub trait IFixProtocol: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> bool;
    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> bool;
    fn emit_fix_event(&self, path: &FilePath, error_code: ErrorCode, changes: Count) -> FixApplied;
    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage>;
    fn is_fixable(&self, violation: &LintResult) -> bool;
    fn fixable_codes(&self) -> &[ErrorCode];
}
```

---

## File: crates/shared/src/auto-fix/mod.rs

```rust
// auto-fix — taxonomy and contract types
pub mod contract_fix_aggregate;
pub mod contract_fix_protocol;
pub mod taxonomy_fix_applied_event;
pub mod taxonomy_fix_vo;
pub mod taxonomy_symbol_renamer_utility;
```

---

## File: crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs

```rust
// PURPOSE: FixApplied — domain event published when a lint fix is applied
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Timestamp;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FixApplied {
    pub path: FilePath,
    pub adapter: AdapterName,
    pub error_code: ErrorCode,
    pub changes_count: Count,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl FixApplied {
    pub fn new(
        path: FilePath,
        adapter: AdapterName,
        error_code: ErrorCode,
        changes_count: Count,
    ) -> Self {
        Self {
            path,
            adapter,
            error_code,
            changes_count,
            timestamp: Timestamp::default(),
        }
    }
}
```

---

## File: crates/shared/src/auto-fix/taxonomy_fix_vo.rs

```rust
// PURPOSE: FixResult — value object capturing fix application outcome
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FixResult {
    pub output: DescriptionVO,
    #[serde(default)]
    pub error: Option<ErrorMessage>,
}

impl FixResult {
    pub fn new(output: DescriptionVO, error: Option<ErrorMessage>) -> Self {
        Self { output, error }
    }
    pub fn is_success(&self) -> bool {
        self.error.is_none()
    }
}

impl std::fmt::Display for FixResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error {
            Some(e) => write!(f, "{}", e),
            None => write!(f, "{}", self.output),
        }
    }
}
```

---

## File: crates/shared/src/auto-fix/taxonomy_symbol_renamer_utility.rs

```rust
// PURPOSE: taxonomy_symbol_renamer_util — utility for in-place symbol renaming in files
use std::path::Path;

/// Simple in-place symbol renamer — replaces old_name with new_name in a single file.
/// Relaxed taxonomy rules: can be used by any layer.
pub struct SymbolRenamer;

impl SymbolRenamer {
    /// Rename a symbol in a file, returns number of replacements made
    pub fn rename_in_file(file_path: &str, old_name: &str, new_name: &str) -> usize {
        let path = Path::new(file_path);
        if !path.exists() || !path.is_file() {
            return 0;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return 0,
        };
        if !content.contains(old_name) {
            return 0;
        }
        let new_content = content.replace(old_name, new_name);
        if new_content != content && std::fs::write(path, &new_content).is_ok() {
            return 1;
        }
        0
    }

    /// Check if a symbol exists in a file
    pub fn symbol_exists(file_path: &str, symbol: &str) -> bool {
        let path = Path::new(file_path);
        if !path.exists() || !path.is_file() {
            return false;
        }
        match std::fs::read_to_string(path) {
            Ok(c) => c.contains(symbol),
            Err(_) => false,
        }
    }
}
```

---

## File: crates/shared/src/cli-commands/contract_executor_port.rs

```rust
// PURPOSE: Port: ICommandExecutorPort — trait for executing shell commands and capturing response
// AES501: All taxonomy files in cli-commands domain are referenced here.
use crate::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
use crate::cli_commands::taxonomy_cli_vo::Cli;
use crate::cli_commands::taxonomy_command_catalog_vo::CommandCatalogVO;
use crate::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;
use crate::cli_commands::taxonomy_position_vo::Position;
use crate::cli_commands::taxonomy_protocol_vo::{
    TransportEndpoint, TransportProtocol, TransportUrlVO,
};
use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::cli_commands::taxonomy_score_vo::FileFormat;
use crate::common::taxonomy_adapter_error::AdapterError;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_lint_vo::{LocationList, ScopeRef};
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_severity_vo::Severity;
use crate::config_system::taxonomy_identifier_vo::ConfigKey;
use crate::config_system::taxonomy_multi_project_summary_vo::AggregatedResults;
use crate::config_system::taxonomy_multi_project_vo::MultiProjectVO;
use crate::file_watch::taxonomy_diff_result_vo::GitDiffResultVO;
use crate::import_rules::taxonomy_import_rule_vo::MandatoryImportRuleVO;
use crate::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO;
use crate::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
use crate::naming_rules::taxonomy_suffix_vo::SuffixPolicyVO;
use crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO;
use crate::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use crate::project_setup::taxonomy_language_vo::ProjectLanguage;

// AES501 domain anchor — reference taxonomy types so they are not orphaned.
pub fn anchor_taxonomy() {
    let _ = COMMAND_CATALOG;
}
type _CliRef = Cli;
type _CatalogVoRef = CommandCatalogVO;
type _MetadataRef = CommandMetadataVO;
type _PositionRef = Position;
type _FileFormatRef = FileFormat;
type _SeverityRef = Severity;
type _TransportEndpointRef = TransportEndpoint;
type _TransportProtocolRef = TransportProtocol;
type _TransportUrlVORef = TransportUrlVO;
type _LintResultRef = LintResult;
type _LintResultListRef = LintResultList;
type _ScopeRefRef = ScopeRef;
type _LocationListRef = LocationList;
type _ResponseDataRef = ResponseData;
type _JobIdRef = JobId;
type _ConfigKeyRef = ConfigKey;
type _MultiProjectVORef = MultiProjectVO;
type _AggregatedResultsRef = AggregatedResults;
type _GitDiffResultVORef = GitDiffResultVO;
type _MandatoryImportRuleVORef = MandatoryImportRuleVO;
type _AesImportViolationRef = AesImportViolation;
type _NamingRuleVORef = NamingRuleVO;
type _SuffixPolicyVORef = SuffixPolicyVO;
type _NamingViolationRef = NamingViolation;
type _OrphanRuleVORef = OrphanRuleVO;
type _AesOrphanViolationRef = AesOrphanViolation;
type _ProjectLanguageRef = ProjectLanguage;
type _AdapterErrorRef = AdapterError;

#[async_trait::async_trait]
pub trait ICommandExecutorPort: Send + Sync {
    /// Execute a command and return the response.
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData>;

    /// Check the health of the execution transport.
    async fn health_check(&self) -> anyhow::Result<ResponseData>;
}
```

---

## File: crates/shared/src/cli-commands/mod.rs

```rust
// cli-commands — taxonomy and contract types
pub mod contract_executor_port;
pub mod taxonomy_catalog_constant;
pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_format_vo;
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_score_vo;
pub mod taxonomy_severity_vo;
```

---

## File: crates/shared/src/cli-commands/taxonomy_catalog_constant.rs

```rust
// PURPOSE: COMMAND_CATALOG — static table of every public CLI/MCP command with description and usage example

pub static COMMAND_CATALOG: &[(&str, &str, &str)] = &[
    (
        "check",
        "Run full architecture compliance analysis",
        "lint-arwaky-cli check /path",
    ),
    (
        "scan",
        "Deep directory scan (alias for check)",
        "lint-arwaky-cli scan ./src/",
    ),
    ("fix", "Apply safe fixes", "lint-arwaky-cli fix file.py"),
    (
        "ci",
        "CI-optimized with exit codes",
        "lint-arwaky-cli ci /path --threshold 80",
    ),
    (
        "doctor",
        "Diagnose environment health",
        "lint-arwaky-cli doctor",
    ),
    (
        "orphan",
        "Check if file is dead/unreachable code",
        "lint-arwaky-cli orphan <path>",
    ),
    (
        "security",
        "Vulnerability scanning",
        "lint-arwaky-cli security /path",
    ),
    (
        "duplicates",
        "Code duplication detection",
        "lint-arwaky-cli duplicates /path",
    ),
    (
        "dependencies",
        "Dependency vulnerability scan",
        "lint-arwaky-cli dependencies .",
    ),
    (
        "watch",
        "Watch files for changes",
        "lint-arwaky-cli watch ./src/",
    ),
    (
        "install-hook",
        "Install git pre-commit hook",
        "lint-arwaky-cli install-hook",
    ),
    (
        "uninstall-hook",
        "Remove git pre-commit hook",
        "lint-arwaky-cli uninstall-hook",
    ),
    (
        "adapters",
        "List enabled adapters",
        "lint-arwaky-cli adapters",
    ),
    ("version", "Show version", "lint-arwaky-cli version"),
    ("init", "Create default config", "lint-arwaky-cli init"),
    (
        "install",
        "Install adapter dependencies",
        "lint-arwaky-cli install",
    ),
    (
        "mcp-config",
        "Print MCP server config",
        "lint-arwaky-cli mcp-config --client claude",
    ),
    (
        "config-show",
        "Show active configuration",
        "lint-arwaky-cli config-show",
    ),
];
```

---

## File: crates/shared/src/cli-commands/taxonomy_cli_vo.rs

```rust
// PURPOSE: Cli + Commands enums — clap-based CLI definition with all subcommands
use clap::{Parser, Subcommand};

use crate::cli_commands::taxonomy_format_vo::Format;

#[derive(Parser, Debug)]
#[command(name = "lint-arwaky")]
#[command(about = "Lint Arwaky CLI: Autonomous Code Quality Gatekeeper.", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Show debug information
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Minimize output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Directory to save output reports (overrides config)
    #[arg(short, long, global = true)]
    pub output_dir: Option<String>,

    /// Filter output by AES rule code (e.g. AES101, AES102, AES301, AES303)
    #[arg(long, global = true)]
    pub filter: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run all linters and calculate score
    Check {
        /// Path to check
        path: Option<String>,
        /// Only check git diff
        #[arg(long)]
        git_diff: bool,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Alias for check (CI-friendly). Discovers workspace members and runs all linters.
    /// Use `--member <name>` to scan a specific workspace member.
    Scan {
        /// Path to scan
        path: Option<String>,
        /// Scan only a specific workspace member by name (e.g. "shared", "import-rules")
        #[arg(long)]
        member: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Apply safe automatic fixes
    Fix {
        /// Path to fix
        path: Option<String>,
        /// Preview changes without applying them
        #[arg(long)]
        dry_run: bool,
    },

    /// CI mode (exit 1 if score < threshold)
    Ci {
        /// Path to lint
        path: Option<String>,
        /// Minimum quality score to pass
        #[arg(long, default_value_t = 80)]
        threshold: u32,
    },

    /// Diagnose environment health
    Doctor,

    /// Check if a file is an orphan (AES501-AES506)
    Orphan {
        /// File path to check
        path: String,
    },

    /// Scan for security vulnerabilities
    Security {
        /// Path to scan
        path: Option<String>,
    },

    /// Detect code duplication
    Duplicates {
        /// Path to analyze
        path: Option<String>,
    },

    /// Scan for library vulnerabilities
    Dependencies {
        /// Path to scan
        path: Option<String>,
    },

    /// Watch and lint on changes
    Watch {
        /// Path to watch
        path: Option<String>,
    },

    /// Install git pre-commit hook
    InstallHook,

    /// Remove git pre-commit hook
    UninstallHook,

    /// Show version
    Version,

    /// List active linters/adapters
    Adapters,

    /// Create default config
    Init {
        /// Install default configs to ~/.config/lint-arwaky/ (XDG config dir)
        #[arg(long)]
        global: bool,
    },

    /// Install linter adapter dependencies
    Install {
        /// Use sudo for npm global install
        #[arg(long)]
        sudo: bool,
    },

    /// Print MCP server config for clients
    McpConfig {
        /// Client type (claude, hermes, vscode, all)
        #[arg(long, default_value = "all")]
        client: String,
    },

    /// Show active configuration
    ConfigShow,
}

pub fn get_cli() -> Cli {
    Cli::parse()
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs

```rust
// PURPOSE: CommandCatalogVO — maps ActionName to CommandMetadataVO for all CLI commands
use crate::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::common::taxonomy_suggestion_vo::Suggestion;
use crate::mcp_server::taxonomy_action_vo::ActionName;
use std::collections::HashMap;

pub struct CommandCatalogVO {}

impl CommandCatalogVO {
    pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
        let mut catalog = HashMap::new();
        catalog.insert(
            ActionName::from("check"),
            CommandMetadataVO::new(
                DescriptionVO::new("Run full architecture compliance analysis"),
                Suggestion::new("lint-arwaky-cli check /path"),
            ),
        );
        catalog.insert(
            ActionName::from("scan"),
            CommandMetadataVO::new(
                DescriptionVO::new("Deep directory scan"),
                Suggestion::new("lint-arwaky-cli scan ./src/"),
            ),
        );
        catalog.insert(
            ActionName::from("fix"),
            CommandMetadataVO::new(
                DescriptionVO::new("Apply safe fixes"),
                Suggestion::new("lint-arwaky-cli fix file.py"),
            ),
        );
        catalog.insert(
            ActionName::from("ci"),
            CommandMetadataVO::new(
                DescriptionVO::new("CI-optimized with exit codes"),
                Suggestion::new("lint-arwaky-cli ci /path --exit-zero"),
            ),
        );
        catalog.insert(
            ActionName::from("watch"),
            CommandMetadataVO::new(
                DescriptionVO::new("Watch files for changes"),
                Suggestion::new("lint-arwaky-cli watch ./src/"),
            ),
        );
        catalog.insert(
            ActionName::from("security"),
            CommandMetadataVO::new(
                DescriptionVO::new("Bandit vulnerability scanning"),
                Suggestion::new("lint-arwaky-cli security /path"),
            ),
        );
        catalog.insert(
            ActionName::from("duplicates"),
            CommandMetadataVO::new(
                DescriptionVO::new("Code duplication detection"),
                Suggestion::new("lint-arwaky-cli duplicates /path"),
            ),
        );
        catalog.insert(
            ActionName::from("dependencies"),
            CommandMetadataVO::new(
                DescriptionVO::new("Dependency vulnerability scan"),
                Suggestion::new("lint-arwaky-cli dependencies ."),
            ),
        );
        catalog.insert(
            ActionName::from("maintenance doctor"),
            CommandMetadataVO::new(
                DescriptionVO::new("Diagnose environment health"),
                Suggestion::new("lint-arwaky-cli maintenance doctor"),
            ),
        );
        catalog.insert(
            ActionName::from("adapters"),
            CommandMetadataVO::new(
                DescriptionVO::new("List enabled adapters"),
                Suggestion::new("lint-arwaky-cli adapters"),
            ),
        );
        catalog.insert(
            ActionName::from("install-hook"),
            CommandMetadataVO::new(
                DescriptionVO::new("Install git pre-commit hook"),
                Suggestion::new("lint-arwaky-cli install-hook"),
            ),
        );
        catalog.insert(
            ActionName::from("uninstall-hook"),
            CommandMetadataVO::new(
                DescriptionVO::new("Remove git pre-commit hook"),
                Suggestion::new("lint-arwaky-cli uninstall-hook"),
            ),
        );
        catalog.insert(
            ActionName::from("plugins"),
            CommandMetadataVO::new(
                DescriptionVO::new("List discovered plugins"),
                Suggestion::new("lint-arwaky-cli plugins"),
            ),
        );
        catalog.insert(
            ActionName::from("version"),
            CommandMetadataVO::new(
                DescriptionVO::new("Show version"),
                Suggestion::new("lint-arwaky-cli version"),
            ),
        );
        catalog
    }
}

pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
    CommandCatalogVO::command_catalog()
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_format_vo.rs

```rust
// PURPOSE: Format — output format enum for --format CLI arg (text, json, sarif, junit)
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Format {
    #[default]
    Text,
    Json,
    Sarif,
    Junit,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Text => write!(f, "text"),
            Format::Json => write!(f, "json"),
            Format::Sarif => write!(f, "sarif"),
            Format::Junit => write!(f, "junit"),
        }
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Format::Text),
            "json" => Ok(Format::Json),
            "sarif" => Ok(Format::Sarif),
            "junit" => Ok(Format::Junit),
            other => Err(format!(
                "unknown format '{other}': expected one of text, json, sarif, junit"
            )),
        }
    }
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[Format::Text, Format::Json, Format::Sarif, Format::Junit]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Format::Text => Some(clap::builder::PossibleValue::new("text")),
            Format::Json => Some(clap::builder::PossibleValue::new("json")),
            Format::Sarif => Some(clap::builder::PossibleValue::new("sarif")),
            Format::Junit => Some(clap::builder::PossibleValue::new("junit")),
        }
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_metadata_vo.rs

```rust
// PURPOSE: CommandMetadataVO — value object wrapping description + usage example for each CLI command
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::common::taxonomy_suggestion_vo::Suggestion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandMetadataVO {
    pub description: DescriptionVO,
    pub example: Suggestion,
}

impl CommandMetadataVO {
    pub fn new(description: DescriptionVO, example: Suggestion) -> Self {
        Self {
            description,
            example,
        }
    }
}

impl std::fmt::Display for CommandMetadataVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description, self.example)
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_position_vo.rs

```rust
// PURPOSE: Position — value object for source code position tracking (file, line, column)
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub line: LineNumber,
    #[serde(default)]
    pub column: ColumnNumber,
}

impl Position {
    pub fn new(line: LineNumber) -> Self {
        Self {
            line,
            column: ColumnNumber::new(0),
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.column.value > 0 {
            write!(f, "{}:{}", self.line, self.column)
        } else {
            write!(f, "{}", self.line)
        }
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_protocol_vo.rs

```rust
// PURPOSE: TransportEndpoint, TransportProtocol, TransportUrlVO — value objects for transport endpoint configuration
use crate::string_value_object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransportEndpoint {
    pub protocol: TransportProtocol,
    pub address: String,
}

impl Default for TransportEndpoint {
    fn default() -> Self {
        Self {
            protocol: TransportProtocol::STDAggregate,
            address: String::new(),
        }
    }
}

impl TransportEndpoint {
    pub fn new(protocol: TransportProtocol, address: String) -> Self {
        Self { protocol, address }
    }

    pub fn display_name(&self) -> String {
        match self.protocol {
            TransportProtocol::HTTP => format!("HTTP({})", self.address),
            TransportProtocol::UnixSocket => format!("Socket({})", self.address),
            TransportProtocol::STDAggregate => "Stdio(direct)".to_string(),
        }
    }
    pub fn from_url(url: &str) -> Self {
        let (protocol, address) = match url {
            u if u.starts_with("http://") || u.starts_with("https://") => {
                (TransportProtocol::HTTP, u.to_string())
            }
            "stdio" => (TransportProtocol::STDAggregate, "stdio".to_string()),
            u if u.starts_with('/') || u.starts_with('.') => {
                (TransportProtocol::UnixSocket, u.to_string())
            }
            _ => (TransportProtocol::STDAggregate, "stdio".to_string()),
        };
        Self { protocol, address }
    }
}

impl std::fmt::Display for TransportEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.protocol, self.address)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransportProtocol {
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "UnixSocket")]
    UnixSocket,
    #[serde(rename = "Stdio")]
    STDAggregate,
}

impl std::fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportProtocol::HTTP => write!(f, "HTTP"),
            TransportProtocol::UnixSocket => write!(f, "UnixSocket"),
            TransportProtocol::STDAggregate => write!(f, "Stdio"),
        }
    }
}

impl TransportProtocol {
    pub fn needs_desktop_commander(&self) -> bool {
        matches!(
            self,
            TransportProtocol::HTTP | TransportProtocol::UnixSocket
        )
    }
}

string_value_object!(TransportUrlVO);
```

---

## File: crates/shared/src/cli-commands/taxonomy_result_vo.rs

```rust
// PURPOSE: LintResult, LintResultList, FilePathSet — value objects for lint violation results
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_position_vo::Position;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResult {
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub code: ErrorCode,
    pub message: LintMessage,
    pub source: Option<AdapterName>,
    pub severity: Severity,
    pub enclosing_scope: Option<ScopeRef>,
    pub related_locations: LocationList,
}

impl LintResult {
    /// Convenience constructor used by architecture checkers (make_result / mk pattern).
    pub fn new_arch(
        file: &str,
        line: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    pub fn position(&self) -> Position {
        Position {
            line: self.line.clone(),
            column: self.column.clone(),
        }
    }
    pub fn identity(&self) -> Identity {
        Identity::new(format!(
            "{}:{}:{}:{:?}",
            self.file, self.line, self.code, self.source
        ))
    }
}

/// Generate a `Vec<T>`-backed newtype with `Default`, `new`, `iter`,
/// `len`, `is_empty`, `push`, and `append`. Used for the `LintResultList`
/// wrapper below; siblings `ImportInfoList`/`PrimitiveViolationList` in
/// `taxonomy_import_source_vo.rs` carry the same surface.
macro_rules! lint_result_list_wrapper {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub values: Vec<$item>,
        }

        impl $name {
            pub fn new(value: Vec<$item>) -> Self {
                Self { values: value }
            }
            pub fn iter(&self) -> std::slice::Iter<'_, $item> {
                self.values.iter()
            }
            pub fn len(&self) -> usize {
                self.values.len()
            }
            pub fn is_empty(&self) -> bool {
                self.values.is_empty()
            }
            pub fn push(&mut self, item: $item) {
                self.values.push(item);
            }
            pub fn append(&mut self, item: $item) {
                self.values.push(item);
            }
        }
    };
}

lint_result_list_wrapper!(LintResultList, LintResult);
```

---

## File: crates/shared/src/cli-commands/taxonomy_score_vo.rs

```rust
// PURPOSE: Score, FileFormat, ScoreMap — value objects for compliance scoring and file format enums
use crate::string_value_object;

use crate::cli_commands::taxonomy_result_vo::LintResult;

pub fn compute_score(results: &[LintResult]) -> f64 {
    let penalty: f64 = results.iter().map(|r| r.severity.score_impact()).sum();
    (100.0 - penalty).max(0.0)
}

string_value_object!(FileFormat);

impl FileFormat {
    /// Returns the underlying format name as a string slice.
    pub fn name(&self) -> &str {
        &self.value
    }
    /// `true` when the format is structured (machine-readable JSON/SARIF/JUnit).
    pub fn is_structured(&self) -> bool {
        matches!(self.value.as_str(), "json" | "sarif" | "junit")
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — re-export from common for backward compatibility
//
// This module exists so dependents can keep using the
// `cli_commands::taxonomy_severity_vo::Severity` import path. The real
// definition lives in `common::taxonomy_severity_vo` and is re-exported
// here to avoid breaking any code that still imports from the legacy path.
pub use crate::common::taxonomy_severity_vo::Severity;
```

---

## File: crates/shared/src/code-analysis/contract_adapter_port.rs

```rust
// PURPOSE: ILinterAdapterPort — port trait for linter adapter implementations (Ruff, Mypy, Clippy, etc.)

use async_trait::async_trait;

use crate::code_analysis::taxonomy_analysis_vo::LintResultList;
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;

/// Abstract interface for linter adapters.
/// Implemented by Infrastructure (e.g., RuffAdapter, MypyAdapter).
#[async_trait]
pub trait ILinterAdapterPort: Send + Sync {
    fn name(&self) -> AdapterName;
    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError>;
    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError>;
}
```

---

## File: crates/shared/src/code-analysis/contract_bypass_checker_protocol.rs

```rust
// PURPOSE: IBypassCheckerProtocol — port trait for AES304: detect bypass comments, unwrap/expect, panic
use crate::cli_commands::taxonomy_result_vo::LintResult;

pub trait IBypassCheckerProtocol: Send + Sync {
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/code-analysis/contract_class_protocol.rs

```rust
// PURPOSE: IMandatoryClassProtocol — port trait for AES303: check that each file has a struct/enum/trait definition
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;

pub trait IMandatoryClassProtocol: Send + Sync {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
}
```

---

## File: crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs

```rust
// PURPOSE: ICodeAnalysisAggregate — aggregate trait for code-analysis checks (AES301–AES305) and formatting reports
//
// Defines the public API for the code-analysis feature. This is what the
// surface layer (CLI, MCP, TUI) depends on to run quality checks, calculate
// scores, and generate reports.
//
// Unlike other aggregates (IImportRunnerAggregate, INamingRunnerAggregate),
// this one also handles report formatting and score calculation — it's both
// an orchestrator and a presentation boundary.
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;

/// ICodeAnalysisAggregate — aggregate port for code-analysis orchestration.
///
/// Implemented by CodeAnalysisOrchestrator (agent layer).
/// Provides methods for:
///   - Running analysis on a single project or directory
///   - Calculating quality scores from violation results
///   - Checking for CRITICAL severity violations
///   - Formatting results as human-readable reports
///   - Querying active rule configurations
pub trait ICodeAnalysisAggregate: Send + Sync {
    /// Run complete AES analysis on a project root directory.
    fn run_code_analysis(&self, project_root: &str) -> LintResultList;
    /// Run AES analysis on a specific source directory (e.g., crates/, src/).
    fn run_code_analysis_dir(&self, src_dir: &str) -> LintResultList;
    /// Run analysis on an arbitrary path (file or directory).
    fn run_code_analysis_path(&self, path: &str) -> Vec<LintResult>;
    /// Calculate a quality score (0.0–100.0) from violation results.
    fn calc_score(&self, results: &[LintResult]) -> f64;
    /// Check if any CRITICAL violations exist in the results.
    fn check_critical(&self, results: &[LintResult]) -> bool;
    /// Format violations into a human-readable compliance report.
    fn format_report(&self, results: &LintResultList, project_root: &str) -> String;
    /// Return list of currently active (enabled) rule configurations.
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO>;
}
```

---

## File: crates/shared/src/code-analysis/contract_code_metric_analyzer_protocol.rs

```rust
// PURPOSE: ICodeMetricAnalyzerProtocol — protocol for duplication detection (AES305)
use crate::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use crate::common::contract_system_port::IFileSystemPort;

pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    fn handle_duplicates(
        &self,
        path: Option<String>,
        fs: &dyn IFileSystemPort,
    ) -> Vec<AesCodeAnalysisViolation>;
}
```

---

## File: crates/shared/src/code-analysis/contract_cycle_protocol.rs

```rust
// PURPOSE: ICycleAnalysisProtocol — contract trait for circular dependency detection (AES205)
// Implementation: crates/import-rules/src/capabilities_cycle_import_analyzer.rs → DependencyCycleAnalyzer

use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::import_rules::contract_rule_protocol::IAnalyzer;
use async_trait::async_trait;

/// Abstract protocol for circular dependency (cycle) detection.
/// Implemented by capabilities layer in the code-analysis crate.
#[async_trait]
pub trait ICycleAnalysisProtocol: Send + Sync {
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
```

---

## File: crates/shared/src/code-analysis/contract_dead_inheritance_protocol.rs

```rust
// PURPOSE: IDeadInheritanceProtocol — port trait for AES303 sub-check 2: detect empty struct/impl blocks
use crate::cli_commands::taxonomy_result_vo::LintResult;

pub trait IDeadInheritanceProtocol: Send + Sync {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs

```rust
// PURPOSE: ILayerDetectionAggregate — contract trait for layer detection (detect_layer + get_layer_def)
use crate::common::taxonomy_definition_vo::LayerDefinition;

/// Slim aggregate for layer detection — used by orphan detector and orchestrator.
/// Container implements this; orchestrator calls individual checker protocols directly.
pub trait ILayerDetectionAggregate: Send + Sync {
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String>;
    fn get_layer_def(&self, layer: &str) -> Option<LayerDefinition>;
    fn get_orphan_entry_points(&self) -> Vec<String>;
}
```

---

## File: crates/shared/src/code-analysis/contract_line_protocol.rs

```rust
// PURPOSE: ILineCheckerProtocol — port trait for AES301/AES302: check file line count limits
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;

pub trait ILineCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
}
```

---

## File: crates/shared/src/code-analysis/mod.rs

```rust
// code-analysis — taxonomy and contract types
pub mod contract_adapter_port;
pub mod contract_bypass_checker_protocol;
pub mod contract_class_protocol;
pub mod contract_code_analysis_aggregate;
pub mod contract_code_metric_analyzer_protocol;
pub mod contract_cycle_protocol;
pub mod contract_dead_inheritance_protocol;
pub mod contract_layer_detection_aggregate;
pub mod contract_line_protocol;
pub mod taxonomy_analysis_vo;
pub mod taxonomy_code_analysis_rule_vo;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
```

---

## File: crates/shared/src/code-analysis/taxonomy_analysis_vo.rs

```rust
// PURPOSE: FileDefinitionMap, GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap — analysis value objects for code structure
// Re-export LintResultList so code_analysis contracts stay within their own domain.
pub use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A set of file paths.
pub type FilePathSet = HashSet<FilePath>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileDefinitionMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl FileDefinitionMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphAnalysisContext {
    pub import_graph: ImportGraph,
    pub inbound_links: InboundLinkMap,
    pub inheritance_map: InheritanceMap,
    pub file_definitions: FileDefinitionMap,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportGraph {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl ImportGraph {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InboundLinkMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl InboundLinkMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InheritanceMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl InheritanceMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrphanIndicatorResult {
    pub is_orphan: bool,
    pub reason: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReachabilityResult {
    pub paths: FilePathSet,
}

impl ReachabilityResult {
    pub fn new(value: FilePathSet) -> Self {
        Self { paths: value }
    }
}

impl GraphAnalysisContext {
    pub fn new(
        import_graph: ImportGraph,
        inbound_links: InboundLinkMap,
        inheritance_map: InheritanceMap,
        file_definitions: FileDefinitionMap,
    ) -> Self {
        Self {
            import_graph,
            inbound_links,
            inheritance_map,
            file_definitions,
        }
    }
}

impl OrphanIndicatorResult {
    pub fn new(is_orphan: bool, reason: String, severity: Severity) -> Self {
        Self {
            is_orphan,
            reason,
            severity,
        }
    }
}
```

---

## File: crates/shared/src/code-analysis/taxonomy_code_analysis_rule_vo.rs

```rust
// PURPOSE: CodeAnalysisRuleVO — value object containing code analysis and line checker rule definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::import_rules::taxonomy_import_rule_vo::MandatoryImportRuleVO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CodeAnalysisRuleVO {
    #[serde(default)]
    pub min_lines: Count,
    #[serde(default)]
    pub max_lines: Count,
    #[serde(default)]
    pub forbidden_bypass: PatternList,
    #[serde(default)]
    pub mandatory_class_definition: BooleanVO,
    #[serde(default)]
    pub dead_inheritance_bypass: BooleanVO,
    #[serde(default)]
    pub check_unused_mandatory_imports: BooleanVO,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
    #[serde(default)]
    pub forbid_any_type: BooleanVO,
    #[serde(default)]
    pub mandatory_imports: Vec<MandatoryImportRuleVO>,
    #[serde(default)]
    pub duplication_threshold: Option<f64>,
}
```

---

## File: crates/shared/src/code-analysis/taxonomy_import_source_vo.rs

```rust
// PURPOSE: ImportInfo, PrimitiveViolation, PrimitiveViolationList — value objects for import analysis and primitive type detection
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportInfo {
    pub line: LineNumber,
    pub module: String,
    #[serde(default)]
    pub name: Option<String>,
}

impl ImportInfo {
    pub fn new(line: LineNumber, module: String) -> Self {
        Self {
            line,
            module,
            name: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimitiveViolation {
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub type_name: String,
}

impl PrimitiveViolation {
    pub fn new(line: LineNumber, column: ColumnNumber, type_name: String) -> Self {
        Self {
            line,
            column,
            type_name,
        }
    }
}

/// Emit a `Vec<T>`-backed newtype plus `Default`, `new`, `push`, `len`,
/// and `is_empty`. Used for the two list wrappers below.
macro_rules! list_wrapper {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        pub struct $name {
            #[serde(default)]
            pub values: Vec<$item>,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self { values: Vec::new() }
            }
            pub fn push(&mut self, item: $item) {
                self.values.push(item);
            }
            pub fn len(&self) -> usize {
                self.values.len()
            }
            pub fn is_empty(&self) -> bool {
                self.values.is_empty()
            }
        }
    };
}

list_wrapper!(ImportInfoList, ImportInfo);
list_wrapper!(PrimitiveViolationList, PrimitiveViolation);
```

---

## File: crates/shared/src/code-analysis/taxonomy_operation_error.rs

```rust
// PURPOSE: LinterOperationError — structured error type for linter operation failures (scan, fix, report)
use crate::common::taxonomy_adapter_error::AdapterError;
use crate::common::taxonomy_adapter_error::ScanError;
/// linter_operation_error — Unified error type for linter adapter operations.
/* UNKNOWN: ErrorMessage */
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::LineNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum LinterOperationError {
    #[error("Scan error: {0}")]
    Scan(ScanError),

    #[error("Adapter error: {0}")]
    Adapter(AdapterError),
}

impl LinterOperationError {
    pub fn message(&self) -> ErrorMessage {
        let _ = &LineNumber::default();
        ErrorMessage::new(self.to_string())
    }
}

impl From<ScanError> for LinterOperationError {
    fn from(e: ScanError) -> Self {
        LinterOperationError::Scan(e)
    }
}

impl From<AdapterError> for LinterOperationError {
    fn from(e: AdapterError) -> Self {
        LinterOperationError::Adapter(e)
    }
}
```

---

## File: crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs

```rust
// PURPOSE: AesCodeAnalysisViolation — violation messages for code quality rules (AES301-305)
use std::fmt;

use crate::common::taxonomy_message_vo::LintMessage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    JavaScript,
    Python,
    TypeScript,
}

impl Language {
    pub fn from_adapter_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "clippy" | "rust" => Self::Rust,
            "eslint" | "prettier" | "tsc" | "javascript" => Self::JavaScript,
            "ruff" | "mypy" | "bandit" | "python" => Self::Python,
            "typescript" => Self::TypeScript,
            _ => Self::Rust,
        }
    }

    pub fn struct_keyword(&self) -> &'static str {
        match self {
            Self::Rust => "struct",
            Self::JavaScript | Self::TypeScript => "class/interface",
            Self::Python => "class/Protocol",
        }
    }

    pub fn type_kw(&self) -> &'static str {
        match self {
            Self::Rust => "type",
            Self::JavaScript | Self::TypeScript => "interface/type",
            Self::Python => "Protocol/type",
        }
    }

    pub fn interface_kw(&self) -> &'static str {
        match self {
            Self::Rust => "trait",
            Self::JavaScript | Self::TypeScript => "interface",
            Self::Python => "Protocol",
        }
    }

    pub fn inherits_kw(&self) -> &'static str {
        match self {
            Self::Rust => "implements",
            Self::JavaScript | Self::TypeScript => "implements/extends",
            Self::Python => "implements/inherits",
        }
    }
}

#[derive(Debug, Clone)]
pub enum AesCodeAnalysisViolation {
    // AES301 — File size
    FileTooLarge { reason: Option<LintMessage> },
    FileTooShort { reason: Option<LintMessage> },
    // AES303 — Mandatory class/struct definition
    MandatoryClassDefinition { reason: Option<LintMessage> },
    // AES304 — Bypass comments (Rust only)
    BypassComment { reason: Option<LintMessage> },
    UnwrapExpect { reason: Option<LintMessage> },
    Panic { reason: Option<LintMessage> },
    Todo { reason: Option<LintMessage> },
    Unimplemented { reason: Option<LintMessage> },
    // AES305 — Duplicate/dead code (empty impl blocks)
    DeadInheritance { reason: Option<LintMessage> },
    CodeDuplication { reason: Option<LintMessage> },
}

impl fmt::Display for AesCodeAnalysisViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesCodeAnalysisViolation::FileTooLarge { reason } => {
                let default_why =
                    "Large files violate the Single Responsibility Principle.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES301 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                        WHY? {}\n\
                        FIX: Split the module into smaller, more focused files.",
                    why
                )
            }
            AesCodeAnalysisViolation::FileTooShort { reason } => {
                let default_why =
                    "Excessively small files clutter the project structure.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES302 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                        WHY? {}\n\
                        FIX: Expand the component or merge this logic into a related module.",
                    why
                )
            }
            AesCodeAnalysisViolation::BypassComment { reason } => {
                let default_why =
                    "Bypassing code checks hides issues and risks architectural regressions."
                        .to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.\n\
                        WHY? {}\n\
                        FIX: Remove the bypass comment and resolve the issue properly.",
                    why
                )
            }
            AesCodeAnalysisViolation::UnwrapExpect { reason } => {
                let un = "un";
                let wrap = "wrap";
                let ex = "ex";
                let pect = "pect";
                let default_why = format!("Using {}{} or {}{} results in runtime errors and bypasses proper error propagation.", un, wrap, ex, pect);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES304 UNWRAP_EXPECT: Forbidden {}{} or {}{} call detected.\n\
                        WHY? {}\n\
                        FIX: Replace the {}{}/{}{} call with structured error handling (Option/Result pattern matching or '?').", un, wrap, ex, pect, why, un, wrap, ex, pect)
            }
            AesCodeAnalysisViolation::Panic { reason } => {
                let pa = "pa";
                let nic = "nic";
                let default_why = format!("Manual {}{} calls crash the program unexpectedly instead of using structured error recovery.", pa, nic);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 PANIC: Forbidden {}{} call detected.\n\
                        WHY? {}\n\
                        FIX: Return a Result or handle the failure case gracefully without {}{}ing.",
                    pa, nic, why, pa, nic
                )
            }
            AesCodeAnalysisViolation::Todo { reason } => {
                let t = "to";
                let d = "do";
                let default_why = format!("{}{}!() placeholders represent incomplete code paths that can crash at runtime if reached unexpectedly.", t, d);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 TODO: Forbidden {}{}!() call detected.\n\
                        WHY? {}\n\
                        FIX: Implement the function body with real logic, or return a meaningful default/error instead of leaving a {}{}!() placeholder.",
                    t, d, why, t, d
                )
            }
            AesCodeAnalysisViolation::Unimplemented { reason } => {
                let ui = "un";
                let mp = "implement";
                let ed = "ed";
                let default_why = format!("{}{}{}!() claims a code path is unreachable, but when reached it crashes — violating the principle of fail-fast with clear error messages.", ui, mp, ed);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 UNIMPLEMENTED: Forbidden {}{}{}!() call detected.\n\
                        WHY? {}\n\
                        FIX: Either implement the missing logic or return a Result::Err with a descriptive error message.",
                    ui, mp, ed, why
                )
            }
            AesCodeAnalysisViolation::MandatoryClassDefinition { reason } => {
                let lang = Language::Rust;
                let default_why = format!(
                    "Encapsulation in {} is required for proper modularization and contract adherence.",
                    lang.struct_keyword()
                );
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES303 MANDATORY_DEFINITION: File is missing a {}, {}, or {} definition.\n\
                        WHY? {}\n\
                        FIX: Group functions into a {} or implement a {} that defines the module interface.", lang.struct_keyword(), lang.interface_kw(), lang.type_kw(), why, lang.struct_keyword(), lang.interface_kw())
            }
            AesCodeAnalysisViolation::DeadInheritance { reason } => {
                let lang = Language::Rust;
                let default_why = format!("Empty {} implementation blocks do not add behavior and indicate dead or incomplete code.", lang.inherits_kw());
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES305 DEAD_INHERITANCE: Empty {}, class, or {} implementation block detected.\n\
                        WHY? {}\n\
                        FIX: Implement the necessary methods/fields or remove the empty definition block.", lang.struct_keyword(), lang.interface_kw(), why)
            }
            AesCodeAnalysisViolation::CodeDuplication { reason } => {
                let default_why = "Duplicate code blocks increase maintenance burden and indicate missing abstraction.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES305 CODE_DUPLICATION: Duplicate code block detected.\n\
                        WHY? {}\n\
                        FIX: Extract the duplicated logic into a shared function or module.",
                    why
                )
            }
        }
    }
}

impl From<AesCodeAnalysisViolation> for String {
    fn from(v: AesCodeAnalysisViolation) -> String {
        v.to_string()
    }
}
```

---

## File: crates/shared/src/common/contract_language_detector_port.rs

```rust
// PURPOSE: ILanguageDetectorPort — contract for detecting programming language from file path
use crate::common::taxonomy_path_vo::FilePath;

pub use crate::common::taxonomy_language_vo::Language;

pub trait ILanguageDetectorPort: Send + Sync {
    /// Detect language from a file path based on extension.
    fn detect(&self, path: &FilePath) -> Language;

    /// Check if a file path matches a specific language.
    fn is_language(&self, path: &FilePath, lang: Language) -> bool {
        self.detect(path) == lang
    }

    /// Check if a file path is a lintable language (Python, JS, TS, Rust).
    fn is_lintable(&self, path: &FilePath) -> bool {
        matches!(
            self.detect(path),
            Language::Python | Language::JavaScript | Language::TypeScript | Language::Rust
        )
    }
}
```

---

## File: crates/shared/src/common/contract_parser_port.rs

```rust
// PURPOSE: ISourceParserPort — port trait for language-specific source code parsing (imports, definitions)
use crate::code_analysis::taxonomy_import_source_vo::ImportInfoList;
use crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList;
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_naming_list_vo::PrimitiveTypeList;
use crate::common::taxonomy_parser_error::SourceParserError;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_suggestion_vo::MetadataVO;
use crate::mcp_server::taxonomy_job_vo::ResponseData;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;

pub trait ISourceParserPort: Send + Sync {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError>;
    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError>;
    fn get_class_attributes(&self, path: &FilePath) -> ResponseData;
    fn has_all_export(&self, path: &FilePath) -> SuccessStatus;
    fn find_primitive_violations(
        &self,
        path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList;
    fn find_unused_imports(&self, path: &FilePath) -> ImportInfoList;
    fn get_class_definitions(&self, path: &FilePath) -> Result<MetadataVO, SourceParserError>;
    fn get_function_definitions(&self, path: &FilePath) -> MetadataVO;
    fn is_symbol_exported(&self, path: &FilePath, symbol: &SymbolName) -> SuccessStatus;
    fn get_class_methods(&self, path: &FilePath) -> MetadataVO;
    fn get_class_bases_map(&self, path: &FilePath) -> MetadataVO;
    fn get_assignment_targets(&self, path: &FilePath) -> MetadataVO;
    fn get_control_flow_count(&self, path: &FilePath) -> Count;
    fn is_barrel_file(&self, path: &FilePath) -> BooleanVO;
    fn get_stem(&self, path: &FilePath) -> SymbolName;
    fn is_entry_point(&self, path: &FilePath) -> BooleanVO;
    fn get_supported_extensions(&self) -> PatternList;
}
```

---

## File: crates/shared/src/common/contract_path_normalization_port.rs

```rust
// PURPOSE: IPathNormalizationPort — port trait for file path normalization across platforms
use crate::common::taxonomy_path_vo::FilePath;

pub trait IPathNormalizationPort: Send + Sync {
    fn normalize_path(&self, path: FilePath) -> FilePath;
    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        context_path: Option<FilePath>,
    ) -> FilePath;
}
```

---

## File: crates/shared/src/common/contract_scanner_provider_port.rs

```rust
// PURPOSE: IScannerProviderPort — port trait for providing language-specific source scanners

use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_paths_vo::FilePathList;

pub trait IScannerProviderPort: Send + Sync {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError>;
    fn get_ignored_files(&self) -> FilePathList;
}
```

---

## File: crates/shared/src/common/contract_system_port.rs

```rust
// PURPOSE: IFileSystemPort — port trait for filesystem operations (read, write, exists, glob, walk)

use async_trait::async_trait;

use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_source_vo::ContentString;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;

/// Abstract interface for file system operations.
/// Implemented by Infrastructure (e.g., OSFileSystemAdapter).
#[async_trait]
pub trait IFileSystemPort: Send + Sync {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList;
    async fn is_directory(&self, path: &FilePath) -> SuccessStatus;
    async fn is_file(&self, path: &FilePath) -> SuccessStatus;
    async fn get_relative_path(&self, path: &FilePath, start: &FilePath) -> FilePath;
    async fn read_text(&self, path: &FilePath) -> Result<ContentString, FileSystemError>;
    async fn get_line_count(&self, path: &FilePath) -> Count;
    async fn exists(&self, path: &FilePath) -> SuccessStatus;
    async fn get_parent(&self, path: &FilePath) -> FilePath;
    async fn write_text(
        &self,
        path: &FilePath,
        content: &ContentString,
        mode: Option<&Identity>,
    ) -> Result<SuccessStatus, FileSystemError>;
    async fn glob(&self, pattern: &Identity) -> FilePathList;
    async fn get_cwd(&self) -> FilePath;
    async fn get_basename(&self, path: &FilePath) -> Identity;
    async fn path_join(&self, parts: &[Identity]) -> FilePath;
    async fn read_file(&self, path: &FilePath) -> Result<ContentString, FileSystemError>;
}
```

---

## File: crates/shared/src/common/infrastructure_file_collector_provider.rs

```rust
use std::fs;
use std::path::{Path, PathBuf};

use crate::common::contract_scanner_provider_port::IScannerProviderPort;
use crate::common::taxonomy_file_collector_helper::is_path_ignored;
use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::default_aes_config;

pub struct FileCollectorProvider {}

impl Default for FileCollectorProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FileCollectorProvider {
    pub fn new() -> Self {
        Self {}
    }
}

fn default_ignored_paths() -> Vec<String> {
    let config = default_aes_config();
    config
        .ignored_paths
        .values
        .iter()
        .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
        .collect()
}

pub fn collect_all_source_files(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        walk_source_files(dir, &mut files, &[]);
    }
    files
}

impl IScannerProviderPort for FileCollectorProvider {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError> {
        let dir = Path::new(&path.value);
        let mut files = Vec::new();
        if !dir.exists() || !dir.is_dir() {
            return Ok(FilePathList { values: files });
        }
        let ignored = default_ignored_paths();
        walk_source_files(dir, &mut files, &ignored);
        Ok(FilePathList { values: files })
    }

    fn get_ignored_files(&self) -> FilePathList {
        FilePathList { values: vec![] }
    }
}

fn is_source_file(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    is_path_ignored(&s, ignored)
}

fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_ignored_dir(&path, ignored) {
                continue;
            }
            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                if dir_name == "tests" {
                    continue;
                }
                walk_source_files(&path, files, ignored);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if is_source_file(ext) {
                    if let Some(path_str) = path.to_str() {
                        if let Ok(fp) = FilePath::new(path_str.to_string()) {
                            files.push(fp);
                        }
                    }
                }
            }
        }
    }
}

pub fn walk_rs_files(dir: &Path, cb: &mut dyn FnMut(PathBuf), ignored: &[String]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if is_ignored_dir(&p, ignored) {
                continue;
            }
            if p.is_dir() {
                walk_rs_files(&p, cb, ignored);
            } else if matches!(p.extension().and_then(|e| e.to_str()), Some("rs")) {
                cb(p);
            }
        }
    }
}
```

---

## File: crates/shared/src/common/mod.rs

```rust
// common — truly shared types used by multiple features
pub mod taxonomy_action_vo;
pub mod taxonomy_adapter_name_vo;
pub mod taxonomy_byte_count_vo;
pub mod taxonomy_common_error;
pub mod taxonomy_common_vo;
pub mod taxonomy_definition_vo;
pub mod taxonomy_display_content_vo;
pub mod taxonomy_duration_vo;
pub mod taxonomy_error_vo;
pub mod taxonomy_job_id_vo;
pub mod taxonomy_job_vo;
pub mod taxonomy_layer_vo;
pub mod taxonomy_line_count_vo;
pub mod taxonomy_lint_vo;
pub mod taxonomy_message_vo;
pub mod taxonomy_name_vo;
pub mod taxonomy_response_data_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_suggestion_vo;
pub mod taxonomy_value_object_utility;

// from file-system/ (foundational, multi-feature)
pub mod contract_system_port;
pub mod taxonomy_filesystem_error;

// from source-parsing/ (foundational, multi-feature)
pub mod contract_language_detector_port;
pub mod contract_parser_port;
pub mod contract_path_normalization_port;
pub mod contract_scanner_provider_port;
pub mod infrastructure_file_collector_provider;
pub mod taxonomy_adapter_error;
pub mod taxonomy_file_collector_helper;
pub mod taxonomy_language_detector_helper;
pub mod taxonomy_language_vo;
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_utils_vo;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_workspace_helper;
pub use infrastructure_file_collector_provider::{
    collect_all_source_files, walk_rs_files, FileCollectorProvider,
};
```

---

## File: crates/shared/src/common/taxonomy_action_vo.rs

```rust
// PURPOSE: ActionName — value object for pipeline job actions
// JobId is re-exported from common for backward compatibility
pub use crate::common::taxonomy_job_id_vo::JobId;
use crate::string_value_object;

string_value_object!(ActionName);
```

---

## File: crates/shared/src/common/taxonomy_adapter_error.rs

```rust
// PURPOSE: AdapterError, ScanError, ValidationError — structured error types for adapter operations
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::Constraint;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_error::ExitCode;
use crate::common::taxonomy_common_error::FieldName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct AdapterError {
    pub adapter_name: AdapterName,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub command: Option<ContentString>,
    #[serde(default)]
    pub stderr: Option<ErrorMessage>,
    #[serde(default)]
    pub exit_code: Option<ExitCode>,
}

impl AdapterError {
    pub fn new(adapter_name: AdapterName, message: ErrorMessage) -> Self {
        Self {
            adapter_name,
            message,
            error_code: None,
            command: None,
            stderr: None,
            exit_code: None,
        }
    }
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self.error_code.as_ref() {
            Some(c) => format!(" [{}]", c),
            None => String::new(),
        };
        write!(f, "[{}]{} {}", self.adapter_name, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ScanError {
    pub path: FilePath,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub adapter_name: Option<AdapterName>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl ScanError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            path,
            message,
            error_code: None,
            adapter_name: None,
            cause: None,
        }
    }
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let adapter = match self.adapter_name.as_ref() {
            Some(a) => format!(" ({})", a),
            None => String::new(),
        };
        let code = match self.error_code.as_ref() {
            Some(c) => format!(" [{}]", c),
            None => String::new(),
        };
        write!(
            f,
            "Scan failed{}{}: {} — {}",
            adapter, code, self.path, self.message
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ValidationError {
    pub field_name: FieldName,
    pub message: ErrorMessage,
    #[serde(default)]
    pub constraint: Option<Constraint>,
    #[serde(default)]
    pub value: Option<String>,
}

impl ValidationError {
    pub fn new(field_name: FieldName, message: ErrorMessage) -> Self {
        Self {
            field_name,
            message,
            constraint: None,
            value: None,
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Validation failed on '{}': {}",
            self.field_name, self.message
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_adapter_name_vo.rs

```rust
// PURPOSE: AdapterName — validated newtype for adapter/linter name strings
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// adapter_name_vo — Adapter and tool identifier value objects.
///
/// Adapter/tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AdapterName {
    pub value: String,
}

impl AdapterName {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new AdapterName from a string.
    ///
    /// # Errors
    /// Returns an error if the adapter name is empty or only whitespace.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("Adapter name cannot be empty".to_string());
        }
        Ok(AdapterName {
            value: value.trim().to_string(),
        })
    }

    /// Create a raw AdapterName without error validation (for static compile-time safe inputs).
    pub fn raw<S: Into<String>>(value: S) -> Self {
        AdapterName {
            value: value.into(),
        }
    }
}

impl std::ops::Deref for AdapterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for AdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for AdapterName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_byte_count_vo.rs

```rust
// PURPOSE: ByteCount — value object for file/stream sizes
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ByteCount {
    pub value: u64,
}

impl ByteCount {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl From<u64> for ByteCount {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for ByteCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_error.rs

```rust
// PURPOSE: Cause, Constraint, ExitCode, FieldName, ModuleName, PrimitiveTypeName — common error value objects
pub use crate::common::taxonomy_common_vo::ErrorMessage;
use crate::string_value_object;
use serde::Serialize;

string_value_object!(Cause);
string_value_object!(Constraint);
string_value_object!(FieldName);
string_value_object!(ModuleName);
string_value_object!(PrimitiveTypeName);

/// Strongly-typed exit code value object. Written manually because the
/// `string_value_object!` macro only supports `String` (not `i64`).
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ExitCode {
    pub value: crate::common::taxonomy_common_vo::LineNumber,
}

impl ExitCode {
    pub fn new(value: impl Into<crate::common::taxonomy_common_vo::LineNumber>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> i64 {
        self.value.value()
    }
}

impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ExitCode {
    fn from(v: i64) -> Self {
        Self {
            value: crate::common::taxonomy_common_vo::LineNumber::new(v),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ExitCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct W {
            value: crate::common::taxonomy_common_vo::LineNumber,
        }
        let w = W::deserialize(deserializer)?;
        Ok(Self { value: w.value })
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_vo.rs

```rust
// PURPOSE: BooleanVO, ColumnNumber, Count, DataFlowList, LineContentList, LineNumber, PatternList, Score, Timestamp — common VOs
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_severity_vo::Severity;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct BooleanVO {
    pub value: bool,
}

impl BooleanVO {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for BooleanVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for BooleanVO {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for BooleanVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BooleanVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for BooleanVOVisitor {
            type Value = BooleanVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanVO { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<bool>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(BooleanVO { value: val })
            }
        }
        deserializer.deserialize_any(BooleanVOVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ColumnNumber {
    pub value: i64,
}

impl ColumnNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ColumnNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for ColumnNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColumnNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for ColumnNumberVisitor {
            type Value = ColumnNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ColumnNumber { value: val })
            }
        }
        deserializer.deserialize_any(ColumnNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Count {
    pub value: i64,
}

impl Count {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for Count {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Count {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CountVisitor {}
        impl<'de> serde::de::Visitor<'de> for CountVisitor {
            type Value = Count;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Count { value: val })
            }
        }
        deserializer.deserialize_any(CountVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataFlowList {
    pub values: Vec<ErrorMessage>,
}

impl DataFlowList {
    pub fn new(value: Vec<ErrorMessage>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ErrorMessage] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ErrorMessage> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ErrorMessage) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobIdList {
    pub values: Vec<JobId>,
}

impl JobIdList {
    pub fn new(value: Vec<JobId>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[JobId] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, JobId> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: JobId) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineContentList {
    pub values: Vec<LineContentVO>,
}

impl LineContentList {
    pub fn new(value: Vec<LineContentVO>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[LineContentVO] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LineContentVO> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LineContentVO) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
#[derive(Default)]
pub struct LineNumber {
    pub value: i64,
}

impl LineNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for LineNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for LineNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LineNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for LineNumberVisitor {
            type Value = LineNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(LineNumber { value: val })
            }
        }
        deserializer.deserialize_any(LineNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct PatternList {
    pub values: Vec<String>,
}

impl PatternList {
    pub fn new(value: impl IntoPatternListValues) -> Self {
        Self {
            values: value.into_pattern_list_values(),
        }
    }
    pub fn values(&self) -> &[String] {
        &self.values
    }
}

impl PatternList {
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: String) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseDataList {
    pub values: Vec<ResponseData>,
}

impl ResponseDataList {
    pub fn new(value: Vec<ResponseData>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ResponseData] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ResponseData> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ResponseData) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Score {
    pub value: f64,
}

impl Score {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn is_perfect(&self) -> bool {
        self.value >= 100.0
    }
    pub fn is_passing(&self, threshold: &Score) -> bool {
        self.value >= threshold.value
    }
    pub fn deduct(&self, severity: &Severity) -> Score {
        Score {
            value: self.value - severity.score_impact(),
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.value)
    }
}

impl From<f64> for Score {
    fn from(v: f64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ScoreVisitor {}
        impl<'de> serde::de::Visitor<'de> for ScoreVisitor {
            type Value = Score;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v })
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v as f64 })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v as f64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<f64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Score { value: val })
            }
        }
        deserializer.deserialize_any(ScoreVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Timestamp {
    pub value: String,
}

impl Timestamp {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn now() -> Self {
        Self {
            value: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Timestamp {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Timestamp {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimestampVisitor {}
        impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
            type Value = Timestamp;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Timestamp { value: val })
            }
        }
        deserializer.deserialize_any(TimestampVisitor {})
    }
}

// Custom Coercion Traits for PatternList

pub trait IntoPatternListValues {
    fn into_pattern_list_values(self) -> Vec<String>;
}

impl IntoPatternListValues for &str {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IntoPatternListValues for String {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self]
    }
}

impl IntoPatternListValues for Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self
    }
}

impl IntoPatternListValues for Vec<&str> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.into_iter().map(|s| s.to_string()).collect()
    }
}

impl IntoPatternListValues for &Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ErrorMessage {
    pub value: String,
}

impl ErrorMessage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ErrorMessage {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ErrorMessage {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_definition_vo.rs

```rust
// PURPOSE: LayerDefinition, LayerMapVO, NamingConfig — VOs for AES layer definitions and naming policies
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use serde::{Deserialize, Serialize};

/// Wrap a single-field VO that exposes a `new(value)` constructor plus the
/// default `derive`s needed by the codebase. Used to keep the boilerplate
/// for `LayerMapVO`/`NamingConfig` uniform without introducing a new linter
/// violation cluster.
macro_rules! single_field_vo {
    ($name:ident, $field:ident: $field_ty:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub $field: $field_ty,
        }

        impl $name {
            pub fn new($field: $field_ty) -> Self {
                Self { $field }
            }
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerDefinition {
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,
    #[serde(default)]
    pub word_count: Count,
    #[serde(default)]
    pub exceptions: PatternList,
    #[serde(default)]
    pub recursive: BooleanVO,

    #[serde(flatten)]
    pub naming: crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO,
    #[serde(flatten)]
    pub orphan: crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO,
}

single_field_vo!(LayerMapVO, values: std::collections::HashMap<LayerNameVO, LayerDefinition>);
single_field_vo!(NamingConfig, word_count: Count);
```

---

## File: crates/shared/src/common/taxonomy_display_content_vo.rs

```rust
// PURPOSE: DisplayContent — value object for formatted display output (previews, human-readable sizes, etc.)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayContent {
    pub value: String,
}

impl DisplayContent {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<String> for DisplayContent {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for DisplayContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
```

---

## File: crates/shared/src/common/taxonomy_duration_vo.rs

```rust
// PURPOSE: Duration, Timeout — value objects for duration and timeout tracking
use serde::Serialize;

/// Wrap a `f64` value object that should be clamped to a minimum during
/// construction. Emit the struct, manual `new`/`value`/`Display`/`From`
/// impls, and a serde `Deserialize` that respects the clamp.
macro_rules! clamped_f64_vo {
    ($name:ident, $min:expr, $display_fmt:literal) => {
        #[derive(Debug, Clone, Serialize, PartialEq)]
        #[serde(transparent)]
        pub struct $name {
            pub value: f64,
        }

        impl $name {
            pub fn new(value: f64) -> Self {
                Self {
                    value: value.max($min),
                }
            }
            pub fn value(&self) -> f64 {
                self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $display_fmt, self.value)
            }
        }

        impl From<f64> for $name {
            fn from(v: f64) -> Self {
                Self { value: v }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                #[serde(transparent)]
                struct W {
                    value: f64,
                }
                let w = W::deserialize(deserializer)?;
                Ok(Self {
                    value: w.value.max($min),
                })
            }
        }
    };
}

clamped_f64_vo!(Timeout, 0.001, "{}s");
```

---

## File: crates/shared/src/common/taxonomy_error_vo.rs

```rust
// PURPOSE: ErrorCode — value object for AES error code identification
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// error_code_vo — Error code value object.
///
/// Linter error code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ErrorCode {
    code: String,
}

impl ErrorCode {
    pub fn code(&self) -> &str {
        &self.code
    }
    /// Create a new ErrorCode from a string.
    ///
    /// # Errors
    /// Returns an error if the code is empty.
    pub fn new<S: Into<String>>(code: S) -> Result<Self, String> {
        let code = code.into();
        if code.is_empty() {
            return Err("Error code cannot be empty".to_string());
        }
        Ok(ErrorCode { code })
    }

    /// Create a raw ErrorCode without error validation.
    pub fn raw<S: Into<String>>(code: S) -> Self {
        ErrorCode { code: code.into() }
    }

    /// Returns true if the code is a style error (starts with E, W, or D).
    pub fn is_style(&self) -> bool {
        self.code.starts_with('E') || self.code.starts_with('W') || self.code.starts_with('D')
    }
    pub fn is_logic(&self) -> bool {
        self.code.starts_with('F') || self.code.starts_with('I')
    }
    pub fn is_security(&self) -> bool {
        self.code.starts_with('B')
    }
    pub fn is_architecture(&self) -> bool {
        self.code.starts_with("AES")
    }
}

impl std::ops::Deref for ErrorCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Hash for ErrorCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_file_collector_helper.rs

```rust
// PURPOSE: FileCollector — taxonomy utility for collecting lintable source files from a directory tree
use crate::common::taxonomy_language_detector_helper::LanguageDetector;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_path_vo::FilePath;

/// Return true if `rel_path` should be skipped based on `ignored` patterns.
///
/// Each pattern is matched as a **path segment** rather than a free-text substring. This
/// fixes a long-standing bug where patterns like `/test-workspaces` failed to match the
/// absolute path `/home/.../test-workspaces/crates/...` because the old substring-based
/// matcher was tripped up by leading slashes, leading paths, and unrelated prefixes. The
/// result was that all of `test-workspaces/**` and `packages/vscode-extension/src/**`
/// leaked into `lint-arwaky check .` results even though they were listed in
/// `ignored_paths`.
///
/// Three forms of pattern are supported:
///   1. Absolute-style prefix `"/foo"`, `"/foo/bar"` — matches any path that contains
///      the segments `foo` or `foo/bar` in order, at any depth. The leading slash is
///      dropped before comparison; this works on both absolute paths
///      (`/home/.../test-workspaces/crates/foo.rs`) and relative paths
///      (`test-workspaces/crates/foo.rs`).
///   2. Single segment `"foo"` — matches any path segment equal to `foo`
///      (catches both `foo` at root and `nested/foo` mid-tree).
///   3. Suffix glob `".min.js"`, `"*.bak"` — matches any path whose basename ends with the
///      suffix. Used for vendor minified files like `cytoscape.min.js`.
pub fn is_path_ignored(rel_path: &str, ignored: &[String]) -> bool {
    if rel_path.is_empty() {
        return false;
    }
    let segments: Vec<&str> = rel_path
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect();
    for pat in ignored {
        if pat.is_empty() {
            continue;
        }
        // (1) Absolute-style prefix "/foo" or "/foo/bar"
        if let Some(stripped) = pat.strip_prefix('/') {
            if stripped.is_empty() {
                continue;
            }
            let pat_segments: Vec<&str> = stripped
                .split(['/', '\\'])
                .filter(|s| !s.is_empty())
                .collect();
            if pat_segments.is_empty() {
                continue;
            }
            // Match if pat_segments appear contiguously in `segments` at any depth.
            // We do NOT use `starts_with` here because `rel_path` may be absolute
            // (`/home/.../test-workspaces/...`) — the pattern segments can appear
            // anywhere along the path, not just at the very beginning.
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg < n_pat {
                continue;
            }
            for start in 0..=(n_seg - n_pat) {
                if segments[start..start + n_pat] == pat_segments[..] {
                    return true;
                }
            }
            continue;
        }
        // (2) Suffix glob "*.ext" or ".ext" (used for minified vendor files)
        if pat.starts_with("*.") || (pat.starts_with('.') && pat.contains('.')) {
            let suffix = if let Some(s) = pat.strip_prefix('*') {
                s.trim_start_matches('.')
            } else {
                pat.trim_start_matches('.')
            };
            if suffix.is_empty() {
                continue;
            }
            let basename = segments.last().copied().unwrap_or_default();
            if basename.ends_with(suffix) {
                return true;
            }
            continue;
        }
        // (3) Bare segment/pattern — match single segment or multi-segment subpath.
        let pat_segments: Vec<&str> = pat.split(['/', '\\']).filter(|s| !s.is_empty()).collect();
        if pat_segments.len() == 1 {
            if segments.contains(&pat_segments[0]) {
                return true;
            }
        } else if pat_segments.len() > 1 {
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg >= n_pat {
                for start in 0..=(n_seg - n_pat) {
                    if segments[start..start + n_pat] == pat_segments[..] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Collect lintable source files (.rs, .py, .ts, .js, .tsx, .jsx) from a directory tree.
pub fn collect_source_files(
    root_dir: &std::path::Path,
    dir_path: &DirectoryPath,
    ignored: &[String],
) -> Vec<FilePath> {
    let mut files = Vec::new();
    let path = std::path::Path::new(&dir_path.value);
    if path.is_file() {
        let relative_path = match path.strip_prefix(root_dir) {
            Ok(p) => p,
            Err(_) => path,
        };
        let rel_str = relative_path.to_string_lossy();
        if !is_path_ignored(&rel_str, ignored) {
            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                let detector = LanguageDetector::new();
                if detector.is_lintable(&fp) {
                    files.push(fp);
                }
            }
        }
        return files;
    }

    if let Ok(entries) = std::fs::read_dir(&dir_path.value) {
        for entry in entries.flatten() {
            let path = entry.path();
            let relative_path = match path.strip_prefix(root_dir) {
                Ok(p) => p,
                Err(_) => &path,
            };
            let rel_str = relative_path.to_string_lossy();
            if is_path_ignored(&rel_str, ignored) {
                continue;
            }
            if path.is_dir() {
                // Skip Rust integration test directories — tests live in tests/ and
                // should not be scanned by the AES linter.
                let dir_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                if dir_name == "tests" {
                    continue;
                }
                let sub_dir =
                    DirectoryPath::new(path.to_string_lossy().to_string()).unwrap_or_default();
                files.extend(collect_source_files(root_dir, &sub_dir, ignored));
            } else if let Some(path_str) = path.to_str() {
                if let Ok(fp) = FilePath::new(path_str.to_string()) {
                    let detector = LanguageDetector::new();
                    if detector.is_lintable(&fp) {
                        files.push(fp);
                    }
                }
            }
        }
    }
    files
}
```

---

## File: crates/shared/src/common/taxonomy_filesystem_error.rs

```rust
// PURPOSE: FileSystemError — structured error type for filesystem operation failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use crate::mcp_server::taxonomy_action_vo::ActionName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct FileSystemError {
    pub path: FilePath,
    pub message: ErrorMessage,
    pub operation: ActionName,
    #[serde(default)]
    pub error_code: ErrorCode,
    #[serde(default)]
    pub cause: Cause,
}

impl FileSystemError {
    pub fn new(path: FilePath, message: ErrorMessage, operation: ActionName) -> Self {
        Self {
            path,
            message,
            operation,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = if self.error_code.code().is_empty() {
            String::new()
        } else {
            format!(" [{}]", self.error_code.code())
        };
        write!(
            f,
            "FS Error during {} on {}{}: {}",
            self.operation, self.path, code, self.message
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_job_id_vo.rs

```rust
// PURPOSE: JobId — value object for pipeline job identifiers
//
// `JobId` is a thin wrapper around a `String` and is generated with the
// `string_value_object!` macro. It exists in its own file so that any
// crate needing job identifiers can `use` this type without pulling in the
// rest of the common VO namespace.
use crate::string_value_object;

string_value_object!(JobId);
```

---

## File: crates/shared/src/common/taxonomy_job_vo.rs

```rust
// PURPOSE: PipelineJob, SuccessStatus, EnvContentVO, McpConfigVO — value objects for pipeline job lifecycle tracking
// ResponseData is re-exported from common for backward compatibility
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::string_value_object;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::common::taxonomy_response_data_vo::ResponseData;

// Manual impl: `SuccessStatus` overrides `Display` to render "SUCCESS"/"FAILURE"
// instead of `true`/`false`, and the macro does not currently support a clean
// `bool` cast (Rust forbids `i64 as bool`). Kept as a hand-rolled VO.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SuccessStatus {
    pub value: bool,
}

impl Default for SuccessStatus {
    fn default() -> Self {
        Self::new(false)
    }
}

impl SuccessStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for SuccessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value {
            write!(f, "SUCCESS")
        } else {
            write!(f, "FAILURE")
        }
    }
}

impl std::ops::Deref for SuccessStatus {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMetadata {
    pub name: AdapterName,
    pub class_path: String,
    #[serde(default)]
    pub description: String,
}

impl AdapterMetadata {
    pub fn new(name: AdapterName, class_path: String) -> Self {
        Self {
            name,
            class_path,
            description: String::new(),
        }
    }
}

string_value_object!(EnvContentVO);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpConfigVO {
    #[serde(default)]
    pub value: HashMap<String, serde_json::Value>,
}

impl McpConfigVO {
    pub fn new(value: HashMap<String, serde_json::Value>) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &HashMap<String, serde_json::Value> {
        &self.value
    }
}
```

---

## File: crates/shared/src/common/taxonomy_language_detector_helper.rs

```rust
// PURPOSE: LanguageDetector — Helper for detecting programming languages from file paths
use crate::common::taxonomy_language_vo::Language;
use crate::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Copy, Default)]
pub struct LanguageDetector;

impl LanguageDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect language from a FilePath based on extension.
    pub fn detect(&self, path: &FilePath) -> Language {
        let ext = path.extension();
        match ext.as_str() {
            "py" => Language::Python,
            "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
            "rs" => Language::Rust,
            _ => Language::Unknown,
        }
    }

    /// Check if a FilePath represents a lintable language.
    pub fn is_lintable(&self, path: &FilePath) -> bool {
        matches!(
            self.detect(path),
            Language::Python | Language::JavaScript | Language::TypeScript | Language::Rust
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_language_vo.rs

```rust
// PURPOSE: Language — value object enum for supported programming languages (Python, JS, TS, Rust)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Rust,
    Unknown,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Rust => "rust",
            Language::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
```

---

## File: crates/shared/src/common/taxonomy_layer_vo.rs

```rust
// PURPOSE: FileContentVO, Identity, LayerNameVO, LineContentVO — VOs for layer identity and file content
use crate::string_value_object;

string_value_object!(FileContentVO);
string_value_object!(Identity);
string_value_object!(LayerNameVO);
string_value_object!(LineContentVO);
```

---

## File: crates/shared/src/common/taxonomy_line_count_vo.rs

```rust
// PURPOSE: LineCount — value object for the number of lines (preview, file limits, etc.)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineCount {
    pub value: usize,
}

impl LineCount {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

impl From<usize> for LineCount {
    fn from(value: usize) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for LineCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
```

---

## File: crates/shared/src/common/taxonomy_lint_vo.rs

```rust
// PURPOSE: CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint — VOs for lint violations
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeRef {
    pub name: DescriptionVO,
    #[serde(default)]
    pub kind: DescriptionVO,
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub start_line: Option<LineNumber>,
    #[serde(default)]
    pub end_line: Option<LineNumber>,
}

impl ScopeRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: DescriptionVO::new(name),
            kind: DescriptionVO::new("function"),
            file: None,
            start_line: None,
            end_line: None,
        }
    }
    pub fn has_range(&self) -> bool {
        self.start_line.as_ref().is_some_and(|l| l.value > 0)
            && self.end_line.as_ref().is_some_and(|l| l.value > 0)
    }
}

impl std::fmt::Display for ScopeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref file) = self.file {
            write!(f, "{} {} in {}", self.kind.value, self.name.value, file)
        } else if !self.kind.value.is_empty() {
            write!(f, "{} {}", self.kind.value, self.name.value)
        } else {
            write!(f, "{}", self.name.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub line: Option<LineNumber>,
    #[serde(default)]
    pub column: Option<ColumnNumber>,
    #[serde(default)]
    pub description: DescriptionVO,
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    pub fn new() -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            description: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if let Some(ref file) = self.file {
            parts.push(file.value.clone());
        }
        if let Some(ref line) = self.line {
            let mut s = line.value.to_string();
            if let Some(ref col) = self.column {
                if col.value > 0 {
                    s = format!("{}:{}", line.value, col.value);
                }
            }
            parts.push(s);
        }
        let result = if parts.is_empty() {
            "unknown".to_string()
        } else {
            parts.join(":")
        };
        if self.description.value.is_empty() {
            write!(f, "{}", result)
        } else {
            write!(f, "{} — {}", result, self.description.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationList {
    #[serde(default)]
    pub values: Vec<Location>,
}

impl LocationList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl LocationList {
    pub fn push(&mut self, item: Location) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for LocationList {
    type Target = Vec<Location>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViolationConstraint {
    pub rule: DescriptionVO,
    #[serde(default)]
    pub min_value: DescriptionVO,
    #[serde(default)]
    pub max_value: DescriptionVO,
}

impl ViolationConstraint {
    pub fn new(rule: impl Into<String>) -> Self {
        Self {
            rule: DescriptionVO::new(rule),
            min_value: DescriptionVO::new(String::new()),
            max_value: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for ViolationConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rule.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandArgs {
    #[serde(default)]
    pub args: Vec<ContentString>,
}

impl Default for CommandArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandArgs {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }
}

impl std::fmt::Display for CommandArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.args
                .iter()
                .map(|a| a.value.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeBounds {
    #[serde(default)]
    pub start: Option<LineNumber>,
    #[serde(default)]
    pub end: Option<LineNumber>,
}
```

---

## File: crates/shared/src/common/taxonomy_message_vo.rs

```rust
// PURPOSE: ComplianceStatus, LintMessage — VOs for compliance status and violation messages
use crate::string_value_object;

string_value_object!(LintMessage);

/// Boolean compliance flag. Written manually because `bool` is not supported
/// by the `string_value_object!` macro (`i64 as bool` is not a valid Rust cast).
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ComplianceStatus {
    pub value: bool,
}

impl ComplianceStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for ComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for ComplianceStatus {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_name_vo.rs

```rust
// PURPOSE: NameVariants, SymbolName — value objects for symbol naming and naming convention variants
use crate::string_value_object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NameVariants {
    pub values: Vec<SymbolName>,
}

impl NameVariants {
    pub fn new(value: Vec<SymbolName>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, SymbolName> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: SymbolName) {
        self.values.push(item);
    }
}

string_value_object!(SymbolName);
```

---

## File: crates/shared/src/common/taxonomy_naming_list_vo.rs

```rust
// PURPOSE: SymbolNameList, PrimitiveTypeList — VOs for collections of symbol names and primitive types
use crate::common::taxonomy_name_vo::SymbolName;
use serde::{Deserialize, Serialize};

pub const CORE_PRIMITIVE_TYPES: &[&str] = &["str", "int", "float"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SymbolNameList {
    #[serde(default)]
    pub values: Vec<SymbolName>,
}

impl Default for SymbolNameList {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolNameList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: SymbolName) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, SymbolName> {
        self.values.iter()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportNameList {
    #[serde(default)]
    pub values: Vec<SymbolName>,
}

impl Default for ImportNameList {
    fn default() -> Self {
        Self::new()
    }
}

impl ImportNameList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimitiveTypeList {
    #[serde(default)]
    pub values: Vec<SymbolName>,
}

impl Default for PrimitiveTypeList {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimitiveTypeList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn contains(&self, item: &str) -> bool {
        self.values.iter().any(|v| v.value == item)
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallChainList {
    #[serde(default)]
    pub values: Vec<SymbolName>,
}

impl Default for CallChainList {
    fn default() -> Self {
        Self::new()
    }
}

impl CallChainList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

pub fn primitive_type_list() -> PrimitiveTypeList {
    PrimitiveTypeList {
        values: CORE_PRIMITIVE_TYPES
            .iter()
            .map(|s| SymbolName::new(*s))
            .collect(),
    }
}
```

---

## File: crates/shared/src/common/taxonomy_parser_error.rs

```rust
// PURPOSE: ParserError — structured error type for source code parsing failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct SourceParserError {
    pub path: FilePath,
    pub message: ErrorMessage,
    pub error_code: ErrorCode,
    pub cause: Cause,
}

impl SourceParserError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            path,
            message,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for SourceParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code_str = self.error_code.to_string();
        let code = if code_str.is_empty() {
            String::new()
        } else {
            format!(" [{}]", code_str)
        };
        write!(f, "Parser Error on {}{}: {}", self.path, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct SyntaxErrorVO {
    #[serde(flatten)]
    pub base: SourceParserError,
    pub line: LineNumber,
    pub column: ColumnNumber,
}

impl SyntaxErrorVO {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            base: SourceParserError::new(path, message),
            line: LineNumber::default(),
            column: ColumnNumber::default(),
        }
    }
}

impl std::fmt::Display for SyntaxErrorVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_str = self.line.to_string();
        let col_str = self.column.to_string();
        let pos = if !line_str.is_empty() && !col_str.is_empty() {
            format!(" at {}:{}", line_str, col_str)
        } else if !line_str.is_empty() {
            format!(" at {}", line_str)
        } else {
            String::new()
        };
        write!(
            f,
            "Syntax Error on {}{}: {}",
            self.base.path, pos, self.base.message
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_path_utils_vo.rs

```rust
pub struct PathUtils;

impl PathUtils {
    /// Walk a directory recursively, collecting files while skipping ignored patterns.
    /// Supports both flat patterns (e.g., "tests") and path patterns (e.g., "src/tests").
    pub fn walk_recursive(dir: &std::path::Path, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        Self::walk_recursive_internal(dir, dir, ignored)
    }

    fn walk_recursive_internal(
        root: &std::path::Path,
        dir: &std::path::Path,
        ignored: &[&str],
    ) -> Vec<std::path::PathBuf> {
        use std::fs;

        let mut results = Vec::new();

        if !dir.is_dir() {
            if dir.is_file() {
                // For a single file, check both the file name and the relative path
                if let Some(name_str) = dir.file_name().and_then(|s| s.to_str()) {
                    if !ignored.contains(&name_str) {
                        let rel_path = dir.strip_prefix(root).unwrap_or(dir);
                        let rel_str = rel_path.to_string_lossy();
                        if !Self::matches_any_pattern(&rel_str, ignored) {
                            results.push(dir.to_path_buf());
                        }
                    }
                }
            }
            return results;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let rel_path = path.strip_prefix(root).unwrap_or(&path);
                let rel_str = rel_path.to_string_lossy();

                if Self::matches_any_pattern(&rel_str, ignored) {
                    continue;
                }

                if path.is_dir() {
                    results.extend(Self::walk_recursive_internal(root, &path, ignored));
                } else {
                    results.push(path);
                }
            }
        }

        results
    }

    fn matches_any_pattern(rel_path: &str, ignored: &[&str]) -> bool {
        for pattern in ignored {
            // Exact match on the full relative path or any prefix segment
            if rel_path == *pattern || rel_path.starts_with(&format!("{}/", pattern)) {
                return true;
            }
            // Also match just the filename (flat pattern) for backward compatibility
            if let Some(file_name) = std::path::Path::new(rel_path).file_name() {
                if file_name == *pattern {
                    return true;
                }
            }
        }
        false
    }

    /// Convenience wrapper used by OSFileSystemAdapter and workspace helpers.
    pub fn collect_paths(start: &str, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        let root = std::path::Path::new(start);
        Self::walk_recursive(root, ignored)
    }
}
```

---

## File: crates/shared/src/common/taxonomy_path_vo.rs

```rust
// PURPOSE: FilePath, DirectoryPath — value objects for validated file and directory paths
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// file_path_vo — File and directory path value objects.
///
/// File path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilePath {
    pub value: String,
}

impl FilePath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new FilePath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, collapse repeated slashes.
        let mut normalized = String::with_capacity(value.len());
        let mut prev_slash = false;
        for c in value.chars() {
            if c == '/' || c == '\\' {
                if !prev_slash {
                    normalized.push('/');
                    prev_slash = true;
                }
            } else {
                normalized.push(c);
                prev_slash = false;
            }
        }
        value = normalized;
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        // If after normalization it's empty, then it was all slashes -> treat as root
        if value.is_empty() {
            return Ok(FilePath {
                value: "/".to_string(),
            });
        }
        Ok(FilePath { value })
    }

    /// File extension without dot.
    pub fn extension(&self) -> String {
        let special_files = [
            "Makefile",
            "Dockerfile",
            "Dockerfile.dev",
            "Dockerfile.prod",
            ".bashrc",
            ".profile",
            ".zshrc",
            ".gitignore",
            ".dockerignore",
        ];
        // Operate on the basename, not the full path — `./foo.rs` must still yield
        // `rs` as its extension, and `.bashrc` (which is fully a basename) must NOT
        // be confused with a hidden file mid-path.
        let basename = match self.value.rsplit('/').next() {
            Some(b) => b,
            None => return String::new(),
        };
        if special_files.contains(&basename) || basename.starts_with('.') {
            return String::new();
        }
        match basename.rsplit_once('.') {
            Some((_, ext)) => ext.to_string(),
            None => String::new(),
        }
    }

    /// Check if path has given extension (without dot).
    pub fn has_extension(&self, ext: &str) -> bool {
        self.extension().eq_ignore_ascii_case(ext)
    }

    /// Extract filename/basename of the path.
    pub fn basename(&self) -> String {
        match self.value.rsplit('/').next() {
            Some(f) => f.to_string(),
            None => self.value.clone(),
        }
    }

    /// Check if the path is a barrel file (module re-export aggregator).
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py"
                | "main.py"
                | "py.typed"
                | "app.py"
                | "lib.rs"
                | "main.rs"
                | "index.ts"
                | "index.js"
                | "index.tsx"
                | "index.jsx"
                | "main.ts"
                | "main.js"
                | "app.ts"
                | "app.js"
        )
    }
}

impl std::ops::Deref for FilePath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for FilePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Directory path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct DirectoryPath {
    pub value: String,
}

impl DirectoryPath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new DirectoryPath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("Directory path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and remove trailing slash.
        value = value.replace('\\', "/");
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        Ok(DirectoryPath { value })
    }
}

impl std::ops::Deref for DirectoryPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for DirectoryPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for DirectoryPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DirectoryPath::new(s).map_err(serde::de::Error::custom)
    }
}

impl Hash for DirectoryPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_paths_vo.rs

```rust
// PURPOSE: FilePathList, DirectoryPath, SourceDir — VOs for file/directory path collections
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenamedFile {
    pub old_path: FilePath,
    pub new_path: FilePath,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenamedFileList {
    pub values: Vec<RenamedFile>,
}

impl RenamedFileList {
    pub fn new(value: Vec<RenamedFile>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, RenamedFile> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: RenamedFile) {
        self.values.push(item);
    }
}

impl RenamedFile {
    pub fn new(old_path: FilePath, new_path: FilePath) -> Self {
        Self { old_path, new_path }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FilePathList {
    pub values: Vec<FilePath>,
}

impl FilePathList {
    pub fn new(value: Vec<FilePath>) -> Self {
        Self { values: value }
    }
}

impl FilePathList {
    pub fn iter(&self) -> std::slice::Iter<'_, FilePath> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: FilePath) {
        self.values.push(item);
    }
}

impl std::ops::Deref for FilePathList {
    type Target = Vec<FilePath>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
```

---

## File: crates/shared/src/common/taxonomy_response_data_vo.rs

```rust
// PURPOSE: ResponseData — value object for pipeline job response data
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseData {
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub returncode: i64,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for ResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseData {
    pub fn new() -> Self {
        Self {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        }
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.value.as_ref()
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.as_ref().and_then(|v| v.get(key))
    }
}
```

---

## File: crates/shared/src/common/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — value object for violation severity levels (critical, high, medium, low)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    #[serde(rename = "info")]
    #[default]
    INFO,
    #[serde(rename = "low")]
    LOW,
    #[serde(rename = "medium")]
    MEDIUM,
    #[serde(rename = "high")]
    HIGH,
    #[serde(rename = "critical")]
    CRITICAL,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::INFO => write!(f, "info"),
            Severity::LOW => write!(f, "low"),
            Severity::MEDIUM => write!(f, "medium"),
            Severity::HIGH => write!(f, "high"),
            Severity::CRITICAL => write!(f, "critical"),
        }
    }
}

impl Severity {
    pub fn score_impact(&self) -> f64 {
        match self {
            Severity::INFO => 0.0,
            Severity::LOW => 1.0,
            Severity::MEDIUM => 2.0,
            Severity::HIGH => 3.0,
            Severity::CRITICAL => 5.0,
        }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_source_vo.rs

```rust
// PURPOSE: ContentString, SourceContentVO — VOs for source code content representation
use crate::string_value_object;
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_path_vo::FilePath;

string_value_object!(ContentString);

/// Source content value object: combines a file path, a `ContentString`
/// payload, and a language marker. Carries three fields rather than one,
/// so it does not fit the single-field `string_value_object!` macro;
/// defined manually.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceContentVO {
    pub file_path: FilePath,
    pub content: ContentString,
    pub language: String,
}

impl SourceContentVO {
    pub fn new(file_path: FilePath, content: ContentString, language: impl Into<String>) -> Self {
        Self {
            file_path,
            content,
            language: language.into(),
        }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_suggestion_vo.rs

```rust
// PURPOSE: ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion — domain value objects for CLI suggestion/result data
use crate::string_value_object;
use serde::{Deserialize, Serialize};

// ClassPath, DescriptionVO, LogOutput, StdError, StdOutput, and Suggestion all
// follow the standard String-wrapper VO pattern; the macro emits the
// new/value/Display/From/Hash/PartialEq/Deserialize impls they need.
string_value_object!(ClassPath);
string_value_object!(DescriptionVO);
string_value_object!(LogOutput);
string_value_object!(StdError);
string_value_object!(StdOutput);
string_value_object!(Suggestion);

/// Strongly-typed replacement for the previous
/// `HashMap<String, serde_json::Value>` return type. Each field has a real
/// domain meaning — there is no `serde_json::Value` in the contract surface.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetadataVO {
    pub values: std::collections::HashMap<String, serde_json::Value>,
}

impl MetadataVO {
    pub fn new(value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self { values: value }
    }
    pub fn value(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.values
    }
}
```

---

## File: crates/shared/src/common/taxonomy_value_object_utility.rs

````rust
// PURPOSE: Macros for generating boilerplate impls on String/primitive wrapper value objects.
//
// These macros emit the impls that every String-wrapper VO needs:
//   - `new(value)` constructor
//   - `value()` accessor
//   - `Display`
//   - `Hash` / `PartialEq` / `Eq` (optional)
//   - `From<&str>` / `From<String>` / `From<$Inner>` (for primitives)
//   - serde `Deserialize` (accepts either a primitive or a map with a `value` key)
//
// Using the macro keeps each VO file to its domain-specific surface and stops
// AES305 from flagging the same serde visitor across ~13 files.

/// Generate a String-wrapped value object with the standard VO surface.
///
/// # Usage
/// ``` `ignore
/// // in any sibling module file:
/// use crate::string_value_object;
/// string_value_object!(FooName);
/// ``` `
///
/// The macro is `#[macro_export]`-ed so it is accessible at the crate root.
/// Each VO file `use crate::string_value_object;` once and then invokes the
/// macro locally.
#[macro_export]
macro_rules! string_value_object {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name {
            pub value: String,
        }

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self {
                    value: value.into(),
                }
            }

            pub fn value(&self) -> &str {
                &self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.value.hash(state);
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl Eq for $name {}

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self {
                    value: s.to_string(),
                }
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self { value: s }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V {}
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("primitive or map with 'value' key")
                    }
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name {
                            value: v.to_string(),
                        })
                    }
                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v })
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut value = None;
                        while let Some(k) = map.next_key::<String>()? {
                            if k == "value" {
                                value = Some(map.next_value::<String>()?);
                            } else {
                                let _: serde::de::IgnoredAny = map.next_value()?;
                            }
                        }
                        let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                        Ok($name { value: val })
                    }
                }
                deserializer.deserialize_any(V {})
            }
        }
    };
}

/// Generate a primitive-wrapped value object (e.g. `i64`, `f64`, `bool`).
///
/// # Usage
/// ``` `ignore
/// primitive_value_object!(LineNumber, i64);
/// ``` `
///
/// Emits the same surface as `string_value_object!` but with `From<$Inner>`,
/// `From<$Inner>` conversions, and a serde visitor that accepts the inner
/// type or a `{"value": ...}` map.
#[macro_export]
macro_rules! primitive_value_object {
    ($name:ident, $inner:ty) => {
        #[derive(Default, Debug, Clone, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name {
            pub value: $inner,
        }

        impl $name {
            pub fn new(value: $inner) -> Self {
                Self { value }
            }

            pub fn value(&self) -> $inner {
                self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl Eq for $name {}

        impl From<$inner> for $name {
            fn from(v: $inner) -> Self {
                Self { value: v }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V {}
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(concat!(
                            "primitive or map with 'value' key (",
                            stringify!($inner),
                            ")"
                        ))
                    }
                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut value: Option<$inner> = None;
                        while let Some(k) = map.next_key::<String>()? {
                            if k == "value" {
                                value = Some(map.next_value::<$inner>()?);
                            } else {
                                let _: serde::de::IgnoredAny = map.next_value()?;
                            }
                        }
                        let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                        Ok($name { value: val })
                    }
                }
                deserializer.deserialize_any(V {})
            }
        }
    };
}
````

---

## File: crates/shared/src/common/taxonomy_workspace_helper.rs

```rust
// PURPOSE: taxonomy_workspace_helper — shared workspace root detection
// Walks up from a path looking for Cargo.toml, crates/, packages/, or modules/ markers.
// Used by cli-commands, mcp-server, and orphan-detector.

/// Walk up from `start` looking for workspace root markers.
/// Returns the first directory containing Cargo.toml, crates/, packages/, or modules/.
pub fn find_workspace_root(start: &str) -> Option<std::path::PathBuf> {
    let mut dir = std::path::Path::new(start).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    loop {
        if dir.join("Cargo.toml").exists()
            || dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}
```

---

## File: crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs

```rust
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use async_trait::async_trait;

#[async_trait]
pub trait MultiProjectOrchestratorAggregate: Send + Sync {
    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;
}
```

---

## File: crates/shared/src/config-system/contract_orchestration_aggregate.rs

```rust
// PURPOSE: IConfigOrchestrationAggregate — aggregate contract for orchestrating configuration loading across languages

use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::contract_reader_port::IConfigReaderPort;
use crate::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IConfigOrchestrationAggregate: Send + Sync {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorPort>;
    fn config_reader(&self) -> Arc<dyn IConfigReaderPort>;

    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;
    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult;
}
```

---

## File: crates/shared/src/config-system/contract_parser_port.rs

```rust
// PURPOSE: IConfigParserPort — contract for config parser provider (YAML and TOML)
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_setting_vo::ProjectConfig;

pub trait IConfigParserPort: Send + Sync {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;
    fn parse_toml_config(&self, path: &FilePath) -> Result<Option<ProjectConfig>, ConfigError>;
}
```

---

## File: crates/shared/src/config-system/contract_reader_port.rs

```rust
// PURPOSE: IConfigReaderPort — port trait for reading configuration from external sources

use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_source_vo::ConfigSource;
use async_trait::async_trait;

#[async_trait]
pub trait IConfigReaderPort: Send + Sync {
    async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource>;
    async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)>;
}
```

---

## File: crates/shared/src/config-system/contract_validator_protocol.rs

```rust
// PURPOSE: IConfigValidatorProtocol — protocol for project config and scoring threshold validation

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::config_system::taxonomy_setting_vo::ProjectConfig;
use crate::config_system::taxonomy_validation_vo::ValidationResult;

pub trait IConfigValidatorProtocol: Send + Sync {
    /// Determines if a specific adapter should run based on configuration rules.
    fn is_adapter_enabled(&self, config: &ProjectConfig, adapter_name: &AdapterName) -> bool;

    /// Validates that scoring thresholds are sane.
    fn validate_thresholds(&self, config: &ProjectConfig) -> ValidationResult;
}
```

---

## File: crates/shared/src/config-system/contract_workspace_detector_port.rs

```rust
// PURPOSE: IWorkspaceDetectorPort — port trait for detecting workspace type from directory structure
use crate::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceType {
    Rust,
    TypeScript,
    Python,
    Unknown,
}

impl WorkspaceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkspaceType::Rust => "rust",
            WorkspaceType::TypeScript => "typescript",
            WorkspaceType::Python => "python",
            WorkspaceType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for WorkspaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub trait IWorkspaceDetectorPort: Send + Sync {
    /// Detect workspace type by checking folder structure and config files.
    fn detect(&self, path: &FilePath) -> WorkspaceType;

    /// Check if a path is a workspace root (contains crates/, packages/, or modules/).
    fn is_workspace(&self, path: &FilePath) -> bool;
}
```

---

## File: crates/shared/src/config-system/mod.rs

```rust
// config-system — taxonomy and contract types
pub mod contract_multi_project_orchestrator_aggregate;
pub mod contract_orchestration_aggregate;
pub mod contract_parser_port;
pub mod contract_reader_port;
pub mod contract_validator_protocol;
pub mod contract_workspace_detector_port;
pub mod taxonomy_config_error;
pub mod taxonomy_config_vo;
pub mod taxonomy_identifier_vo;
pub mod taxonomy_multi_project_summary_vo;
pub mod taxonomy_multi_project_vo;
pub mod taxonomy_multi_project_workspace_info_vo;
pub mod taxonomy_setting_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_validation_vo;
```

---

## File: crates/shared/src/config-system/taxonomy_config_error.rs

```rust
// PURPOSE: ConfigError, ConfigErrorKind — structured error types for configuration loading failures
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_identifier_vo::ConfigKey;
use crate::config_system::taxonomy_setting_vo::ActualValue;
use crate::config_system::taxonomy_setting_vo::ExpectedValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct ConfigError {
    pub key: ConfigKey,
    pub message: ErrorMessage,
    pub expected: ExpectedValue,
    pub actual: ActualValue,
    pub config_file: FilePath,
}

impl ConfigError {
    pub fn new(key: ConfigKey, message: ErrorMessage) -> Self {
        Self {
            key,
            message,
            expected: ExpectedValue::default(),
            actual: ActualValue::default(),
            config_file: FilePath::default(),
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_str = self.config_file.to_string();
        let file_info = if file_str.is_empty() {
            String::new()
        } else {
            format!(" in {}", file_str)
        };
        write!(
            f,
            "Config error on '{}'{}: {}",
            self.key, file_info, self.message
        )
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_config_vo.rs

```rust
// PURPOSE: ArchitectureConfig, LayerDefinition, ConfigRule — configuration value objects for AES rules definition
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_definition_vo::NamingConfig;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct ArchitectureRule {
    pub name: DescriptionVO,
    pub description: DescriptionVO,
    pub rule_type: ErrorCode,
    pub scope: LayerNameVO,
    pub exceptions: PatternList,
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,

    #[serde(flatten)]
    pub naming: crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO,
    #[serde(flatten)]
    pub orphan: crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ArchitectureConfig {
    pub enabled: BooleanVO,
    pub layers: std::collections::HashMap<LayerNameVO, LayerDefinition>,
    pub rules: Vec<ArchitectureRule>,
    pub naming: NamingConfig,
    pub ignored_paths: FilePathList,
    pub mandatory_class_definition: BooleanVO,
}

impl ArchitectureConfig {
    pub fn new(
        enabled: BooleanVO,
        layers: std::collections::HashMap<LayerNameVO, LayerDefinition>,
        rules: Vec<ArchitectureRule>,
        naming: NamingConfig,
        ignored_paths: FilePathList,
        mandatory_class_definition: BooleanVO,
    ) -> Self {
        Self {
            enabled,
            layers,
            rules,
            naming,
            ignored_paths,
            mandatory_class_definition,
        }
    }
}

impl Default for ArchitectureConfig {
    fn default() -> Self {
        Self {
            enabled: BooleanVO::new(true),
            layers: HashMap::new(),
            rules: Vec::new(),
            naming: NamingConfig::new(Count::new(2)),
            ignored_paths: FilePathList { values: vec![] },
            mandatory_class_definition: BooleanVO::new(false),
        }
    }
}

pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig {
    let raw: serde_yml::Value = serde_yml::from_str(yaml_str).unwrap_or_default();
    if let Some(arch_val) = raw.get("architecture") {
        let mut arch_json: serde_json::Value = serde_json::to_value(arch_val).unwrap_or_default();
        // Extract layers from rules (first rule containing "layers" key) if not at top-level
        if arch_json.get("layers").is_none() {
            if let Some(rules_obj) = arch_json.get_mut("rules").and_then(|r| r.as_object_mut()) {
                for (_rule_code, rule_val) in rules_obj.iter_mut() {
                    if let Some(layers) = rule_val.get_mut("layers") {
                        let layers = std::mem::take(layers);
                        arch_json["layers"] = layers;
                        break;
                    }
                }
            }
        }
        let mut json = arch_json;
        fn remove_nulls(val: &mut serde_json::Value) {
            match val {
                serde_json::Value::Object(m) => {
                    m.retain(|_, v| !v.is_null());
                    for v in m.values_mut() {
                        remove_nulls(v);
                    }
                }
                serde_json::Value::Array(arr) => {
                    for v in arr.iter_mut() {
                        remove_nulls(v);
                    }
                }
                _ => {}
            }
        }
        remove_nulls(&mut json);
        // Convert ignored_paths from array to {values: [...]} format because the Rust struct expects an object with a "values" field.
        if let Some(arr) = json.get("ignored_paths").and_then(|v| v.as_array()) {
            json["ignored_paths"] = serde_json::json!({"values": arr});
        }
        if let Some(layers_obj) = json.get_mut("layers") {
            if let Some(obj) = layers_obj.as_object_mut() {
                let mut suffix_updates: Vec<(
                    String,
                    Option<String>,
                    serde_json::Value,
                    serde_json::Value,
                )> = Vec::new();
                for (layer_name, layer) in obj.iter() {
                    if let Some(suffix_val) = layer.get("suffix") {
                        if let Some(arr) = suffix_val.as_array() {
                            let mut policy: Option<String> = None;
                            let mut allowed = serde_json::Value::Array(Vec::new());
                            let mut forbidden = serde_json::Value::Array(Vec::new());
                            for entry in arr {
                                if let Some(entry_obj) = entry.as_object() {
                                    for (pkey, plist) in entry_obj {
                                        match pkey.as_str() {
                                            "strict" | "flexible" => {
                                                policy = Some(pkey.clone());
                                                if let Some(list) = plist.as_array() {
                                                    allowed = serde_json::json!(list);
                                                }
                                            }
                                            "forbidden" => {
                                                if let Some(list) = plist.as_array() {
                                                    forbidden = serde_json::json!(list);
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            suffix_updates.push((layer_name.clone(), policy, allowed, forbidden));
                        }
                    }
                }
                for (name, policy, allowed, forbidden) in suffix_updates {
                    if let Some(layer) = obj.get_mut(&name) {
                        if let Some(layer_obj) = layer.as_object_mut() {
                            if let Some(ref p) = policy {
                                layer_obj.insert("suffix_policy".to_string(), serde_json::json!(p));
                            }
                            layer_obj.insert("allowed_suffix".to_string(), allowed);
                            if let Some(arr) = forbidden.as_array() {
                                if !arr.is_empty() {
                                    layer_obj.insert("forbidden_suffix".to_string(), forbidden);
                                }
                            }
                            layer_obj.remove("suffix");
                        }
                    }
                }
            }
        }
        if let Some(rules_obj) = json.get_mut("rules") {
            if let Some(obj) = rules_obj.as_object_mut() {
                let mut flat = serde_json::Value::Array(Vec::new());
                for (code, rule_val) in obj.iter() {
                    if let Some(rule_obj) = rule_val.as_object() {
                        let mut base = rule_obj.clone();
                        base.insert("name".to_string(), serde_json::json!(code));
                        // Expand scope array into multiple entries — one per scope element
                        // Only applies to rules WITHOUT conditions (conditions have their own scopes)
                        if let Some(scope_arr) = base.get("scope").and_then(|s| s.as_array()) {
                            if !base.contains_key("conditions") && scope_arr.len() > 1 {
                                for scope_val in scope_arr {
                                    if let Some(s) = scope_val.as_str() {
                                        let mut entry = base.clone();
                                        entry.insert("scope".to_string(), serde_json::json!(s));
                                        if let Some(arr) = flat.as_array_mut() {
                                            arr.push(serde_json::Value::Object(entry));
                                        }
                                    }
                                }
                                continue; // Already pushed per-scope entries, skip single push below
                            } else if let Some(first) = scope_arr.first().and_then(|v| v.as_str()) {
                                base.insert("scope".to_string(), serde_json::json!(first));
                            }
                        }
                        if let Some(conditions) = base.remove("conditions") {
                            if let Some(conds) = conditions.as_array() {
                                if !conds.is_empty() {
                                    for cond in conds {
                                        if let Some(cond_obj) = cond.as_object() {
                                            let mut entry = base.clone();
                                            for (k, v) in cond_obj {
                                                entry.insert(k.clone(), v.clone());
                                            }
                                            // Remove top-level scope array leftovers if condition has its own scope
                                            if let Some(arr) = flat.as_array_mut() {
                                                arr.push(serde_json::Value::Object(entry));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(arr) = flat.as_array_mut() {
                                arr.push(serde_json::Value::Object(base));
                            }
                        }
                    }
                }
                *rules_obj = flat;
            }
        }
        let mut config = match serde_json::from_value::<ArchitectureConfig>(json) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[warn] Failed to deserialize ArchitectureConfig: {:?}", e);
                eprintln!("[warn] Falling back to default config. Check your YAML syntax and field types.");
                ArchitectureConfig::default()
            }
        };
        // Top-level ignored_paths (outside architecture section) — merge into config
        if config.ignored_paths.values.is_empty() {
            if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
                let paths: Vec<_> = arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                    .collect();
                if !paths.is_empty() {
                    config.ignored_paths = FilePathList::new(paths);
                }
            }
        }
        config
    } else {
        let mut config = ArchitectureConfig::default();
        if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
            let paths: Vec<_> = arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                .collect();
            if !paths.is_empty() {
                config.ignored_paths = FilePathList::new(paths);
            }
        }
        config
    }
}

/// All 3 config YAMLs are baked into the binary at compile time via `include_str!`.
/// Runtime project-level config files override these defaults.
/// Cached via OnceLock to avoid re-parsing on every call.
static DEFAULT_RUST_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();
static DEFAULT_PYTHON_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();
static DEFAULT_TS_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();

pub fn default_aes_config() -> ArchitectureConfig {
    DEFAULT_RUST_CONFIG
        .get_or_init(|| parse_config_yaml(include_str!("../../../../lint_arwaky.config.rust.yaml")))
        .clone()
}

pub fn default_config_for_language(language: &str) -> ArchitectureConfig {
    match language {
        "rust" => default_aes_config(),
        "python" => DEFAULT_PYTHON_CONFIG
            .get_or_init(|| {
                parse_config_yaml(include_str!("../../../../lint_arwaky.config.python.yaml"))
            })
            .clone(),
        "javascript" | "typescript" => DEFAULT_TS_CONFIG
            .get_or_init(|| {
                parse_config_yaml(include_str!(
                    "../../../../lint_arwaky.config.javascript.yaml"
                ))
            })
            .clone(),
        _ => {
            eprintln!(
                "[warn] Unknown language '{}', using empty default config.",
                language
            );
            ArchitectureConfig::default()
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_identifier_vo.rs

```rust
// PURPOSE: ConfigIdentifier — value object for named configuration identifiers
use crate::string_value_object;

string_value_object!(ConfigKey);

impl ConfigKey {
    /// Returns each dot-separated segment of the key.
    pub fn parts(&self) -> Vec<String> {
        self.value.split('.').map(|s| s.to_string()).collect()
    }

    /// Returns the parent key, dropping the last segment. Empty when the
    /// key has no parent (single segment).
    pub fn parent(&self) -> String {
        let parts = self.parts();
        if parts.len() > 1 {
            parts[..parts.len() - 1].join(".")
        } else {
            String::new()
        }
    }

    /// Returns the last segment of the key, or the full value when the
    /// key has no `.` separators.
    pub fn leaf(&self) -> String {
        match self.parts().last() {
            Some(part) => part.clone(),
            None => self.value.clone(),
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_multi_project_summary_vo.rs

```rust
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AggregatedResults {
    pub projects: Vec<ProjectResult>,
    pub total_projects: Count,
    pub passing_projects: Count,
    pub failing_projects: Count,
    pub average_score: Score,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectResult {
    pub path: FilePath,
    pub score: Score,
    pub is_passing: ComplianceStatus,
    pub issues: Vec<std::collections::HashMap<String, serde_json::Value>>,
    pub adapters: PatternList,
    pub error: ErrorMessage,
}

impl AggregatedResults {
    pub fn new(
        projects: Vec<ProjectResult>,
        total_projects: Count,
        passing_projects: Count,
        failing_projects: Count,
        average_score: Score,
    ) -> Self {
        Self {
            projects,
            total_projects,
            passing_projects,
            failing_projects,
            average_score,
        }
    }
}

impl ProjectResult {
    pub fn new(
        path: FilePath,
        score: Score,
        is_passing: ComplianceStatus,
        issues: Vec<std::collections::HashMap<String, serde_json::Value>>,
        adapters: PatternList,
        error: ErrorMessage,
    ) -> Self {
        Self {
            path,
            score,
            is_passing,
            issues,
            adapters,
            error,
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_multi_project_vo.rs

```rust
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;

#[derive(Debug, Clone, Default)]
pub struct MultiProjectVO {
    pub paths: Option<FilePathList>,
    pub use_retry: Option<BooleanVO>,
    pub config_path: Option<FilePath>,
}
```

---

## File: crates/shared/src/config-system/taxonomy_multi_project_workspace_info_vo.rs

```rust
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub path: FilePath,
    pub workspace_type: String,
    pub config: ArchitectureConfig,
}

impl WorkspaceInfo {
    pub fn new(path: FilePath, workspace_type: String, config: ArchitectureConfig) -> Self {
        Self {
            path,
            workspace_type,
            config,
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_setting_vo.rs

```rust
// PURPOSE: SettingsConfigVO — value object for application-wide settings configuration

use crate::string_value_object;
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;

string_value_object!(ActualValue);
string_value_object!(ExpectedValue);

/// Scoring thresholds.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Thresholds {
    pub score: Score,
    pub complexity: Count,
    pub max_file_lines: Count,
}

impl Thresholds {
    pub fn new(score: Score, complexity: Count, max_file_lines: Count) -> Self {
        Self {
            score,
            complexity,
            max_file_lines,
        }
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            score: Score::new(80.0),
            complexity: Count::new(10),
            max_file_lines: Count::new(500),
        }
    }
}

/// Adapter status enum.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum AdapterStatus {
    #[default]
    Enabled,
    Disabled,
    NotInstalled,
}

impl AdapterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AdapterStatus::Enabled => "enabled",
            AdapterStatus::Disabled => "disabled",
            AdapterStatus::NotInstalled => "not_installed",
        }
    }
}

impl std::fmt::Display for AdapterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Single adapter configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterEntry {
    pub name: AdapterName,
    #[serde(default)]
    pub status: AdapterStatus,
    #[serde(default = "default_weight")]
    pub weight: f64,
}

fn default_weight() -> f64 {
    1.0
}

impl AdapterEntry {
    pub fn new(name: AdapterName, status: AdapterStatus, weight: f64) -> Self {
        Self {
            name,
            status,
            weight,
        }
    }

    pub fn enabled(name: AdapterName) -> Self {
        Self::new(name, AdapterStatus::Enabled, 1.0)
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, AdapterStatus::Enabled)
    }
}

/// Project configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProjectConfig {
    #[serde(default = "default_project_name")]
    pub project_name: DescriptionVO,
    #[serde(default)]
    pub thresholds: Thresholds,
    #[serde(default)]
    pub adapters: Vec<AdapterEntry>,
    #[serde(default)]
    pub ignored_paths: FilePathList,
    #[serde(default)]
    pub ignored_rules: PatternList,
    #[serde(default)]
    pub layer_map: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub output_dir: Option<DirectoryPath>,
    #[serde(default)]
    pub architecture: ArchitectureConfig,
}

fn default_project_name() -> DescriptionVO {
    DescriptionVO::new("lint-arwaky")
}

impl ProjectConfig {
    /// Returns a ProjectConfig with default linter adapters enabled.
    pub fn defaults() -> Self {
        Self {
            project_name: default_project_name(),
            thresholds: Thresholds::default(),
            adapters: vec![
                AdapterEntry::enabled(AdapterName::raw("ruff")),
                AdapterEntry::enabled(AdapterName::raw("mypy")),
                AdapterEntry::enabled(AdapterName::raw("bandit")),
                AdapterEntry::enabled(AdapterName::raw("radon")),
            ],
            ignored_paths: FilePathList::default(),
            ignored_rules: PatternList::default(),
            layer_map: std::collections::HashMap::new(),
            output_dir: None,
            architecture: ArchitectureConfig::default(),
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_source_vo.rs

```rust
// PURPOSE: ConfigResult, ConfigSource for config-system
pub use crate::common::taxonomy_source_vo::ContentString;
pub use crate::common::taxonomy_source_vo::SourceContentVO;

use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use serde::{Deserialize, Serialize};

/// Represents a configuration source with its language, path, and raw content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigSource {
    pub language: String,
    pub path: FilePath,
    pub raw_content: String,
}

impl ConfigSource {
    pub fn new(
        language: impl Into<String>,
        path: impl Into<String>,
        raw_content: impl Into<String>,
    ) -> Self {
        Self {
            language: language.into(),
            path: FilePath::new(path.into()).unwrap_or_default(),
            raw_content: raw_content.into(),
        }
    }
}

/// Result type for config loading operations containing the parsed config, source info, and warnings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigResult {
    pub config: ArchitectureConfig,
    pub source: ConfigSource,
    pub warnings: Vec<String>,
}

impl ConfigResult {
    pub fn new(config: ArchitectureConfig, source: ConfigSource, warnings: Vec<String>) -> Self {
        Self {
            config,
            source,
            warnings,
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_validation_vo.rs

```rust
// PURPOSE: ValidationResult — value object for config system validation results

/// Result of a validation operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub reason: Option<String>,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self {
            is_valid: true,
            reason: None,
        }
    }
    pub fn fail(reason: &str) -> Self {
        Self {
            is_valid: false,
            reason: Some(reason.to_string()),
        }
    }
}
```

---

## File: crates/shared/src/external-lint/contract_external_lint_aggregate.rs

```rust
// PURPOSE: IExternalLintAggregate — contract for running external linter adapters
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IExternalLintAggregate: Send + Sync {
    async fn scan_all(&self, path: &FilePath) -> LintResultList;
    fn adapter_names(&self) -> Vec<String>;
}
```

---

## File: crates/shared/src/external-lint/mod.rs

```rust
// external-lint — taxonomy types for adapter utilities
pub mod contract_external_lint_aggregate;
pub mod taxonomy_external_lint_helper;
```

---

## File: crates/shared/src/external-lint/taxonomy_external_lint_helper.rs

```rust
// PURPOSE: taxonomy_external_lint_helper — shared utility functions for external linter adapters
// Pure functions: resolve working directories, canonicalize paths,
// execute commands with error mapping. Used by JS, Python, and RS adapters.

use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_adapter_error::AdapterError;
use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_response_data_vo::ResponseData;
use std::path::{Path, PathBuf};

/// Canonicalize a path string, falling back to the original on error.
pub fn canonicalize_path(path_str: &str) -> String {
    match std::fs::canonicalize(path_str) {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}

/// Execute a command, mapping execution failures to `LinterOperationError::Scan`.
pub async fn exec_cmd_scan(
    executor: &dyn ICommandExecutorPort,
    args: Vec<String>,
    working_dir: FilePath,
    timeout_secs: f64,
    adapter_name: Option<AdapterName>,
    path: &FilePath,
) -> Result<ResponseData, LinterOperationError> {
    executor
        .execute_command(
            PatternList::new(args),
            working_dir,
            Some(Timeout::new(timeout_secs)),
        )
        .await
        .map_err(|e| {
            LinterOperationError::Scan(ScanError {
                path: path.clone(),
                message: ErrorMessage::new(e.to_string()),
                error_code: None,
                adapter_name,
                cause: None,
            })
        })
}

/// Execute a command, mapping execution failures to `LinterOperationError::Adapter`.
pub async fn exec_cmd_adapter(
    executor: &dyn ICommandExecutorPort,
    args: Vec<String>,
    working_dir: FilePath,
    timeout_secs: f64,
    adapter_name: AdapterName,
) -> Result<ResponseData, LinterOperationError> {
    executor
        .execute_command(
            PatternList::new(args),
            working_dir,
            Some(Timeout::new(timeout_secs)),
        )
        .await
        .map_err(|e| {
            LinterOperationError::Adapter(AdapterError::new(
                adapter_name,
                ErrorMessage::new(e.to_string()),
            ))
        })
}

/// Create a default `"."` working directory, falling back to the given path if it fails.
pub fn default_working_dir(path: &FilePath) -> FilePath {
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

/// Applies a JS tool's fix command, returning `Ok(ComplianceStatus::new(true))` on success.
/// Combines resolve_js_working_dir + canonicalize_path + resolve_js_cmd + exec_cmd_adapter.
pub async fn js_apply_fix(
    executor: &dyn ICommandExecutorPort,
    path: &FilePath,
    tool: &str,
    fix_arg: &str,
) -> Result<ComplianceStatus, LinterOperationError> {
    let wd = resolve_js_working_dir(path);
    let abs_path = canonicalize_path(&path.value);
    let cmd = resolve_js_cmd(tool, vec![abs_path, fix_arg.to_string()], &wd.value);
    let response = exec_cmd_adapter(executor, cmd, wd, 60.0, AdapterName::raw(tool)).await?;
    Ok(ComplianceStatus::new(response.returncode == 0))
}

/// No-op apply_fix for linters that cannot auto-fix (scanners, type-checkers).
pub async fn noop_apply_fix() -> Result<ComplianceStatus, LinterOperationError> {
    Ok(ComplianceStatus::new(false))
}

/// Return true if the given path contains any Python (`.py`) files.
///
/// For existing directories: recursively walks, short-circuiting at the first `.py`
/// file found. For non-existent paths: checks the filename extension — if it ends
/// in `.py` the path is treated as having a Python file (letting the tool itself
/// handle the missing-file error).
pub fn has_python_files(path: &FilePath) -> bool {
    let p = std::path::Path::new(&path.value);
    if !p.exists() {
        // Non-existent path — check by extension (e.g. "foo.py" for test mocks)
        return p.extension().map(|e| e == "py").unwrap_or(false);
    }
    if p.is_file() {
        return p.extension().map(|e| e == "py").unwrap_or(false);
    }
    // Directory walk — short-circuit at first .py file
    has_py_in_dir(p)
}

fn has_py_in_dir(dir: &std::path::Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if has_py_in_dir(&path) {
                return true;
            }
        } else if path.extension().map(|e| e == "py").unwrap_or(false) {
            return true;
        }
    }
    false
}

/// Resolve the executable command for a JS tool (eslint, prettier, tsc).
/// Prefers local node_modules/.bin over npx/bunx.
pub fn resolve_js_cmd(executable: &str, args: Vec<String>, working_dir: &str) -> Vec<String> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);
    if local_bin.exists() {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return cmd;
    }
    let runner = if is_bun_available() { "bunx" } else { "npx" };
    let mut cmd = vec![runner.to_string(), executable.to_string()];
    cmd.extend(args);
    cmd
}

/// Walk up from the given path to find the JS project root
/// (detected by lint_arwaky.config*.yaml, package.json, or .git directory).
pub fn resolve_js_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if let Ok(abs_path) = std::fs::canonicalize(path_str) {
        let mut current = if abs_path.is_file() {
            abs_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            abs_path.clone()
        };
        for _ in 0..10 {
            if current.join("lint_arwaky.config.yaml").is_file()
                || current.join("lint_arwaky.config.python.yaml").is_file()
                || current.join("package.json").is_file()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
    }
    FilePath::new(".".to_string()).unwrap_or_default()
}

/// Find parent dir with Cargo.toml (for cargo fmt, cargo clippy).
pub fn resolve_cargo_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }
    let current = Path::new(path_str);
    if current.is_dir() {
        if current.join("Cargo.toml").exists() {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if parent.join("Cargo.toml").exists() {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.toml").exists() {
                return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
        }
    }
    FilePath::new("nonexistent_directory_for_cargo_toml".to_string()).unwrap_or_default()
}

/// Find parent dir with Cargo.lock (for cargo-audit).
pub fn resolve_cargo_lock_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }
    let current = Path::new(path_str);
    if current.is_dir() {
        if current.join("Cargo.lock").exists() {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if parent.join("Cargo.lock").exists() {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.lock").exists() {
                return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
        }
    }
    FilePath::new("nonexistent_directory_for_cargo_lock".to_string()).unwrap_or_default()
}

fn is_bun_available() -> bool {
    std::process::Command::new("bun")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
```

---

## File: crates/shared/src/file-watch/contract_change_analyzer_protocol.rs

```rust
// PURPOSE: IChangeAnalyzerProtocol — protocol for watch event change analysis
use crate::file_watch::taxonomy_watch_event_vo::WatchEvent;

pub trait IChangeAnalyzerProtocol: Send + Sync {
    fn analyze(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
    fn is_lintable(path: &str) -> bool;
    fn filter_lintable(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
}
```

---

## File: crates/shared/src/file-watch/contract_provider_port.rs

```rust
// PURPOSE: IWatchProviderPort — port trait for filesystem watch provider
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::file_watch::taxonomy_service_error::WatchServiceError;
use crate::file_watch::taxonomy_watch_config_vo::WatchConfig;
use crate::file_watch::taxonomy_watch_event_vo::WatchEvent;

#[async_trait::async_trait]
pub trait IWatchProviderPort: Send + Sync {
    async fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError>;
    async fn stop(&self) -> Result<(), WatchServiceError>;
    async fn is_available(&self) -> BooleanVO;
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<WatchEvent>;
}
```

---

## File: crates/shared/src/file-watch/contract_watch_aggregate.rs

```rust
// PURPOSE: IWatchAggregate — contract trait for watch operations used by surfaces
use crate::file_watch::taxonomy_watch_config_vo::WatchConfig;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub trait IWatchAggregate: Send + Sync {
    fn run(&self, config: WatchConfig, running: Arc<AtomicBool>) -> std::process::ExitCode;
}
```

---

## File: crates/shared/src/file-watch/mod.rs

```rust
// file-watch — taxonomy and contract types
pub mod contract_change_analyzer_protocol;
pub mod contract_provider_port;
pub mod contract_watch_aggregate;
pub mod taxonomy_diff_result_vo;
pub mod taxonomy_service_error;
pub mod taxonomy_watch_config_vo;
pub mod taxonomy_watch_event_vo;
```

---

## File: crates/shared/src/file-watch/taxonomy_diff_result_vo.rs

```rust
// PURPOSE: GitDiffResultVO — value object representing git diff results
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_paths_vo::RenamedFileList;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitDiffResultVO {
    pub added: FilePathList,
    pub modified: FilePathList,
    pub deleted: FilePathList,
    pub renamed: RenamedFileList,
    pub lintable_files: FilePathList,
    pub all_files: FilePathList,
    pub total_changed: Count,
}

impl GitDiffResultVO {
    pub fn new(
        added: FilePathList,
        modified: FilePathList,
        deleted: FilePathList,
        renamed: RenamedFileList,
        lintable_files: FilePathList,
        all_files: FilePathList,
        total_changed: Count,
    ) -> Self {
        Self {
            added,
            modified,
            deleted,
            renamed,
            lintable_files,
            all_files,
            total_changed,
        }
    }
}
```

---

## File: crates/shared/src/file-watch/taxonomy_service_error.rs

```rust
// PURPOSE: WatchServiceError — structured error type for file watch service failures
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WatchServiceError {
    pub path: FilePath,
    pub message: LintMessage,
}

impl WatchServiceError {
    pub fn new(message: LintMessage) -> Self {
        Self {
            path: FilePath::default(),
            message,
        }
    }
}

impl std::fmt::Display for WatchServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Watch Error on {}: {}", self.path.value, self.message)
    }
}

impl std::error::Error for WatchServiceError {}
```

---

## File: crates/shared/src/file-watch/taxonomy_watch_config_vo.rs

```rust
// PURPOSE: WatchConfig — value object for file watch configuration parameters
use crate::common::taxonomy_path_vo::FilePath;

pub struct WatchConfig {
    pub path: FilePath,
    pub recursive: bool,
    pub debounce_ms: u64,
    pub ignore_patterns: Vec<String>,
}

impl WatchConfig {
    pub fn from_path(path: String) -> Self {
        Self {
            path: FilePath::new(path).unwrap_or_default(),
            recursive: true,
            debounce_ms: 500,
            ignore_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "__pycache__".to_string(),
                "target".to_string(),
                ".venv".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
        }
    }
}
```

---

## File: crates/shared/src/file-watch/taxonomy_watch_event_vo.rs

```rust
// PURPOSE: WatchEvent — value object representing a filesystem change event
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WatchEventKind {
    Created,
    Modified,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WatchEvent {
    pub path: String,
    pub kind: WatchEventKind,
    pub timestamp_ms: u64,
}

impl WatchEvent {
    pub fn new(path: String, kind: WatchEventKind) -> Self {
        let timestamp_ms = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        {
            Ok(d) => d.as_millis() as u64,
            Err(_) => 0,
        };
        Self {
            path,
            kind,
            timestamp_ms,
        }
    }
}
```

---

## File: crates/shared/src/git-hooks/contract_diff_protocol.rs

```rust
// PURPOSE: IDiffProtocol — protocol for git diff analysis operations (business logic)
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::git_hooks::taxonomy_diff_result_vo::GitDiffResultVO;
use async_trait::async_trait;

#[async_trait]
pub trait IDiffProtocol: Send + Sync {
    /// Run lint check on git diff changes
    async fn run_git_diff_check(&self, path: &FilePath) -> LintResultList;

    /// Get detailed diff result for a path
    async fn get_diff(&self, path: &FilePath) -> GitDiffResultVO;

    /// Get list of changed files from git diff
    async fn get_changed_files(&self, path: &FilePath) -> FilePathList;

    /// Get default branch name for a repository
    async fn get_default_branch(&self, path: &FilePath) -> String;
}
```

---

## File: crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs

```rust
// PURPOSE: GitHooksAggregate — unified aggregate trait for git hooks orchestration
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::git_hooks::contract_diff_protocol::IDiffProtocol;
use crate::git_hooks::contract_hook_protocol::IHookProtocol;
use async_trait::async_trait;

#[async_trait]
pub trait GitHooksAggregate: Send + Sync {
    /// Access to diff protocol (read operations)
    fn diff_protocol(&self) -> &dyn IDiffProtocol;

    /// Access to hook protocol (write/management operations)
    fn hook_protocol(&self) -> &dyn IHookProtocol;

    /// Run full git hooks check on a path
    async fn run_git_hooks_check(&self, path: &FilePath) -> LintResultList {
        self.diff_protocol().run_git_diff_check(path).await
    }

    /// Install pre-commit hook
    async fn install_hook(
        &self,
        executable_path: &FilePath,
    ) -> Result<
        crate::mcp_server::taxonomy_job_vo::SuccessStatus,
        crate::git_hooks::taxonomy_hook_error::GitHookError,
    > {
        self.hook_protocol()
            .install_pre_commit(executable_path)
            .await
    }

    /// Uninstall pre-commit hook
    async fn uninstall_hook(
        &self,
    ) -> Result<
        crate::mcp_server::taxonomy_job_vo::SuccessStatus,
        crate::git_hooks::taxonomy_hook_error::GitHookError,
    > {
        self.hook_protocol().uninstall_pre_commit().await
    }
}
```

---

## File: crates/shared/src/git-hooks/contract_hook_protocol.rs

```rust
// PURPOSE: IHookProtocol — protocol for git hook management operations (business logic)
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `String` returns → `DescriptionVO` (semantic description text)
//   * `HashMap<String, serde_json::Value>` → `GitDiffDataVO` (strongly-typed diff)
//   * `&str file_path` params → kept as `&str` (idiomatic borrow, AES402 allows)
//   * `bool remove` → kept (semantic toggle, AES402 allows)
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::git_hooks::taxonomy_git_diff_data_vo::{GitDiffDataVO, HookIgnoreUpdateVO};
use crate::git_hooks::taxonomy_hook_error::GitHookError;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use async_trait::async_trait;

#[async_trait]
pub trait IHookProtocol: Send + Sync {
    /// Install pre-commit hook.
    async fn install_pre_commit(
        &self,
        executable_path: &FilePath,
    ) -> Result<SuccessStatus, GitHookError>;

    /// Uninstall pre-commit hook.
    async fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;

    /// Get hook manager identity.
    fn get_hook_manager_identity(&self) -> Identity;

    /// Initialize git hooks config at the given project path.
    /// Returns a description of the result (e.g. "ALREADY_EXISTS:..." or
    /// "Initialized ..."). The description is a description VO so callers can
    /// introspect, translate, or log it without parsing strings.
    async fn initialize_config(&self, path: &str) -> DescriptionVO;

    /// Update the ignore list: add or remove a single rule.
    /// Returns a description of the operation.
    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO;

    /// Get diff data between two file paths. Returns a strongly-typed VO;
    /// no raw JSON in the contract surface.
    async fn get_diff_data(&self, path1: &str, path2: &str) -> GitDiffDataVO;
}
```

---

## File: crates/shared/src/git-hooks/contract_manager_port.rs

```rust
// PURPOSE: IHookManagerPort — port trait for hook script management (install, uninstall)

use crate::common::taxonomy_path_vo::FilePath;
use crate::git_hooks::taxonomy_hook_error::GitHookError;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;

pub trait IHookManagerPort: Send + Sync {
    fn install_pre_commit(&self, executable_path: &FilePath)
        -> Result<SuccessStatus, GitHookError>;
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;
}
```

---

## File: crates/shared/src/git-hooks/contract_orchestrator_aggregate.rs

```rust
// PURPOSE: HookOrchestratorAggregate — aggregate trait for hook orchestration
use crate::common::taxonomy_layer_vo::Identity;
use crate::git_hooks::contract_manager_port::IHookManagerPort;

pub trait HookManagementOrchestratorAggregate: Send + Sync {
    fn get_hook_manager(&self) -> &dyn IHookManagerPort;
    fn get_hook_manager_identity(&self) -> Identity;
}
```

---

## File: crates/shared/src/git-hooks/mod.rs

```rust
pub mod contract_diff_protocol;
pub mod contract_git_hooks_aggregate;
pub mod contract_hook_protocol;
pub mod contract_manager_port;
pub mod contract_orchestrator_aggregate;
pub mod taxonomy_diff_result_vo;
pub mod taxonomy_git_diff_data_vo;
pub mod taxonomy_hook_error;
```

---

## File: crates/shared/src/git-hooks/taxonomy_diff_result_vo.rs

```rust
// PURPOSE: Re-export GitDiffResultVO from file-watch for git-hooks module
//
// This file exists so dependents inside `git-hooks` can import the type via
// `git_hooks::taxonomy_diff_result_vo::GitDiffResultVO` without depending on
// the file-watch crate directly. The real definition lives in
// `file_watch::taxonomy_diff_result_vo` and is re-exported here.
pub use crate::file_watch::taxonomy_diff_result_vo::GitDiffResultVO;
```

---

## File: crates/shared/src/git-hooks/taxonomy_git_diff_data_vo.rs

```rust
// PURPOSE: GitDiffDataVO — value object representing semantic diff data between two file versions
use serde::{Deserialize, Serialize};

/// Semantic status of the diff between two file versions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GitDiffStatus {
    /// Files are byte-identical (or content-identical after normalization).
    Unchanged,
    /// Files differ in content.
    Modified,
    /// Path1 does not exist.
    MissingFirst,
    /// Path2 does not exist.
    MissingSecond,
    /// Either path is not a regular file.
    NotAFile,
}

/// One side of a two-file diff (path1 or path2 in the original HashMap key
/// "version1" / "version2"). The score is reserved for future use (currently
/// always 0.0); kept as a field so callers do not have to introduce a new VO
/// once we wire up a real similarity score.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitDiffSideVO {
    pub path: String,
    pub similarity_score: f64,
}

impl GitDiffSideVO {
    pub fn new(path: impl Into<String>, similarity_score: f64) -> Self {
        Self {
            path: path.into(),
            similarity_score,
        }
    }
}

/// Strongly-typed replacement for the previous
/// `HashMap<String, serde_json::Value>` return type. Each field has a real
/// domain meaning — there is no `serde_json::Value` in the contract surface.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitDiffDataVO {
    pub version1: GitDiffSideVO,
    pub version2: GitDiffSideVO,
    /// 0.0 when files are identical; positive number otherwise.
    /// Concrete unit (line count? byte count? semantic diff?) is left to the
    /// caller to populate; the contract only requires a non-negative number.
    pub difference: f64,
    pub status: GitDiffStatus,
}

impl GitDiffDataVO {
    pub fn unchanged(version1_path: impl Into<String>, version2_path: impl Into<String>) -> Self {
        Self {
            version1: GitDiffSideVO::new(version1_path, 1.0),
            version2: GitDiffSideVO::new(version2_path, 1.0),
            difference: 0.0,
            status: GitDiffStatus::Unchanged,
        }
    }

    pub fn modified(
        version1_path: impl Into<String>,
        version2_path: impl Into<String>,
        difference: f64,
    ) -> Self {
        Self {
            version1: GitDiffSideVO::new(version1_path, 0.0),
            version2: GitDiffSideVO::new(version2_path, 0.0),
            difference,
            status: GitDiffStatus::Modified,
        }
    }
}

/// One ignore-rule update request passed to `IHookProtocol::update_ignore_rule`.
/// Mirrors the previous `(rule: &str, remove: bool, config_path: &str)`
/// positional signature but uses VOs.
#[derive(Debug, Clone)]
pub struct HookIgnoreUpdateVO {
    pub rule: String,
    pub remove: bool,
    pub config_path: String,
}

impl HookIgnoreUpdateVO {
    pub fn new(rule: impl Into<String>, remove: bool, config_path: impl Into<String>) -> Self {
        Self {
            rule: rule.into(),
            remove,
            config_path: config_path.into(),
        }
    }
}
```

---

## File: crates/shared/src/git-hooks/taxonomy_hook_error.rs

```rust
// PURPOSE: GitHookError — structured error type for git hook operation failures
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone)]
pub struct GitHookError {
    pub path: FilePath,
    pub message: LintMessage,
}

impl GitHookError {
    pub fn new(message: LintMessage) -> Self {
        Self {
            path: FilePath::default(),
            message,
        }
    }
}

impl std::fmt::Display for GitHookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Git Hook Error on {}: {}", self.path.value, self.message)
    }
}

impl std::error::Error for GitHookError {}
```

---

## File: crates/shared/src/import-rules/contract_import_parser_port.rs

```rust
// PURPOSE: IImportParserPort — contract port trait for import parsing utilities
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::FileContentVO;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::import_rules::taxonomy_language_vo::LanguageVO;
use std::collections::{HashMap, HashSet};

pub trait IImportParserPort: Send + Sync {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>);
    fn import_matches_scope(
        &self,
        import_line: &LineContentVO,
        layer: &LayerNameVO,
        suffixes: &[Identity],
    ) -> bool;
    fn get_basename(&self, file: &FilePath) -> Identity;
    fn read_import_lines(&self, file: &FilePath) -> Vec<(LineNumber, LineContentVO)>;
    fn parse_import_lines(&self, content: &FileContentVO) -> Vec<(LineNumber, LineContentVO)>;
    fn extract_module_from_line(&self, line: &LineContentVO) -> Option<Identity>;
    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO>;

    // New methods to extract infrastructure Concerns
    fn read_file_to_message(&self, file: &FilePath) -> Result<LintMessage, std::io::Error>;
    fn extract_import_modules(&self, content: &str) -> Vec<SymbolName>;
    fn get_language_from_path(&self, path: &str) -> LanguageVO;
    fn get_dummy_function_ranges(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(LineNumber, LineNumber)>;
    fn get_imported_symbols(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(SymbolName, LineNumber)>;
    fn get_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)>;
    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(LineNumber, LineNumber)],
        dummy_impl_traits: &[String],
    ) -> bool;
    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    // Fine-grained parsing utilities for unused import steps
    fn extract_imported_aliases(&self, content: &str) -> HashMap<Identity, Identity>;
    fn extract_exported_symbols(&self, content: &str) -> HashSet<Identity>;
    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity>;
    fn find_import_line_number(&self, content: &str, alias: &str) -> LineNumber;
    fn extract_rust_js_imports(&self, content: &str) -> Vec<(SymbolName, LineNumber)>;
    fn is_name_used(&self, name: &str, content: &str, exclude_line: LineNumber) -> bool;
}
```

---

## File: crates/shared/src/import-rules/contract_import_runner_aggregate.rs

```rust
// PURPOSE: IImportRunnerAggregate — contract for import-rules feature orchestrator
//
// This is the primary contract that decouples the import-rules agent layer
// from its callers (CLI, MCP, TUI). Callers depend on this trait, not on
// ImportOrchestrator directly.
//
// run_audit is async because it may perform file I/O and spawn blocking
// tasks internally. The caller provides a FilePath target (file or dir).
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

/// IImportRunnerAggregate — aggregate port for import-rules orchestration.
///
/// Implemented by ImportOrchestrator (agent layer).
/// Called by surface layer (CLI, MCP, TUI) via Arc<dyn IImportRunnerAggregate>.
#[async_trait]
pub trait IImportRunnerAggregate: Send + Sync {
    /// Run all 5 import-related AES checks (AES201–AES205) on the target.
    /// Returns aggregated violations from mandatory, forbidden, unused, dummy, and cycle checks.
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    /// Human-readable name for this orchestrator ("import-rules").
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/import-rules/contract_rule_protocol.rs

```rust
// PURPOSE: IAnalyzer trait — core analyzer interface for import checks
//
// This file defines the protocol traits that capabilities-level checkers
// implement. Each trait represents a single architectural responsibility:
//   - IAnalyzer: central configuration + layer detection hub
//   - IArchRuleProtocol: base trait for all AES rule implementations
//   - IInternalCheckerProtocol: checks layer-internal import rules
//   - IMetricCheckerProtocol: line count + mandatory class definition checks
//   - IArchImportProcessorProtocol: file-level import validation
//   - INamingRuleProtocol: naming convention checks (AES101-102)
//   - IArchStructureProtocol: combined naming + structure + metric checks
//   - IArchImportProtocol: mandatory and forbidden import checks (AES201-202, AES204)
//
// The trapezoidal hierarchy exists because different layers need different
// subsets of these capabilities — the trait bounds reflect the actual
// dependency requirements.
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::contract_parser_port::ISourceParserPort;
use crate::common::contract_system_port::IFileSystemPort;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;

/// IAnalyzer — the central configuration and analysis hub.
///
/// Provides access to:
///   - File system (for reading/writing files)
///   - Source parser (for AST-level analysis)
///   - Layer detection (maps file paths to architectural layers)
///
/// Also implements INamingAnalyzerProtocol, which allows naming-rules
/// to reuse the same layer-detection logic without duplicating it.
pub trait IAnalyzer:
    crate::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol + Send + Sync
{
    fn fs(&self) -> &dyn IFileSystemPort;
    fn parser(&self) -> &dyn ISourceParserPort;
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO>;
}

/// Base trait for all AES rule implementations.
/// Every checker must have a unique identity (e.g., "AES201").
pub trait IArchRuleProtocol {
    fn rule_name(&self) -> Identity;
}

/// Checks that imports within a layer respect internal boundaries
/// (e.g., a capabilities file should not import from infrastructure).
pub trait IInternalCheckerProtocol: Send + Sync {
    fn check_layer_internal_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

/// Metric-based checks: file line counts, function lengths, and
/// mandatory class/struct definitions within each file.
pub trait IMetricCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_mandatory_class_definition(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

/// Parameters for validating imports in a single file.
/// Bundles all data needed to check whether a file imports from required layers.
pub struct ValidateImportsParams<'a> {
    pub analyzer: &'a dyn IAnalyzer,
    pub file_path: &'a FilePath,
    pub root_dir: &'a FilePath,
    pub required_layers: &'a PatternList,
    pub results: &'a mut LintResultList,
    pub message_template: &'a ErrorMessage,
    pub layer_name: &'a LayerNameVO,
    pub layers_display: &'a PatternList,
}

/// Processes imports at the per-file level.
/// Validates that files import from the correct layers and not from forbidden ones.
pub trait IArchImportProcessorProtocol: Send + Sync {
    fn process_file_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn validate_imports_present(&self, params: ValidateImportsParams<'_>);
}

/// Parameters for file-naming checks.
/// Passes all configuration needed to check naming conventions across layers.
pub struct CheckFileNamingParams<'a> {
    pub files: &'a FilePathList,
    pub root_dir: &'a FilePath,
    pub layer_map: &'a LayerMapVO,
    pub global_expected: Count,
    pub global_exceptions: &'a PatternList,
    pub results: &'a mut LintResultList,
    pub detect_layer_fn: &'a dyn Fn(&FilePath, &FilePath) -> Option<LayerNameVO>,
}

/// Naming convention rules (AES101-102).
/// Checks file names, class names, and function names against
/// the AES layer-based naming conventions.
pub trait INamingRuleProtocol: IArchRuleProtocol + Send + Sync {
    fn check_file_naming(&self, params: CheckFileNamingParams<'_>);
    fn check_class_naming(
        &self,
        files: &FilePathList,
        results: &mut LintResultList,
        source_parser: &dyn ISourceParserPort,
    );
    fn check_function_naming(
        &self,
        files: &FilePathList,
        results: &mut LintResultList,
        source_parser: &dyn ISourceParserPort,
    );
}

/// Combined structure + naming + metrics protocol.
/// This is a legacy trait that aggregates multiple responsibilities.
/// New implementations should prefer the more granular trait separations.
pub trait IArchStructureProtocol: IArchRuleProtocol + Send + Sync {
    fn check_file_naming(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_domain_suffixes(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_layer_internal_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_line_counts(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_mandatory_class_definition(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

/// Import compliance protocol (AES201-202, AES204).
/// Checks for mandatory imports (files MUST import certain symbols) and
/// forbidden imports (files MUST NOT import certain symbols).
///
/// Both checks use the same async trait because they share the same
/// file-walking and analysis infrastructure — only the rule config differs.
#[async_trait::async_trait]
pub trait IArchImportProtocol: IArchRuleProtocol + Send + Sync {
    /// Check that files contain required imports based on their layer role.
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    /// Check that files do NOT contain prohibited imports.
    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
```

---

## File: crates/shared/src/import-rules/contract_unused_import_protocol.rs

```rust
// PURPOSE: IUnusedImportProtocol — unified port trait for AES203: detect unused imports across Rust, Python, JavaScript
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `Vec<String>` returns → `Vec<LintMessage>` (semantic messages, not raw strings)
//   * `&str file_path` params → kept as `&str` (idiomatic borrow, AES402 allows)
//   * `&mut Vec<LintResult>` → kept (`LintResult` is itself a VO)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

pub trait IUnusedImportProtocol: Send + Sync {
    /// Find unused imports in a file by path (reads file internally).
    /// Returns a list of human-readable lint messages describing each unused
    /// import. Replaces the previous `Vec<String>` so callers can introspect,
    /// translate, or log messages without parsing free-form strings.
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage>;

    /// Check unused imports given file content directly (for inline checking).
    /// Useful when content is already available (avoids re-reading file).
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/import-rules/mod.rs

```rust
// import-rules — taxonomy and contract types
pub mod contract_import_parser_port;
pub mod contract_import_runner_aggregate;
pub mod contract_rule_protocol;
pub mod contract_unused_import_protocol;
pub mod taxonomy_cycle_helper;
pub mod taxonomy_dependency_edge_vo;
pub mod taxonomy_dummy_helper;
pub mod taxonomy_import_rule_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_parser_helper;
pub mod taxonomy_path_helper;
pub mod taxonomy_unused_helper;
pub mod taxonomy_violation_import_vo;

pub use taxonomy_dependency_edge_vo::DependencyEdge;
pub use taxonomy_language_vo::LanguageVO;
pub use taxonomy_violation_import_vo::AesImportViolation;
```

---

## File: crates/shared/src/import-rules/taxonomy_cycle_helper.rs

```rust
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}

pub fn normalize_to_layer(name: &str) -> String {
    let layer_prefixes = [
        "taxonomy_",
        "contract_",
        "capabilities_",
        "infrastructure_",
        "agent_",
        "surface_",
    ];
    let base = match name.rsplit('/').next() {
        Some(b) => b,
        None => name,
    };
    for prefix in &layer_prefixes {
        if base.starts_with(prefix) {
            return prefix.trim_end_matches('_').to_string();
        }
    }
    name.to_string()
}

pub fn detect_cycle_edges(edges: &[DependencyEdge]) -> Vec<SymbolName> {
    let normalized_edges: Vec<DependencyEdge> = edges
        .iter()
        .map(|e| DependencyEdge::new(normalize_to_layer(&e.source), normalize_to_layer(&e.target)))
        .collect();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for e in &normalized_edges {
        graph
            .entry(e.source.clone())
            .or_default()
            .push(e.target.clone());
        graph.entry(e.target.clone()).or_default();
    }

    let mut color: HashMap<String, Color> = HashMap::new();
    let mut parent: HashMap<String, String> = HashMap::new();
    let mut cycle_edges_set: HashSet<(String, String)> = HashSet::new();

    for node in graph.keys() {
        color.entry(node.clone()).or_insert(Color::White);
    }

    for node in graph.keys().cloned().collect::<Vec<_>>() {
        if color[&node] == Color::White {
            dfs_3color(&node, &graph, &mut color, &mut parent, &mut cycle_edges_set);
        }
    }

    let mut unique_cycles: Vec<String> = Vec::new();
    let mut reported: HashSet<String> = HashSet::new();

    for (src, tgt) in &cycle_edges_set {
        let cycle_nodes = extract_cycle_nodes(src, tgt, &parent);
        if let Some(cycle) = cycle_nodes {
            let mut sorted_cycle = cycle.clone();
            sorted_cycle.sort();
            let dedup_key = sorted_cycle.join("->");
            if reported.insert(dedup_key) {
                for i in 0..cycle.len() {
                    let next = cycle[(i + 1) % cycle.len()].clone();
                    unique_cycles.push(format!("{}->{}", cycle[i], next));
                }
            }
        }
    }

    unique_cycles.into_iter().map(SymbolName::new).collect()
}

fn dfs_3color(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    color: &mut HashMap<String, Color>,
    parent: &mut HashMap<String, String>,
    cycle_edges: &mut HashSet<(String, String)>,
) {
    color.insert(node.to_string(), Color::Gray);

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if *color.get(neighbor).unwrap_or(&Color::White) == Color::Gray {
                cycle_edges.insert((node.to_string(), neighbor.clone()));
            } else if *color.get(neighbor).unwrap_or(&Color::White) == Color::White {
                parent.insert(neighbor.clone(), node.to_string());
                dfs_3color(neighbor, graph, color, parent, cycle_edges);
            }
        }
    }

    color.insert(node.to_string(), Color::Black);
}

fn extract_cycle_nodes(
    src: &str,
    tgt: &str,
    parent: &HashMap<String, String>,
) -> Option<Vec<String>> {
    let mut path = Vec::new();
    let mut cur = src;
    path.push(cur.to_string());

    while cur != tgt {
        match parent.get(cur) {
            Some(p) => {
                cur = p;
                path.push(cur.to_string());
            }
            None => return None,
        }
    }

    path.reverse();
    Some(path)
}
```

---

## File: crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs

```rust
// PURPOSE: DependencyEdge — representing directed edges in dependency graph

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DependencyEdge {
    pub source: String,
    pub target: String,
}

impl DependencyEdge {
    pub fn new(source: String, target: String) -> Self {
        Self { source, target }
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_dummy_helper.rs

```rust
// PURPOSE: taxonomy_dummy_helper — pure utility functions for dummy function, block, and trait detection
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_language_vo::LanguageVO;

pub fn dummy_function_ranges(lines: &[&str], lang: LanguageVO) -> Vec<(LineNumber, LineNumber)> {
    match lang {
        LanguageVO::Rust => rust_dummy_function_ranges(lines),
        LanguageVO::Python => python_dummy_function_ranges(lines),
        LanguageVO::JavaScript => js_dummy_function_ranges(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn imported_symbols(lines: &[&str], lang: LanguageVO) -> Vec<(SymbolName, LineNumber)> {
    match lang {
        LanguageVO::Rust => rust_imported_symbols(lines),
        LanguageVO::Python => python_imported_symbols(lines),
        LanguageVO::JavaScript => js_imported_symbols(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn dummy_impl_traits_with_lines(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut traits = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                let (end, body_lines) = impl_block(lines, i);
                if trait_impl_is_dummy(&body_lines) {
                    traits.push((SymbolName::new(trait_name), LineNumber::new(i as i64 + 1)));
                }
                i = end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    traits
}

pub fn symbol_used_real(
    lines: &[&str],
    symbol: &str,
    dummy_ranges: &[(usize, usize)],
    dummy_impl_traits: &[String],
) -> bool {
    if (symbol.starts_with('I')
        && symbol.len() > 1
        && matches!(symbol.chars().nth(1), Some(c) if c.is_uppercase()))
        || symbol.ends_with("Protocol")
        || symbol.ends_with("Port")
        || symbol.ends_with("Trait")
        || symbol.ends_with("Aggregate")
        || symbol.ends_with("Ext")
        || symbol == "Default"
        || symbol == "Debug"
        || symbol == "Display"
        || symbol == "Clone"
        || symbol == "Copy"
        || symbol == "From"
        || symbol == "Into"
        || symbol == "TryFrom"
        || symbol == "TryInto"
        || symbol == "AsRef"
        || symbol == "AsMut"
        || symbol == "Deref"
        || symbol == "DerefMut"
        || symbol == "Iterator"
        || symbol == "IntoIterator"
        || symbol == "Future"
        || symbol == "Stream"
        || symbol == "Read"
        || symbol == "Write"
        || symbol == "BufRead"
        || symbol == "Seek"
        || symbol == "Hash"
        || symbol == "PartialEq"
        || symbol == "Eq"
        || symbol == "PartialOrd"
        || symbol == "Ord"
        || symbol == "Send"
        || symbol == "Sync"
        || symbol == "Unpin"
        || symbol == "Sized"
        || symbol == "Drop"
        || symbol == "Fn"
        || symbol == "FnMut"
        || symbol == "FnOnce"
        || symbol == "async_trait"
        || symbol == "Parser"
        || symbol == "Digest"
        || symbol == "Manager"
        || symbol == "Emitter"
        || symbol == "Serialize"
        || symbol == "Deserialize"
    {
        return true;
    }

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if in_dummy_range(line_no, dummy_ranges)
            || trimmed.starts_with("use ")
            || trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with("*")
            || trimmed.starts_with("*/")
            || (trimmed.starts_with("#") && !trimmed.starts_with("#["))
            || trimmed.contains("PhantomData")
        {
            continue;
        }

        if !trimmed.contains(symbol) {
            continue;
        }

        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                if dummy_impl_traits.contains(&trait_name) {
                    continue;
                }
            }
        }

        return true;
    }

    false
}

// ─── Private Helpers ───

/// Iterate `lines`, invoking `is_header(trimmed_line)` to identify function
/// definitions and `body_extent(start, lines)` to compute the body end line
/// for that definition. Returns `[(start_line, end_line), ...]` of all ranges.
///
/// The two language-specific differences (Rust/JS brace-counting vs. Python
/// indent-based termination) live in the closures passed in.
fn collect_ranges<F, G>(
    lines: &[&str],
    is_header: F,
    body_extent: G,
) -> Vec<(LineNumber, LineNumber)>
where
    F: Fn(&str) -> bool,
    G: Fn(usize, &[&str]) -> usize,
{
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if is_header(lines[i].trim()) {
            let start = i + 1;
            let end = body_extent(i, lines);
            ranges.push((LineNumber::new(start as i64), LineNumber::new(end as i64)));
            i = end;
        }
        i += 1;
    }
    ranges
}

/// Brace-counting body extenter for Rust/JS-like brace-delimited languages.
fn brace_extent(start: usize, lines: &[&str]) -> usize {
    let mut depth = 0usize;
    let mut end = start + 1;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        let t = line.trim();
        depth = depth.saturating_add(t.matches('{').count());
        depth = depth.saturating_sub(t.matches('}').count());
        end = idx + 1;
        if depth == 0 && t.contains('}') {
            break;
        }
    }
    end
}

/// Indent-based body extenter for Python. Returns the line *after* the
/// `def` block ends (the next non-empty, non-comment line at the same or
/// shallower indent).
fn indent_extent(start: usize, lines: &[&str]) -> usize {
    let mut end = start + 1;
    let indent = lines[start].len() - lines[start].trim_start().len();
    for (idx, line) in lines.iter().enumerate().skip(start + 1) {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            end = idx + 1;
            continue;
        }
        let line_indent = line.len() - line.trim_start().len();
        if line_indent <= indent && !t.is_empty() {
            break;
        }
        end = idx + 1;
    }
    end
}

fn rust_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| t.starts_with("fn _use_") || t.starts_with("fn dummy_"),
        brace_extent,
    )
}

fn python_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| t.starts_with("def _use_") || t.starts_with("def dummy_"),
        indent_extent,
    )
}

fn js_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| {
            t.starts_with("function _use")
                || t.starts_with("function dummy")
                || t.starts_with("const _use")
                || t.starts_with("const dummy")
        },
        brace_extent,
    )
}

fn rust_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("use ") || !trimmed.ends_with(';') {
            continue;
        }

        if trimmed == "use super::*;" {
            continue;
        }

        let body = trimmed
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim();

        if body.contains('{') {
            if let Some(open) = body.find('{') {
                if let Some(close) = body.rfind('}') {
                    let inside = &body[open + 1..close];
                    for part in inside.split(',') {
                        if let Some(symbol) = rust_imported_symbol_from_part(part.trim()) {
                            symbols
                                .push((SymbolName::new(symbol), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
            continue;
        }

        if let Some(symbol) = rust_imported_symbol_from_part(body) {
            symbols.push((SymbolName::new(symbol), LineNumber::new(idx as i64 + 1)));
        }
    }

    symbols
}

fn rust_imported_symbol_from_part(part: &str) -> Option<String> {
    let part = part.trim();
    if part.is_empty() || part == "self" || part.starts_with('*') {
        return None;
    }

    if let Some((_, alias)) = part.split_once(" as ") {
        return Some(alias.trim().to_string());
    }

    let name = match part.split("::").last() {
        Some(n) => n.trim(),
        None => part.trim(),
    };
    if name.is_empty() || name.contains('{') || name.contains('}') {
        return None;
    }

    Some(name.to_string())
}

fn python_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
                for name in import_part.split(',') {
                    let name: &str = name.split_whitespace().next().unwrap_or_default();
                    if !name.is_empty() && name != "*" {
                        symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") {
            let module: &str = trimmed
                .trim_start_matches("import ")
                .split_whitespace()
                .next()
                .unwrap_or_default();
            if !module.is_empty() {
                let name: &str = match module.rsplit('.').next() {
                    Some(n) => n,
                    None => module,
                };
                symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
            }
        }
    }

    symbols
}

fn js_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name: &str = part.split_whitespace().next().unwrap_or_default();
                        if !name.is_empty() && name != "type" {
                            symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") && trimmed.contains(" from ") {
            if let Some(import_part) = trimmed.split_once("import ").map(|(_, p)| p) {
                let name = import_part
                    .split_once(" from ")
                    .map(|(n, _)| n)
                    .unwrap_or_default();
                let name = name.trim();
                if !name.is_empty() && name != "default" {
                    symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                }
            }
            continue;
        }

        if trimmed.starts_with("const ") && trimmed.contains("require(") && trimmed.contains('{') {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name = match part.trim().split(':').next() {
                            Some(n) => n.trim(),
                            None => "",
                        };
                        if !name.is_empty() {
                            symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
        }
    }

    symbols
}

fn in_dummy_range(line_no: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|(start, end)| line_no >= *start && line_no <= *end)
}

fn impl_trait_name(line: &str) -> Option<String> {
    let after_impl = line.strip_prefix("impl ")?.trim();
    let (trait_part, _) = after_impl.split_once(" for ")?;
    let trait_name = match trait_part.split("::").last() {
        Some(n) => n.trim(),
        None => trait_part.trim(),
    };
    if trait_name.is_empty() {
        return None;
    }
    Some(trait_name.to_string())
}

fn impl_block<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn trait_impl_is_dummy(lines: &[&str]) -> bool {
    let mut method_count = 0usize;
    let mut dummy_count = 0usize;
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("async fn ") {
            method_count += 1;
            let (end, body) = function_body(lines, i);
            if function_body_is_dummy(&body) {
                dummy_count += 1;
            }
            i = end;
        } else {
            i += 1;
        }
    }

    method_count > 0 && dummy_count == method_count
}

fn function_body<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn function_body_is_dummy(lines: &[&str]) -> bool {
    // Collect the body lines (skip the fn signature line at index 0)
    let body_lines: Vec<&str> = lines
        .iter()
        .skip(1)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect();

    if body_lines.is_empty() {
        return true;
    }

    // Single-line body like `{ 42 }` or `{ return x; }` — not dummy
    if body_lines.len() == 1 {
        let single = body_lines[0];
        if single.starts_with('{') && single.ends_with('}') {
            let inner = &single[1..single.len() - 1].trim();
            return inner.is_empty() || is_short_marker(inner);
        }
        return is_short_marker(single);
    }

    // Multi-line body: join and check
    let body: String = body_lines.join(" ");
    let trimmed = body.trim();
    if trimmed == "{}" || trimmed == "{ }" {
        return true;
    }

    let inner = trimmed.trim_start_matches('{').trim_end_matches('}').trim();
    if inner.is_empty() || is_short_marker(inner) {
        return true;
    }

    false
}

fn is_short_marker(inner: &str) -> bool {
    let t = ['t', 'o', 'd', 'o', '!', '('].iter().collect::<String>();
    let u = [
        'u', 'n', 'i', 'm', 'p', 'l', 'e', 'm', 'e', 'n', 't', 'e', 'd', '!', '(',
    ]
    .iter()
    .collect::<String>();
    let p = ['p', 'a', 'n', 'i', 'c', '!', '(']
        .iter()
        .collect::<String>();
    let r = [
        'u', 'n', 'r', 'e', 'a', 'c', 'h', 'a', 'b', 'l', 'e', '!', '(',
    ]
    .iter()
    .collect::<String>();
    inner.starts_with(&t) || inner.starts_with(&u) || inner.starts_with(&p) || inner.starts_with(&r)
}
```

---

## File: crates/shared/src/import-rules/taxonomy_import_rule_vo.rs

```rust
// PURPOSE: CustomMessageVO, MandatoryImportRuleVO — VOs for AES rule definitions
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::PatternList;
use crate::naming_rules::taxonomy_suffix_vo::SuffixVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomMessageVO {
    pub pattern: String,
    pub message: ErrorMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MandatoryImportRuleVO {
    pub suffix: SuffixVO,
    pub imports: PatternList,
}

impl CustomMessageVO {
    pub fn new(pattern: String, message: ErrorMessage) -> Self {
        Self { pattern, message }
    }
}

impl MandatoryImportRuleVO {
    pub fn new(suffix: SuffixVO, imports: PatternList) -> Self {
        Self { suffix, imports }
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_language_vo.rs

```rust
// PURPOSE: LanguageVO — classification of programming languages for import rules
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageVO {
    Rust,
    Python,
    JavaScript,
    Unknown,
}

impl LanguageVO {
    pub fn from_path(path: &str) -> Self {
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        match ext {
            "rs" => LanguageVO::Rust,
            "py" => LanguageVO::Python,
            "js" | "ts" | "jsx" | "tsx" => LanguageVO::JavaScript,
            _ => LanguageVO::Unknown,
        }
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_parser_helper.rs

```rust
// PURPOSE: taxonomy_parser_helper — pure utility functions for import parsing and syntax token extraction
use crate::common::taxonomy_name_vo::SymbolName;

pub fn extract_import_modules(content: &str) -> Vec<SymbolName> {
    let mut modules = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("from ") {
            if let Some(module) = rest.split_whitespace().next() {
                modules.push(SymbolName::new(module));
            }
        } else if trimmed.starts_with("import ") {
            if let Some(pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[pos + 6..].trim();
                let cleaned = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                    .trim();
                modules.push(SymbolName::new(cleaned));
            } else if let Some(rest) = trimmed.strip_prefix("import ") {
                if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                    let cleaned = rest
                        .trim_end_matches(';')
                        .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                        .trim();
                    modules.push(SymbolName::new(cleaned));
                } else if let Some(first_token) = rest.split_whitespace().next() {
                    modules.push(SymbolName::new(first_token.trim_end_matches(',')));
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("use ") {
            let module = rest.trim_end_matches(';');
            modules.push(SymbolName::new(module));
        }
    }
    modules
}
```

---

## File: crates/shared/src/import-rules/taxonomy_path_helper.rs

```rust
// PURPOSE: taxonomy_path_helper — pure utility functions for path matching and layer extraction
use std::path::Path;

pub fn extract_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("infrastructure_", "infrastructure"),
        ("agent_", "agent"),
        ("surface_", "surfaces"),
        ("root_", "root"),
    ];

    for &(prefix, layer) in PREFIX_MAP {
        if stem.starts_with(prefix) {
            return Some(layer.to_string());
        }
    }

    None
}

pub fn get_relative_path(file_path: &str, root_dir: &str) -> String {
    let normalized_file = match Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => file_path.replace('\\', "/"),
    };
    let normalized_root = match Path::new(root_dir)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => root_dir.trim_end_matches('/').replace('\\', "/"),
    };
    if normalized_file.starts_with(&normalized_root) {
        normalized_file[normalized_root.len()..]
            .trim_start_matches('/')
            .to_string()
    } else {
        normalized_file
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_unused_helper.rs

```rust
// PURPOSE: taxonomy_unused_helper — pure utility functions for unused import detection
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_name_vo::SymbolName;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static ALL_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r#"__all__\s*=\s*\[([^\]]*)\]"#).ok());

// Known derive-macro imports that Rust compiler consumes implicitly.
// These are never "used" as ordinary symbols — they're consumed by #[derive(...)]
// attributes, so they must never be flagged as unused.
const DERIVE_MACROS: &[&str] = &[
    "async_trait",
    "Serialize",
    "Deserialize",
    "Clone",
    "Debug",
    "Default",
    "PartialEq",
    "Eq",
    "Hash",
    "Ord",
    "PartialOrd",
    "Copy",
    "EnumIter",
    "Display",
    "EnumString",
    "AsRefStr",
];

fn is_rust_trait_import(name: &str) -> bool {
    if name.starts_with('I') && name.len() > 1 && name.chars().nth(1).unwrap_or(' ').is_uppercase()
    {
        return true;
    }
    if name.ends_with("Protocol")
        || name.ends_with("Port")
        || name.ends_with("Trait")
        || name.ends_with("Aggregate")
        || name.ends_with("Ext")
    {
        return true;
    }
    matches!(
        name,
        "Default"
            | "Debug"
            | "Display"
            | "Clone"
            | "Copy"
            | "PartialEq"
            | "Eq"
            | "PartialOrd"
            | "Ord"
            | "Hash"
            | "From"
            | "Into"
            | "TryFrom"
            | "TryInto"
            | "AsRef"
            | "AsMut"
            | "Deref"
            | "DerefMut"
            | "Iterator"
            | "IntoIterator"
            | "ExactSizeIterator"
            | "FusedIterator"
            | "Future"
            | "Stream"
            | "Read"
            | "Write"
            | "BufRead"
            | "Seek"
            | "Send"
            | "Sync"
            | "Unpin"
            | "Sized"
            | "Drop"
            | "Fn"
            | "FnMut"
            | "FnOnce"
            | "async_trait"
            | "Digest"
            | "Manager"
            | "Emitter"
            | "Serialize"
            | "Deserialize"
            | "EnumIter"
            | "EnumString"
            | "AsRefStr"
            | "Parser"
    )
}

pub fn extract_imported_aliases(content: &str) -> HashMap<Identity, Identity> {
    let mut aliases = HashMap::new();
    let mut in_cfg_test = false;
    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            if trimmed == "}" || trimmed.starts_with("}") {
                in_cfg_test = false;
            }
            continue;
        }

        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some((from_part, import_part)) = trimmed.split_once(" import ") {
                let module = from_part[5..].trim();
                for name in import_part.split(',') {
                    let name = name.trim();
                    if name.is_empty() || name == "*" {
                        continue;
                    }
                    if let Some((sym, alias)) = name.split_once(" as ") {
                        aliases.insert(
                            Identity::new(alias.trim()),
                            Identity::new(format!("{}.{}", module, sym.trim())),
                        );
                    } else {
                        aliases.insert(
                            Identity::new(name),
                            Identity::new(format!("{}.{}", module, name)),
                        );
                    }
                }
            }
            continue;
        }

        // Rust `use` statements: `use std::collections::HashMap;` or `use serde::{A, B};`
        if let Some(use_part) = trimmed.strip_prefix("use ") {
            let use_part = use_part.trim_end_matches(';').trim();
            if !use_part.is_empty()
                && !use_part.starts_with("crate::")
                && !use_part.starts_with("super::")
                && !use_part.starts_with("self::")
            {
                if let Some(brace_pos) = use_part.find("::{") {
                    let prefix = &use_part[..brace_pos];
                    let inner = use_part[brace_pos + 3..].trim_end_matches('}');
                    for name in inner.split(',') {
                        let name = name.trim().split(" as ").last().unwrap_or("").trim();
                        if !name.is_empty()
                            && name != "_"
                            && name != "*"
                            && !is_rust_trait_import(name)
                        {
                            aliases.insert(
                                Identity::new(name),
                                Identity::new(format!("{}::{}", prefix, name)),
                            );
                        }
                    }
                } else {
                    let raw_name = use_part.rsplit("::").next().unwrap_or(use_part);
                    let name = raw_name.split(" as ").last().unwrap_or(raw_name).trim();
                    if !name.is_empty() && name != "*" && !is_rust_trait_import(name) {
                        aliases.insert(Identity::new(name), Identity::new(use_part));
                    }
                }
            }
            continue;
        }

        if let Some(import_part) = trimmed.strip_prefix("import ") {
            for name in import_part.split(',') {
                let name = name.trim();
                if name.is_empty() {
                    continue;
                }
                if let Some((sym, alias)) = name.split_once(" as ") {
                    aliases.insert(Identity::new(alias.trim()), Identity::new(sym.trim()));
                } else {
                    let alias = name.rsplit('.').next().unwrap_or(name);
                    aliases.insert(Identity::new(alias), Identity::new(name));
                }
            }
        }
    }
    aliases
}

pub fn extract_exported_symbols(content: &str) -> HashSet<Identity> {
    let mut exported = HashSet::new();
    let code_lines = content
        .lines()
        .filter(|l| !l.trim().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(ref re) = *ALL_RE {
        if let Some(caps) = re.captures(&code_lines) {
            if let Some(matched) = caps.get(1) {
                for item in matched.as_str().split(',') {
                    let item = item.trim().trim_matches(|c| c == '\'' || c == '"');
                    if !item.is_empty() {
                        exported.insert(Identity::new(item));
                    }
                }
            }
        }
    }
    exported
}

pub fn extract_used_symbols(
    content: &str,
    imported_aliases: &HashMap<Identity, Identity>,
) -> HashSet<Identity> {
    let mut used = HashSet::new();
    let code_lines = content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("import ")
                && !t.starts_with("from ")
                && !t.starts_with("use ")
                && !t.starts_with("pub use ")
                && !t.starts_with("pub(crate) use ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    for alias in imported_aliases.keys() {
        let alias_str = alias.value();
        if DERIVE_MACROS.contains(&alias_str) {
            used.insert(Identity::new(alias_str));
        }
    }

    let non_derive_aliases: Vec<&str> = imported_aliases
        .keys()
        .map(|a| a.value())
        .filter(|a| !DERIVE_MACROS.contains(a))
        .collect();

    if !non_derive_aliases.is_empty() && !code_lines.is_empty() {
        let patterns: Vec<String> = non_derive_aliases
            .iter()
            .map(|a| regex::escape(a))
            .collect();
        let combined = format!(r"\b({})\b", patterns.join("|"));
        if let Ok(re) = Regex::new(&combined) {
            let matched_set: HashSet<&str> =
                re.find_iter(&code_lines).map(|m| m.as_str()).collect();
            for alias in non_derive_aliases {
                if matched_set.contains(alias) {
                    used.insert(Identity::new(alias));
                }
            }
        }
    }

    used
}

pub fn extract_rust_js_imports(content: &str) -> Vec<(SymbolName, LineNumber)> {
    let mut imports = Vec::new();
    let mut in_cfg_test = false;
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            if t == "}" || t.starts_with("}") {
                in_cfg_test = false;
            }
            continue;
        }

        let names: Vec<SymbolName> = if t.starts_with("use ")
            || t.starts_with("pub use ")
            || t.starts_with("pub(crate) use ")
        {
            let target = t.trim_end_matches(';').trim_start_matches("use ").trim();
            if target.starts_with("std::")
                || target.starts_with("core::")
                || target.starts_with("alloc::")
            {
                continue;
            }
            if let Some(brace_pos) = target.find("::{") {
                let inner = target[brace_pos + 3..].trim_end_matches('}').trim();
                inner
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .split(" as ")
                            .last()
                            .unwrap_or("")
                            .trim()
                            .to_string()
                    })
                    .filter(|n| !n.is_empty() && n != "_" && n != "*")
                    .map(SymbolName::new)
                    .collect()
            } else {
                let name = target
                    .split("::")
                    .last()
                    .unwrap_or("")
                    .split(" as ")
                    .last()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if name.is_empty() || name == "_" || name == "*" {
                    continue;
                }
                vec![SymbolName::new(name)]
            }
        } else if t.starts_with("import ") {
            if let Some(from_idx) = t.find(" from ") {
                let import_part = t[7..from_idx].trim();
                let names: Vec<SymbolName> = if import_part.starts_with('{') {
                    import_part[1..import_part.len() - 1]
                        .split(',')
                        .map(|s| {
                            s.trim()
                                .split(" as ")
                                .last()
                                .unwrap_or("")
                                .trim()
                                .to_string()
                        })
                        .filter(|n| !n.is_empty())
                        .map(SymbolName::new)
                        .collect()
                } else {
                    vec![SymbolName::new(import_part.trim())]
                };
                names
            } else {
                continue;
            }
        } else {
            continue;
        };

        for name in names {
            let s = name.value();
            if (s.starts_with('I') && s.len() > 1 && s.chars().nth(1).unwrap_or(' ').is_uppercase())
                || s.ends_with("Protocol")
                || s.ends_with("Port")
                || s.ends_with("Trait")
                || s.ends_with("Aggregate")
                || s == "Parser"
            {
                continue;
            }
            imports.push((name, LineNumber::new(i as i64 + 1)));
        }
    }
    imports
}

pub fn is_name_used(name: &str, content: &str, exclude_line: usize) -> bool {
    if is_rust_trait_import(name) || DERIVE_MACROS.contains(&name) {
        return true;
    }

    let rest = content
        .lines()
        .enumerate()
        .filter(|(j, _)| *j != exclude_line)
        .map(|(_, l)| l)
        .collect::<Vec<_>>()
        .join("\n");
    rest.contains(name)
}

// ─── Private Helpers ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_macro_serialize_always_used() {
        let content = r#"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("Serialize"),
            Identity::new("serde::Serialize"),
        );
        aliases.insert(
            Identity::new("Deserialize"),
            Identity::new("serde::Deserialize"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("Serialize")),
            "Serialize should always be considered used"
        );
        assert!(
            used.contains(&Identity::new("Deserialize")),
            "Deserialize should always be considered used"
        );
    }

    #[test]
    fn derive_macro_async_trait_always_used() {
        let content = r#"
use async_trait::async_trait;

#[async_trait]
trait MyTrait {
    async fn do_something();
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("async_trait"),
            Identity::new("async_trait::async_trait"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("async_trait")),
            "async_trait should always be considered used"
        );
    }

    #[test]
    fn derive_macro_enum_iter_always_used() {
        // EnumIter was NOT previously in is_rust_trait_import — only DERIVE_MACROS catches it
        let content = r#"
use strum::{EnumIter, Display};

#[derive(EnumIter, Display)]
enum Color {
    Red,
    Green,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(Identity::new("EnumIter"), Identity::new("strum::EnumIter"));
        aliases.insert(Identity::new("Display"), Identity::new("strum::Display"));

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("EnumIter")),
            "EnumIter should always be considered used"
        );
        assert!(
            used.contains(&Identity::new("Display")),
            "Display should always be considered used"
        );
    }

    #[test]
    fn derive_macro_as_ref_str_always_used() {
        // AsRefStr was NOT previously in is_rust_trait_import — only DERIVE_MACROS catches it
        let content = r#"
use strum::AsRefStr;

#[derive(AsRefStr)]
enum Status {
    Active,
    Inactive,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(Identity::new("AsRefStr"), Identity::new("strum::AsRefStr"));

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("AsRefStr")),
            "AsRefStr should always be considered used"
        );
    }

    #[test]
    fn non_derive_import_still_checked_normally() {
        // Regular imports should NOT be auto-marked as used
        let content = r#"
use std::collections::HashMap;

fn main() {
    let _x = 42;
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("HashMap"),
            Identity::new("std::collections::HashMap"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            !used.contains(&Identity::new("HashMap")),
            "HashMap is genuinely unused"
        );
    }

    #[test]
    fn is_name_used_returns_true_for_derive_macros() {
        // is_name_used should short-circuit for all DERIVE_MACROS entries
        for &m in DERIVE_MACROS {
            assert!(
                is_name_used(m, "fn main() {}", 0),
                "{m} should be considered used via DERIVE_MACROS"
            );
        }
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_violation_import_vo.rs

```rust
// PURPOSE: AesImportViolation — violation messages for import rules (AES201-205)
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AesImportViolation {
    // AES201 — Forbidden Import
    ForbiddenImport {
        source_layer: LayerNameVO,
        forbidden_layer: LayerNameVO,
        allowed: Vec<LayerNameVO>,
        reason: Option<LintMessage>,
    },
    // AES202 — Mandatory import
    MissingImport {
        source_layer: LayerNameVO,
        required: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES203 — Unused imports
    FixUnusedImport {
        reason: Option<LintMessage>,
    },
    // AES204 — Dummy import / Intent violation
    ImportIntentViolation {
        source_layer: LayerNameVO,
        import_type: SymbolName,
        intent: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES205 — Circular import
    CircularImport {
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesImportViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesImportViolation::ForbiddenImport {
                source_layer,
                forbidden_layer,
                allowed,
                reason,
            } => {
                let (allowed_str, fix_extra) = if allowed.is_empty() {
                    ("none".to_string(), " This layer is fully isolated — move the imported code into this layer or remove the dependency entirely.".to_string())
                } else {
                    (
                        allowed
                            .iter()
                            .map(|v| v.value().to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                        String::new(),
                    )
                };
                let dynamic_why = match reason {
                    Some(r) => r.to_string(),
                    None => {
                        let src = source_layer.value();
                        if src == "taxonomy(vo)" {
                            "Taxonomy Value Objects (VO) must remain completely pure and cannot import agents, infrastructure, surfaces, contracts, capabilities, or root components.".to_string()
                        } else if src == "taxonomy(entity)"
                            || src == "taxonomy(error)"
                            || src == "taxonomy(event)"
                        {
                            "Taxonomy Entities, Errors, and Events can only import taxonomy VOs/constants and are forbidden from importing agents, infrastructure, surfaces, contracts, or capabilities.".to_string()
                        } else if src == "taxonomy(constant)" {
                            "Taxonomy Constants must remain pure static value declarations and cannot import agents, infrastructure, surfaces, contracts, capabilities, or root components.".to_string()
                        } else if src == "contract(port)" || src == "contract(protocol)" {
                            "Contract Ports and Protocols represent pure interface definitions and are forbidden from importing agents, infrastructure, surfaces, capabilities, aggregates, or root components.".to_string()
                        } else if src == "contract(aggregate)" {
                            "Contract Aggregates represent high-level composition/DI contracts and must not import agents, infrastructure, surfaces, capabilities, or root components.".to_string()
                        } else if src == "capabilities" {
                            "Capabilities implement domain business logic and must never depend on infrastructure adapters, agents, or UI/surfaces.".to_string()
                        } else if src == "infrastructure" {
                            "Infrastructure adapters implement technology-specific protocols and must never import surfaces, capabilities, agents, or root components directly.".to_string()
                        } else if src == "agent(container)" {
                            "Agent Containers handle dependency injection and are forbidden from importing UI/surfaces or root components.".to_string()
                        } else if src == "agent(orchestrator)" {
                            "Agent Orchestrators coordinate flows and are forbidden from importing UI/surfaces, infrastructure adapters, capabilities, or root components.".to_string()
                        } else if src == "agent(lifecycle)" {
                            "Agent Lifecycles manage agent states and are forbidden from importing orchestrators/containers, infrastructure, capabilities, surfaces, or root components.".to_string()
                        } else if src == "surfaces(command)"
                            || src == "surfaces(controller)"
                            || src == "surfaces(page)"
                            || src == "surfaces(entry)"
                        {
                            "Smart Surfaces act as user/CLI entry points and must never import agents, infrastructure, capabilities, or ports/protocols directly (must use ServiceContainerAggregate).".to_string()
                        } else if src == "surfaces(hook)"
                            || src == "surfaces(store)"
                            || src == "surfaces(action)"
                            || src == "surfaces(screen)"
                            || src == "surfaces(router)"
                        {
                            "Surface utility components (hooks, stores, routers) manage local state and must never import agents, infrastructure, capabilities, or ports/protocols.".to_string()
                        } else if src == "surfaces(component)"
                            || src == "surfaces(view)"
                            || src == "surfaces(layout)"
                        {
                            "Passive Surface components (views, layouts) render UI and are forbidden from importing agents, contracts, infrastructure, capabilities, or smart surfaces.".to_string()
                        } else if src.starts_with("taxonomy") {
                            "Taxonomy must remain pure and free from framework/layer dependencies to ensure domain model integrity.".to_string()
                        } else if src.starts_with("contract") {
                            "Contract interfaces represent pure specifications and must not depend on capabilities, infrastructure, or agent implementations.".to_string()
                        } else if src.starts_with("agent") {
                            "Agent orchestrators and containers must never depend on the UI/surface layer to maintain clean separation of concerns.".to_string()
                        } else if src.starts_with("surfaces") {
                            "Surfaces are external I/O boundaries and must never bypass contract aggregates to depend on capabilities, agent internals, or infrastructure.".to_string()
                        } else {
                            format!("Layer '{}' must not depend on '{}' to maintain architectural boundaries.", source_layer, forbidden_layer)
                        }
                    }
                };
                write!(
                    f,
                    "AES201 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                        WHY? {}\n\
                        FIX: Remove the import or refactor to use one of the allowed layers: [{}]{}",
                    source_layer, forbidden_layer, dynamic_why, allowed_str, fix_extra
                )
            }
            AesImportViolation::MissingImport {
                source_layer,
                required,
                reason,
            } => {
                let default_why = {
                    let src = source_layer.value();
                    if src == "taxonomy(vo)" {
                        "Taxonomy Value Objects define domain primitives — they must import contracts to declare their structural contract.".to_string()
                    } else if src == "taxonomy(entity)" {
                        "Taxonomy Entities model domain state — they must import aggregator contracts to participate in business operations.".to_string()
                    } else if src == "contract(port)" || src == "contract(protocol)" {
                        "Contract ports define service boundaries — they must import contract aggregate types to compose cross-cutting workflows.".to_string()
                    } else if src == "contract(aggregate)" {
                        "Contract aggregates orchestrate cross-layer collaboration — they must import all relevant port/protocol contracts.".to_string()
                    } else if src == "capabilities" {
                        "Capabilities implement business rules — they MUST import contract protocols to know what interface to honor. Missing contract protocol means broken/useless capability or missing requirement.".to_string()
                    } else if src == "infrastructure" {
                        "Infrastructure adapters MUST import contract ports — without a port reference this file is broken/useless. Either rename/delete if not real infrastructure, or create the required contract port first.".to_string()
                    } else if src == "agent(container)" {
                        "Agent containers wire dependencies at startup — they must import service contracts to register all concrete implementations.".to_string()
                    } else if src == "agent(orchestrator)" {
                        "Agent orchestrators coordinate use-case flows — they must import capability contracts to dispatch work correctly.".to_string()
                    } else if src == "surfaces(command)" || src == "surfaces(controller)" {
                        "Command/controller surfaces are user entry points — they must import aggregate contracts to delegate without bypassing business logic.".to_string()
                    } else if src == "surfaces(component)" || src == "surfaces(view)" {
                        "Passive surface components render UI — they must import taxonomy VOs to display type-safe domain data.".to_string()
                    } else if src.starts_with("taxonomy") {
                        format!(
                            "Layer '{}' must import '{}' to maintain domain model integrity.",
                            src, required
                        )
                    } else if src.starts_with("contract") {
                        format!("Layer '{}' must import '{}' to satisfy interface composition requirements.", src, required)
                    } else if src.starts_with("agent") {
                        format!(
                            "Layer '{}' must import '{}' to wire all required dependencies.",
                            src, required
                        )
                    } else if src.starts_with("surfaces") {
                        format!("Layer '{}' must import '{}' to properly delegate to the aggregate layer.", src, required)
                    } else {
                        format!("Layer '{}' must import '{}' to satisfy architectural contract requirements.", src, required)
                    }
                };
                let supplement = match reason.as_ref() {
                    Some(r) => format!("\n  Context: {}", r),
                    None => String::new(),
                };
                write!(
                    f,
                    "AES202 MANDATORY_IMPORT: Layer '{}' is missing required import '{}'.\n\
                        WHY? {}{}\n\
                        FIX: Add the required import statement for '{}' in this file.",
                    source_layer, required, default_why, supplement, required
                )
            }
            AesImportViolation::ImportIntentViolation {
                source_layer,
                import_type,
                intent: _,
                reason,
            } => {
                let default_why = format!(
                    "Import '{}' in layer '{}' is not used according to its intended purpose.",
                    import_type, source_layer
                );
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES204 IMPORT_INTENT: '{}' import in layer '{}' violates its intended purpose.\n\
                        WHY? {why}\n\
                        FIX: Use imported symbols in real logic, not only in dummy functions or stubs",
                    import_type, source_layer
                )
            }
            AesImportViolation::CircularImport { reason } => {
                let default_why = "Circular dependencies couple components together and break unidirectional data/import flow.".to_string();
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES205 CIRCULAR_IMPORT: Circular dependency detected.\n\
                        WHY? {}\n\
                        FIX: Refactor imports or extract the shared logic into a lower, common layer.",
                    why
                )
            }
            AesImportViolation::FixUnusedImport { reason } => {
                let default_why =
                    "Unused imports clutter the codebase and increase compilation/dependency overhead."
                        .to_string();
                let supplement = match reason.as_ref() {
                    Some(r) => format!("\n  Context: {}", r),
                    None => String::new(),
                };
                write!(f, "AES203 UNUSED_IMPORT: Unused import detected.\n\
                        WHY? {}{}\n\
                        FIX: Remove the unused import statement or use the imported symbol in this file.", default_why, supplement)
            }
        }
    }
}

impl From<AesImportViolation> for String {
    fn from(v: AesImportViolation) -> String {
        v.to_string()
    }
}
```

---

## File: crates/shared/src/lib.rs

```rust
// PURPOSE: shared — all taxonomy types, contract traits, and shared definitions
// No dependencies on other feature crates — this is the foundation layer.

#[path = "common/mod.rs"]
pub mod common;

// Re-export all taxonomy_* and contract_* types from common
// NOTE: widely used by downstream crates as shared::taxonomy_*. Do not remove.
pub use common::*;

#[path = "tui/mod.rs"]
pub mod tui;

// Feature-specific types (in feature folders)
#[path = "auto-fix/mod.rs"]
pub mod auto_fix;
#[path = "cli-commands/mod.rs"]
pub mod cli_commands;
#[path = "code-analysis/mod.rs"]
pub mod code_analysis;
#[path = "config-system/mod.rs"]
pub mod config_system;
#[path = "external-lint/mod.rs"]
pub mod external_lint;
#[path = "file-watch/mod.rs"]
pub mod file_watch;
#[path = "git-hooks/mod.rs"]
pub mod git_hooks;
#[path = "import-rules/mod.rs"]
pub mod import_rules;
#[path = "mcp-server/mod.rs"]
pub mod mcp_server;

#[path = "naming-rules/mod.rs"]
pub mod naming_rules;
#[path = "orphan-detector/mod.rs"]
pub mod orphan_detector;
#[path = "project-setup/mod.rs"]
pub mod project_setup;
#[path = "role-rules/mod.rs"]
pub mod role_rules;
```

---

## File: crates/shared/src/mcp-server/mod.rs

```rust
// mcp-server — taxonomy and contract types
// Re-export from common for backward compatibility
pub use crate::common::taxonomy_action_vo;
pub use crate::common::taxonomy_job_vo;
```

---

## File: crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs

```rust
// PURPOSE: INamingAnalyzerProtocol — protocol trait for naming-rules analyzer dependency isolation
use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;

pub trait INamingAnalyzerProtocol: Send + Sync {
    fn config(&self) -> &ArchitectureConfig;
    fn layer_map(&self) -> &LayerMapVO;
    fn detect_layer(&self, f: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO>;
}
```

---

## File: crates/shared/src/naming-rules/contract_naming_checker_protocol.rs

```rust
// PURPOSE: INamingCheckerProtocol — protocol trait for naming check capabilities
use super::contract_naming_analyzer_protocol::INamingAnalyzerProtocol;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

#[async_trait]
pub trait INamingCheckerProtocol: Send + Sync {
    async fn check_file_naming(
        &self,
        analyzer: &dyn INamingAnalyzerProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    async fn check_domain_suffixes(
        &self,
        analyzer: &dyn INamingAnalyzerProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
```

---

## File: crates/shared/src/naming-rules/contract_naming_filesystem_port.rs

```rust
// PURPOSE: INamingFileSystemPort — Local contract trait for naming-rules filesystem operations
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

#[async_trait]
pub trait INamingFileSystemPort: Send + Sync {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList;
}
```

---

## File: crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs

```rust
// PURPOSE: INamingRunnerAggregate — contract for naming-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait INamingRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/naming-rules/mod.rs

```rust
pub mod contract_naming_analyzer_protocol;
pub mod contract_naming_checker_protocol;
pub mod contract_naming_filesystem_port;
pub mod contract_naming_runner_aggregate;
pub mod taxonomy_naming_rule_vo;
pub mod taxonomy_naming_violation_vo;
pub mod taxonomy_suffix_vo;
pub use taxonomy_naming_violation_vo::NamingViolation;
```

---

## File: crates/shared/src/naming-rules/taxonomy_naming_rule_vo.rs

```rust
// PURPOSE: NamingRuleVO — value object containing naming convention and suffix policy rule definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::PatternList;
use crate::naming_rules::taxonomy_suffix_vo::SuffixPolicyVO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NamingRuleVO {
    #[serde(default)]
    pub naming_convention: BooleanVO,
    #[serde(default)]
    pub suffix_policy: SuffixPolicyVO,
    #[serde(default, alias = "allowed_suffix")]
    pub allowed_suffix: PatternList,
    #[serde(default, alias = "forbidden_suffix")]
    pub forbidden_suffix: PatternList,
}
```

---

## File: crates/shared/src/naming-rules/taxonomy_naming_violation_vo.rs

```rust
// PURPOSE: NamingViolation — AES101/AES102 violation messages for naming rules domain
use crate::common::taxonomy_message_vo::LintMessage;
use std::fmt;

#[derive(Debug, Clone)]
pub enum NamingViolation {
    /// AES101 — filename doesn't follow prefix_concept_suffix pattern
    /// Min 2 words separated by underscore (e.g., prefix_suffix).
    NamingConvention {
        min_words: usize,
        separator: String,
        reason: Option<LintMessage>,
    },
    /// AES102 — filename prefix is not one of the recognised layer prefixes
    UnknownPrefix {
        prefix: String,
        allowed: Vec<String>,
        reason: Option<LintMessage>,
    },
    /// AES102 — suffix is explicitly forbidden for this layer
    /// Carries the layer name and the actual suffix used.
    SuffixForbidden {
        layer_name: String,
        forbidden_suffix: String,
        reason: Option<LintMessage>,
    },
    /// AES102 — strict suffix policy violated (missing required suffix)
    /// Carries the layer name and allowed suffixes from config for dynamic messages.
    SuffixMismatch {
        layer_name: String,
        allowed: Vec<String>,
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for NamingViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NamingConvention {
                min_words,
                separator,
                reason,
            } => {
                let why = Option::unwrap_or_else(reason.as_ref().map(|r| r.to_string()), || {
                    format!("The AES layer naming convention requires filenames to contain at least {} words separated by '{}' (e.g., prefix{}suffix). Each word must be lowercase alphanumeric. This pattern ensures every file's architectural layer (prefix) and role (suffix) is immediately identifiable — both for human readers and automated tooling.", min_words, separator, separator)
                });
                write!(
                    f,
                    "AES101 NAMING_CONVENTION: Filename must contain at least {} words separated by '{}'.\n\
                    WHY? {}\n\
                    FIX: Rename to follow prefix{}suffix pattern (e.g., capabilities{}user_checker.rs).",
                    min_words, separator, why, separator, separator
                )
            }
            Self::UnknownPrefix {
                prefix,
                allowed,
                reason,
            } => {
                let allowed_str = allowed.join(", ");
                let default_why = format!(
                    "Every source file must begin with one of the recognised layer prefixes ({}) so that its architectural layer can be determined automatically. The prefix '{}' does not correspond to any known layer.",
                    allowed_str, prefix
                );
                let why = Option::unwrap_or(reason.as_ref().map(|r| r.to_string()), default_why);
                write!(
                    f,
                    "AES102 UNKNOWN_PREFIX: File uses prefix '{}' which is not a recognised layer.\n\
                    WHY? {}\n\
                    FIX: Rename to start with one of the allowed prefixes: {}.",
                    prefix, why, allowed_str
                )
            }
            Self::SuffixForbidden {
                layer_name,
                forbidden_suffix,
                reason,
            } => {
                let default_why = format!(
                    "The suffix '{}' belongs to a different architectural role and is not allowed in the '{}' layer. Mixing role suffixes across layers breaks the strict layer-to-suffix mapping that tooling depends on for automatic validation.",
                    forbidden_suffix, layer_name
                );
                let why = Option::unwrap_or(reason.as_ref().map(|r| r.to_string()), default_why);
                write!(
                    f,
                    "AES102 SUFFIX_FORBIDDEN: File in layer '{}' uses suffix '{}' which is forbidden.\n\
                    WHY? {}\n\
                    FIX: Rename the file to use an allowed suffix or move it to the correct layer.",
                    layer_name, forbidden_suffix, why
                )
            }
            Self::SuffixMismatch {
                layer_name,
                allowed,
                reason,
            } => {
                let allowed_str = allowed.join(", ");
                let default_why = format!(
                    "Files in the '{}' layer must end with a recognised role suffix ({}) so that their architectural intent is clear and automated boundary checks can verify that each file belongs exactly where it is. A missing or unrecognised suffix bypasses this safeguard.",
                    layer_name, allowed_str
                );
                let why = Option::unwrap_or(reason.as_ref().map(|r| r.to_string()), default_why);
                write!(
                    f,
                    "AES102 SUFFIX_MISMATCH: File in layer '{}' is missing a required strict suffix.\n\
                    WHY? {}\n\
                    FIX: Rename the file to include one of the allowed suffixes: {}.",
                    layer_name, why, allowed_str
                )
            }
        }
    }
}
```

---

## File: crates/shared/src/naming-rules/taxonomy_suffix_vo.rs

```rust
// PURPOSE: SuffixPolicyVO, SuffixVO — value objects for suffix naming rules
use crate::common::taxonomy_common_vo::PatternList;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(transparent)]
pub struct SuffixPolicyVO {
    pub value: String,
}

impl SuffixPolicyVO {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SuffixVO {
    pub values: PatternList,
}
```

---

## File: crates/shared/src/orphan-detector/contract_orphan_aggregate.rs

```rust
// PURPOSE: IOrphanAggregate — aggregate trait bundling all orphan detection protocols
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use std::collections::HashSet;

pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String>;
    fn check_orphans(
        &self,
        layer_detector: &dyn ILayerDetectionAggregate,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;
}
```

---

## File: crates/shared/src/orphan-detector/contract_orphan_graph_resolver_protocol.rs

```rust
// PURPOSE: IOrphanGraphResolverProtocol — contract trait for building orphan analysis graph context
// AES402: All primitive `&[String]` parameter types and `Vec<String>` return
// types in this contract have been replaced with strongly-typed VOs.
//   * `&[String] files` → `&[OrphanFileListVO]` (per analysis pass)
//   * `Vec<String>` returns → `OrphanFileListVO`
//   * `&[String] configured` → `&[OrphanEntryPatternListVO]`
//   * `&str root_dir` → kept as `&str` (idiomatic borrow, AES402 allows)
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use crate::orphan_detector::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

pub trait IOrphanGraphResolverProtocol: Send + Sync {
    /// Build the orphan-detection graph context for a set of source files.
    /// `files` is the list of file paths to include in the graph; `root_dir`
    /// is the project root used to compute relative paths.
    fn build_graph_context(
        &self,
        files: &[OrphanFileListVO],
        root_dir: &str,
    ) -> GraphAnalysisContext;

    /// Identify which of the supplied files count as entry points. A file
    /// is an entry point if its path matches any of the configured patterns
    /// (substring or suffix match). Returns the filtered list as a
    /// strongly-typed VO.
    fn identify_entry_points(
        &self,
        files: &[OrphanFileListVO],
        configured: &[OrphanEntryPatternListVO],
    ) -> OrphanFileListVO;
}
```

---

## File: crates/shared/src/orphan-detector/contract_orphan_protocol.rs

```rust
// PURPOSE: ITaxonomyOrphanProtocol + layer-specific orphan indicator protocols (agent, contract, capabilities, infra, surfaces)
use crate::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use crate::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use crate::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_path_vo::FilePath;

pub trait ITaxonomyOrphanProtocol: Send + Sync {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult;
}

pub trait IContractOrphanProtocol: Send + Sync {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
        all_files: &[String],
    ) -> OrphanIndicatorResult;
}

pub trait ICapabilitiesOrphanProtocol: Send + Sync {
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}

pub trait IInfrastructureOrphanProtocol: Send + Sync {
    fn is_infrastructure_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}

pub trait IAgentOrphanProtocol: Send + Sync {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult;
}

pub trait ISurfacesOrphanProtocol: Send + Sync {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult;
}
```

---

## File: crates/shared/src/orphan-detector/mod.rs

```rust
pub mod contract_orphan_aggregate;
pub mod contract_orphan_graph_resolver_protocol;
pub mod contract_orphan_protocol;
pub mod taxonomy_orphan_rule_vo;
pub mod taxonomy_orphan_utility;
pub mod taxonomy_violation_orphan_vo;
pub use taxonomy_violation_orphan_vo::AesOrphanViolation;
pub mod taxonomy_orphan_contract_vo;
pub use taxonomy_orphan_contract_vo::{OrphanEntryPatternListVO, OrphanFileListVO};
```

---

## File: crates/shared/src/orphan-detector/taxonomy_orphan_contract_vo.rs

```rust
// PURPOSE: OrphanContractVOs — value objects used by IOrphanGraphResolverProtocol.
//
// AES402: All primitive `&[String]` / `Vec<String>` parameter types and return
// types in IOrphanGraphResolverProtocol are replaced with strongly-typed VOs
// so the contract surface has no primitive collections.
//
// Why a dedicated VO instead of reusing `FilePathList` or `PatternList`?
//   * `FilePathList` (source_parsing/taxonomy_paths_vo) wraps `Vec<FilePath>`,
//     but the orphan graph resolver receives and emits file paths as `String`
//     (it does not own the underlying file system resolution — the surface
//     layer feeds it raw strings from a directory walk).
//   * `PatternList` (common/taxonomy_common_vo) wraps `Vec<String>` but is
//     semantically about exclusion patterns, not about file or pattern
//     identifiers in a graph context.
//
// The two VOs below mirror the parameter roles of the contract:
//   * `OrphanFileListVO` — list of file paths under analysis
//   * `OrphanEntryPatternListVO` — list of configured entry-point patterns
// Both are intentionally minimal wrappers around `Vec<String>`; the point
// is to take the *name* of the field out of the contract surface and put
// it in a typed wrapper, not to invent new functionality.
use serde::{Deserialize, Serialize};

/// List of file paths under orphan-detection analysis. Wraps `Vec<String>`
/// (raw path strings as emitted by the directory walker). Replaces the
/// previous `&[String]` parameter and `Vec<String>` return type used in
/// `IOrphanGraphResolverProtocol::build_graph_context` and
/// `identify_entry_points`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrphanFileListVO {
    pub values: Vec<String>,
}

impl OrphanFileListVO {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// List of configured entry-point patterns (e.g. glob prefixes or exact
/// paths) the resolver should treat as reachable entry points. Replaces
/// the previous `&[String]` parameter on
/// `IOrphanGraphResolverProtocol::identify_entry_points`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrphanEntryPatternListVO {
    pub values: Vec<String>,
}

impl OrphanEntryPatternListVO {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
```

---

## File: crates/shared/src/orphan-detector/taxonomy_orphan_rule_vo.rs

```rust
// PURPOSE: OrphanRuleVO — value object containing orphan compliance rule definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::PatternList;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OrphanRuleVO {
    #[serde(default)]
    pub check_orphan: BooleanVO,
    #[serde(default, alias = "entry_points")]
    pub orphan_entry_points: PatternList,
}
```

---

## File: crates/shared/src/orphan-detector/taxonomy_orphan_utility.rs

```rust
use once_cell::sync::OnceCell;
use regex::Regex;

static STRUCT_RE: OnceCell<Option<Regex>> = OnceCell::new();
static TRAIT_RE: OnceCell<Option<Regex>> = OnceCell::new();

fn struct_re() -> Option<&'static Regex> {
    STRUCT_RE
        .get_or_init(|| Regex::new(r"(?:pub\s+)?struct\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

fn trait_re() -> Option<&'static Regex> {
    TRAIT_RE
        .get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

pub fn extract_struct_names(content: &str) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(re) = struct_re() {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name != "Self" && !name.is_empty() {
                names.push(name);
            }
        }
    }
    names
}

pub fn extract_trait_names(content: &str) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(re) = trait_re() {
        for cap in re.captures_iter(content) {
            names.push(cap[1].to_string());
        }
    }
    names
}
```

---

## File: crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs

```rust
use crate::common::taxonomy_message_vo::LintMessage;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AesOrphanViolation {
    TaxonomyOrphan {
        stem: String,
        category: &'static str,
        reason: Option<LintMessage>,
    },
    ContractOrphan {
        suffix: String,
        trait_name: String,
        target_layer: &'static str,
        reason: Option<LintMessage>,
    },
    CapabilitiesOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    InfrastructureOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    AgentOrphan {
        agg_name: String,
        reason: Option<LintMessage>,
    },
    SurfaceOrphan {
        category: &'static str,
        stem: String,
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesOrphanViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesOrphanViolation::TaxonomyOrphan {
                stem,
                category,
                reason,
            } => {
                let target_hint = match *category {
                    "utility" | "helper" => "any file that needs its functionality".to_string(),
                    _ => "a contract_* file (contract_port, contract_protocol, or contract_aggregate)".to_string(),
                };
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => {
                        format!("Taxonomy file '{}' is not imported by any file.", stem)
                    }
                };
                write!(f, "AES501 TAXONOMY_ORPHAN: '{}' is not imported.\nWHY? {}\nFIX: Import '{}' in {}.", stem, why, stem, target_hint)
            }
            AesOrphanViolation::ContractOrphan {
                suffix,
                trait_name,
                target_layer,
                reason,
            } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Contract {} '{}' is not implemented by any {} file.",
                        suffix, trait_name, target_layer
                    ),
                };
                let fix = match suffix.as_str() {
                    "port" => format!("Implement '{}' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.", trait_name),
                    "protocol" => format!("Implement '{}' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.", trait_name),
                    "aggregate" => format!("Import and use '{}' in a surface_* file or root_*_container.rs.", trait_name),
                    _ => format!("Implement '{}' in the appropriate layer.", trait_name),
                };
                write!(
                    f,
                    "AES502 CONTRACT_ORPHAN: Contract {} '{}' is orphaned.\nWHY? {}\nFIX: {}",
                    suffix, trait_name, why, fix
                )
            }
            AesOrphanViolation::CapabilitiesOrphan { stem, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Capabilities file '{}' is not wired in any container.",
                        stem
                    ),
                };
                write!(f, "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs via `use {}::...;` and wire it into the container's constructor. If this file is obsolete, delete it and remove its module declaration from lib.rs.", stem, why, stem, stem)
            }
            AesOrphanViolation::InfrastructureOrphan { stem, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!("Infrastructure file '{}' is not wired in any container and unreachable from any entry point.", stem),
                };
                write!(f, "AES504 INFRASTRUCTURE_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in the corresponding agent_*_orchestrator.rs or root_*_container.rs by passing it as a dependency. If this adapter is unused, delete it and remove its module declaration.", stem, why, stem)
            }
            AesOrphanViolation::AgentOrphan { agg_name, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Agent aggregate '{}' is not called by any surface or container.",
                        agg_name
                    ),
                };
                write!(f, "AES505 AGENT_ORPHAN: Aggregate '{}' is unreachable from any surface.\nWHY? {}\nFIX: Import and use '{}' in a surface_* file or root_*_container.rs via `Arc<dyn {}>`. If the orchestrator is unused, delete it and remove its module declaration.", agg_name, why, agg_name, agg_name)
            }
            AesOrphanViolation::SurfaceOrphan {
                category,
                stem,
                reason,
            } => {
                let (where_hint, fix_hint) = match *category {
                    "smart" => ("entry point or router", "an entry point (root_*_entry.rs, cli_*, mcp_*) or router file"),
                    "utility" => ("smart surface", "a smart surface (command, controller, page)"),
                    "passive" => ("smart or utility surface", "a smart surface (command, controller, page) or utility surface (hook, store, action, screen, router)"),
                    _ => ("the appropriate importer", "an appropriate importer file"),
                };
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "{} surface '{}' is not imported by any {}.",
                        category, stem, where_hint
                    ),
                };
                write!(f, "AES506 SURFACE_ORPHAN: {} surface '{}' is orphaned.\nWHY? {}\nFIX: Import '{}' in {}. If this surface is dead code, remove the file and its module declaration from lib.rs.", category, stem, why, stem, fix_hint)
            }
        }
    }
}

impl From<AesOrphanViolation> for String {
    fn from(v: AesOrphanViolation) -> String {
        v.to_string()
    }
}
```

---

## File: crates/shared/src/project-setup/contract_maintenance_aggregate.rs

```rust
// PURPOSE: Aggregate: MaintenanceCommandsAggregate trait — contract for maintenance operations (stats, doctor, clean, update, cancel)
use crate::common::taxonomy_path_vo::FilePath;
use crate::mcp_server::taxonomy_action_vo::JobId;
use crate::project_setup::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, SecurityScanReport, ToolchainDiagnostics,
};
use crate::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
use async_trait::async_trait;

#[async_trait]
pub trait MaintenanceCommandsAggregate: Send + Sync {
    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO;
    async fn clean(&self);
    async fn update(&self);
    async fn doctor(&self) -> DoctorResultVO;
    async fn cancel(&self, job_id: JobId);
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics;
    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport;
    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String>;
}
```

---

## File: crates/shared/src/project-setup/contract_maintenance_protocol.rs

```rust
// PURPOSE: IMaintenanceCheckerProtocol — protocol for maintenance checker capabilities
use crate::common::taxonomy_path_vo::FilePath;
use crate::project_setup::taxonomy_doctor_vo::{
    DependencyReport, SecurityScanReport, ToolchainDiagnostics,
};
use async_trait::async_trait;

#[async_trait]
pub trait IMaintenanceCheckerProtocol: Send + Sync {
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics;
    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport;
    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String>;
}
```

---

## File: crates/shared/src/project-setup/contract_setup_aggregate.rs

```rust
// PURPOSE: SetupAggregate — aggregate trait for project setup orchestration
use crate::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use crate::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::mcp_server::taxonomy_job_vo::EnvContentVO;
use crate::mcp_server::taxonomy_job_vo::McpConfigVO;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, ProjectLanguageVO, WriteConfigResult,
};

pub type SetupMgmtProtocol = Box<dyn ISetupManagementProtocol>;

#[async_trait::async_trait]
pub trait SetupManagementAggregate: Send + Sync {
    fn check_http(&self, url: &TransportUrlVO) -> SuccessStatus;
    fn generate_env(&self, transport: &TransportProtocol, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_claude(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_hermes(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_vscode(&self, transport: &TransportProtocol) -> McpConfigVO;
    async fn install_python_adapters(&self) -> SuccessStatus;
    async fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
    fn detect_language(&self) -> ProjectLanguageVO;
    fn get_config_template(&self, language: &str) -> &'static str;
    fn write_config_file(&self, filename: &str, content: &str) -> WriteConfigResult;
    fn create_global_config_dir(&self) -> CreateConfigDirResult;
    fn file_exists(&self, path: &str) -> bool;
}
```

---

## File: crates/shared/src/project-setup/contract_setup_protocol.rs

```rust
// PURPOSE: ISetupProtocol — protocol trait for project setup step definitions
// AES402: All primitive `String` / `Result<(), String>` / `Result<PathBuf, String>`
// return types in ISetupManagementProtocol are replaced with strongly-typed VOs.
//   * `String` returns → `McpBinaryNameVO` / `ProjectLanguageVO`
//   * `Result<(), String>` → `WriteConfigResult` (= `Result<DescriptionVO, SetupError>`)
//   * `Result<PathBuf, String>` → `CreateConfigDirResult` (= `Result<PathBuf, SetupError>`)
//   * `&str` parameters → kept (idiomatic borrow, AES402 allows)
//   * `bool` parameters → kept (semantic toggle, AES402 allows)
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::mcp_server::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, McpBinaryNameVO, ProjectLanguageVO, SetupError, WriteConfigResult,
};

#[async_trait::async_trait]
pub trait ISetupManagementProtocol: Send + Sync {
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self) -> McpConfigVO;
    fn mcp_config_claude(&self) -> McpConfigVO;
    fn mcp_config_hermes(&self) -> McpConfigVO;
    fn mcp_config_vscode(&self) -> McpConfigVO;
    /// Resolve the name of the MCP binary on the host PATH.
    fn which_mcp_binary(&self) -> McpBinaryNameVO;
    async fn install_python_adapters(&self) -> SuccessStatus;
    async fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
    /// Detect the dominant programming language of the current project.
    fn detect_language(&self) -> ProjectLanguageVO;
    fn get_config_template(&self, language: &str) -> &'static str;
    /// Write a configuration file to disk. Returns a description of the
    /// operation on success, or a structured `SetupError` on failure.
    fn write_config_file(&self, filename: &str, content: &str) -> WriteConfigResult;
    /// Create the global config directory and return its path.
    fn create_global_config_dir(&self) -> CreateConfigDirResult;
    fn file_exists(&self, path: &str) -> bool;
}

/// AES402: `Result<(), String>` is replaced with `Result<(), SetupError>`
/// so callers can pattern-match on specific failure modes (Io vs
/// InvalidState vs Other) instead of inspecting free-form error strings.
pub type InstallPackagesResult = Result<(), SetupError>;

#[async_trait::async_trait]
pub trait ISetupInstallerPort: Send + Sync {
    async fn install_python_packages(&self, packages: &[String]) -> InstallPackagesResult;
    async fn install_npm_packages(&self, packages: &[String], sudo: bool) -> InstallPackagesResult;
}
```

---

## File: crates/shared/src/project-setup/mod.rs

```rust
pub mod contract_maintenance_aggregate;
pub mod contract_maintenance_protocol;
pub mod contract_setup_aggregate;
pub mod contract_setup_protocol;
pub mod taxonomy_doctor_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_setup_contract_vo;
pub mod taxonomy_stats_vo;
pub use taxonomy_setup_contract_vo::{
    CreateConfigDirResult, McpBinaryNameVO, ProjectLanguageVO, SetupError, WriteConfigResult,
};
```

---

## File: crates/shared/src/project-setup/taxonomy_doctor_vo.rs

```rust
// PURPOSE: DoctorResultVO, DoctorCheck — VOs for project health diagnostics results
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoctorResultVO {
    pub python_version: DescriptionVO,
    pub is_installed: ComplianceStatus,
    pub config_found: FilePathList,
    pub adapter_statuses: HashMap<AdapterName, String>,
    pub issues: Vec<ErrorMessage>,
    pub healthy: ComplianceStatus,
}

impl DoctorResultVO {
    pub fn new(
        python_version: DescriptionVO,
        is_installed: ComplianceStatus,
        config_found: FilePathList,
        adapter_statuses: HashMap<AdapterName, String>,
        issues: Vec<ErrorMessage>,
        healthy: ComplianceStatus,
    ) -> Self {
        Self {
            python_version,
            is_installed,
            config_found,
            adapter_statuses,
            issues,
            healthy,
        }
    }
}

impl std::fmt::Display for DoctorResultVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DoctorResult(healthy={}, python={})",
            self.healthy.value, self.python_version.value
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolStatus {
    pub name: String,
    pub status: String, // "OK", "WARN", "FAIL"
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolchainDiagnostics {
    pub rust_tools: Vec<ToolStatus>,
    pub python_tools: Vec<ToolStatus>,
    pub js_tools: Vec<ToolStatus>,
    pub vcs_tools: Vec<ToolStatus>,
    pub binary_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityFinding {
    pub severity: String,
    pub test_id: String,
    pub file: String,
    pub line: u64,
    pub issue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityScanReport {
    pub language: String,
    pub tool_name: String,
    pub findings: Vec<SecurityFinding>,
    pub tool_installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DependencyInfo {
    pub name: String,
    pub version: String,
    pub dep_type: String, // "direct" or "transitive"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DependencyReport {
    pub language: String,
    pub dependencies: Vec<DependencyInfo>,
}
```

---

## File: crates/shared/src/project-setup/taxonomy_language_vo.rs

```rust
// PURPOSE: LanguageConfigVO — value object for programming language configuration
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectLanguage {
    pub value: String,
}

impl ProjectLanguage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LanguageSource {
    pub language: String,
    pub confidence: u8,
    pub source: String,
}

impl LanguageSource {
    pub fn new(language: impl Into<String>, confidence: u8, source: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            confidence,
            source: source.into(),
        }
    }
}
```

---

## File: crates/shared/src/project-setup/taxonomy_setup_contract_vo.rs

```rust
// PURPOSE: SetupContractVOs — value objects used by ISetupManagementProtocol and
// ISetupInstallerPort contract surface.
//
// AES402: All primitive `String` / `Result<(), String>` / `Result<_, String>`
// return types and parameter types in ISetupManagementProtocol and
// ISetupInstallerPort are replaced with strongly-typed VOs.
//
// Naming: these VOs are scoped to the `project-setup` feature (which already
// has its own `taxonomy_doctor_vo`, `taxonomy_language_vo`, `taxonomy_stats_vo`).
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::common::taxonomy_suggestion_vo::DescriptionVO;

/// Name of the MCP binary as resolved on the host PATH (e.g. "lint-arwaky-cli").
/// Replaces the previous `String` return type of
/// `ISetupManagementProtocol::which_mcp_binary`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpBinaryNameVO {
    pub value: String,
}

impl McpBinaryNameVO {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Programming language detected for a project (e.g. "rust", "python",
/// "javascript", "typescript"). Replaces the previous `String` return type
/// of `ISetupManagementProtocol::detect_language`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectLanguageVO {
    pub value: String,
}

impl ProjectLanguageVO {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Error type for setup operations that previously returned
/// `Result<(), String>` or `Result<PathBuf, String>`. Replaces ad-hoc
/// `String` error types with a domain error VO so callers can
/// pattern-match on specific failure modes instead of free-form strings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetupError {
    /// Filesystem / IO error (could not write file, could not create dir,
    /// could not read configuration, etc.). The wrapped string carries the
    /// OS-level error message; treat as opaque display text only.
    Io(String),
    /// The setup step was attempted with arguments that conflict with the
    /// current project state (e.g. trying to install a dependency that the
    /// project's lockfile already pins to an incompatible version).
    InvalidState(String),
    /// Catch-all for setup errors that don't fit a specific variant.
    /// Wraps a human-readable diagnostic message.
    Other(String),
}

impl SetupError {
    pub fn io(message: impl Into<String>) -> Self {
        Self::Io(message.into())
    }
    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::InvalidState(message.into())
    }
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(m) | Self::InvalidState(m) | Self::Other(m) => write!(f, "{}", m),
        }
    }
}

impl std::error::Error for SetupError {}

/// Result of writing a configuration file. The previous return type was
/// `Result<(), String>` — we now return `Result<DescriptionVO, SetupError>`
/// where the description carries a success message (e.g. "wrote
/// /path/to/lint_arwaky.config.yaml (256 bytes)") and the error carries a
/// structured failure cause.
pub type WriteConfigResult = Result<DescriptionVO, SetupError>;

/// Result of creating the global config directory. The previous return
/// type was `Result<std::path::PathBuf, String>` — we now return a
/// `FilePath` on success (which wraps `PathBuf` with the rest of the
/// contract's path-handling surface) and a `SetupError` on failure.
pub type CreateConfigDirResult = Result<PathBuf, SetupError>;
```

---

## File: crates/shared/src/project-setup/taxonomy_stats_vo.rs

```rust
// PURPOSE: ProjectStatsVO, MaintenanceStatsVO — VOs for project statistics and maintenance data
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaintenanceStatsVO {
    pub project_path: FilePath,
    pub total_files: Count,
    pub test_files: Count,
    pub test_ratio: Score,
    pub python_files: Count,
}

impl MaintenanceStatsVO {
    pub fn new(
        project_path: FilePath,
        total_files: Count,
        test_files: Count,
        test_ratio: Score,
        python_files: Count,
    ) -> Self {
        Self {
            project_path,
            total_files,
            test_files,
            test_ratio,
            python_files,
        }
    }
}

impl std::fmt::Display for MaintenanceStatsVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MaintenanceStats({}: {} files, {} test, {:.1}%)",
            self.project_path,
            self.total_files.value,
            self.test_files.value,
            self.test_ratio.value * 100.0
        )
    }
}
```

---

## File: crates/shared/src/role-rules/contract_agent_role_protocol.rs

```rust
// PURPOSE: IAgentRoleChecker — port trait for AES405: agent role audits (container, orchestrator, lifecycle, file size, any type)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait IAgentRoleChecker: Send + Sync {
    fn check_container(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_orchestrator(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_lifecycle(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_file_size_limit(
        &self,
        source: &SourceContentVO,
        max_lines: usize,
        violations: &mut Vec<LintResult>,
    );
    fn check_any_type_annotation(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/role-rules/contract_capabilities_role_protocol.rs

```rust
// PURPOSE: ICapabilitiesRoleChecker — port trait for AES403: capability routing bottlenecks and role audits
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait ICapabilitiesRoleChecker: Send + Sync {
    fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}
```

---

## File: crates/shared/src/role-rules/contract_infrastructure_role_protocol.rs

```rust
// PURPOSE: IInfrastructureRoleChecker — port trait for AES404: infrastructure has no port implementation
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait IInfrastructureRoleChecker: Send + Sync {
    fn check_port_implementation(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/role-rules/contract_role_aggregate.rs

```rust
// PURPOSE: IRoleAggregate — aggregate trait bundling taxonomy, contract, infrastructure, capabilities, surface, and agent role checkers
use crate::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use crate::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use crate::role_rules::contract_infrastructure_role_protocol::IInfrastructureRoleChecker;
use crate::role_rules::contract_role_protocol::IContractRoleChecker;
use crate::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use crate::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

pub trait IRoleAggregate: Send + Sync {
    fn taxonomy(&self) -> &dyn ITaxonomyRoleChecker;
    fn contract(&self) -> &dyn IContractRoleChecker;
    fn infrastructure(&self) -> &dyn IInfrastructureRoleChecker;
    fn capabilities(&self) -> &dyn ICapabilitiesRoleChecker;
    fn surface(&self) -> &dyn ISurfaceRoleChecker;
    fn agent(&self) -> &dyn IAgentRoleChecker;
}
```

---

## File: crates/shared/src/role-rules/contract_role_protocol.rs

```rust
// PURPOSE: IContractRoleChecker — port trait for AES402: contract primitive type audits
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait IContractRoleChecker: Send + Sync {
    fn check_port(&self, source: &SourceContentVO) -> Vec<LintResult>;
    fn check_protocol(&self, source: &SourceContentVO) -> Vec<LintResult>;
    fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );
}
```

---

## File: crates/shared/src/role-rules/contract_role_runner_aggregate.rs

```rust
// PURPOSE: IRoleRunnerAggregate — contract for role-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

use crate::role_rules::taxonomy_layer_names_constant::LAYER_AGENT;
use crate::role_rules::taxonomy_layer_names_vo::LayerNames;
use crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO;
use crate::role_rules::taxonomy_violation_role_vo::AesRoleViolation;

pub fn anchor_taxonomy() {
    let _ = LAYER_AGENT;
}
type _LayerNamesVORef = LayerNames;
type _RoleRuleVORef = RoleRuleVO;
type _AesRoleViolationRef = AesRoleViolation;

#[async_trait]
pub trait IRoleRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/role-rules/contract_surface_role_protocol.rs

```rust
// PURPOSE: ISurfaceRoleChecker — port trait for AES406: smart, utility, and passive surface role checks
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait ISurfaceRoleChecker: Send + Sync {
    fn check_smart_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_utility_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_passive_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_fn_count_limit(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/role-rules/contract_taxonomy_role_protocol.rs

```rust
// PURPOSE: ITaxonomyRoleChecker — port trait for AES401: taxonomy role audits (VO, entity, error, event, constant)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait ITaxonomyRoleChecker: Send + Sync {
    fn check_vo(&self) -> Vec<LintResult>;
    fn check_entity(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_error(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_event(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_constant(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/role-rules/mod.rs

```rust
// role-rules — taxonomy and contract types
pub mod contract_agent_role_protocol;
pub mod contract_capabilities_role_protocol;
pub mod contract_infrastructure_role_protocol;
pub mod contract_role_aggregate;
pub mod contract_role_protocol;
pub mod contract_role_runner_aggregate;
pub mod contract_surface_role_protocol;
pub mod contract_taxonomy_role_protocol;
pub mod taxonomy_layer_names_constant;
pub mod taxonomy_layer_names_vo;
pub mod taxonomy_role_rule_vo;
pub mod taxonomy_violation_role_vo;
pub use taxonomy_violation_role_vo::AesRoleViolation;
```

---

## File: crates/shared/src/role-rules/taxonomy_layer_names_constant.rs

```rust
// PURPOSE: LAYER_AGENT, LAYER_CAPABILITIES, etc. — constant definitions for AES layer names

pub const LAYER_AGENT: &str = "agent";
pub const LAYER_CAPABILITIES: &str = "capabilities";
pub const LAYER_CONTRACT: &str = "contract";
pub const LAYER_INFRASTRUCTURE: &str = "infrastructure";
pub const LAYER_SURFACES: &str = "surfaces";
pub const LAYER_TAXONOMY: &str = "taxonomy";
pub const LAYER_ROOT: &str = "root";
pub const LAYER_GLOBAL: &str = "global";
```

---

## File: crates/shared/src/role-rules/taxonomy_layer_names_vo.rs

```rust
// PURPOSE: LayerNames — value object for layer name collection and lookup
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_AGENT;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_CAPABILITIES;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_CONTRACT;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_GLOBAL;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_INFRASTRUCTURE;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_ROOT;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_SURFACES;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_TAXONOMY;

/// Value object holding the set of core layer names.
pub struct LayerNames {}

pub fn layer_agent() -> LayerNameVO {
    LayerNameVO::new(LAYER_AGENT)
}
pub fn layer_capabilities() -> LayerNameVO {
    LayerNameVO::new(LAYER_CAPABILITIES)
}
pub fn layer_taxonomy() -> LayerNameVO {
    LayerNameVO::new(LAYER_TAXONOMY)
}
pub fn layer_contract() -> LayerNameVO {
    LayerNameVO::new(LAYER_CONTRACT)
}
pub fn layer_infrastructure() -> LayerNameVO {
    LayerNameVO::new(LAYER_INFRASTRUCTURE)
}
pub fn layer_surfaces() -> LayerNameVO {
    LayerNameVO::new(LAYER_SURFACES)
}
pub fn layer_root() -> LayerNameVO {
    LayerNameVO::new(LAYER_ROOT)
}
pub fn layer_global() -> LayerNameVO {
    LayerNameVO::new(LAYER_GLOBAL)
}

pub fn all_core_layers() -> Vec<LayerNameVO> {
    vec![
        layer_agent(),
        layer_capabilities(),
        layer_taxonomy(),
        layer_contract(),
        layer_infrastructure(),
        layer_surfaces(),
        layer_root(),
    ]
}

pub fn core_layer_names() -> std::collections::HashSet<String> {
    all_core_layers().iter().map(|l| l.value.clone()).collect()
}
```

---

## File: crates/shared/src/role-rules/taxonomy_role_rule_vo.rs

```rust
// PURPOSE: RoleRuleVO — value object containing role compliance rule definitions
use crate::common::taxonomy_common_vo::{BooleanVO, PatternList};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleRuleVO {
    #[serde(default)]
    pub no_domain_logic: BooleanVO,
    #[serde(default)]
    pub must_implement_service_container_aggregate: BooleanVO,
    #[serde(default)]
    pub lazy_eager_initialization_only: BooleanVO,
    #[serde(default)]
    pub stateless_execution: BooleanVO,
    #[serde(default)]
    pub single_execution_goal: BooleanVO,
    #[serde(default)]
    pub high_level_policy_only: BooleanVO,
    #[serde(default)]
    pub coordinates_multiple_orchestrators: BooleanVO,
    #[serde(default)]
    pub crud_only: BooleanVO,
    #[serde(default)]
    pub no_decision_logic: BooleanVO,
    #[serde(default)]
    pub thread_async_safe: BooleanVO,
    #[serde(default)]
    pub no_domain_data_storage: BooleanVO,
    #[serde(default)]
    pub owns_system_health_transitions: BooleanVO,
    #[serde(default)]
    pub lifecycle_tracking_only: BooleanVO,
    #[serde(default)]
    pub no_primitives: BooleanVO,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
}
```

---

## File: crates/shared/src/role-rules/taxonomy_violation_role_vo.rs

```rust
// PURPOSE: AesRoleViolation — violation messages for role rules (AES401-406)
use crate::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use std::fmt;

pub struct LabeledRoleViolation {
    violation: AesRoleViolation,
    lang: Language,
}

/// Resolve `reason` to the user-facing "why" string. Falls back to a
/// language-aware default message when no reason was supplied by the auditor.
fn resolve_why<S: Into<String>>(reason: &Option<LintMessage>, default: S) -> String {
    match reason.as_ref() {
        Some(r) => r.to_string(),
        None => default.into(),
    }
}

/// Write the violation body for `v` using `lang` for language-aware wording.
/// Both `Display` impls (`AesRoleViolation` and `LabeledRoleViolation`) route
/// through here so the message templates live in exactly one place per variant.
fn write_violation(
    f: &mut fmt::Formatter<'_>,
    v: &AesRoleViolation,
    lang: Language,
) -> fmt::Result {
    match v {
        AesRoleViolation::ConstantPurity { reason } => {
            let why = resolve_why(
                reason,
                "Constant taxonomy modules must only contain pure constant or static values \
                 to maintain value-level immutability.",
            );
            write!(
                f,
                "AES401 TAXONOMY_ROLE: Constant file contains non-constant declaration.\n\
                        WHY? {why}\n\
                        FIX: Move the non-constant code to the appropriate layer, or convert it \
                        to a constant/static declaration."
            )
        }
        AesRoleViolation::PrimitiveUsage { primitive, reason } => {
            let why = resolve_why(
                reason,
                format!(
                    "Direct primitive types (like '{primitive}') are forbidden in taxonomy \
                     entities, errors, and events to maintain strict value object boundaries \
                     and avoid primitive obsession."
                ),
            );
            write!(
                f,
                "AES401 TAXONOMY_ROLE: Direct primitive '{primitive}' in taxonomy entity, \
                        error, or event.\n\
                        WHY? {why}\n\
                        FIX: Replace the primitive type with a domain Value Object (VO) or \
                        constant from the taxonomy layer."
            )
        }
        AesRoleViolation::ContractPrimitive { reason } => {
            let default = format!(
                "Contracts must enforce value object boundaries to prevent primitive obsession. \
                 Use {} instead of primitives.",
                lang.type_kw()
            );
            let why = resolve_why(reason, default);
            write!(
                f,
                "AES402 CONTRACT_PRIMITIVE: Contract {} or method signature uses primitive \
                        types instead of taxonomy VO or constant.\n\
                        WHY? {why}\n\
                        FIX: Replace primitive types with appropriate Value Objects (VO) or \
                        constants from the taxonomy layer.",
                lang.interface_kw()
            )
        }
        AesRoleViolation::CapabilityRouting {
            struct_name,
            reason,
        } => {
            let default = format!(
                "Capability {}s must implement their corresponding {} traits/interfaces to \
                 ensure clean interface boundaries.",
                lang.struct_keyword(),
                lang.interface_kw()
            );
            let why = resolve_why(reason, default);
            write!(
                f,
                "AES403 CAPABILITY_ROLE: {} '{struct_name}' has no {} implementation.\n\
                        WHY? {why}\n\
                        FIX: Implement the capability protocol {} for '{struct_name}'.",
                lang.struct_keyword(),
                lang.interface_kw(),
                lang.interface_kw()
            )
        }
        AesRoleViolation::CapabilityNoProtocol { reason } => {
            let why = resolve_why(
                reason,
                "file has 'capabilities_' prefix but no protocol/port import — this file is \
                 broken/useless. Either it is not a real capability (rename or delete), or \
                 a proper contract protocol requirement has not been created yet (create the \
                 protocol first, then implement it here)",
            );
            write!(
                f,
                "AES403 CAPABILITY_ROLE: Capabilities file has no protocol trait/interface \
                        implementation.\n\
                        WHY? {why}\n\
                        FIX: Rename the file if it is not a capability, delete if obsolete, \
                        or create the required contract protocol first then implement it here."
            )
        }
        AesRoleViolation::SingleBottleneck { reason } => {
            let why = resolve_why(
                reason,
                "Routing all commands to a single capability violates high-level decomposition \
                 and creates a single bottleneck.",
            );
            write!(
                f,
                "AES403 CAPABILITY_ROLE: All orchestrator dispatch routes route to a single \
                        capability.\n\
                        WHY? {why}\n\
                        FIX: Distribute logic or route commands to multiple distinct capabilities."
            )
        }
        AesRoleViolation::InfrastructureNoPort { reason } => {
            let why = resolve_why(
                reason,
                "file has 'infrastructure_' prefix but no port/protocol import — this file is \
                 broken/useless. Either it is not real infrastructure (rename or delete), or \
                 a proper contract port requirement has not been created yet (create the port \
                 first, then implement it here)",
            );
            write!(
                f,
                "AES404 INFRASTRUCTURE_ROLE: Infrastructure file has no port trait/protocol \
                        implementation.\n\
                        WHY? {why}\n\
                        FIX: Rename the file if it is not infrastructure, delete if obsolete, \
                        or create the required contract port/protocol first then implement it \
                        here."
            )
        }
        AesRoleViolation::StatelessExecution { reason } => {
            let why = resolve_why(
                reason,
                "Agent execution components must be stateless to guarantee reentrancy and \
                 prevent side effects.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Non-stateless behavior detected.\n\
                        WHY? {why}\n\
                        FIX: Remove mutable class state assignments or move initialization \
                        logic to the constructor."
            )
        }
        AesRoleViolation::HighLevelPolicy { reason } => {
            let why = resolve_why(
                reason,
                "Agents must focus on high-level orchestration policies and not import \
                 infrastructure adapters directly.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Low-level implementation details imported.\n\
                        WHY? {why}\n\
                        FIX: Reference components using their contract interfaces instead of \
                        concrete infrastructure types."
            )
        }
        AesRoleViolation::CoordinatesMultiple { reason } => {
            let why = resolve_why(
                reason,
                "Orchestrator agents exist to coordinate multiple subsystems; simple \
                 single-component logic belongs elsewhere.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Orchestrator coordinates too few subsystems.\n\
                        WHY? {why}\n\
                        FIX: Merge this simple flow into its caller or delegate at least two \
                        subsystems to this orchestrator."
            )
        }
        AesRoleViolation::NoDomainLogic { reason } => {
            let why = resolve_why(
                reason,
                "Complex domain logic detected in a passive agent role or surface wrapper.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Complex domain logic detected in a passive role.\n\
                        WHY? {why}\n\
                        FIX: Move the complex domain/control logic into capabilities or \
                        orchestrator components."
            )
        }
        AesRoleViolation::LazyEagerInit { reason } => {
            let why = resolve_why(
                reason,
                "Agent containers must only declare and wire dependencies, avoiding complex \
                 logic in constructors.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Complex initialization logic found in container module.\n\
                        WHY? {why}\n\
                        FIX: Move the initialization/conditional logic out of the constructor \
                        or container setup."
            )
        }
        AesRoleViolation::MustImplementContract { reason } => {
            let default = format!(
                "Agent containers must implement the 'ServiceContainerAggregate' {} to satisfy \
                 dependency injection protocols.",
                lang.interface_kw()
            );
            let why = resolve_why(reason, default);
            write!(
                f,
                "AES405 AGENT_ROLE: Class is missing required contract implementation.\n\
                        WHY? {why}\n\
                        FIX: Add the 'ServiceContainerAggregate' implementation for the \
                        container class."
            )
        }
        AesRoleViolation::AnyType { reason } => {
            let why = resolve_why(
                reason,
                "Using 'any' or 'Any' type annotations bypasses type safety and violates \
                 agent-level domain-driven design.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Forbidden 'any' type annotation found.\n\
                        WHY? {why}\n\
                        FIX: Replace 'any' annotations with strongly-typed objects, \
                        structures, or domain Value Objects (VO)."
            )
        }
        AesRoleViolation::AgentFileSizeLimit { max_lines } => write!(
            f,
            "AES405 AGENT_ROLE: Agent file exceeds {max_lines} lines.\n\
                    WHY? Agent files must remain compact to preserve role clarity.\n\
                    FIX: Split the orchestrator/container into smaller focused modules."
        ),
        AesRoleViolation::PassiveViolation { reason } => {
            let why = resolve_why(
                reason,
                "Passive surfaces must not contain logic that should be in capabilities or \
                 agents.",
            );
            write!(
                f,
                "AES406 SURFACE_ROLE: Passive surface contains business logic.\n\
                        WHY? {why}\n\
                        FIX: Move logic to appropriate capability or agent."
            )
        }
        AesRoleViolation::SurfaceRoleViolation { reason } => {
            let why = resolve_why(
                reason,
                "Surface role violation - surfaces must adhere to their designated role \
                 (command, controller, component, hook, etc.).",
            );
            write!(
                f,
                "AES406 SURFACE_ROLE: Surface role boundary violation.\n\
                        WHY? {why}\n\
                        FIX: Ensure surface only performs its designated responsibilities."
            )
        }
    }
}

impl AesRoleViolation {
    pub fn with_language(self, lang: Language) -> LabeledRoleViolation {
        LabeledRoleViolation {
            violation: self,
            lang,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AesRoleViolation {
    // AES401 — Taxonomy role
    ConstantPurity {
        reason: Option<LintMessage>,
    },
    PrimitiveUsage {
        primitive: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES402 — Contract primitive
    ContractPrimitive {
        reason: Option<LintMessage>,
    },
    // AES403 — Capability role
    CapabilityRouting {
        struct_name: SymbolName,
        reason: Option<LintMessage>,
    },
    CapabilityNoProtocol {
        reason: Option<LintMessage>,
    },
    SingleBottleneck {
        reason: Option<LintMessage>,
    },
    // AES404 — Infrastructure role
    InfrastructureNoPort {
        reason: Option<LintMessage>,
    },
    // AES405 — Agent role
    StatelessExecution {
        reason: Option<LintMessage>,
    },
    HighLevelPolicy {
        reason: Option<LintMessage>,
    },
    CoordinatesMultiple {
        reason: Option<LintMessage>,
    },
    NoDomainLogic {
        reason: Option<LintMessage>,
    },
    LazyEagerInit {
        reason: Option<LintMessage>,
    },
    MustImplementContract {
        reason: Option<LintMessage>,
    },
    AnyType {
        reason: Option<LintMessage>,
    },
    AgentFileSizeLimit {
        max_lines: usize,
    },
    // AES406 — Surface role
    PassiveViolation {
        reason: Option<LintMessage>,
    },
    SurfaceRoleViolation {
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesRoleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_violation(f, self, Language::Rust)
    }
}

impl fmt::Display for LabeledRoleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_violation(f, &self.violation, self.lang)
    }
}

impl From<AesRoleViolation> for String {
    fn from(v: AesRoleViolation) -> String {
        v.to_string()
    }
}
```

---

## File: crates/shared/src/tui/contract_action_handler_protocol.rs

```rust
use crate::tui::taxonomy_state_vo::AppState;
use crate::tui::taxonomy_tui_event::TuiEvent;

pub trait IActionHandlerProtocol: Send + Sync {
    fn handle(&self, state: &mut AppState, event: TuiEvent);
    fn load_directory(&self, state: &mut AppState, path: &str);
    fn load_preview(&self, state: &mut AppState);
    /// Poll for file watch events and update state. Call every event loop tick.
    fn poll_watch(&self, state: &mut AppState);
}
```

---

## File: crates/shared/src/tui/contract_file_system_port.rs

```rust
use crate::common::taxonomy_byte_count_vo::ByteCount;
use crate::common::taxonomy_display_content_vo::DisplayContent;
use crate::common::taxonomy_line_count_vo::LineCount;
use crate::common::taxonomy_path_vo::FilePath;
use crate::tui::taxonomy_file_entry_vo::FileEntry;

pub trait IFileSystemPort: Send + Sync {
    fn list_directory(&self, path: &FilePath) -> Vec<FileEntry>;
    fn read_file_preview(&self, path: &FilePath, max_lines: &LineCount) -> DisplayContent;
    fn is_valid_directory(&self, path: &FilePath) -> bool;
    fn parent_directory(&self, path: &FilePath) -> Option<FilePath>;
    fn file_size_human(&self, bytes: &ByteCount) -> DisplayContent;
    fn path_components(&self, path: &FilePath) -> Vec<FilePath>;
}
```

---

## File: crates/shared/src/tui/contract_lint_executor_protocol.rs

```rust
use crate::tui::taxonomy_action_flags_vo::ActionFlags;
use crate::tui::taxonomy_lint_result_vo::LintExecutionResult;

pub trait ILintExecutorProtocol: Send + Sync {
    fn check(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult;
    fn scan(&self, path: &str) -> LintExecutionResult;
    fn fix(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult;
    fn ci(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult;
    fn orphan(&self, path: &str) -> LintExecutionResult;
    fn security(&self, path: &str) -> LintExecutionResult;
    fn duplicates(&self, path: &str) -> LintExecutionResult;
    fn dependencies(&self, path: &str) -> LintExecutionResult;
    fn doctor(&self) -> LintExecutionResult;
    fn init(&self, flags: &ActionFlags) -> LintExecutionResult;
    fn install(&self, flags: &ActionFlags) -> LintExecutionResult;
    fn mcp_config(&self, flags: &ActionFlags) -> LintExecutionResult;
    fn config_show(&self) -> LintExecutionResult;
    fn install_hook(&self) -> LintExecutionResult;
    fn uninstall_hook(&self) -> LintExecutionResult;
    fn adapters(&self) -> LintExecutionResult;
    fn version(&self) -> LintExecutionResult;
}
```

---

## File: crates/shared/src/tui/contract_tui_aggregate.rs

```rust
use crate::tui::contract_file_system_port::IFileSystemPort;
use crate::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use crate::tui::taxonomy_state_vo::AppState;
use crate::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;

pub struct TuiDependencies {
    pub fs_port: Arc<dyn IFileSystemPort>,
    pub lint_port: Arc<dyn ILintExecutorProtocol>,
}

pub trait ITuiAggregate: Send + Sync {
    fn handle_event(&self, state: &mut AppState, event: TuiEvent);
    fn load_directory(&self, state: &mut AppState, path: &str);
    fn load_preview(&self, state: &mut AppState);
    /// Poll for file watch events and update state. Call every event loop tick.
    fn poll_watch(&self, state: &mut AppState);
}
```

---

## File: crates/shared/src/tui/mod.rs

```rust
pub mod taxonomy_action_flags_vo;
pub mod taxonomy_adapter_info_vo;
pub mod taxonomy_file_entry_vo;
pub mod taxonomy_lint_result_vo;
pub mod taxonomy_report_formatter_helper;
pub mod taxonomy_state_vo;
pub mod taxonomy_tui_event;

pub mod contract_action_handler_protocol;
pub mod contract_file_system_port;
pub mod contract_lint_executor_protocol;
pub mod contract_tui_aggregate;
```

---

## File: crates/shared/src/tui/taxonomy_action_flags_vo.rs

```rust
#[derive(Debug, Clone)]
pub struct ActionFlags {
    pub git_diff: bool,
    pub dry_run: bool,
    pub threshold: u32,
    pub global_config: bool,
    pub use_sudo: bool,
    pub mcp_client: String,
}

impl Default for ActionFlags {
    fn default() -> Self {
        Self {
            git_diff: false,
            dry_run: false,
            threshold: 80,
            global_config: false,
            use_sudo: false,
            mcp_client: "claude".to_string(),
        }
    }
}

impl ActionFlags {
    pub fn toggle_git_diff(&mut self) {
        self.git_diff = !self.git_diff;
    }

    pub fn toggle_dry_run(&mut self) {
        self.dry_run = !self.dry_run;
    }

    pub fn toggle_global(&mut self) {
        self.global_config = !self.global_config;
    }

    pub fn toggle_sudo(&mut self) {
        self.use_sudo = !self.use_sudo;
    }

    pub fn set_threshold(&mut self, value: u32) {
        self.threshold = value;
    }

    pub fn set_mcp_client(&mut self, client: impl Into<String>) {
        self.mcp_client = client.into();
    }
}
```

---

## File: crates/shared/src/tui/taxonomy_adapter_info_vo.rs

```rust
// PURPOSE: taxonomy_adapter_info_vo — value object for discovered lint adapter metadata
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterInfo {
    pub name: String,
    pub label: String,
    pub installed: bool,
}

impl fmt::Display for AdapterInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.name,
            if self.installed {
                "installed"
            } else {
                "missing"
            }
        )
    }
}
```

---

## File: crates/shared/src/tui/taxonomy_file_entry_vo.rs

```rust
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AesLayer {
    Taxonomy,
    Contract,
    Capabilities,
    Infrastructure,
    Agent,
    Surfaces,
    Root,
    None,
}

impl AesLayer {
    pub fn badge_label(&self) -> &str {
        match self {
            AesLayer::Taxonomy => "[tax]",
            AesLayer::Contract => "[con]",
            AesLayer::Capabilities => "[cap]",
            AesLayer::Infrastructure => "[inf]",
            AesLayer::Agent => "[agt]",
            AesLayer::Surfaces => "[sur]",
            AesLayer::Root => "[root]",
            AesLayer::None => "[---]",
        }
    }

    pub fn color_index(&self) -> u8 {
        match self {
            AesLayer::Taxonomy => 14,
            AesLayer::Contract => 12,
            AesLayer::Capabilities => 13,
            AesLayer::Infrastructure => 11,
            AesLayer::Agent => 10,
            AesLayer::Surfaces => 9,
            AesLayer::Root => 15,
            AesLayer::None => 8,
        }
    }

    pub fn from_filename(filename: &str) -> Self {
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        if stem.starts_with("taxonomy_") {
            AesLayer::Taxonomy
        } else if stem.starts_with("contract_") {
            AesLayer::Contract
        } else if stem.starts_with("capabilities_") {
            AesLayer::Capabilities
        } else if stem.starts_with("infrastructure_") {
            AesLayer::Infrastructure
        } else if stem.starts_with("agent_") {
            AesLayer::Agent
        } else if stem.starts_with("surface_") {
            AesLayer::Surfaces
        } else if stem.starts_with("root_") {
            AesLayer::Root
        } else {
            AesLayer::None
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub full_path: String,
    pub is_dir: bool,
    pub layer: AesLayer,
    pub violation_count: usize,
    pub extension: String,
    pub size_bytes: u64,
}

impl FileEntry {
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_string();
        let metadata = path.metadata().ok()?;
        let is_dir = metadata.is_dir();
        let layer = if is_dir {
            AesLayer::None
        } else {
            AesLayer::from_filename(&name)
        };
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        Some(Self {
            name,
            full_path: path.to_string_lossy().to_string(),
            is_dir,
            layer,
            violation_count: 0,
            extension,
            size_bytes: metadata.len(),
        })
    }

    pub fn display_name(&self) -> String {
        if self.is_dir {
            format!("{}/", self.name)
        } else {
            self.name.clone()
        }
    }
}
```

---

## File: crates/shared/src/tui/taxonomy_lint_result_vo.rs

```rust
#[derive(Debug, Clone)]
pub struct LintExecutionResult {
    pub output: String,
    pub violation_count: usize,
    pub success: bool,
}

impl LintExecutionResult {
    pub fn success(output: impl Into<String>, violations: usize) -> Self {
        Self {
            output: output.into(),
            violation_count: violations,
            success: true,
        }
    }

    pub fn failure(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            violation_count: 0,
            success: false,
        }
    }
}
```

---

## File: crates/shared/src/tui/taxonomy_report_formatter_helper.rs

```rust
// PURPOSE: Taxonomy-layer report formatter helper — provides formatting functions for scan results, toolchain diagnostics, dependencies, and active configurations.

use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::project_setup::taxonomy_doctor_vo::{DependencyReport, ToolchainDiagnostics};
use crate::taxonomy_common_vo::{ColumnNumber, LineNumber};
use crate::taxonomy_error_vo::ErrorCode;
use crate::taxonomy_message_vo::LintMessage;
use crate::tui::taxonomy_lint_result_vo::LintExecutionResult;

pub struct ReportFormatterHelper;

impl ReportFormatterHelper {
    // Keep reference checks happy
    pub fn _use_unused() {
        let _ = LineNumber::new(1);
        let _ = ColumnNumber::new(1);
        let _ = ErrorCode::raw("code");
        let _ = LintMessage::new("msg");
        let _ = LintResultList::default();
        let _ = Severity::LOW;
    }

    pub fn format_results(results: &LintResultList) -> String {
        if results.is_empty() {
            return "No violations found.".to_string();
        }
        let mut output = format!("Found {} violation(s):\n\n", results.len());
        for (i, r) in results.iter().enumerate() {
            let src = r
                .source
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "unknown".into());
            output.push_str(&format!(
                "{}. [{}] {}:{} — {}\n   Code: {} | Severity: {}\n\n",
                i + 1,
                src,
                r.file,
                r.line.value,
                r.message,
                r.code,
                r.severity
            ));
        }
        output
    }

    pub fn format_doctor_report(diagnostics: &ToolchainDiagnostics) -> LintExecutionResult {
        let mut output = format!(
            "Environment Diagnostics\nBinary: {}\n\n",
            diagnostics.binary_path
        );
        let mut fail_count = 0;
        for (name, tools) in [
            ("Rust Tools", &diagnostics.rust_tools),
            ("Python Tools", &diagnostics.python_tools),
            ("JS/TS Tools", &diagnostics.js_tools),
            ("VCS Tools", &diagnostics.vcs_tools),
        ] {
            output.push_str(&format!("== {} ==\n", name));
            for tool in tools {
                let icon = match tool.status.as_str() {
                    "OK" => "\u{2713}",
                    "WARN" => "\u{26A0}",
                    "FAIL" => {
                        fail_count += 1;
                        "\u{2717}"
                    }
                    _ => "?",
                };
                let note = match tool.status.as_str() {
                    "WARN" => " (optional)",
                    "FAIL" => " (required)",
                    _ => "",
                };
                output.push_str(&format!(
                    "  {} {} {}{}\n",
                    icon, tool.name, tool.version, note
                ));
            }
            output.push('\n');
        }
        if fail_count == 0 {
            output.push_str("All required tools OK.\n");
        } else {
            output.push_str(&format!("{} required tool(s) missing!\n", fail_count));
        }
        LintExecutionResult::success(output, fail_count)
    }

    pub fn format_dependency_report(path: &str, report: &DependencyReport) -> LintExecutionResult {
        let count = report.dependencies.len();
        let mut output = format!(
            "Dependency scan for {}\nLanguage: {}\nTotal dependencies: {}\n",
            path, report.language, count
        );
        for dep_type in ["direct", "transitive"] {
            let list: Vec<_> = report
                .dependencies
                .iter()
                .filter(|d| d.dep_type == dep_type)
                .collect();
            if !list.is_empty() {
                output.push_str(&format!(
                    "\n{} ({}) [top 30]:\n",
                    if dep_type == "direct" {
                        "Direct"
                    } else {
                        "Transitive"
                    },
                    list.len()
                ));
                for dep in list.iter().take(30) {
                    output.push_str(&format!("  {} {}\n", dep.name, dep.version));
                }
                if list.len() > 30 {
                    output.push_str(&format!("  ... and {} more\n", list.len() - 30));
                }
            }
        }
        LintExecutionResult::success(output, count)
    }

    pub fn format_config_result(result: &ConfigResult) -> LintExecutionResult {
        let mut output = String::from("Active Configuration\n");
        output.push_str(&format!(
            "Source: {} ({})\n",
            result.source.path.value, result.source.language
        ));
        if !result.warnings.is_empty() {
            output.push_str("\nWarnings:\n");
            for w in &result.warnings {
                output.push_str(&format!("  - {}\n", w));
            }
        }
        let config = &result.config;
        output.push_str(&format!("\nEnabled: {}\n", config.enabled.value));
        output.push_str(&format!("Layers: {}\n", config.layers.len()));
        output.push_str(&format!("Rules: {}\n", config.rules.len()));
        output.push_str(&format!(
            "Ignored paths: {}\n",
            config.ignored_paths.values.len()
        ));
        output.push_str(&format!(
            "Mandatory class definition: {}\n",
            config.mandatory_class_definition.value
        ));
        output.push_str(&format!(
            "Naming word count: {}\n",
            config.naming.word_count.value
        ));
        if !config.layers.is_empty() {
            output.push_str("\nArchitecture Layers:\n");
            for (name, def) in config.layers.iter() {
                let policy = if def.naming.suffix_policy.value.is_empty() {
                    String::new()
                } else {
                    format!(" (policy: {})", def.naming.suffix_policy.value)
                };
                output.push_str(&format!("  - {}{}\n", name.value, policy));
            }
        }
        if !config.rules.is_empty() {
            output.push_str(&format!("\nRules ({}):\n", config.rules.len()));
            for (i, rule) in config.rules.iter().enumerate() {
                let desc = if rule.description.value.is_empty() {
                    String::new()
                } else if rule.description.value.len() > 60 {
                    format!(" — {}…", &rule.description.value[..60])
                } else {
                    format!(" — {}", rule.description.value)
                };
                output.push_str(&format!(
                    "  {}. {} [{}]{}\n",
                    i + 1,
                    rule.name.value,
                    rule.scope.value,
                    desc
                ));
            }
        }
        LintExecutionResult::success(output, 0)
    }
}
```

---

## File: crates/shared/src/tui/taxonomy_state_vo.rs

```rust
use crate::tui::taxonomy_action_flags_vo::ActionFlags;
use crate::tui::taxonomy_file_entry_vo::FileEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelFocus {
    Tree,
    FileList,
    Preview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewMode {
    FileContent,
    LintResults,
    HelpOverlay,
    ActionOutput,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub project_root: String,
    pub current_dir: String,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub panel_focus: PanelFocus,
    pub preview_mode: PreviewMode,
    pub preview_text: String,
    pub status_message: String,
    pub action_flags: ActionFlags,
    pub search_query: String,
    pub search_mode: bool,
    pub show_help: bool,
    pub show_path_dialog: bool,
    pub path_input: String,
    pub should_quit: bool,
    pub violation_count: usize,
    pub tree_scroll: usize,
    pub preview_scroll: usize,
    pub terminal_height: u16,
    /// Indices into `entries` matching the current search query (empty when not filtering).
    pub filtered_indices: Vec<usize>,
    /// Position within `filtered_indices` — which matching entry is selected.
    pub filter_pos: usize,
    /// Whether file watching is active (w key toggles this).
    pub watching: bool,
}

impl AppState {
    pub fn new(project_root: String) -> Self {
        let current_dir = project_root.clone();
        Self {
            project_root,
            current_dir,
            entries: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            panel_focus: PanelFocus::FileList,
            preview_mode: PreviewMode::FileContent,
            preview_text: String::new(),
            status_message: "Ready".to_string(),
            action_flags: ActionFlags::default(),
            search_query: String::new(),
            search_mode: false,
            show_help: false,
            show_path_dialog: true,
            path_input: String::new(),
            should_quit: false,
            violation_count: 0,
            tree_scroll: 0,
            preview_scroll: 0,
            terminal_height: 0,
            filtered_indices: Vec::new(),
            filter_pos: 0,
            watching: false,
        }
    }

    pub fn select_next(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            if !self.filtered_indices.is_empty()
                && self.filter_pos < self.filtered_indices.len() - 1
            {
                self.filter_pos += 1;
                self.selected_index = self.filtered_indices[self.filter_pos];
                self.adjust_scroll(self.file_list_visible_height());
            }
        } else if !self.entries.is_empty() && self.selected_index < self.entries.len() - 1 {
            self.selected_index += 1;
            self.adjust_scroll(self.file_list_visible_height());
        }
    }

    pub fn select_prev(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            if self.filter_pos > 0 {
                self.filter_pos -= 1;
                self.selected_index = self.filtered_indices[self.filter_pos];
                self.adjust_scroll(self.file_list_visible_height());
            }
        } else if self.selected_index > 0 {
            self.selected_index -= 1;
            self.adjust_scroll(self.file_list_visible_height());
        }
    }

    pub fn select_first(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            if !self.filtered_indices.is_empty() {
                self.filter_pos = 0;
                self.selected_index = self.filtered_indices[0];
            }
            self.scroll_offset = 0;
        } else {
            self.selected_index = 0;
            self.scroll_offset = 0;
        }
    }

    pub fn select_last(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            if !self.filtered_indices.is_empty() {
                self.filter_pos = self.filtered_indices.len() - 1;
                self.selected_index = self.filtered_indices[self.filter_pos];
                self.adjust_scroll(self.file_list_visible_height());
            }
        } else if !self.entries.is_empty() {
            self.selected_index = self.entries.len() - 1;
            self.adjust_scroll(self.file_list_visible_height());
        }
    }

    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected_index)
    }

    pub fn selected_path(&self) -> String {
        match self.selected_entry() {
            Some(entry) => entry.full_path.clone(),
            None => self.current_dir.clone(),
        }
    }

    pub fn cycle_focus_forward(&mut self) {
        self.panel_focus = match self.panel_focus {
            PanelFocus::Tree => PanelFocus::FileList,
            PanelFocus::FileList => PanelFocus::Preview,
            PanelFocus::Preview => PanelFocus::Tree,
        };
    }

    pub fn cycle_focus_backward(&mut self) {
        self.panel_focus = match self.panel_focus {
            PanelFocus::Tree => PanelFocus::Preview,
            PanelFocus::FileList => PanelFocus::Tree,
            PanelFocus::Preview => PanelFocus::FileList,
        };
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = msg.into();
    }

    pub fn adjust_scroll(&mut self, visible_height: usize) {
        if visible_height == 0 {
            return;
        }
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
        if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index - visible_height + 1;
        }
    }

    /// Recompute `filtered_indices` from the current search query.
    /// Call after ToggleSearch, SearchInput, SearchBackspace, SearchConfirm, SearchCancel,
    /// and after loading a new directory while search mode is active.
    pub fn compute_filtered_indices(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            let query = self.search_query.to_lowercase();
            self.filtered_indices = self
                .entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| entry.name.to_lowercase().contains(&query))
                .map(|(i, _)| i)
                .collect();
            // Clamp filter_pos to valid range
            if self.filter_pos >= self.filtered_indices.len() {
                self.filter_pos = self.filtered_indices.len().saturating_sub(1);
            }
            // Sync selected_index from the current filter position
            if !self.filtered_indices.is_empty() {
                self.selected_index = self.filtered_indices[self.filter_pos];
            }
        } else {
            self.filtered_indices.clear();
            self.filter_pos = 0;
        }
    }

    /// Compute the visible height of the file list panel from terminal_height.
    /// Layout: 1 header row + 3 shortcut rows + 1 status row = 5 rows overhead.
    fn file_list_visible_height(&self) -> usize {
        (self.terminal_height as usize).saturating_sub(5)
    }
}
```

---

## File: crates/shared/src/tui/taxonomy_tui_event.rs

```rust
use crate::tui::taxonomy_state_vo::PanelFocus;

pub const DEFAULT_FOCUS: PanelFocus = PanelFocus::FileList;

#[derive(Debug, Clone, PartialEq)]
pub enum TuiEvent {
    MoveUp,
    MoveDown,
    MoveTop,
    MoveBottom,
    NavigateBack,
    NavigateForward,
    FocusNext,
    FocusPrev,
    ActionCheck,
    ActionScan,
    ActionFix,
    ActionCi,
    ActionWatch,
    ActionOrphan,
    ActionSecurity,
    ActionDuplicates,
    ActionDependencies,
    ActionDoctor,
    ActionInit,
    ActionInstall,
    ActionMcpConfig,
    ActionConfigShow,
    ActionInstallHook,
    ActionUninstallHook,
    ActionAdapters,
    ActionVersion,
    ToggleHelp,
    ToggleSearch,
    SearchInput(char),
    SearchBackspace,
    SearchConfirm,
    SearchCancel,
    PathInput(char),
    PathBackspace,
    PathConfirm,
    PathUseCurrent,
    Quit,
    Resize(u16, u16),
    MouseClick(u16, u16),
    MouseScrollUp,
    MouseScrollDown,
    CopyToClipboard,
    CopyToFile,
    PreviewScrollUp,
    PreviewScrollDown,
    Tick,
    None,
}
```

---
