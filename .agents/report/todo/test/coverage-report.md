# Test Coverage Report — Crates vs Planned Tests

Generated: 2026-07-22
Scope: All 17 crates in `crates/` + `shared/`

---

## Summary

| # | Crate | Plan Exists? | Planned | Actual | Status | Notes |
|---|-------|-------------|---------|--------|--------|-------|
| 1 | auto-fix | ✅ | 13 | 14 | ✅ Over-delivered | +1: `integration_fix_orchestrator.rs` |
| 2 | cli-commands | ✅ | 18 | 20 | ✅ Over-delivered | +2: `integration_cli_commands.rs`, `unit_cli_commands_config_redaction.rs` |
| 3 | code-analysis | ✅ | 17 | 18 | ✅ Over-delivered | +1: `integration_code_analysis_container.rs` |
| 4 | config-system | ✅ | 15 | 16 | ✅ Over-delivered | +1: `unit_config_system_orchestrator.rs` |
| 5 | external-lint | ✅ | 13 | 14 | ✅ Over-delivered | +1: `unit_external_lint_adapters.rs` (re-verified) |
| 6 | file-watch | ✅ | 13 | 13 | ✅ Match | Perfect |
| 7 | git-hooks | ✅ | 13 | 14 | ✅ Over-delivered | +1: `unit_git_hooks_orchestrator.rs` |
| 8 | import-rules | ✅ | 15 | 16 | ✅ Over-delivered | +1: `unit_import_rules_orchestrator.rs` |
| 9 | maintenance | ✅ | 10 | 10 | ✅ Match | Perfect |
| 10 | mcp-server | ✅ | 13 | 13 | ✅ Match | Perfect |
| 11 | naming-rules | ✅ | 8 | 14 | ✅ Over-delivered | +6: unit, integration, bench files expanded |
| 12 | orphan-detector | ✅ | 19 | 20 | ✅ Over-delivered | +1: `unit_orphan_detector_agent_analyzer.rs` |
| 13 | role-rules | ❌ | — | 1 | ⚠️ No plan, minimal coverage | Only `unit_utility_role_macro.rs` |
| 14 | shared | ❌ | — | 9 | ℹ️ Shared utilities | Utility tests only |
| 15 | project-setup | ❌ | — | 0 | ❌ No tests | No `tests/` directory |
| 16 | report-formatter | ❌ | — | 0 | ❌ No tests | No `tests/` directory |
| 17 | tui | ❌ | — | 0 | ❌ No tests | No `tests/` directory |

---

## Detailed Per-Crate Analysis

### ✅ Complete (Plan matches or exceeds)

#### 1. auto-fix (14 files)
**Planned:** contract, 3 unit, integration, smoke, e2e, 4 acceptance, bench = 13
**Actual:** contract, 3 unit, integration + `integration_fix_orchestrator.rs`, smoke, e2e, 4 acceptance, bench = 14

| File | Planned? | Status |
|------|----------|--------|
| contract_auto_fix.rs | ✅ | Present |
| unit_auto_fix_file_adapter.rs | ✅ | Present |
| unit_auto_fix_fix_processor.rs | ✅ | Present |
| unit_auto_fix_orchestrator.rs | ✅ | Present |
| integration_auto_fix.rs | ✅ | Present |
| integration_fix_orchestrator.rs | ❌ | **Extra** |
| smoke_auto_fix.rs | ✅ | Present |
| e2e_auto_fix_flow.rs | ✅ | Present |
| acceptance_frd_unused_import.rs | ✅ | Present |
| acceptance_frd_bypass_warning.rs | ✅ | Present |
| acceptance_frd_file_naming.rs | ✅ | Present |
| acceptance_frd_idempotency.rs | ✅ | Present |
| bench_auto_fix_throughput.rs | ✅ | Present |

#### 2. cli-commands (20 files)
**Planned:** contract, 7 unit, integration, smoke, 2 e2e, 8 acceptance, bench = 18
**Actual:** contract, 7 unit + `unit_cli_commands_config_redaction.rs`, integration + `integration_cli_commands.rs`, smoke, 2 e2e, 8 acceptance, bench = 20

| File | Planned? | Status |
|------|----------|--------|
| contract_cli_commands.rs | ✅ | Present |
| unit_cli_commands_format_output.rs | ✅ | Present |
| unit_cli_commands_config_redaction.rs | ❌ | **Extra** |
| unit_cli_commands_common_command.rs | ✅ | Present |
| unit_cli_commands_check_action.rs | ✅ | Present |
| unit_cli_commands_fix_command.rs | ✅ | Present |
| unit_cli_commands_setup_command.rs | ✅ | Present |
| unit_cli_commands_taxonomy.rs | ✅ | Present |
| integration_cli_commands.rs | ❌ | **Extra** |
| smoke_cli_commands.rs | ✅ | Present |
| e2e_check_scan_flow.rs | ✅ | Present |
| e2e_fix_flow.rs | ✅ | Present |
| acceptance_FRD_check.rs | ✅ | Present |
| acceptance_FRD_ci.rs | ✅ | Present |
| acceptance_FRD_config_show.rs | ✅ | Present |
| acceptance_FRD_exit_codes.rs | ✅ | Present |
| acceptance_FRD_formats.rs | ✅ | Present |
| acceptance_FRD_maintenance.rs | ✅ | Present |
| acceptance_FRD_scan.rs | ✅ | Present |
| acceptance_FRD_setup.rs | ✅ | Present |
| bench_cli_commands_formatting.rs | ✅ | Present |

#### 3. code-analysis (18 files)
**Planned:** contract, 5 unit, integration, smoke, e2e, 6 acceptance, bench = 17
**Actual:** +1 extra integration file = 18

| File | Planned? | Status |
|------|----------|--------|
| contract_code_analysis.rs | ✅ | Present |
| unit_code_analysis_bypass_checker.rs | ✅ | Present |
| unit_code_analysis_line_checker.rs | ✅ | Present |
| unit_code_analysis_mandatory_definition.rs | ✅ | Present |
| unit_code_analysis_duplication.rs | ✅ | Present |
| unit_code_analysis_orchestrator.rs | ✅ | Present |
| integration_code_analysis.rs | ✅ | Present |
| integration_code_analysis_container.rs | ❌ | **Extra** |
| smoke_code_analysis.rs | ✅ | Present |
| e2e_code_analysis_flow.rs | ✅ | Present |
| acceptance_FR_001..006.rs | ✅ | All 6 present |
| bench_code_analysis_throughput.rs | ✅ | Present |

#### 4. config-system (16 files)
**Planned:** contract, 5 unit, integration, smoke, e2e, 5 acceptance, bench = 15
**Actual:** +1 extra unit file = 16

| File | Planned? | Status |
|------|----------|--------|
| contract_config_system.rs | ✅ | Present |
| unit_config_system_rules_validator.rs | ✅ | Present |
| unit_config_system_workspace_detector.rs | ✅ | Present |
| unit_config_system_yaml_reader.rs | ✅ | Present |
| unit_config_system_parser_provider.rs | ✅ | Present |
| unit_config_system_orchestrator.rs | ❌ | **Extra** |
| integration_config_system.rs | ✅ | Present |
| smoke_config_system.rs | ✅ | Present |
| e2e_config_system_flow.rs | ✅ | Present |
| acceptance_US_1..5.rs | ✅ | All 5 present |
| bench_config_system.rs | ✅ | Present |

#### 5. file-watch (13 files) — PERFECT MATCH

All 13 planned files present, no extras, no missing:
- contract_file_watch.rs
- unit_file_watch_change_analyzer.rs
- unit_file_watch_notify_provider.rs
- unit_file_watch_watch_orchestrator.rs
- integration_file_watch.rs
- smoke_file_watch.rs
- e2e_file_watch_flow.rs
- acceptance_FRD_file_watch_001..004.rs (4 files)
- bench_file_watch_change_analyzer.rs

#### 6. git-hooks (14 files)
**Planned:** contract, 5 unit, integration, smoke, e2e, 4 acceptance, bench = 13
**Actual:** +1 extra orchestrator unit = 14

| File | Planned? | Status |
|------|----------|--------|
| contract_git_hooks.rs | ✅ | Present |
| unit_git_hooks_diff_checker.rs | ✅ | Present |
| unit_git_hooks_git_command_adapter.rs | ✅ | Present |
| unit_git_hooks_hook_adapter.rs | ✅ | Present |
| unit_git_hooks_hook_manager.rs | ✅ | Present |
| unit_git_hooks_orchestrator.rs | ❌ | **Extra** |
| integration_git_hooks.rs | ✅ | Present |
| smoke_git_hooks.rs | ✅ | Present |
| e2e_git_hooks_pre_commit_flow.rs | ✅ | Present |
| acceptance_FRD_001..004.rs | ✅ | All 4 present |
| bench_git_hooks_diff.rs | ✅ | Present |

#### 7. import-rules (16 files)
**Planned:** contract, 5 unit, integration, smoke, e2e, 5 acceptance, bench = 15
**Actual:** +1 extra orchestrator unit = 16

| File | Planned? | Status |
|------|----------|--------|
| contract_import_rules.rs | ✅ | Present |
| unit_import_rules_unused_checker.rs | ✅ | Present |
| unit_import_rules_dummy_checker.rs | ✅ | Present |
| unit_import_rules_forbidden_checker.rs | ✅ | Present |
| unit_import_rules_mandatory_checker.rs | ✅ | Present |
| unit_import_rules_cycle_analyzer.rs | ✅ | Present |
| unit_import_rules_orchestrator.rs | ❌ | **Extra** |
| integration_import_rules.rs | ✅ | Present |
| smoke_import_rules.rs | ✅ | Present |
| e2e_import_rules_audit_flow.rs | ✅ | Present |
| acceptance_FR_001..005.rs | ✅ | All 5 present |
| bench_import_rules_throughput.rs | ✅ | Present |

#### 8. maintenance (10 files) — PERFECT MATCH

All 10 planned files present, no extras, no missing:
- contract_maintenance.rs
- unit_maintenance_checker.rs
- unit_maintenance_tool_executor.rs
- unit_maintenance_orchestrator.rs
- integration_maintenance.rs
- smoke_maintenance.rs
- e2e_maintenance_flow.rs
- acceptance_FRD_dep_update.rs
- acceptance_FRD_audit.rs
- bench_maintenance_throughput.rs

#### 9. mcp-server (13 files) — PERFECT MATCH

All 13 planned files present, no extras, no missing:
- contract_mcp_server.rs
- unit_mcp_server_orchestrator.rs
- unit_mcp_server_surface.rs
- unit_mcp_server_container.rs
- integration_mcp_server.rs
- smoke_mcp_server.rs
- e2e_mcp_server_flow.rs
- acceptance_FRD_mcp_001..004.rs (4 files)
- bench_mcp_server.rs

#### 10. naming-rules (14 files)
**Planned:** contract, 2 unit, integration, 2 acceptance, bench = 8
**Actual:** +6 extras (extra unit files + integration_naming_rules_container.rs + more acceptance) = 14

| File | Planned? | Status |
|------|----------|--------|
| contract_naming_rules.rs | ✅ | Present |
| unit_naming_rules_convention_checker.rs | ✅ | Present |
| unit_naming_rules_suffix_prefix_checker.rs | ✅ | Present |
| unit_naming_rules_orchestrator.rs | ❌ | **Extra** |
| integration_naming_rules.rs | ✅ | Present |
| integration_naming_rules_container.rs | ❌ | **Extra** |
| acceptance_FRD_001.rs | ✅ | Present |
| acceptance_FRD_002.rs | ✅ | Present |
| bench_naming_rules_throughput.rs | ✅ | Present |

#### 11. orphan-detector (20 files)
**Planned:** contract, 8 unit, integration, smoke, e2e, 6 acceptance, bench = 19
**Actual:** +1 extra agent_analyzer unit = 20

| File | Planned? | Status |
|------|----------|--------|
| contract_orphan_detector.rs | ✅ | Present |
| unit_orphan_detector_graph_resolver.rs | ✅ | Present |
| unit_orphan_detector_taxonomy_analyzer.rs | ✅ | Present |
| unit_orphan_detector_contract_analyzer.rs | ✅ | Present |
| unit_orphan_detector_capabilities_analyzer.rs | ✅ | Present |
| unit_orphan_detector_utility_analyzer.rs | ✅ | Present |
| unit_orphan_detector_agent_analyzer.rs | ❌ | **Extra** |
| unit_orphan_detector_surfaces_analyzer.rs | ✅ | Present |
| unit_orphan_detector_orchestrator.rs | ✅ | Present |
| integration_orphan_detector.rs | ✅ | Present |
| smoke_orphan_detector.rs | ✅ | Present |
| e2e_orphan_detection_flow.rs | ✅ | Present |
| acceptance_AES501..006.rs | ✅ | All 6 present |
| bench_orphan_detector_graph.rs | ✅ | Present |

---

### ⚠️ Incomplete / No Plan

#### 12. external-lint (14 files) — PERFECT MATCH (re-verified)
**Planned:** contract, 6 unit, integration, smoke, e2e, 3 acceptance, bench = 13
**Actual:** 14 files — all planned present + 1 extra utility adapter test

| File | Planned? | Status |
|------|----------|--------|
| contract_external_lint.rs | ✅ | Present |
| unit_external_lint_selector.rs | ✅ | Present |
| unit_external_lint_language_detector.rs | ✅ | Present |
| unit_external_lint_executor.rs | ✅ | Present |
| unit_external_lint_stdio_client.rs | ✅ | Present |
| unit_external_lint_adapters.rs | ✅ | Present (was missed by initial find) |
| unit_external_lint_utility_adapter.rs | ✅ | Present |
| integration_external_lint.rs | ✅ | Present |
| smoke_external_lint.rs | ✅ | Present |
| e2e_external_lint_scan_flow.rs | ✅ | Present |
| acceptance_FRD_tool_discovery.rs | ✅ | Present |
| acceptance_FRD_report_unification.rs | ✅ | Present |
| acceptance_FRD_severity_mapping.rs | ✅ | Present |
| bench_external_lint_selector.rs | ✅ | Present |

**Note:** Initial `find | grep` missed `unit_external_lint_adapters.rs` due to grep pattern. Re-verified with `ls` — all 14 files present.

#### 13. role-rules (1 file) — NO PLAN
No todo file was created for this crate. Only 1 unit test exists.

| File | Status |
|------|--------|
| unit_utility_role_macro.rs | Only test present |

**Missing:** contract, additional unit, integration, smoke, acceptance, bench

#### 14. shared (9 files) — NO PLAN
Shared utility tests exist but no formal plan:

| File | Status |
|------|--------|
| utility_path_normalizer_tests.rs | Present |
| utility_import_symbol_extractor_tests.rs | Present |
| utility_dummy_detector_tests.rs | Present |
| utility_config_merger_tests.rs | Present |
| utility_column_tests.rs | Present |
| utility_bypass_tests.rs | Present |
| utility_file_tests.rs | Present |
| utility_naming_tests.rs | Present |
| utility_orphan_tests.rs | Present |

#### 15. project-setup (0 files) — NO TESTS
No `tests/` directory at all.

#### 16. report-formatter (0 files) — NO TESTS
No `tests/` directory at all.

#### 17. tui (0 files) — NO TESTS
No `tests/` directory at all.

---

## Overall Statistics

| Metric | Value |
|--------|-------|
| Total crates | 17 (+ shared) |
| Crates with formal test plans | 12 |
| Crates matching plan exactly | 3 (file-watch, maintenance, mcp-server) |
| Crates over-delivered (more tests than planned) | 9 |
| Crates under-delivered (fewer tests than planned) | 0 |
| Crates with NO formal plan | 5 (role-rules, shared, project-setup, report-formatter, tui) |
| **Total test files** | **~183** |

---

## Action Items

### High Priority
1. **role-rules**: Create formal test plan — currently only 1 unit test (`unit_utility_role_macro.rs`), needs contract + integration + smoke + acceptance + bench
2. **project-setup**: Needs full test suite (no `tests/` directory exists)
3. **report-formatter**: Needs full test suite (no `tests/` directory exists)
4. **tui**: Needs test suite (no `tests/` directory exists)

### Medium Priority
5. **shared**: Consider formalizing test plan for the 9 existing utility tests

### Low Priority — Already Complete
- All 12 planned crates have substantial test coverage (all planned files present)
- 9 of 12 planned crates exceeded their test plans (over-delivered)
- 3 planned crates match perfectly (file-watch, maintenance, mcp-server)
- 0 crates under-delivered on planned tests
