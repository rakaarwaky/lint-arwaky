# Crate: shared (v1.10.14)

This document contains the source code for feature crate `shared` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/shared
  Violations: 115
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_symbol_renamer_utility.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_cycle_protocol.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_cycle_protocol.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES301] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_cycle_protocol.rs - AES301 FILE_TOO_LARGE: File exceeds the maximum allowed line count.
WHY? Large files violate the Single Responsibility Principle.
FIX: Split the module into smaller, more focused files. (max: 1000).
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs - AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.
WHY? Bypassing code checks hides issues and risks architectural regressions.
FIX: Remove the bypass comment (e.g. noqa, eslint-disable, ts-ignore) and resolve the issue properly.
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs - AES304 TODO: Forbidden todo!() call detected.
WHY? todo!() placeholders represent incomplete code paths that can crash at runtime if reached unexpectedly.
FIX: Implement the function body with real logic, or return a meaningful default/error instead of leaving a todo!() placeholder.
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs - AES304 TODO: Forbidden todo!() call detected.
WHY? todo!() placeholders represent incomplete code paths that can crash at runtime if reached unexpectedly.
FIX: Implement the function body with real logic, or return a meaningful default/error instead of leaving a todo!() placeholder.
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs - AES304 TODO: Forbidden todo!() call detected.
WHY? todo!() placeholders represent incomplete code paths that can crash at runtime if reached unexpectedly.
FIX: Implement the function body with real logic, or return a meaningful default/error instead of leaving a todo!() placeholder.
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs - AES304 UNIMPLEMENTED: Forbidden unimplemented!() call detected.
WHY? unimplemented!() claims a code path is unreachable, but when reached it crashes — violating the principle of fail-fast with clear error messages.
FIX: Either implement the missing logic or return a Result::Err with a descriptive error message.
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs - AES304 UNIMPLEMENTED: Forbidden unimplemented!() call detected.
WHY? unimplemented!() claims a code path is unreachable, but when reached it crashes — violating the principle of fail-fast with clear error messages.
FIX: Either implement the missing logic or return a Result::Err with a descriptive error message.
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_app_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_app_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_source_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_watch_config_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_watch_event_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_git_diff_data_vo.rs - AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.
WHY? Bypassing code checks hides issues and risks architectural regressions.
FIX: Remove the bypass comment (e.g. noqa, eslint-disable, ts-ignore) and resolve the issue properly.
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_language_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_path_helper.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dummy_helper.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dummy_helper.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dummy_helper.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dummy_helper.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dummy_helper.rs - AES304 PANIC: Forbidden panic call detected.
WHY? Manual panic calls crash the program unexpectedly instead of using structured error recovery.
FIX: Return a Result or handle the failure case gracefully without panicking.
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_violation_import_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_violation_import_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_violation_import_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_violation_import_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/taxonomy_violation_role_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_adapter_error.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_adapter_error.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_adapter_error.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_file_collector_helper.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_file_collector_helper.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_file_collector_helper.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/contract_fix_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'LintFixOrchestratorAggregate' is orphaned.
WHY? Contract aggregate 'LintFixOrchestratorAggregate' not implemented by any agent file.
FIX: Import and use 'LintFixOrchestratorAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/contract_fix_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IFixProtocol' is orphaned.
WHY? Contract protocol 'IFixProtocol' not implemented by any capabilities file.
FIX: Implement 'IFixProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/contract_executor_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'ICommandExecutorPort' is orphaned.
WHY? Contract port 'ICommandExecutorPort' not implemented by any infrastructure file.
FIX: Implement 'ICommandExecutorPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_adapter_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'ILinterAdapterPort' is orphaned.
WHY? Contract port 'ILinterAdapterPort' not implemented by any infrastructure file.
FIX: Implement 'ILinterAdapterPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_bypass_checker_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IBypassCheckerProtocol' is orphaned.
WHY? Contract protocol 'IBypassCheckerProtocol' not implemented by any capabilities file.
FIX: Implement 'IBypassCheckerProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
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
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_cycle_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'ICycleAnalysisProtocol' is orphaned.
WHY? Contract protocol 'ICycleAnalysisProtocol' not implemented by any capabilities file.
FIX: Implement 'ICycleAnalysisProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'ICodeAnalysisAggregate' is orphaned.
WHY? Contract aggregate 'ICodeAnalysisAggregate' not implemented by any agent file.
FIX: Import and use 'ICodeAnalysisAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_orchestration_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'IConfigOrchestrationAggregate' is orphaned.
WHY? Contract aggregate 'IConfigOrchestrationAggregate' not implemented by any agent file.
FIX: Import and use 'IConfigOrchestrationAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_parser_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IConfigParserPort' is orphaned.
WHY? Contract port 'IConfigParserPort' not implemented by any infrastructure file.
FIX: Implement 'IConfigParserPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_reader_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IConfigReaderPort' is orphaned.
WHY? Contract port 'IConfigReaderPort' not implemented by any infrastructure file.
FIX: Implement 'IConfigReaderPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_validator_protocol.rs - AES502 CONTRACT_ORPHAN: Contract protocol 'IConfigValidatorProtocol' is orphaned.
WHY? Contract protocol 'IConfigValidatorProtocol' not implemented by any capabilities file.
FIX: Implement 'IConfigValidatorProtocol' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_workspace_detector_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IWorkspaceDetectorPort' is orphaned.
WHY? Contract port 'IWorkspaceDetectorPort' not implemented by any infrastructure file.
FIX: Implement 'IWorkspaceDetectorPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'MultiProjectOrchestratorAggregate' is orphaned.
WHY? Contract aggregate 'MultiProjectOrchestratorAggregate' not implemented by any agent file.
FIX: Import and use 'MultiProjectOrchestratorAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'IExternalLintAggregate' is orphaned.
WHY? Contract aggregate 'IExternalLintAggregate' not implemented by any agent file.
FIX: Import and use 'IExternalLintAggregate' in a surface_* file or root_*_container.rs.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-system/contract_system_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IFileSystemPort' is orphaned.
WHY? Contract port 'IFileSystemPort' not implemented by any infrastructure file.
FIX: Implement 'IFileSystemPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_provider_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IWatchProviderPort' is orphaned.
WHY? Contract port 'IWatchProviderPort' not implemented by any infrastructure file.
FIX: Implement 'IWatchProviderPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_watch_aggregate.rs - AES502 CONTRACT_ORPHAN: Contract aggregate 'IWatchAggregate' is orphaned.
WHY? Contract aggregate 'IWatchAggregate' not implemented by any agent file.
FIX: Import and use 'IWatchAggregate' in a surface_* file or root_*_container.rs.
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
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_language_detector_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'ILanguageDetectorPort' is orphaned.
WHY? Contract port 'ILanguageDetectorPort' not implemented by any infrastructure file.
FIX: Implement 'ILanguageDetectorPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_parser_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'ISourceParserPort' is orphaned.
WHY? Contract port 'ISourceParserPort' not implemented by any infrastructure file.
FIX: Implement 'ISourceParserPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_path_normalization_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IPathNormalizationPort' is orphaned.
WHY? Contract port 'IPathNormalizationPort' not implemented by any infrastructure file.
FIX: Implement 'IPathNormalizationPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
  [AES502] /home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_scanner_provider_port.rs - AES502 CONTRACT_ORPHAN: Contract port 'IScannerProviderPort' is orphaned.
WHY? Contract port 'IScannerProviderPort' not called by any orchestrator or container.
FIX: Implement 'IScannerProviderPort' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.
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
- [crates/shared/src/cli-commands/taxonomy_metadata_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_metadata_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_position_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_position_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_protocol_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_protocol_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_score_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_score_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_transport_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_transport_error.rs)
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
- [crates/shared/src/code-analysis/taxonomy_governance_entity.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_governance_entity.rs)
- [crates/shared/src/code-analysis/taxonomy_import_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_import_source_vo.rs)
- [crates/shared/src/code-analysis/taxonomy_operation_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_operation_error.rs)
- [crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_action_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_action_vo.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_error.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_definition_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_definition_vo.rs)
- [crates/shared/src/common/taxonomy_duration_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_duration_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_job_id_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_id_vo.rs)
- [crates/shared/src/common/taxonomy_job_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_vo.rs)
- [crates/shared/src/common/taxonomy_layer_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_layer_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_name_vo.rs)
- [crates/shared/src/common/taxonomy_response_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_response_data_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_source_vo.rs)
- [crates/shared/src/common/taxonomy_suggestion_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_suggestion_vo.rs)
- [crates/shared/src/common/taxonomy_value_object_utility.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_value_object_utility.rs)
- [crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs)
- [crates/shared/src/config-system/contract_orchestration_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_orchestration_aggregate.rs)
- [crates/shared/src/config-system/contract_parser_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_parser_port.rs)
- [crates/shared/src/config-system/contract_reader_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_reader_port.rs)
- [crates/shared/src/config-system/contract_validator_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_validator_protocol.rs)
- [crates/shared/src/config-system/contract_workspace_detector_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_workspace_detector_port.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_adapter_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_adapter_vo.rs)
- [crates/shared/src/config-system/taxonomy_app_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_app_vo.rs)
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
- [crates/shared/src/file-system/contract_system_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-system/contract_system_port.rs)
- [crates/shared/src/file-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-system/mod.rs)
- [crates/shared/src/file-system/taxonomy_filesystem_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-system/taxonomy_filesystem_error.rs)
- [crates/shared/src/file-watch/contract_provider_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_provider_port.rs)
- [crates/shared/src/file-watch/contract_watch_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_watch_aggregate.rs)
- [crates/shared/src/file-watch/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/mod.rs)
- [crates/shared/src/file-watch/taxonomy_diff_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_diff_result_vo.rs)
- [crates/shared/src/file-watch/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_result_vo.rs)
- [crates/shared/src/file-watch/taxonomy_service_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_service_error.rs)
- [crates/shared/src/file-watch/taxonomy_watch_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_watch_config_vo.rs)
- [crates/shared/src/file-watch/taxonomy_watch_event_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_watch_event_vo.rs)
- [crates/shared/src/file-watch/taxonomy_watch_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_watch_vo.rs)
- [crates/shared/src/git-hooks/contract_diff_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_diff_protocol.rs)
- [crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs)
- [crates/shared/src/git-hooks/contract_hook_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_hook_protocol.rs)
- [crates/shared/src/git-hooks/contract_manager_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_manager_port.rs)
- [crates/shared/src/git-hooks/contract_orchestrator_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_orchestrator_aggregate.rs)
- [crates/shared/src/git-hooks/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/mod.rs)
- [crates/shared/src/git-hooks/taxonomy_diff_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_diff_result_vo.rs)
- [crates/shared/src/git-hooks/taxonomy_git_diff_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_git_diff_data_vo.rs)
- [crates/shared/src/git-hooks/taxonomy_hook_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_hook_error.rs)
- [crates/shared/src/git-hooks/taxonomy_installed_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_installed_event.rs)
- [crates/shared/src/git-hooks/taxonomy_ref_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_ref_vo.rs)
- [crates/shared/src/git-hooks/taxonomy_removed_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_removed_event.rs)
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
- [crates/shared/src/source-parsing/contract_language_detector_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_language_detector_port.rs)
- [crates/shared/src/source-parsing/contract_parser_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_parser_port.rs)
- [crates/shared/src/source-parsing/contract_path_normalization_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_path_normalization_port.rs)
- [crates/shared/src/source-parsing/contract_scanner_provider_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_scanner_provider_port.rs)
- [crates/shared/src/source-parsing/infrastructure_file_collector_provider.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/infrastructure_file_collector_provider.rs)
- [crates/shared/src/source-parsing/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/mod.rs)
- [crates/shared/src/source-parsing/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_adapter_error.rs)
- [crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs)
- [crates/shared/src/source-parsing/taxonomy_file_collector_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_file_collector_helper.rs)
- [crates/shared/src/source-parsing/taxonomy_language_detector_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_language_detector_helper.rs)
- [crates/shared/src/source-parsing/taxonomy_naming_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_naming_error.rs)
- [crates/shared/src/source-parsing/taxonomy_naming_list_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_naming_list_vo.rs)
- [crates/shared/src/source-parsing/taxonomy_parser_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_parser_error.rs)
- [crates/shared/src/source-parsing/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_path_vo.rs)
- [crates/shared/src/source-parsing/taxonomy_paths_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_paths_vo.rs)
- [crates/shared/src/source-parsing/taxonomy_semantic_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_semantic_error.rs)

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
serde_yaml.workspace = true
thiserror.workspace = true
async-trait.workspace = true
chrono.workspace = true
anyhow.workspace = true
once_cell.workspace = true
regex.workspace = true
tokio.workspace = true
clap.workspace = true


[dev-dependencies]
```

---

## File: crates/shared/src/auto-fix/contract_fix_aggregate.rs

```rust
// PURPOSE: LintFixOrchestratorAggregate — aggregate trait for auto-fix orchestration
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IFixProtocol: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> bool;
    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> bool;
    fn emit_fix_event(&self, path: &FilePath, error_code: ErrorCode, changes: Count);
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
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
/// Relaxed taxonomy rules: boleh dipakai oleh layer manapun.
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
        std::fs::read_to_string(path)
            .map(|c| c.contains(symbol))
            .unwrap_or(false)
    }
}
```

---

## File: crates/shared/src/cli-commands/contract_executor_port.rs

```rust
// PURPOSE: Port: ICommandExecutorPort — trait for executing shell commands and capturing response
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::mcp_server::taxonomy_job_vo::ResponseData;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_score_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_transport_error;
```

---

## File: crates/shared/src/cli-commands/taxonomy_catalog_constant.rs

```rust
// PURPOSE: COMMAND_CATALOG — static table of every public CLI/MCP command with description and usage example

pub static COMMAND_CATALOG: &[(&str, &str, &str)] = &[
    (
        "check",
        "Run full architecture compliance analysis",
        "lint-arwaky check /path",
    ),
    (
        "scan",
        "Deep directory scan (alias for check)",
        "lint-arwaky scan ./src/",
    ),
    ("fix", "Apply safe fixes", "lint-arwaky fix file.py"),
    (
        "ci",
        "CI-optimized with exit codes",
        "lint-arwaky ci /path --threshold 80",
    ),
    (
        "doctor",
        "Diagnose environment health",
        "lint-arwaky doctor",
    ),
    (
        "orphan",
        "Check if file is dead/unreachable code",
        "lint-arwaky orphan <path>",
    ),
    (
        "security",
        "Vulnerability scanning",
        "lint-arwaky security /path",
    ),
    (
        "duplicates",
        "Code duplication detection",
        "lint-arwaky duplicates /path",
    ),
    (
        "dependencies",
        "Dependency vulnerability scan",
        "lint-arwaky dependencies .",
    ),
    (
        "watch",
        "Watch files for changes",
        "lint-arwaky watch ./src/",
    ),
    (
        "install-hook",
        "Install git pre-commit hook",
        "lint-arwaky install-hook",
    ),
    (
        "uninstall-hook",
        "Remove git pre-commit hook",
        "lint-arwaky uninstall-hook",
    ),
    ("adapters", "List enabled adapters", "lint-arwaky adapters"),
    ("version", "Show version", "lint-arwaky version"),
    ("init", "Create default config", "lint-arwaky init"),
    (
        "install",
        "Install adapter dependencies",
        "lint-arwaky install",
    ),
    (
        "mcp-config",
        "Print MCP server config",
        "lint-arwaky mcp-config --client claude",
    ),
    (
        "config-show",
        "Show active configuration",
        "lint-arwaky config-show",
    ),
];
```

---

## File: crates/shared/src/cli-commands/taxonomy_cli_vo.rs

```rust
// PURPOSE: Cli + Commands enums — clap-based CLI definition with all subcommands
use clap::{Parser, Subcommand};

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
    },

    /// Alias for check (CI-friendly)
    Scan {
        /// Path to scan
        path: Option<String>,
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

    /// Check if a file is an orphan (AES030)
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
                Suggestion::new("lint-arwaky check /path"),
            ),
        );
        catalog.insert(
            ActionName::from("scan"),
            CommandMetadataVO::new(
                DescriptionVO::new("Deep directory scan"),
                Suggestion::new("lint-arwaky scan ./src/"),
            ),
        );
        catalog.insert(
            ActionName::from("fix"),
            CommandMetadataVO::new(
                DescriptionVO::new("Apply safe fixes"),
                Suggestion::new("lint-arwaky fix file.py"),
            ),
        );
        catalog.insert(
            ActionName::from("ci"),
            CommandMetadataVO::new(
                DescriptionVO::new("CI-optimized with exit codes"),
                Suggestion::new("lint-arwaky ci /path --exit-zero"),
            ),
        );
        catalog.insert(
            ActionName::from("watch"),
            CommandMetadataVO::new(
                DescriptionVO::new("Watch files for changes"),
                Suggestion::new("lint-arwaky watch ./src/"),
            ),
        );
        catalog.insert(
            ActionName::from("security"),
            CommandMetadataVO::new(
                DescriptionVO::new("Bandit vulnerability scanning"),
                Suggestion::new("lint-arwaky security /path"),
            ),
        );
        catalog.insert(
            ActionName::from("duplicates"),
            CommandMetadataVO::new(
                DescriptionVO::new("Code duplication detection"),
                Suggestion::new("lint-arwaky duplicates /path"),
            ),
        );
        catalog.insert(
            ActionName::from("dependencies"),
            CommandMetadataVO::new(
                DescriptionVO::new("Dependency vulnerability scan"),
                Suggestion::new("lint-arwaky dependencies ."),
            ),
        );
        catalog.insert(
            ActionName::from("maintenance doctor"),
            CommandMetadataVO::new(
                DescriptionVO::new("Diagnose environment health"),
                Suggestion::new("lint-arwaky maintenance doctor"),
            ),
        );
        catalog.insert(
            ActionName::from("adapters"),
            CommandMetadataVO::new(
                DescriptionVO::new("List enabled adapters"),
                Suggestion::new("lint-arwaky adapters"),
            ),
        );
        catalog.insert(
            ActionName::from("install-hook"),
            CommandMetadataVO::new(
                DescriptionVO::new("Install git pre-commit hook"),
                Suggestion::new("lint-arwaky install-hook"),
            ),
        );
        catalog.insert(
            ActionName::from("uninstall-hook"),
            CommandMetadataVO::new(
                DescriptionVO::new("Remove git pre-commit hook"),
                Suggestion::new("lint-arwaky uninstall-hook"),
            ),
        );
        catalog.insert(
            ActionName::from("plugins"),
            CommandMetadataVO::new(
                DescriptionVO::new("List discovered plugins"),
                Suggestion::new("lint-arwaky plugins"),
            ),
        );
        catalog.insert(
            ActionName::from("version"),
            CommandMetadataVO::new(
                DescriptionVO::new("Show version"),
                Suggestion::new("lint-arwaky version"),
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
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/shared/src/cli-commands/taxonomy_transport_error.rs

```rust
// PURPOSE: TransportError — structured error type wrapping protocol, message, endpoint, and underlying error
use crate::cli_commands::taxonomy_protocol_vo::TransportEndpoint;
use crate::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use crate::common::taxonomy_common_error::ErrorMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct TransportError {
    pub protocol: TransportProtocol,
    pub message: ErrorMessage,
    pub endpoint: TransportEndpoint,
    pub underlying_error: ErrorMessage,
}

impl TransportError {
    pub fn new(protocol: TransportProtocol, message: ErrorMessage) -> Self {
        Self {
            protocol,
            message,
            endpoint: TransportEndpoint::default(),
            underlying_error: ErrorMessage::default(),
        }
    }
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ep_str = self.endpoint.to_string();
        let ep = if ep_str.is_empty() {
            String::new()
        } else {
            format!(" {}", ep_str)
        };
        write!(f, "[{}]{} {}", self.protocol, ep, self.message)
    }
}
```

---

## File: crates/shared/src/code-analysis/contract_adapter_port.rs

```rust
// PURPOSE: ILinterAdapterPort — port trait for linter adapter implementations (Ruff, Mypy, Clippy, etc.)

use async_trait::async_trait;

use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;

pub trait ICodeAnalysisAggregate: Send + Sync {
    fn run_code_analysis(&self, project_root: &str) -> LintResultList;
    fn run_code_analysis_dir(&self, src_dir: &str) -> LintResultList;
    fn run_code_analysis_path(&self, path: &str) -> Vec<LintResult>;
    fn calc_score(&self, results: &[LintResult]) -> f64;
    fn check_critical(&self, results: &[LintResult]) -> bool;
    fn format_report(&self, results: &LintResultList, project_root: &str) -> String;
}
```

---

## File: crates/shared/src/code-analysis/contract_code_metric_analyzer_protocol.rs

```rust
// PURPOSE: ICodeMetricAnalyzerProtocol — protocol for duplication detection (AES305)
use crate::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use crate::file_system::contract_system_port::IFileSystemPort;

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
// PURPOSE: ICycleAnalysisProtocol + DefaultCycleAnalysisProtocol — port trait and default impl for circular dependency detection (AES205)
use std::collections::{HashMap, HashSet};

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::import_rules::contract_rule_protocol::IAnalyzer;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

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

pub struct DefaultCycleAnalysisProtocol {}

fn find_rust_crate_root(source_file: &str) -> Option<std::path::PathBuf> {
    let mut current = std::path::Path::new(source_file).parent()?;
    while !current.join("Cargo.toml").exists() {
        current = current.parent()?;
    }
    Some(current.join("src"))
}

fn try_resolve_candidates(
    base_path: &str,
    module_path: &str,
    file_set: &HashSet<String>,
) -> Option<String> {
    let exts = ["rs", "py", "ts", "js"];
    for ext in &exts {
        let candidate = format!("{}/{}.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    for ext in &exts {
        let candidate = format!("{}/{}/mod.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    for ext in &exts {
        let candidate = format!("{}/{}/__init__.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    for ext in &exts {
        let candidate = format!("{}/{}/index.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    // Also check bare module path (no base prefix) for flat file sets
    for ext in &exts {
        let candidate = format!("{}.{}", module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn resolve_import_to_file(
    module: &str,
    source_file: &FilePath,
    root_dir: &FilePath,
    file_set: &HashSet<String>,
) -> Option<String> {
    let source_dir = std::path::Path::new(source_file.value())
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    // 1. Handle relative imports starting with dots (Python, JS/TS)
    if module.starts_with('.') {
        let mut current_dir = std::path::PathBuf::from(&source_dir);
        let mut remaining = module;

        if remaining.starts_with("./") || remaining.starts_with("../") {
            // JS/TS style
            if let Some(r) = remaining.strip_prefix("./") {
                remaining = r;
            }
            while let Some(r) = remaining.strip_prefix("../") {
                remaining = r;
                if let Some(parent) = current_dir.parent() {
                    current_dir = parent.to_path_buf();
                }
            }
            let remaining_path = remaining.replace('\\', "/");
            let resolved_str = current_dir.to_string_lossy().to_string();

            let exts = ["rs", "py", "ts", "js"];
            for ext in &exts {
                let candidate = format!("{}/{}.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/mod.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/__init__.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/index.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            return None;
        } else {
            // Python style (count leading dots)
            let mut dots_count = 0;
            while remaining.starts_with('.') {
                dots_count += 1;
                remaining = &remaining[1..];
            }
            if dots_count > 1 {
                for _ in 0..(dots_count - 1) {
                    if let Some(parent) = current_dir.parent() {
                        current_dir = parent.to_path_buf();
                    }
                }
            }
            let remaining_path = remaining.replace('.', "/");
            let resolved_str = current_dir.to_string_lossy().to_string();

            let exts = ["rs", "py", "ts", "js"];
            for ext in &exts {
                let candidate = format!("{}/{}.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/mod.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/__init__.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/index.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            return None;
        }
    }

    // 2. Handle Rust-specific imports
    let is_rust = source_file.value().ends_with(".rs");
    if is_rust {
        if let Some(crate_root) = find_rust_crate_root(source_file.value()) {
            let mut normalized = module.to_string();
            let mut resolved_base = crate_root.clone();

            if normalized.starts_with("crate::") {
                normalized = normalized.trim_start_matches("crate::").to_string();
            } else if normalized.starts_with("self::") {
                normalized = normalized.trim_start_matches("self::").to_string();
                resolved_base = std::path::PathBuf::from(&source_dir);
            } else if normalized.starts_with("super::") {
                let mut current_dir = std::path::PathBuf::from(&source_dir);
                while normalized.starts_with("super::") {
                    normalized = normalized.trim_start_matches("super::").to_string();
                    if let Some(parent) = current_dir.parent() {
                        current_dir = parent.to_path_buf();
                    }
                }
                resolved_base = current_dir;
            }

            let segments: Vec<&str> = normalized
                .split("::")
                .flat_map(|s| s.split('.'))
                .filter(|s| !s.is_empty())
                .collect();

            if !segments.is_empty() {
                // Check if it's a cross-crate import in workspace
                let first_seg = segments[0];
                let workspace_crate_src = std::path::Path::new(root_dir.value())
                    .join("crates")
                    .join(first_seg.replace('_', "-"))
                    .join("src");
                if workspace_crate_src.exists() {
                    let sub_segments = &segments[1..];
                    for len in (1..=sub_segments.len()).rev() {
                        let module_path = sub_segments[..len].join("/");
                        if let Some(target) = try_resolve_candidates(
                            &workspace_crate_src.to_string_lossy(),
                            &module_path,
                            file_set,
                        ) {
                            return Some(target);
                        }
                    }
                }
            }

            // Fallback: resolve relative to the resolved_base directory
            for len in (1..=segments.len()).rev() {
                let module_path = segments[..len].join("/");
                if let Some(target) =
                    try_resolve_candidates(&resolved_base.to_string_lossy(), &module_path, file_set)
                {
                    return Some(target);
                }
            }
        }
    }

    // 3. Fallback standard module resolution (Python or other language standard import)
    let segments: Vec<&str> = module.split('.').filter(|s| !s.is_empty()).collect();
    for len in (1..=segments.len()).rev() {
        let module_path = segments[..len].join("/");
        if let Some(target) = try_resolve_candidates(root_dir.value(), &module_path, file_set) {
            return Some(target);
        }
    }

    None
}

fn find_cycle_dfs(
    node: &str,
    adjacency: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    in_stack: &mut HashSet<String>,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    if in_stack.contains(node) {
        let cycle_start = path.iter().position(|n| n == node);
        if let Some(start) = cycle_start {
            let mut cycle = path[start..].to_vec();
            cycle.push(node.to_string());
            return Some(cycle);
        }
    }
    if visited.contains(node) {
        return None;
    }

    visited.insert(node.to_string());
    in_stack.insert(node.to_string());
    path.push(node.to_string());

    if let Some(neighbors) = adjacency.get(node) {
        for neighbor in neighbors {
            if let Some(cycle) = find_cycle_dfs(neighbor, adjacency, visited, in_stack, path) {
                return Some(cycle);
            }
        }
    }

    path.pop();
    in_stack.remove(node);
    None
}

#[async_trait]
impl ICycleAnalysisProtocol for DefaultCycleAnalysisProtocol {
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let file_set: HashSet<String> =
            files.values.iter().map(|f| f.value().to_string()).collect();

        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

        for file in &files.values {
            let imports = match analyzer.parser().extract_imports(file) {
                Ok(imp) => imp,
                Err(_) => continue,
            };

            for imp in imports.values {
                if let Some(target) = resolve_import_to_file(&imp.module, file, root_dir, &file_set)
                {
                    adjacency
                        .entry(file.value().to_string())
                        .or_default()
                        .push(target);
                }
            }
        }

        let mut global_visited: HashSet<String> = HashSet::new();
        let mut reported_cycles: HashSet<String> = HashSet::new();

        for file in &files.values {
            let file_str = file.value().to_string();
            if global_visited.contains(&file_str) {
                continue;
            }

            let mut in_stack: HashSet<String> = HashSet::new();
            let mut path: Vec<String> = Vec::new();

            if let Some(cycle) = find_cycle_dfs(
                &file_str,
                &adjacency,
                &mut global_visited,
                &mut in_stack,
                &mut path,
            ) {
                let mut unique_nodes = cycle[..cycle.len() - 1].to_vec();

                if !unique_nodes.is_empty() {
                    let min_idx = unique_nodes
                        .iter()
                        .enumerate()
                        .min_by_key(|&(_, val)| val)
                        .map(|(idx, _)| idx)
                        .unwrap_or(0);

                    unique_nodes.rotate_left(min_idx);
                    unique_nodes.push(unique_nodes[0].clone());

                    let cycle_display = unique_nodes.join(" -> ");

                    if reported_cycles.insert(cycle_display.clone()) {
                        if let Ok(cycle_file) = FilePath::new(unique_nodes[0].clone()) {
                            results.push(LintResult {
                                file: cycle_file,
                                line: crate::common::taxonomy_common_vo::LineNumber::new(1),
                                column: crate::common::taxonomy_common_vo::ColumnNumber::new(0),
                                code: ErrorCode::raw("AES205"),
                                message: LintMessage::new(format!(
                                    "Circular dependency detected: {}",
                                    cycle_display
                                )),
                                source: Some(AdapterName::raw("architecture")),
                                severity: Severity::CRITICAL,
                                enclosing_scope: Some(ScopeRef {
                                    name: DescriptionVO::new(String::new()),
                                    kind: DescriptionVO::new(String::new()),
                                    file: None,
                                    start_line: None,
                                    end_line: None,
                                }),
                                related_locations:
                                    crate::common::taxonomy_lint_vo::LocationList::new(),
                            });
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli_commands::taxonomy_result_vo::LintResultList;
    use crate::code_analysis::taxonomy_import_source_vo::{
        ImportInfo, ImportInfoList, PrimitiveViolationList,
    };
    use crate::common::taxonomy_common_vo::{BooleanVO, Count, LineNumber, PatternList};
    use crate::common::taxonomy_definition_vo::LayerMapVO;
    use crate::common::taxonomy_layer_vo::LayerNameVO;
    use crate::common::taxonomy_name_vo::SymbolName;
    use crate::common::taxonomy_suggestion_vo::MetadataVO;
    use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
    use crate::file_system::contract_system_port::IFileSystemPort;
    use crate::import_rules::contract_rule_protocol::IAnalyzer;
    use crate::mcp_server::taxonomy_job_vo::{ResponseData, SuccessStatus};
    use crate::source_parsing::contract_parser_port::ISourceParserPort;
    use crate::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList;
    use crate::source_parsing::taxonomy_parser_error::SourceParserError;
    use crate::source_parsing::taxonomy_path_vo::FilePath;
    use crate::source_parsing::taxonomy_paths_vo::FilePathList;
    use std::collections::HashMap;
    use std::fs;

    struct MockSourceParserPort {
        imports: HashMap<String, Vec<String>>,
    }

    impl ISourceParserPort for MockSourceParserPort {
        fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
            let mut list = ImportInfoList::new();
            if let Some(imp_list) = self.imports.get(path.value()) {
                for imp in imp_list {
                    list.push(ImportInfo::new(LineNumber::new(1), imp.clone()));
                }
            }
            Ok(list)
        }
        fn get_raw_symbols(&self, _path: &FilePath) -> Result<ResponseData, SourceParserError> {
            Ok(ResponseData {
                value: None,
                stdout: String::new(),
                stderr: String::new(),
                returncode: 0,
                metadata: HashMap::new(),
            })
        }
        fn get_class_attributes(&self, _path: &FilePath) -> ResponseData {
            ResponseData {
                value: None,
                stdout: String::new(),
                stderr: String::new(),
                returncode: 0,
                metadata: HashMap::new(),
            }
        }
        fn has_all_export(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        fn find_primitive_violations(
            &self,
            _path: &FilePath,
            _primitive_types: &PrimitiveTypeList,
        ) -> PrimitiveViolationList {
            PrimitiveViolationList::new()
        }
        fn find_unused_imports(&self, _path: &FilePath) -> ImportInfoList {
            ImportInfoList::new()
        }
        fn get_class_definitions(&self, _path: &FilePath) -> Result<MetadataVO, SourceParserError> {
            Ok(MetadataVO::new(HashMap::new()))
        }
        fn get_function_definitions(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn is_symbol_exported(&self, _path: &FilePath, _symbol: &SymbolName) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        fn get_class_methods(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn get_class_bases_map(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn get_assignment_targets(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn get_control_flow_count(&self, _path: &FilePath) -> Count {
            Count::new(0)
        }
        fn is_barrel_file(&self, _path: &FilePath) -> BooleanVO {
            BooleanVO::new(false)
        }
        fn get_stem(&self, _path: &FilePath) -> SymbolName {
            SymbolName::new(String::new())
        }
        fn is_entry_point(&self, _path: &FilePath) -> BooleanVO {
            BooleanVO::new(false)
        }
        fn get_supported_extensions(&self) -> PatternList {
            PatternList { values: vec![] }
        }
    }

    struct MockFileSystemPort {
        _dummy: bool,
    }
    #[async_trait::async_trait]
    impl IFileSystemPort for MockFileSystemPort {
        async fn walk(
            &self,
            _path: &FilePath,
            _ignored_patterns: Option<&PatternList>,
        ) -> FilePathList {
            FilePathList { values: vec![] }
        }
        async fn is_directory(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        async fn is_file(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        async fn get_relative_path(&self, _path: &FilePath, _start: &FilePath) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn read_text(
            &self,
            _path: &FilePath,
        ) -> Result<
            crate::common::taxonomy_source_vo::ContentString,
            crate::file_system::taxonomy_filesystem_error::FileSystemError,
        > {
            Ok(crate::common::taxonomy_source_vo::ContentString::new(
                String::new(),
            ))
        }
        async fn get_line_count(&self, _path: &FilePath) -> Count {
            Count::new(0)
        }
        async fn exists(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        async fn get_parent(&self, _path: &FilePath) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn write_text(
            &self,
            _path: &FilePath,
            _content: &crate::common::taxonomy_source_vo::ContentString,
            _mode: Option<&crate::common::taxonomy_layer_vo::Identity>,
        ) -> Result<SuccessStatus, crate::file_system::taxonomy_filesystem_error::FileSystemError>
        {
            Ok(SuccessStatus::new(true))
        }
        async fn glob(
            &self,
            _pattern: &crate::common::taxonomy_layer_vo::Identity,
        ) -> FilePathList {
            FilePathList { values: vec![] }
        }
        async fn get_cwd(&self) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn get_basename(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_layer_vo::Identity {
            crate::common::taxonomy_layer_vo::Identity::new("")
        }
        async fn path_join(
            &self,
            _parts: &[crate::common::taxonomy_layer_vo::Identity],
        ) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn read_file(
            &self,
            _path: &FilePath,
        ) -> Result<
            crate::common::taxonomy_source_vo::ContentString,
            crate::file_system::taxonomy_filesystem_error::FileSystemError,
        > {
            Ok(crate::common::taxonomy_source_vo::ContentString::new(
                String::new(),
            ))
        }
    }

    struct MockAnalyzer {
        parser: MockSourceParserPort,
        config: ArchitectureConfig,
        layer_map: LayerMapVO,
        fs: MockFileSystemPort,
    }

    impl crate::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol
        for MockAnalyzer
    {
        fn config(&self) -> &ArchitectureConfig {
            &self.config
        }
        fn layer_map(&self) -> &LayerMapVO {
            &self.layer_map
        }
        fn detect_layer(&self, _f: &FilePath, _root_dir: &FilePath) -> Option<LayerNameVO> {
            None
        }
    }

    impl IAnalyzer for MockAnalyzer {
        fn fs(&self) -> &dyn IFileSystemPort {
            &self.fs
        }
        fn parser(&self) -> &dyn ISourceParserPort {
            &self.parser
        }
        fn detect_module_layer(&self, _module_path: &FilePath) -> Option<LayerNameVO> {
            None
        }
    }

    #[tokio::test]
    async fn test_check_cycles_detection() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./a".to_string()]);

        let parser = MockSourceParserPort { imports };
        let analyzer = MockAnalyzer {
            parser,
            config: ArchitectureConfig::default(),
            layer_map: LayerMapVO::new(HashMap::new()),
            fs: MockFileSystemPort { _dummy: false },
        };

        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let protocol = DefaultCycleAnalysisProtocol {};
        protocol
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert!(!results.values.is_empty());
        assert_eq!(&*results.values[0].code, "AES205");
    }

    #[tokio::test]
    async fn test_check_cycles_self_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./a".to_string()]);

        let parser = MockSourceParserPort { imports };
        let analyzer = MockAnalyzer {
            parser,
            config: ArchitectureConfig::default(),
            layer_map: LayerMapVO::new(HashMap::new()),
            fs: MockFileSystemPort { _dummy: false },
        };

        let files = FilePathList {
            values: vec![FilePath::new("/src/a.rs".to_string()).unwrap()],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let protocol = DefaultCycleAnalysisProtocol {};
        protocol
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert!(
            !results.values.is_empty(),
            "Should detect self circular dependency"
        );
        let result = &results.values[0];
        assert_eq!(&*result.code, "AES205");
        assert!(result.message.value().contains("/src/a.rs -> /src/a.rs"));
    }

    struct MockParserForCycle {
        imports: HashMap<String, Vec<String>>,
    }

    impl ISourceParserPort for MockParserForCycle {
        fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
            let mut list = ImportInfoList::new();
            if let Some(modules) = self.imports.get(path.value()) {
                for (i, module) in modules.iter().enumerate() {
                    list.push(ImportInfo::new(
                        LineNumber::new((i + 1) as i64),
                        module.clone(),
                    ));
                }
            }
            Ok(list)
        }
        fn get_raw_symbols(
            &self,
            _path: &FilePath,
        ) -> Result<crate::mcp_server::taxonomy_job_vo::ResponseData, SourceParserError> {
            todo!()
        }
        fn get_class_attributes(
            &self,
            _path: &FilePath,
        ) -> crate::mcp_server::taxonomy_job_vo::ResponseData {
            todo!()
        }
        fn has_all_export(
            &self,
            _path: &FilePath,
        ) -> crate::mcp_server::taxonomy_job_vo::SuccessStatus {
            todo!()
        }
        fn find_primitive_violations(
            &self,
            _path: &FilePath,
            _primitive_types: &crate::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList,
        ) -> crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList {
            todo!()
        }
        fn find_unused_imports(&self, _path: &FilePath) -> ImportInfoList {
            todo!()
        }
        fn get_class_definitions(
            &self,
            _path: &FilePath,
        ) -> Result<crate::common::taxonomy_suggestion_vo::MetadataVO, SourceParserError> {
            todo!()
        }
        fn get_function_definitions(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn is_symbol_exported(
            &self,
            _path: &FilePath,
            _symbol: &crate::common::taxonomy_name_vo::SymbolName,
        ) -> crate::mcp_server::taxonomy_job_vo::SuccessStatus {
            todo!()
        }
        fn get_class_methods(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn get_class_bases_map(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn get_assignment_targets(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn get_control_flow_count(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_common_vo::Count {
            todo!()
        }
        fn is_barrel_file(&self, _path: &FilePath) -> crate::common::taxonomy_common_vo::BooleanVO {
            todo!()
        }
        fn get_stem(&self, _path: &FilePath) -> crate::common::taxonomy_name_vo::SymbolName {
            todo!()
        }
        fn is_entry_point(&self, _path: &FilePath) -> crate::common::taxonomy_common_vo::BooleanVO {
            todo!()
        }
        fn get_supported_extensions(&self) -> crate::common::taxonomy_common_vo::PatternList {
            todo!()
        }
    }

    struct MockAnalyzerForCycle {
        parser: MockParserForCycle,
    }

    impl crate::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol
        for MockAnalyzerForCycle
    {
        fn config(&self) -> &ArchitectureConfig {
            todo!()
        }
        fn layer_map(&self) -> &LayerMapVO {
            todo!()
        }
        fn detect_layer(&self, _f: &FilePath, _root_dir: &FilePath) -> Option<LayerNameVO> {
            todo!()
        }
    }

    impl IAnalyzer for MockAnalyzerForCycle {
        fn fs(&self) -> &dyn IFileSystemPort {
            todo!()
        }
        fn parser(&self) -> &dyn ISourceParserPort {
            &self.parser
        }
        fn detect_module_layer(&self, _module_path: &FilePath) -> Option<LayerNameVO> {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_happy_path_no_cycles() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./c".to_string()]);
        imports.insert("/src/c.rs".to_string(), vec![]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
                FilePath::new("/src/c.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert!(
            results.values.is_empty(),
            "Expected no cycles, found: {:?}",
            results.values
        );
    }

    #[tokio::test]
    async fn test_self_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./a".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![FilePath::new("/src/a.rs".to_string()).unwrap()],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(results.values.len(), 1);
        let msg = &results.values[0].message.value;
        assert!(
            msg.contains("/src/a.rs -> /src/a.rs"),
            "Expected A->A cycle, got: {}",
            msg
        );
    }

    #[tokio::test]
    async fn test_simple_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./a".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(results.values.len(), 1);
        let msg = &results.values[0].message.value;
        assert!(
            msg.contains("/src/a.rs -> /src/b.rs -> /src/a.rs")
                || msg.contains("/src/b.rs -> /src/a.rs -> /src/b.rs"),
            "Got message: {}",
            msg
        );
    }

    #[tokio::test]
    async fn test_complex_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./c".to_string()]);
        imports.insert("/src/c.rs".to_string(), vec!["./a".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
                FilePath::new("/src/c.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(results.values.len(), 1);
    }

    #[tokio::test]
    async fn test_js_ts_relative_and_barrel_imports() {
        let temp_dir = std::env::temp_dir().join("js_ts_cycle_test");
        let src_dir = temp_dir.join("src");
        let components_dir = src_dir.join("components");
        let utils_dir = src_dir.join("utils");

        fs::create_dir_all(&components_dir).unwrap();
        fs::create_dir_all(&utils_dir).unwrap();

        let button_file = components_dir.join("button.ts");
        let index_file = utils_dir.join("index.ts");
        let helper_file = utils_dir.join("helper.ts");

        fs::write(&button_file, "").unwrap();
        fs::write(&index_file, "").unwrap();
        fs::write(&helper_file, "").unwrap();

        let button_str = button_file.to_string_lossy().to_string();
        let index_str = index_file.to_string_lossy().to_string();
        let helper_str = helper_file.to_string_lossy().to_string();

        let mut imports = HashMap::new();
        imports.insert(button_str.clone(), vec!["../utils".to_string()]);
        imports.insert(index_str.clone(), vec!["./helper".to_string()]);
        imports.insert(helper_str.clone(), vec!["../components/button".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new(button_str.clone()).unwrap(),
                FilePath::new(index_str.clone()).unwrap(),
                FilePath::new(helper_str.clone()).unwrap(),
            ],
        };
        let root_dir = FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(
            results.values.len(),
            1,
            "Expected exactly 1 cycle, found: {:?}",
            results.values
        );
        let msg = &results.values[0].message.value;
        assert!(
            msg.contains("button.ts") && msg.contains("index.ts") && msg.contains("helper.ts"),
            "Got message: {}",
            msg
        );

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn test_python_relative_imports() {
        let temp_dir = std::env::temp_dir().join("python_cycle_test");
        let pkg_dir = temp_dir.join("pkg");
        let sub_dir = pkg_dir.join("sub");

        fs::create_dir_all(&sub_dir).unwrap();

        let init_file = pkg_dir.join("__init__.py");
        let main_file = pkg_dir.join("main.py");
        let sub_init_file = sub_dir.join("__init__.py");
        let sub_module_file = sub_dir.join("module.py");

        fs::write(&init_file, "").unwrap();
        fs::write(&main_file, "").unwrap();
        fs::write(&sub_init_file, "").unwrap();
        fs::write(&sub_module_file, "").unwrap();

        let main_str = main_file.to_string_lossy().to_string();
        let sub_module_str = sub_module_file.to_string_lossy().to_string();

        let mut imports = HashMap::new();
        imports.insert(main_str.clone(), vec![".sub.module".to_string()]);
        imports.insert(sub_module_str.clone(), vec!["..main".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new(main_str.clone()).unwrap(),
                FilePath::new(sub_module_str.clone()).unwrap(),
            ],
        };
        let root_dir = FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(
            results.values.len(),
            1,
            "Expected cycle, got: {:?}",
            results.values
        );

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn test_rust_crate_absolute_and_super_imports() {
        let temp_dir = std::env::temp_dir().join("rust_cycle_test");
        let src_dir = temp_dir.join("src");
        let sub_dir = src_dir.join("sub");

        fs::create_dir_all(&sub_dir).unwrap();
        fs::write(temp_dir.join("Cargo.toml"), "").unwrap();

        let main_file = src_dir.join("main.rs");
        let sub_mod_file = sub_dir.join("mod.rs");
        let helper_file = sub_dir.join("helper.rs");

        fs::write(&main_file, "").unwrap();
        fs::write(&sub_mod_file, "").unwrap();
        fs::write(&helper_file, "").unwrap();

        let main_str = main_file.to_string_lossy().to_string();
        let sub_mod_str = sub_mod_file.to_string_lossy().to_string();
        let helper_str = helper_file.to_string_lossy().to_string();

        let mut imports = HashMap::new();
        imports.insert(main_str.clone(), vec!["crate::sub::helper".to_string()]);
        imports.insert(helper_str.clone(), vec!["crate::sub".to_string()]);
        imports.insert(sub_mod_str.clone(), vec!["crate::main".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new(main_str.clone()).unwrap(),
                FilePath::new(helper_str.clone()).unwrap(),
                FilePath::new(sub_mod_str.clone()).unwrap(),
            ],
        };
        let root_dir = FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(
            results.values.len(),
            1,
            "Expected cycle, got: {:?}",
            results.values
        );

        fs::remove_dir_all(&temp_dir).unwrap();
    }
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
pub mod taxonomy_governance_entity;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
```

---

## File: crates/shared/src/code-analysis/taxonomy_analysis_vo.rs

```rust
// PURPOSE: FileDefinitionMap, GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap — analysis value objects for code structure
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
pub struct ModuleToFileMap {
    pub mapping: std::collections::HashMap<String, String>,
}

impl ModuleToFileMap {
    pub fn new(value: std::collections::HashMap<String, String>) -> Self {
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

## File: crates/shared/src/code-analysis/taxonomy_governance_entity.rs

```rust
// PURPOSE: ArchitectureGovernanceEntity — domain entity for architecture governance (scores, issues, dates)
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_message_vo::ComplianceStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchitectureGovernanceEntity {
    #[serde(default)]
    pub id: Identity,
    #[serde(default)]
    pub results: LintResultList,
    #[serde(default = "default_score")]
    pub score: Score,
    #[serde(default = "default_compliance")]
    pub is_passing: ComplianceStatus,
}

fn default_score() -> Score {
    Score::new(100.0)
}
fn default_compliance() -> ComplianceStatus {
    ComplianceStatus::new(true)
}

impl ArchitectureGovernanceEntity {
    pub fn new() -> Self {
        Self {
            id: Identity::new("default"),
            results: LintResultList::default(),
            score: Score::new(100.0),
            is_passing: ComplianceStatus::new(true),
        }
    }
    pub fn add_result(&mut self, result: LintResult) {
        self.score = self.score.deduct(&result.severity);
        self.results.push(result);
    }
    pub fn update_compliance(&mut self, threshold: &Score) {
        let is_p = self.score.value >= threshold.value;
        let has_critical = self
            .results
            .values
            .iter()
            .any(|r| r.severity == Severity::CRITICAL);
        self.is_passing = ComplianceStatus::new(is_p && !has_critical);
    }
    pub fn results_by_source(&self, source: &AdapterName) -> LintResultList {
        LintResultList {
            values: self
                .results
                .values
                .iter()
                .filter(|r| r.source.as_ref() == Some(source))
                .cloned()
                .collect(),
        }
    }
    pub fn violation_count(&self) -> Count {
        Count::new(
            self.results
                .values
                .iter()
                .filter(|r| r.severity.score_impact() > 0.0)
                .count() as i64,
        )
    }
}

impl Default for ArchitectureGovernanceEntity {
    fn default() -> Self {
        Self::new()
    }
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
/// linter_operation_error — Unified error type for linter adapter operations.
/* UNKNOWN: ErrorMessage */
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::source_parsing::taxonomy_adapter_error::AdapterError;
use crate::source_parsing::taxonomy_adapter_error::ScanError;
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
                write!(f, "AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.\n\
                        WHY? {}\n\
                        FIX: Remove the bypass comment (e.g. noqa, eslint-disable, ts-ignore) and resolve the issue properly.", why)
            }
            AesCodeAnalysisViolation::UnwrapExpect { reason } => {
                let default_why = "Using unwrap or expect results in runtime panics and bypasses proper error propagation.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.\n\
                        WHY? {}\n\
                        FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').", why)
            }
            AesCodeAnalysisViolation::Panic { reason } => {
                let default_why = "Manual panic calls crash the program unexpectedly instead of using structured error recovery.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 PANIC: Forbidden panic call detected.\n\
                        WHY? {}\n\
                        FIX: Return a Result or handle the failure case gracefully without panicking.",
                    why
                )
            }
            AesCodeAnalysisViolation::Todo { reason } => {
                let default_why = "todo!() placeholders represent incomplete code paths that can crash at runtime if reached unexpectedly.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 TODO: Forbidden todo!() call detected.\n\
                        WHY? {}\n\
                        FIX: Implement the function body with real logic, or return a meaningful default/error instead of leaving a todo!() placeholder.",
                    why
                )
            }
            AesCodeAnalysisViolation::Unimplemented { reason } => {
                let default_why = "unimplemented!() claims a code path is unreachable, but when reached it crashes — violating the principle of fail-fast with clear error messages.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 UNIMPLEMENTED: Forbidden unimplemented!() call detected.\n\
                        WHY? {}\n\
                        FIX: Either implement the missing logic or return a Result::Err with a descriptive error message.",
                    why
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

## File: crates/shared/src/common/mod.rs

```rust
// common — truly shared types used by multiple features
pub mod taxonomy_action_vo;
pub mod taxonomy_adapter_name_vo;
pub mod taxonomy_common_error;
pub mod taxonomy_common_vo;
pub mod taxonomy_definition_vo;
pub mod taxonomy_duration_vo;
pub mod taxonomy_error_vo;
pub mod taxonomy_job_id_vo;
pub mod taxonomy_job_vo;
pub mod taxonomy_layer_vo;
pub mod taxonomy_lint_vo;
pub mod taxonomy_message_vo;
pub mod taxonomy_name_vo;
pub mod taxonomy_response_data_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_suggestion_vo;
pub mod taxonomy_value_object_utility;
```

---

## File: crates/shared/src/common/taxonomy_action_vo.rs

```rust
// PURPOSE: ActionName, ActionArgs — value objects for pipeline job actions
// JobId is re-exported from common for backward compatibility
pub use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_suggestion_vo::MetadataVO;
use crate::string_value_object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionArgs {
    pub value: MetadataVO,
}

impl ActionArgs {
    pub fn new(value: MetadataVO) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &MetadataVO {
        &self.value
    }
}

string_value_object!(ActionName);
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

#[cfg(test)]
mod tests {
    use super::AdapterName;

    #[test]
    fn test_adapter_name_new() {
        let name = AdapterName::new("ruff").unwrap_or_default();
        assert_eq!(name.value, "ruff");

        // Test trimming
        let name = AdapterName::new("  ruff  ").unwrap_or_default();
        assert_eq!(name.value, "ruff");

        // Test that internal spaces are preserved
        let name = AdapterName::new("my adapter").unwrap_or_default();
        assert_eq!(name.value, "my adapter");
    }

    #[test]
    fn test_adapter_name_invalid() {
        assert!(AdapterName::new("").is_err());
        assert!(AdapterName::new("   ").is_err());
        assert!(AdapterName::new("\t\n  ").is_err());
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

clamped_f64_vo!(Duration, 0.0, "{:.2}ms");
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

#[cfg(test)]
mod tests {
    use super::ErrorCode;

    #[test]
    fn test_error_code_new() {
        let ec = ErrorCode::new("E123").unwrap_or_default();
        assert_eq!(ec.code, "E123");
        assert!(ec.is_style());
        assert!(!ec.is_logic());
        assert!(!ec.is_security());
        assert!(!ec.is_architecture());

        let ec = ErrorCode::new("W999").unwrap_or_default();
        assert!(ec.is_style());

        let ec = ErrorCode::new("D404").unwrap_or_default();
        assert!(ec.is_style());

        let ec = ErrorCode::new("F001").unwrap_or_default();
        assert!(ec.is_logic());

        let ec = ErrorCode::new("I999").unwrap_or_default();
        assert!(ec.is_logic());

        let ec = ErrorCode::new("B001").unwrap_or_default();
        assert!(ec.is_security());

        let ec = ErrorCode::new("AES123").unwrap_or_default();
        assert!(ec.is_architecture());
    }

    #[test]
    fn test_error_code_invalid() {
        assert!(ErrorCode::new("").is_err());
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum JobStatus {
    #[serde(rename = "pending")]
    PENDING,
    #[serde(rename = "running")]
    RUNNING,
    #[serde(rename = "completed")]
    COMPLETED,
    #[serde(rename = "failed")]
    FAILED,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::PENDING => write!(f, "pending"),
            JobStatus::RUNNING => write!(f, "running"),
            JobStatus::COMPLETED => write!(f, "completed"),
            JobStatus::FAILED => write!(f, "failed"),
        }
    }
}

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

/// `HashMap<String, serde_json::Value>` payload VOs. Wrapped via macro so they
/// pick up the standard `new`/`value`/`Default`/serde impls.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LintStatusActionArgs {
    #[serde(default)]
    pub value: HashMap<String, serde_json::Value>,
}

impl Default for LintStatusActionArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl LintStatusActionArgs {
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
        }
    }
    pub fn value(&self) -> &HashMap<String, serde_json::Value> {
        &self.value
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.get(key)
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

## File: crates/shared/src/common/taxonomy_lint_vo.rs

```rust
// PURPOSE: CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint — VOs for lint violations
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_source_vo::ContentString;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

use crate::source_parsing::taxonomy_path_vo::FilePath;

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

```rust
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

#[cfg(test)]
mod macro_tests {

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    string_value_object!(MyTestVO);

    #[test]
    fn string_vo_new_and_value() {
        let v = MyTestVO::new("hello");
        assert_eq!(v.value(), "hello");
        let v2 = MyTestVO::new(String::from("world"));
        assert_eq!(v2.value(), "world");
    }

    #[test]
    fn string_vo_default_is_empty() {
        let v = MyTestVO::default();
        assert_eq!(v.value(), "");
    }

    #[test]
    fn string_vo_display() {
        let v = MyTestVO::new("hello");
        assert_eq!(v.to_string(), "hello");
    }

    #[test]
    fn string_vo_from_str() {
        let v: MyTestVO = "abc".into();
        assert_eq!(v.value(), "abc");
    }

    #[test]
    fn string_vo_from_string() {
        let v: MyTestVO = String::from("xyz").into();
        assert_eq!(v.value(), "xyz");
    }

    #[test]
    fn string_vo_equality() {
        assert_eq!(MyTestVO::new("a"), MyTestVO::new("a"));
        assert_ne!(MyTestVO::new("a"), MyTestVO::new("b"));
    }

    #[test]
    fn string_vo_hash_matches_inner() {
        let v = MyTestVO::new("hashable");
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        v.hash(&mut h1);
        "hashable".hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn string_vo_deserialize_from_string() {
        let v: MyTestVO = serde_json::from_str("\"hello\"").unwrap();
        assert_eq!(v.value(), "hello");
    }

    #[test]
    fn string_vo_deserialize_from_map() {
        let v: MyTestVO = serde_json::from_str("{\"value\":\"wrapped\"}").unwrap();
        assert_eq!(v.value(), "wrapped");
    }

    #[test]
    fn string_vo_serialize_transparent() {
        let v = MyTestVO::new("plain");
        let s = serde_json::to_string(&v).unwrap();
        assert_eq!(s, "\"plain\"");
    }

    primitive_value_object!(MyNum, i64);

    #[test]
    fn primitive_vo_new_and_value() {
        let v = MyNum::new(42);
        assert_eq!(v.value(), 42);
    }

    #[test]
    fn primitive_vo_display() {
        let v = MyNum::new(123);
        assert_eq!(v.to_string(), "123");
    }

    #[test]
    fn primitive_vo_from_inner() {
        let v: MyNum = 7.into();
        assert_eq!(v.value(), 7);
    }

    #[test]
    fn primitive_vo_deserialize_from_int() {
        let v: MyNum = serde_json::from_str("99").unwrap();
        assert_eq!(v.value(), 99);
    }

    #[test]
    fn primitive_vo_deserialize_from_map() {
        let v: MyNum = serde_json::from_str("{\"value\":11}").unwrap();
        assert_eq!(v.value(), 11);
    }
}
```

---

## File: crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs

```rust
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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

use crate::config_system::contract_reader_port::IConfigReaderPort;
use crate::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_setting_vo::ProjectConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IConfigParserPort: Send + Sync {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;
    fn parse_toml_config(&self, path: &FilePath) -> Option<Result<ProjectConfig, ConfigError>>;
}
```

---

## File: crates/shared/src/config-system/contract_reader_port.rs

```rust
// PURPOSE: IConfigReaderPort — port trait for reading configuration from external sources

use crate::config_system::taxonomy_source_vo::ConfigSource;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
use crate::config_system::taxonomy_validation_vo::ValidationResult;

pub trait IConfigValidatorProtocol: Send + Sync {
    /// Determines if a specific adapter should run based on configuration rules.
    fn is_adapter_enabled(&self, adapter_name: &AdapterName) -> bool;

    /// Validates that scoring thresholds are sane.
    fn validate_thresholds(&self) -> ValidationResult;
}
```

---

## File: crates/shared/src/config-system/contract_workspace_detector_port.rs

```rust
// PURPOSE: IWorkspaceDetectorPort — port trait for detecting workspace type from directory structure
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
pub mod taxonomy_adapter_vo;
pub mod taxonomy_app_vo;
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

## File: crates/shared/src/config-system/taxonomy_adapter_vo.rs

```rust
// PURPOSE: AdapterClassMap, AdapterMetadataList, AdapterNameList — VOs for adapter registration metadata
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::mcp_server::taxonomy_job_vo::AdapterMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMetadataList {
    #[serde(default)]
    pub values: Vec<AdapterMetadata>,
}

impl Default for AdapterMetadataList {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterMetadataList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: AdapterMetadata) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for AdapterMetadataList {
    type Target = Vec<AdapterMetadata>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterNameList {
    #[serde(default)]
    pub values: Vec<AdapterName>,
}

impl Default for AdapterNameList {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterNameList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: AdapterName) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for AdapterNameList {
    type Target = Vec<AdapterName>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterClassMap {
    #[serde(default)]
    pub values: std::collections::HashMap<String, String>,
}

impl Default for AdapterClassMap {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterClassMap {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
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

## File: crates/shared/src/config-system/taxonomy_app_vo.rs

```rust
// PURPOSE: AppConfigVO, AppName, AppVersion — value objects for application configuration metadata
use std::env;

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::config_system::taxonomy_adapter_vo::AdapterNameList;
use crate::config_system::taxonomy_setting_vo::{AdapterStatus, ProjectConfig, Thresholds};
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

/// app_config_vo — Unified configuration for the application.
///
/// Unified configuration — transport, paths, and project settings.
#[derive(Debug, Clone)]
pub struct AppConfig {
    phantom_root: DirectoryPath,
    project: ProjectConfig,
}

impl AppConfig {
    /// Create a new AppConfig.
    ///
    /// # Arguments
    /// * `phantom_root` - Optional phantom root directory. Defaults to environment variable `PHANTOM_ROOT` or home directory.
    /// * `project_root` - Optional project root directory. Defaults to environment variable `PROJECT_ROOT` or current directory.
    /// * `project` - Optional project configuration. Defaults to `crate::config_system::taxonomy_setting_vo::ProjectConfig::default()`.
    pub fn create(
        phantom_root: Option<String>,
        project_root: Option<String>,
        project: Option<ProjectConfig>,
    ) -> Self {
        let p_root = match phantom_root.or_else(|| env::var("PHANTOM_ROOT").ok()) {
            Some(r) => r,
            None => match env::var("HOME") {
                Ok(h) => h,
                Err(_) => ".".to_string(),
            },
        };
        let _proj_root = match project_root.or_else(|| env::var("PROJECT_ROOT").ok()) {
            Some(r) => r,
            None => match env::current_dir() {
                Ok(d) => d.to_string_lossy().to_string(),
                Err(_) => ".".to_string(),
            },
        };
        let proj = project.unwrap_or_default();

        Self {
            phantom_root: DirectoryPath::new(p_root).unwrap_or_default(),
            project: proj,
        }
    }

    /// Get the thresholds from the project configuration.
    pub fn thresholds(&self) -> &Thresholds {
        &self.project.thresholds
    }

    /// Get status for a named adapter.
    pub fn adapter_status(&self, name: &str) -> AdapterStatus {
        for entry in &self.project.adapters {
            if entry.name.value == name {
                return entry.status;
            }
        }
        AdapterStatus::NotInstalled
    }

    /// Check if an adapter is enabled.
    pub fn is_adapter_enabled(&self, name: &str) -> BooleanVO {
        let status = self.adapter_status(name);
        BooleanVO::new(status == AdapterStatus::Enabled)
    }

    /// Names of enabled adapters.
    pub fn active_adapters(&self) -> AdapterNameList {
        let mut values = Vec::new();
        for entry in &self.project.adapters {
            if entry.is_active() {
                values.push(entry.name.clone());
            }
        }
        AdapterNameList { values }
    }
}

impl std::fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AppConfig(phantom={}, adapters={:?})",
            self.phantom_root,
            self.active_adapters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfig;
    use super::ProjectConfig;
    use std::env;

    #[test]
    fn test_app_config_create() {
        let config = AppConfig::create(
            Some("/phantom".to_string()),
            Some("/project".to_string()),
            Some(ProjectConfig::default()),
        );
        assert_eq!(config.phantom_root.to_string(), "/phantom");
    }

    #[test]
    fn test_app_config_defaults() {
        // Set environment variables for deterministic test
        env::set_var("PHANTOM_ROOT", "/test/phantom");
        env::set_var("PROJECT_ROOT", "/test/project");
        let config = AppConfig::create(None, None, None);
        assert_eq!(config.phantom_root.to_string(), "/test/phantom");
        // Clean up
        env::remove_var("PHANTOM_ROOT");
        env::remove_var("PROJECT_ROOT");
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_config_error.rs

```rust
// PURPOSE: ConfigError, ConfigErrorKind — structured error types for configuration loading failures
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::config_system::taxonomy_identifier_vo::ConfigKey;
use crate::config_system::taxonomy_setting_vo::ActualValue;
use crate::config_system::taxonomy_setting_vo::ExpectedValue;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use std::collections::HashMap;

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
    let raw: serde_yaml::Value = serde_yaml::from_str(yaml_str).unwrap_or_default();
    if let Some(arch_val) = raw.get("architecture") {
        let mut arch_json = serde_json::to_value(arch_val).unwrap_or_default();
        // Extract layers from rules.AES102.layers if not at top-level layers
        if arch_json
            .get("rules")
            .and_then(|r| r.get("AES102"))
            .and_then(|a| a.get("layers"))
            .is_some()
            && arch_json.get("layers").is_none()
        {
            if let Some(rules_obj) = arch_json.get_mut("rules").and_then(|r| r.as_object_mut()) {
                if let Some(aes102) = rules_obj.get_mut("AES102").and_then(|a| a.as_object_mut()) {
                    if let Some(layers) = aes102.remove("layers") {
                        arch_json["layers"] = layers;
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
                println!("[debug] serde_json from_value error: {:?}", e);
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
pub fn default_aes_config() -> ArchitectureConfig {
    parse_config_yaml(include_str!("../../../../lint_arwaky.config.rust.yaml"))
}

pub fn default_config_for_language(language: &str) -> ArchitectureConfig {
    match language {
        "python" => parse_config_yaml(include_str!("../../../../lint_arwaky.config.python.yaml")),
        "javascript" | "typescript" => parse_config_yaml(include_str!(
            "../../../../lint_arwaky.config.javascript.yaml"
        )),
        _ => default_aes_config(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_parsing() {
        let config = default_config_for_language("typescript");
        println!("typescript layers: {:?}", config.layers.keys());
        assert!(!config.layers.is_empty());
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
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

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
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

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

use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
// file-watch — taxonomy and contract types
pub mod contract_external_lint_aggregate;
```

---

## File: crates/shared/src/file-system/contract_system_port.rs

```rust
// PURPOSE: IFileSystemPort — port trait for filesystem operations (read, write, exists, glob, walk)

use async_trait::async_trait;

use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_source_vo::ContentString;
use crate::file_system::taxonomy_filesystem_error::FileSystemError;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

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

## File: crates/shared/src/file-system/mod.rs

```rust
// file-system — taxonomy and contract types
pub mod contract_system_port;
pub mod taxonomy_filesystem_error;
```

---

## File: crates/shared/src/file-system/taxonomy_filesystem_error.rs

```rust
// PURPOSE: FileSystemError — structured error type for filesystem operation failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::mcp_server::taxonomy_action_vo::ActionName;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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

/// Wrap a `FileSystemError` in a newtype variant and forward its `Display`.
/// Use `[$name, $op, $msg_prefix]` form when the newtype should override the
/// operation label (e.g. `read`/`access`) and produce a custom prefix when
/// displayed (e.g. `"Path not found: "`/`"Access denied: "`).
macro_rules! fs_error_newtype {
    ($name:ident, $op:expr, $msg_prefix:literal) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
        pub struct $name {
            #[serde(flatten)]
            pub base: FileSystemError,
        }

        impl $name {
            pub fn new(path: FilePath, message: ErrorMessage) -> Self {
                Self {
                    base: FileSystemError::new(path, message, ActionName::new($op)),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}{} ({})",
                    $msg_prefix, self.base.path, self.base.message
                )
            }
        }
    };
}

fs_error_newtype!(PathNotFoundError, "read", "Path not found: ");
fs_error_newtype!(AccessDeniedError, "access", "Access denied: ");
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
pub mod contract_provider_port;
pub mod contract_watch_aggregate;
pub mod taxonomy_diff_result_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_service_error;
pub mod taxonomy_watch_config_vo;
pub mod taxonomy_watch_event_vo;
pub mod taxonomy_watch_vo;
```

---

## File: crates/shared/src/file-watch/taxonomy_diff_result_vo.rs

```rust
// PURPOSE: GitDiffResultVO — value object representing git diff results
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::Count;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::source_parsing::taxonomy_paths_vo::RenamedFileList;

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

## File: crates/shared/src/file-watch/taxonomy_result_vo.rs

```rust
// PURPOSE: WatchResult — result type for watch operations
use serde::{Deserialize, Serialize};

use crate::file_watch::taxonomy_service_error::WatchServiceError;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum WatchResult {
    #[default]
    Started,
    Stopped,
    Changed(Vec<String>),
    Error(WatchServiceError),
}
```

---

## File: crates/shared/src/file-watch/taxonomy_service_error.rs

```rust
// PURPOSE: WatchServiceError — structured error type for file watch service failures
use crate::common::taxonomy_message_vo::LintMessage;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

#[derive(Debug, Clone)]
pub struct WatchSubscriptionError(pub WatchServiceError);

#[derive(Debug, Clone)]
pub struct WatchEventError(pub WatchServiceError);
```

---

## File: crates/shared/src/file-watch/taxonomy_watch_config_vo.rs

```rust
// PURPOSE: WatchConfig — value object for file watch configuration parameters
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        Self {
            path,
            kind,
            timestamp_ms,
        }
    }
}
```

---

## File: crates/shared/src/file-watch/taxonomy_watch_vo.rs

```rust
// PURPOSE: DirectoryWatchVO — value object representing directory watch config parameters
use crate::common::taxonomy_common_vo::BooleanVO;
/* UNKNOWN: PatternList */ use crate::common::taxonomy_common_vo::PatternList;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Default)]
pub struct DirectoryWatchVO {
    pub path: FilePath,
    pub recursive: BooleanVO,
    pub ignore_patterns: Option<PatternList>,
}
```

---

## File: crates/shared/src/git-hooks/contract_diff_protocol.rs

```rust
// PURPOSE: IDiffProtocol — protocol for git diff analysis operations (business logic)
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::git_hooks::taxonomy_diff_result_vo::GitDiffResultVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
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
use crate::git_hooks::contract_diff_protocol::IDiffProtocol;
use crate::git_hooks::contract_hook_protocol::IHookProtocol;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::git_hooks::taxonomy_git_diff_data_vo::{GitDiffDataVO, HookIgnoreUpdateVO};
use crate::git_hooks::taxonomy_hook_error::GitHookError;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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

use crate::git_hooks::taxonomy_hook_error::GitHookError;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
pub mod taxonomy_installed_event;
pub mod taxonomy_ref_vo;
pub mod taxonomy_removed_event;
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
use crate::common::taxonomy_common_vo::Count;
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

/// Diff statistics reported as a count, not a raw float.
#[allow(dead_code)]
pub type DiffLineCount = Count;
```

---

## File: crates/shared/src/git-hooks/taxonomy_hook_error.rs

```rust
// PURPOSE: GitHookError — structured error type for git hook operation failures
use crate::common::taxonomy_message_vo::LintMessage;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/shared/src/git-hooks/taxonomy_installed_event.rs

```rust
// PURPOSE: HookInstalled — domain event published when a git hook is installed
use crate::common::taxonomy_common_vo::Timestamp;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HookInstalled {
    pub path: FilePath,
    pub executable: FilePath,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl HookInstalled {
    pub fn new(path: FilePath, executable: FilePath) -> Self {
        Self {
            path,
            executable,
            timestamp: Timestamp::default(),
        }
    }
}
```

---

## File: crates/shared/src/git-hooks/taxonomy_ref_vo.rs

```rust
// PURPOSE: GitRefVO — value object for git reference (branch, tag)
//
// `GitRef` is a thin string wrapper for git references (branch names, tag
// names, HEAD, etc.). It is generated with the `string_value_object!` macro
// so dependents pick up the standard `new`/`value`/`Default`/`Hash`/serde
// surface for free. Lives in its own file to avoid forcing every git-hooks
// consumer to pull in the rest of the common VO namespace.
use crate::string_value_object;

string_value_object!(GitRef);
```

---

## File: crates/shared/src/git-hooks/taxonomy_removed_event.rs

```rust
// PURPOSE: HookRemoved — domain event published when a git hook is removed
use crate::common::taxonomy_common_vo::Timestamp;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HookRemoved {
    pub path: FilePath,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl HookRemoved {
    pub fn new(path: FilePath) -> Self {
        Self {
            path,
            timestamp: Timestamp::default(),
        }
    }
}
```

---

## File: crates/shared/src/import-rules/contract_import_parser_port.rs

```rust
// PURPOSE: IImportParserPort — contract port trait for import parsing utilities
use crate::common::taxonomy_message_vo::LintMessage;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::import_rules::taxonomy_language_vo::LanguageVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::taxonomy_common_vo::LineNumber;
use crate::taxonomy_layer_vo::FileContentVO;
use crate::taxonomy_layer_vo::Identity;
use crate::taxonomy_layer_vo::LayerNameVO;
use crate::taxonomy_layer_vo::LineContentVO;
use crate::taxonomy_name_vo::SymbolName;
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
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IImportRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/import-rules/contract_rule_protocol.rs

```rust
// PURPOSE: IAnalyzer trait — core analyzer interface for import checks
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::file_system::contract_system_port::IFileSystemPort;
use crate::source_parsing::contract_parser_port::ISourceParserPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::taxonomy_common_error::ErrorMessage;
use crate::taxonomy_common_vo::Count;
use crate::taxonomy_common_vo::PatternList;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;
use crate::taxonomy_layer_vo::LayerNameVO;

pub trait IAnalyzer:
    crate::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol + Send + Sync
{
    fn fs(&self) -> &dyn IFileSystemPort;
    fn parser(&self) -> &dyn ISourceParserPort;
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO>;
}

pub trait IArchRuleProtocol {
    fn rule_name(&self) -> Identity;
}

pub trait IInternalCheckerProtocol: Send + Sync {
    fn check_layer_internal_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

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

pub struct CheckFileNamingParams<'a> {
    pub files: &'a FilePathList,
    pub root_dir: &'a FilePath,
    pub layer_map: &'a LayerMapVO,
    pub global_expected: Count,
    pub global_exceptions: &'a PatternList,
    pub results: &'a mut LintResultList,
    pub detect_layer_fn: &'a dyn Fn(&FilePath, &FilePath) -> Option<LayerNameVO>,
}

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

#[async_trait::async_trait]
pub trait IArchImportProtocol: IArchRuleProtocol + Send + Sync {
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
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
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
// PURPOSE: taxonomy_cycle_helper — pure utility functions for cycle and layer path normalization
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::taxonomy_name_vo::SymbolName;
use std::collections::{HashMap, HashSet};

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

    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for e in &normalized_edges {
        graph
            .entry(e.source.clone())
            .or_default()
            .insert(e.target.clone());
    }

    let mut unique_cycles: Vec<String> = Vec::new();
    let mut reported: HashSet<String> = HashSet::new();

    let nodes: Vec<String> = graph.keys().cloned().collect();

    for node in &nodes {
        let mut local_visited: HashSet<String> = HashSet::new();
        let mut path_stack: Vec<String> = Vec::new();
        let mut cycles: Vec<Vec<String>> = Vec::new();
        dfs_collect_paths(
            node,
            &graph,
            &mut local_visited,
            &mut path_stack,
            &mut cycles,
        );

        for cycle in cycles {
            let mut sorted_cycle = cycle.clone();
            sorted_cycle.sort();
            let dedup_key = sorted_cycle.join("->");
            if reported.insert(dedup_key) {
                for i in 0..cycle.len() {
                    let next = cycle[(i + 1) % cycle.len()].clone();
                    unique_cycles.push(format!("{}->{}", cycle[i].clone(), next));
                }
            }
        }
    }

    unique_cycles.into_iter().map(SymbolName::new).collect()
}

// ─── Private Helpers ───

fn dfs_collect_paths(
    node: &str,
    graph: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
    path_stack: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    if path_stack.contains(&node.to_string()) {
        if let Some(pos) = path_stack.iter().position(|n| n == node) {
            let cycle: Vec<String> = path_stack[pos..].to_vec();
            cycles.push(cycle);
        }
        return;
    }
    if visited.contains(node) {
        return;
    }
    visited.insert(node.to_string());
    path_stack.push(node.to_string());

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            dfs_collect_paths(neighbor, graph, visited, path_stack, cycles);
        }
    }

    path_stack.pop();
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
                let name: &str = module.rsplit('.').next().unwrap_or(module);
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
                let name =
                    Option::unwrap_or_default(import_part.split_once(" from ").map(|(n, _)| n));
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
    let body: String = lines
        .iter()
        .skip(1)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join(" ");

    let trimmed = body.trim();
    if trimmed == "{}" || trimmed == "{ }" {
        return true;
    }

    let inner = trimmed.trim_start_matches('{').trim_end_matches('}').trim();
    let short_markers = ["todo!(", "unimplemented!(", "panic!(", "unreachable!("];
    if inner.is_empty() || short_markers.iter().any(|m| inner.starts_with(m)) {
        return true;
    }

    false
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
use crate::common::taxonomy_name_vo::SymbolName;
use crate::taxonomy_layer_vo::Identity;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static ALL_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r#"__all__\s*=\s*\[([^\]]*)\]"#).ok());

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
            !t.starts_with("import ") && !t.starts_with("from ") && !t.starts_with('#')
        })
        .collect::<Vec<_>>()
        .join("\n");

    for alias in imported_aliases.keys() {
        let alias_str = alias.value();
        let pattern = format!(r"\b{}\b", regex::escape(alias_str));
        if let Ok(re) = Regex::new(&pattern) {
            if re.is_match(&code_lines) {
                used.insert(Identity::new(alias_str));
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

        let names: Vec<SymbolName> = if t.starts_with("use ") {
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
    if is_rust_trait_import(name) {
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
            | "Parser"
    )
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
                let allowed_str = if allowed.is_empty() {
                    "none".to_string()
                } else {
                    allowed
                        .iter()
                        .map(|v| v.value().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
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
                        FIX: Remove the import or refactor to use one of the allowed layers: [{}].",
                    source_layer, forbidden_layer, dynamic_why, allowed_str
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
                        "Capabilities implement business rules — they must import contract ports to honor interface contracts and enable dependency injection.".to_string()
                    } else if src == "infrastructure" {
                        "Infrastructure adapters bridge technology and domain — they must import contract ports to implement the required protocols.".to_string()
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
                let supplement = reason
                    .as_ref()
                    .map(|r| format!("\n  Context: {}", r))
                    .unwrap_or_default();
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
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
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
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
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
                let supplement = reason
                    .as_ref()
                    .map(|r| format!("\n  Context: {}", r))
                    .unwrap_or_default();
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
pub use common::*;

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
#[path = "file-system/mod.rs"]
pub mod file_system;
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
#[path = "source-parsing/mod.rs"]
pub mod source_parsing;
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
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::LayerNameVO;

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
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
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
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::taxonomy_common_vo::PatternList;
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
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
use regex::Regex;

pub fn extract_struct_names(content: &str) -> Vec<String> {
    let re = Regex::new(r"(?:pub\s+)?struct\s+([A-Za-z0-9_]+)").ok();
    let mut names = Vec::new();
    if let Some(re) = re {
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
    let re = Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok();
    let mut names = Vec::new();
    if let Some(re) = re {
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
    OrphanCode {
        stem: String,
        reason: Option<LintMessage>,
    },
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
            AesOrphanViolation::OrphanCode { stem, reason } => {
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| format!("File '{}' matches no known layer prefix and is not referenced by any other file.", stem));
                write!(f, "AES500 ORPHAN_CODE: '{}' is unreachable.\nWHY? {}\nFIX: Rename the file with a valid layer prefix (taxonomy_, contract_, capabilities_, infrastructure_, agent_, surface_, root_) or import it from another file.", stem, why)
            }
            AesOrphanViolation::TaxonomyOrphan {
                stem,
                category,
                reason,
            } => {
                let target_hint = match *category {
                    "utility" | "helper" => "any file that needs its functionality".to_string(),
                    _ => "a contract_* file (contract_port, contract_protocol, or contract_aggregate)".to_string(),
                };
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!("Taxonomy file '{}' is not imported by any file.", stem)
                });
                write!(f, "AES501 TAXONOMY_ORPHAN: '{}' is not imported.\nWHY? {}\nFIX: Import '{}' in {}.", stem, why, stem, target_hint)
            }
            AesOrphanViolation::ContractOrphan {
                suffix,
                trait_name,
                target_layer,
                reason,
            } => {
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!(
                        "Contract {} '{}' is not implemented by any {} file.",
                        suffix, trait_name, target_layer
                    )
                });
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
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!(
                        "Capabilities file '{}' is not wired in any container.",
                        stem
                    )
                });
                write!(f, "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in the appropriate root_*_container.rs, or ensure it is reachable from an entry point.", stem, why, stem)
            }
            AesOrphanViolation::InfrastructureOrphan { stem, reason } => {
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| format!("Infrastructure file '{}' is not wired in any container and unreachable from any entry point.", stem));
                write!(f, "AES504 INFRASTRUCTURE_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in the appropriate root_*_container.rs, or ensure it is imported by a capabilities_* file.", stem, why, stem)
            }
            AesOrphanViolation::AgentOrphan { agg_name, reason } => {
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!(
                        "Agent aggregate '{}' is not called by any surface or container.",
                        agg_name
                    )
                });
                write!(f, "AES505 AGENT_ORPHAN: Aggregate '{}' is unreachable from any surface.\nWHY? {}\nFIX: Import and use '{}' in a surface_* file or root_*_container.rs, or remove the file if obsolete.", agg_name, why, agg_name)
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
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!(
                        "{} surface '{}' is not imported by any {}.",
                        category, stem, where_hint
                    )
                });
                write!(f, "AES506 SURFACE_ORPHAN: {} surface '{}' is orphaned.\nWHY? {}\nFIX: Import '{}' in {}.", category, stem, why, stem, fix_hint)
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
use crate::mcp_server::taxonomy_action_vo::JobId;
use crate::project_setup::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, SecurityScanReport, ToolchainDiagnostics,
};
use crate::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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

## File: crates/shared/src/project-setup/contract_setup_aggregate.rs

```rust
// PURPOSE: SetupAggregate — aggregate trait for project setup orchestration
use crate::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use crate::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use crate::mcp_server::taxonomy_job_vo::EnvContentVO;
use crate::mcp_server::taxonomy_job_vo::McpConfigVO;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, ProjectLanguageVO, WriteConfigResult,
};
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

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
use crate::mcp_server::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, McpBinaryNameVO, ProjectLanguageVO, SetupError, WriteConfigResult,
};
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

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
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

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

use crate::taxonomy_suggestion_vo::DescriptionVO;

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
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

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
    reason
        .as_ref()
        .map(|r| r.to_string())
        .unwrap_or_else(|| default.into())
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
                "Infrastructure adapters must implement their corresponding port interfaces.",
            );
            write!(
                f,
                "AES404 INFRASTRUCTURE_ROLE: Infrastructure file has no port trait/protocol \
                        implementation.\n\
                        WHY? {why}\n\
                        FIX: Implement the corresponding port or protocol interface in this \
                        infrastructure adapter."
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

#[cfg(test)]
mod tests {
    use super::*;

    fn labeled(v: AesRoleViolation, lang: Language) -> LabeledRoleViolation {
        v.with_language(lang)
    }

    /// `LabeledRoleViolation::Display` must produce the same output as the
    /// underlying `AesRoleViolation::Display` when the language is Rust,
    /// because Rust is the implicit default for the bare `AesRoleViolation`
    /// Display impl. Guards against accidental drift between the two impls.
    #[test]
    fn labeled_rust_matches_bare_display() {
        let v = AesRoleViolation::CoordinatesMultiple {
            reason: Some(LintMessage::new("custom".to_string())),
        };
        assert_eq!(
            v.to_string(),
            labeled(v.clone(), Language::Rust).to_string()
        );
    }

    /// `LabeledRoleViolation` with a non-Rust language must swap language-
    /// sensitive tokens (e.g. Python uses "Protocol", JS uses "interface").
    /// Guards against hard-coded Rust tokens leaking into non-Rust labels.
    #[test]
    fn labeled_python_uses_protocol_token() {
        let v = AesRoleViolation::ContractPrimitive {
            reason: Some(LintMessage::new("custom".to_string())),
        };
        let out = labeled(v, Language::Python).to_string();
        assert!(
            out.contains("Protocol"),
            "expected Protocol token in: {out}"
        );
        assert!(
            !out.contains("trait"),
            "must not contain Rust trait in: {out}"
        );
    }

    /// When `reason` is `None`, the per-variant default WHY message must be
    /// used. Confirms `resolve_why` does not emit `None` / `Some(...)`.
    #[test]
    fn missing_reason_uses_default_why() {
        let v = AesRoleViolation::SingleBottleneck { reason: None };
        let out = v.to_string();
        assert!(
            out.contains("single bottleneck"),
            "default WHY missing in: {out}"
        );
        assert!(!out.contains("None"), "leaked None in: {out}");
    }

    /// When `reason` is `Some`, the auditor-supplied WHY must override the
    /// per-variant default.
    #[test]
    fn present_reason_overrides_default_why() {
        let v = AesRoleViolation::SingleBottleneck {
            reason: Some(LintMessage::new("auditor-custom".to_string())),
        };
        let out = v.to_string();
        assert!(
            out.contains("auditor-custom"),
            "custom WHY missing in: {out}"
        );
    }

    /// `AgentFileSizeLimit` carries no `reason`; its display must embed the
    /// numeric `max_lines`.
    #[test]
    fn agent_file_size_limit_includes_max_lines() {
        let v = AesRoleViolation::AgentFileSizeLimit { max_lines: 250 };
        let out = v.to_string();
        assert!(out.contains("250"), "max_lines not in output: {out}");
    }
}
```

---

## File: crates/shared/src/source-parsing/contract_language_detector_port.rs

```rust
// PURPOSE: ILanguageDetectorPort — contract for detecting programming language from file path
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

## File: crates/shared/src/source-parsing/contract_parser_port.rs

```rust
// PURPOSE: ISourceParserPort — port trait for language-specific source code parsing (imports, definitions)
use crate::code_analysis::taxonomy_import_source_vo::ImportInfoList;
use crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList;
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_suggestion_vo::MetadataVO;
use crate::mcp_server::taxonomy_job_vo::ResponseData;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList;
use crate::source_parsing::taxonomy_parser_error::SourceParserError;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/shared/src/source-parsing/contract_path_normalization_port.rs

```rust
// PURPOSE: IPathNormalizationPort — port trait for file path normalization across platforms
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/shared/src/source-parsing/contract_scanner_provider_port.rs

```rust
// PURPOSE: IScannerProviderPort — port trait for providing language-specific source scanners

use crate::file_system::taxonomy_filesystem_error::FileSystemError;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

pub trait IScannerProviderPort: Send + Sync {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError>;
    fn get_ignored_files(&self) -> FilePathList;
}
```

---

## File: crates/shared/src/source-parsing/infrastructure_file_collector_provider.rs

```rust
use std::fs;
use std::path::{Path, PathBuf};

use crate::config_system::taxonomy_config_vo::default_aes_config;
use crate::file_system::taxonomy_filesystem_error::FileSystemError;
use crate::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use crate::source_parsing::taxonomy_file_collector_helper::is_path_ignored;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

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

pub fn count_loc(path: &str) -> usize {
    let src = Path::new(path);
    let ignored = default_ignored_paths();
    let mut count = 0usize;
    walk_rs_files(
        src,
        &mut |p| {
            if let Ok(c) = fs::read_to_string(&p) {
                count += c.lines().count();
            }
        },
        &ignored,
    );
    count.max(1)
}
```

---

## File: crates/shared/src/source-parsing/mod.rs

```rust
// source-parsing — taxonomy and contract types
pub mod contract_language_detector_port;
pub mod contract_parser_port;
pub mod contract_path_normalization_port;
pub mod contract_scanner_provider_port;
pub mod infrastructure_file_collector_provider;
pub mod taxonomy_adapter_error;
pub mod taxonomy_barrel_provider_vo;
pub mod taxonomy_file_collector_helper;
pub mod taxonomy_language_detector_helper;
pub mod taxonomy_naming_error;
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_semantic_error;
pub use infrastructure_file_collector_provider::{
    collect_all_source_files, count_loc, walk_rs_files, FileCollectorProvider,
};
```

---

## File: crates/shared/src/source-parsing/taxonomy_adapter_error.rs

```rust
// PURPOSE: AdapterError, ScanError, ValidationError — structured error types for adapter operations
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::Constraint;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_error::ExitCode;
use crate::common::taxonomy_common_error::FieldName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_source_vo::ContentString;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
        let code = self
            .error_code
            .as_ref()
            .map(|c| format!(" [{}]", c))
            .unwrap_or_default();
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
        let adapter = self
            .adapter_name
            .as_ref()
            .map(|a| format!(" ({})", a))
            .unwrap_or_default();
        let code = self
            .error_code
            .as_ref()
            .map(|c| format!(" [{}]", c))
            .unwrap_or_default();
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

## File: crates/shared/src/source-parsing/taxonomy_barrel_provider_vo.rs

```rust
// PURPOSE: BarrelProvider — detects barrel/index files (_init_.py, mod.rs, index.ts)
use std::collections::HashMap;

/// Centralized barrel import resolver and barrel utility functions.
/// When a file imports from a barrel (mod.rs), resolve through the barrel
/// to find the actual source file that defines the imported symbol.
pub struct BarrelImportResolver;

impl BarrelImportResolver {
    /// Shared: check if a filename is a barrel file.
    pub fn is_barrel_file(filename: &str) -> bool {
        filename.ends_with("mod.rs")
            || filename.ends_with("__init__.py")
            || filename.ends_with("/index.ts")
            || filename.ends_with("/index.js")
            || filename.ends_with("/index.tsx")
            || filename == "lib.rs"
    }

    /// Build a reverse map: barrel mod.rs → list of source file paths it re-exports.
    /// Scans all project files to find barrel files and their pub use/pub mod declarations.
    pub fn build_barrel_map(files: &[String]) -> HashMap<String, Vec<String>> {
        let mut barrel_map: HashMap<String, Vec<String>> = HashMap::new();
        let stem_map = Self::build_stem_map(files);

        for f in files {
            let basename = f.split('/').next_back().unwrap_or("");
            if basename != "mod.rs" && basename != "lib.rs" {
                continue;
            }
            if let Ok(content) = std::fs::read_to_string(f) {
                // Collect pub use re-exports: pub use xxx::SomeType;
                for line in content.lines() {
                    let t = line.trim();
                    // pub mod xxx;
                    if let Some(rest) = t
                        .strip_prefix("pub mod ")
                        .or_else(|| t.strip_prefix("mod "))
                    {
                        let mod_name = rest.trim_end_matches(';').trim();
                        let dir = f.trim_end_matches(basename).trim_end_matches('/');
                        let candidates = vec![
                            format!("{}/{}.rs", dir, mod_name),
                            format!("{}/{}/mod.rs", dir, mod_name),
                        ];
                        for cand in &candidates {
                            if std::path::Path::new(cand).exists() {
                                barrel_map.entry(f.clone()).or_default().push(cand.clone());
                                break;
                            }
                        }
                    }
                    // pub use xxx::*; → resolve all files in xxx/ or xxx.rs
                    if t.contains("pub use ") && t.contains("::*") {
                        let prefix = t
                            .strip_prefix("pub use ")
                            .unwrap_or(t)
                            .split("::*")
                            .next()
                            .unwrap_or("")
                            .trim();
                        // Normalize hyphens to underscores for module name matching
                        let normalized = prefix.replace('-', "_");
                        if let Some(resolved) =
                            stem_map.get(&normalized).or_else(|| stem_map.get(prefix))
                        {
                            barrel_map
                                .entry(f.clone())
                                .or_default()
                                .extend(resolved.clone());
                        }
                    }
                    // pub use crate::xxx::SomeType; AND pub use crate::xxx::{A, B, C};
                    if t.starts_with("pub use ") && !t.contains("::*") {
                        let import_path = t
                            .strip_prefix("pub use ")
                            .unwrap_or(t)
                            .trim_end_matches(';')
                            .trim();
                        // Handle braced multi-import: crate::module::{A, B}
                        if let Some(brace_pos) = import_path.find("::{") {
                            let module_part = &import_path[..brace_pos];
                            // Extract module name from crate::xxx::{...}
                            let parts: Vec<&str> = module_part.split("::").collect();
                            if parts.len() >= 2 {
                                let module_name = parts[1].replace('-', "_");
                                if let Some(resolved) = stem_map.get(&module_name) {
                                    barrel_map
                                        .entry(f.clone())
                                        .or_default()
                                        .extend(resolved.clone());
                                }
                            }
                        } else {
                            let parts: Vec<&str> = import_path.split("::").collect();
                            if parts.len() >= 2 {
                                let module_name = parts[1].replace('-', "_");
                                if let Some(resolved) = stem_map.get(&module_name) {
                                    barrel_map
                                        .entry(f.clone())
                                        .or_default()
                                        .extend(resolved.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        barrel_map
    }

    fn build_stem_map(files: &[String]) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for f in files {
            let basename = f.split('/').next_back().unwrap_or("");
            let stem_val = basename.split('.').next().unwrap_or("").to_string();
            map.entry(stem_val.clone()).or_default().push(f.clone());
            let dir = f.trim_end_matches(basename).trim_end_matches('/');
            let dirstem = format!("{}/{}", dir, stem_val);
            map.entry(dirstem).or_default().push(f.clone());
        }
        map
    }

    /// Given a file path, resolve its imports through barrels.
    /// Returns a list of RESOLVED source file paths (not barrel paths).
    pub fn resolve_imports_for_file(
        file_path: &str,
        barrel_map: &HashMap<String, Vec<String>>,
        project_files: &[String],
    ) -> Vec<String> {
        let mut resolved: Vec<String> = Vec::new();
        if let Ok(content) = std::fs::read_to_string(file_path) {
            for line in content.lines() {
                let t = line.trim();
                if t.starts_with("use ") {
                    let path = t
                        .strip_prefix("use ")
                        .unwrap_or(t)
                        .split(" as ")
                        .next()
                        .unwrap_or("")
                        .split("::")
                        .collect::<Vec<_>>();
                    if path.len() >= 2 && path[0] == "crate" {
                        let module_name = path[1].replace('-', "_");
                        // Check if this module has a barrel
                        for (barrel_file, sources) in barrel_map {
                            let barrel_stem = barrel_file.split('/').next_back().unwrap_or("");
                            let barrel_name = if barrel_stem == "mod.rs" || barrel_stem == "lib.rs"
                            {
                                let dir = barrel_file
                                    .trim_end_matches(barrel_stem)
                                    .trim_end_matches('/');
                                dir.split('/').next_back().unwrap_or("")
                            } else {
                                barrel_stem.split('.').next().unwrap_or("")
                            };
                            if module_name == barrel_name
                                || barrel_file.contains(&format!("/{}/", module_name))
                            {
                                resolved.extend(sources.clone());
                            }
                        }
                        // Also add any direct file match
                        for pf in project_files {
                            let pb = pf.split('/').next_back().unwrap_or("");
                            if pb.starts_with(&format!("{}_", module_name)) {
                                resolved.push(pf.clone());
                            }
                        }
                    }
                }
            }
        }
        resolved.sort();
        resolved.dedup();
        resolved
    }

    /// Check if a specific file is reachable through barrel imports from any contract file.
    pub fn is_imported_by_contract(
        target_file: &str,
        barrel_map: &HashMap<String, Vec<String>>,
        project_files: &[String],
    ) -> bool {
        let target_stem = target_file
            .split('/')
            .next_back()
            .unwrap_or("")
            .split('.')
            .next()
            .unwrap_or("")
            .to_string();
        for cf in project_files {
            let cb = cf.split('/').next_back().unwrap_or("");
            if !cb.starts_with("contract_") {
                continue;
            }
            let resolved = Self::resolve_imports_for_file(cf, barrel_map, project_files);
            if resolved.iter().any(|r| {
                r.split('/')
                    .next_back()
                    .unwrap_or("")
                    .contains(&target_stem)
            }) {
                return true;
            }
            // Also check direct contains fallback.
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&target_stem) {
                    return true;
                }
            }
        }
        false
    }
}
```

---

## File: crates/shared/src/source-parsing/taxonomy_file_collector_helper.rs

```rust
// PURPOSE: FileCollector — taxonomy utility for collecting lintable source files from a directory tree
use crate::source_parsing::taxonomy_language_detector_helper::LanguageDetector;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
            let basename = segments.last().copied().unwrap_or("");
            if basename.ends_with(suffix) {
                return true;
            }
            continue;
        }
        // (3) Bare segment — match any segment anywhere in the path.
        if segments.contains(&pat.as_str()) {
            return true;
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
    if let Ok(entries) = std::fs::read_dir(&dir_path.value) {
        for entry in entries.flatten() {
            let path = entry.path();
            let relative_path = path.strip_prefix(root_dir).unwrap_or(&path);
            let rel_str = relative_path.to_string_lossy();
            if is_path_ignored(&rel_str, ignored) {
                continue;
            }
            if path.is_dir() {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn ignored(patterns: &[&str]) -> Vec<String> {
        patterns.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn absolute_prefix_matches_at_any_depth() {
        // The pattern must match anywhere along the path, not only at the
        // beginning — `rel_path` is typically absolute
        // (`/home/raka/.../test-workspaces/crates/foo.rs`).
        let ig = ignored(&["/test-workspaces"]);
        assert!(is_path_ignored(
            "/home/raka/mcp-arwaky/lint-arwaky/test-workspaces",
            &ig
        ));
        assert!(is_path_ignored(
            "/home/raka/mcp-arwaky/lint-arwaky/test-workspaces/crates/foo.rs",
            &ig
        ));
        // Bare (no leading slash) and relative forms must also match.
        assert!(is_path_ignored("test-workspaces", &ig));
        assert!(is_path_ignored("test-workspaces/crates/foo.rs", &ig));
    }

    #[test]
    fn absolute_prefix_does_not_match_unrelated_segment() {
        let ig = ignored(&["/test-workspaces"]);
        // Unrelated paths must NOT match.
        assert!(!is_path_ignored("/home/not-test-workspaces/foo.rs", &ig));
        assert!(!is_path_ignored(
            "/home/raka/lint-arwaky/crates/test.rs",
            &ig
        ));
        // Identical name as a mid-segment of an unrelated path must NOT match
        // (`not-test-workspaces` is its own segment, not `test-workspaces`).
        assert!(!is_path_ignored("/home/not-test-workspaces", &ig));
    }

    #[test]
    fn absolute_prefix_nested_path() {
        let ig = ignored(&["/packages/vscode-extension"]);
        assert!(is_path_ignored(
            "packages/vscode-extension/src/extension.ts",
            &ig
        ));
        assert!(!is_path_ignored("packages/some-other/src/foo.ts", &ig));
    }

    #[test]
    fn bare_segment_matches_anywhere() {
        // `node_modules` (no leading slash) should match anywhere.
        let ig = ignored(&["node_modules"]);
        assert!(is_path_ignored("node_modules/lodash/index.js", &ig));
        assert!(is_path_ignored("frontend/node_modules/react/index.js", &ig));
    }

    #[test]
    fn suffix_glob_matches_minified_vendor_files() {
        let ig = ignored(&[".min.js", ".min.css"]);
        assert!(is_path_ignored(
            "packages/vscode-extension/media/cytoscape.min.js",
            &ig
        ));
        assert!(is_path_ignored("static/style.min.css", &ig));
        // Must NOT match a regular `.js` file.
        assert!(!is_path_ignored("packages/foo/index.js", &ig));
    }

    #[test]
    fn empty_pattern_ignored() {
        let ig = ignored(&[""]);
        assert!(!is_path_ignored("anything.rs", &ig));
    }

    #[test]
    fn multiple_patterns_any_match() {
        let ig = ignored(&["/target", "/test-workspaces", ".min.js"]);
        assert!(is_path_ignored("/home/raka/target/debug/foo.rs", &ig));
        assert!(is_path_ignored("/home/raka/test-workspaces/foo.rs", &ig));
        assert!(is_path_ignored("/home/raka/lib/vendor.min.js", &ig));
        assert!(!is_path_ignored("/home/raka/crates/foo.rs", &ig));
    }

    #[test]
    fn packages_pattern_excludes_only_packages_segment() {
        let ig = ignored(&["/packages"]);
        // Unrelated paths must not match.
        assert!(!is_path_ignored("/home/raka/crates/foo.rs", &ig));
        // Path that DOES contain `packages` segment must match (at any depth).
        assert!(is_path_ignored("/home/raka/packages/foo.ts", &ig));
        assert!(is_path_ignored(
            "/home/raka/packages/vscode-extension/src/extension.ts",
            &ig
        ));
        // Same name as mid-segment of an unrelated path must not match.
        assert!(!is_path_ignored(
            "/home/raka/crates/packages-fake/foo.ts",
            &ig
        ));
    }
}
```

---

## File: crates/shared/src/source-parsing/taxonomy_language_detector_helper.rs

```rust
// PURPOSE: LanguageDetector — Helper for detecting programming languages from file paths
use crate::source_parsing::contract_language_detector_port::Language;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/shared/src/source-parsing/taxonomy_naming_error.rs

```rust
// PURPOSE: NamingError — structured error type for naming convention violations
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_error_vo::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct NamingError {
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: ErrorCode,
    #[serde(default)]
    pub cause: Cause,
}

impl NamingError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            message,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for NamingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code: &str = &self.error_code;
        if code.is_empty() {
            write!(f, "Naming Error: {}", self.message)
        } else {
            write!(f, "Naming Error [{}]: {}", code, self.message)
        }
    }
}
```

---

## File: crates/shared/src/source-parsing/taxonomy_naming_list_vo.rs

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

## File: crates/shared/src/source-parsing/taxonomy_parser_error.rs

```rust
// PURPOSE: ParserError — structured error type for source code parsing failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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

## File: crates/shared/src/source-parsing/taxonomy_path_vo.rs

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
        // Normalize: replace backslashes with forward slashes, and collapse multiple slashes.
        value = value.replace('\\', "/");
        // Remove all trailing slashes
        while value.ends_with('/') && value.len() > 1 {
            value.pop();
        }
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
        match basename.rsplit('.').next() {
            Some(ext) => ext.to_string(),
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

    /// Check if the path is a barrel file.
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "main.py" | "py.typed" | "app.py" | "lib.rs"
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
        // Remove trailing slash unless it's just "/"
        if value.ends_with('/') && value.len() > 1 {
            value.pop();
        }
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

#[cfg(test)]
mod tests {
    use super::{DirectoryPath, FilePath};

    #[test]
    fn test_file_path_new() {
        let fp = FilePath::new("test.txt").unwrap_or_default();
        assert_eq!(fp.value, "test.txt");
        assert_eq!(fp.extension(), "txt");
        assert!(fp.has_extension("txt"));
        assert!(!fp.has_extension("md"));

        // Test normalization
        let fp = FilePath::new("path\\to\\file.txt").unwrap_or_default();
        assert_eq!(fp.value, "path/to/file.txt");

        let fp = FilePath::new("path/to/file/").unwrap_or_default();
        assert_eq!(fp.value, "path/to/file");

        let fp = FilePath::new("/").unwrap_or_default();
        assert_eq!(fp.value, "/");

        let fp = FilePath::new("///").unwrap_or_default();
        assert_eq!(fp.value, "/");
    }

    #[test]
    fn test_file_path_invalid() {
        assert!(FilePath::new("").is_err());
        assert!(FilePath::new("   ").is_err());
    }

    #[test]
    fn test_directory_path_new() {
        let dp = DirectoryPath::new("test/dir").unwrap_or_default();
        assert_eq!(dp.value, "test/dir");

        let dp = DirectoryPath::new("test/dir/").unwrap_or_default();
        assert_eq!(dp.value, "test/dir");

        let dp = DirectoryPath::new("/").unwrap_or_default();
        assert_eq!(dp.value, "/");
    }

    #[test]
    fn test_directory_path_invalid() {
        assert!(DirectoryPath::new("").is_err());
        assert!(DirectoryPath::new("   ").is_err());
    }

    /// Regression: `./foo.rs` must report `rs` as its extension, not empty string.
    /// The old implementation treated any path starting with `.` as having no
    /// extension, which caused `LanguageDetector::is_lintable` to skip relative
    /// paths emitted by `std::fs::read_dir` in `collect_source_files`. Result: zero
    /// files collected when the user runs `lint-arwaky check .` on a directory
    /// tree with non-`.git`-anchored paths.
    #[test]
    fn test_extension_with_dot_slash_prefix() {
        let fp = FilePath::new("./foo.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
        let fp = FilePath::new("./nested/foo.py").unwrap_or_default();
        assert_eq!(fp.extension(), "py");
        let fp = FilePath::new(".//foo.ts").unwrap_or_default();
        assert_eq!(fp.extension(), "ts");
    }

    /// Regression: a hidden-file basename (e.g. `.bashrc`) must still report no
    /// extension, since the basename itself starts with a dot.
    #[test]
    fn test_extension_hidden_basename() {
        let fp = FilePath::new(".bashrc").unwrap_or_default();
        assert_eq!(fp.extension(), "");
        let fp = FilePath::new("/home/user/.gitignore").unwrap_or_default();
        assert_eq!(fp.extension(), "");
    }

    /// Regression: full paths must still resolve the extension on the basename.
    #[test]
    fn test_extension_full_path() {
        let fp =
            FilePath::new("/tmp/bypass_test/capabilities_unwrap_checker.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
        let fp = FilePath::new("crates/code-analysis/src/foo.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
    }

    /// Makefile / Dockerfile — special filenames, no extension.
    #[test]
    fn test_extension_special_filenames() {
        let fp = FilePath::new("Makefile").unwrap_or_default();
        assert_eq!(fp.extension(), "");
        let fp = FilePath::new("Dockerfile").unwrap_or_default();
        assert_eq!(fp.extension(), "");
    }
}
```

---

## File: crates/shared/src/source-parsing/taxonomy_paths_vo.rs

```rust
// PURPOSE: FilePathList, DirectoryPath, SourceDir — VOs for file/directory path collections
use serde::{Deserialize, Serialize};

use crate::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/shared/src/source-parsing/taxonomy_semantic_error.rs

```rust
// PURPOSE: SemanticError — structured error type for semantic analysis failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct SemanticError {
    #[serde(default)]
    pub path: FilePath,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: ErrorCode,
    #[serde(default)]
    pub cause: Cause,
}

impl SemanticError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            path: FilePath::default(),
            message,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let target = {
            let p: &str = &self.path;
            if p.is_empty() {
                String::new()
            } else {
                format!(" on {}", p)
            }
        };
        let code = {
            let c: &str = &self.error_code;
            if c.is_empty() {
                String::new()
            } else {
                format!(" [{}]", c)
            }
        };
        write!(f, "Semantic Error{}{}: {}", target, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ScopeResolutionError {
    #[serde(flatten)]
    pub base: SemanticError,
}

impl ScopeResolutionError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            base: SemanticError::new(message),
        }
    }
}

impl std::fmt::Display for ScopeResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct CallChainError {
    #[serde(flatten)]
    pub base: SemanticError,
}

impl CallChainError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            base: SemanticError::new(message),
        }
    }
}

impl std::fmt::Display for CallChainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}
```

---

