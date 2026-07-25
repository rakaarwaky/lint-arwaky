#!/usr/bin/env python3
"""
Refactor all feature crate imports to use barrel re-exports with grouped format.
Transforms:
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::common::taxonomy_severity_vo::Severity;
Into:
    use shared::common::{
        FilePath,
        Severity,
    };

And groups imports from the same module.
"""

import re
import os
import glob

# ─── Mapping: (old_deep_submodule_path, new_barrel_export_path) ──────
# These are the submodule paths that have barrel re-exports in mod.rs
SUBSTITUTIONS = {
    # ── common module ──
    "shared::common::taxonomy_path_vo::": "shared::common::",
    "shared::common::taxonomy_paths_vo::": "shared::common::",
    "shared::common::taxonomy_severity_vo::": "shared::common::",
    "shared::common::taxonomy_adapter_error::": "shared::common::",
    "shared::common::taxonomy_adapter_name_vo::": "shared::common::",
    "shared::common::taxonomy_adapter_list_vo::": "shared::common::",
    "shared::common::taxonomy_common_vo::": "shared::common::",
    "shared::common::taxonomy_display_content_vo::": "shared::common::",
    "shared::common::taxonomy_git_vo::": "shared::common::",
    "shared::common::taxonomy_message_vo::": "shared::common::",
    "shared::common::taxonomy_response_data_vo::": "shared::common::",
    "shared::common::taxonomy_source_vo::": "shared::common::",
    "shared::common::taxonomy_common_error::": "shared::common::",
    "shared::common::taxonomy_job_vo::": "shared::common::",
    "shared::common::taxonomy_duration_vo::": "shared::common::",
    "shared::common::taxonomy_suggestion_vo::": "shared::common::",
    "shared::common::taxonomy_threshold_vo::": "shared::common::",
    "shared::common::taxonomy_error_vo::": "shared::common::",
    "shared::common::taxonomy_language_vo::": "shared::common::",
    "shared::common::taxonomy_lint_vo::": "shared::common::",
    "shared::common::taxonomy_name_vo::": "shared::common::",
    "shared::common::taxonomy_language_info_vo::": "shared::common::",
    "shared::common::taxonomy_layer_vo::": "shared::common::",
    "shared::common::taxonomy_line_count_vo::": "shared::common::",
    "shared::common::taxonomy_definition_vo::": "shared::common::",
    "shared::common::taxonomy_filesystem_error::": "shared::common::",
    "shared::common::taxonomy_action_vo::": "shared::common::",
    "shared::common::contract_executor_protocol::": "shared::common::",
    "shared::common::utility_layer_detector": "shared::common::utility_layer_detector",  # keep
    "shared::common::utility_file_handler": "shared::common::utility_file_handler",  # keep
    "shared::common::utility_language_detector": "shared::common::utility_language_detector",  # keep
    "shared::common::utility_compliance_score": "shared::common::utility_compliance_score",  # keep
    "shared::common::utility_signature_parser": "shared::common::utility_signature_parser",  # keep
    "shared::common::utility_command_runner": "shared::common::utility_command_runner",  # keep
    "shared::common::utility_value_object_generator": "shared::common::utility_value_object_generator",  # keep

    # ── code-analysis module ──
    "shared::code_analysis::contract_bypass_checker_protocol::": "shared::code_analysis::",
    "shared::code_analysis::contract_class_protocol::": "shared::code_analysis::",
    "shared::code_analysis::contract_code_analysis_aggregate::": "shared::code_analysis::",
    "shared::code_analysis::contract_code_metric_analyzer_protocol::": "shared::code_analysis::",
    "shared::code_analysis::contract_dead_inheritance_protocol::": "shared::code_analysis::",
    "shared::code_analysis::contract_line_protocol::": "shared::code_analysis::",
    "shared::code_analysis::contract_adapter_protocol::": "shared::code_analysis::",
    "shared::code_analysis::taxonomy_analysis_vo::": "shared::code_analysis::",
    "shared::code_analysis::taxonomy_code_analysis_rule_vo::": "shared::code_analysis::",
    "shared::code_analysis::taxonomy_operation_error::": "shared::code_analysis::",
    "shared::code_analysis::taxonomy_violation_code_analysis_vo::": "shared::code_analysis::",
    "shared::code_analysis::utility_file_reader": "shared::code_analysis::utility_file_reader",  # keep
    "shared::code_analysis::utility_target_resolver": "shared::code_analysis::utility_target_resolver",  # keep
    "shared::code_analysis::utility_bypass_detector": "shared::code_analysis::utility_bypass_detector",  # keep
    "shared::code_analysis::utility_mandatory_checker": "shared::code_analysis::utility_mandatory_checker",  # keep
    "shared::code_analysis::utility_column_index": "shared::code_analysis::utility_column_index",  # keep
    "shared::code_analysis::utility_language_mapper": "shared::code_analysis::utility_language_mapper",  # keep
    "shared::code_analysis::utility_code_duplication_detector": "shared::code_analysis::utility_code_duplication_detector",  # keep

    # ── config-system module ──
    "shared::config_system::contract_config_orchestrator_aggregate::": "shared::config_system::",
    "shared::config_system::contract_parser_protocol::": "shared::config_system::",
    "shared::config_system::contract_reader_protocol::": "shared::config_system::",
    "shared::config_system::contract_validator_protocol::": "shared::config_system::",
    "shared::config_system::contract_workspace_detector_protocol::": "shared::config_system::",
    "shared::config_system::taxonomy_config_error::": "shared::config_system::",
    "shared::config_system::taxonomy_config_language_vo::": "shared::config_system::",
    "shared::config_system::taxonomy_config_vo::": "shared::config_system::",
    "shared::config_system::taxonomy_identifier_vo::": "shared::config_system::",
    "shared::config_system::taxonomy_multi_project_summary_vo::": "shared::config_system::",
    "shared::config_system::taxonomy_multi_project_workspace_info_vo::": "shared::config_system::",
    "shared::config_system::taxonomy_setting_vo::": "shared::config_system::",
    "shared::config_system::taxonomy_source_vo::": "shared::config_system::",
    "shared::config_system::taxonomy_validation_vo::": "shared::config_system::",
    "shared::config_system::utility_config_defaults": "shared::config_system::utility_config_defaults",  # keep
    "shared::config_system::utility_config_parser": "shared::config_system::utility_config_parser",  # keep
    "shared::config_system::utility_config_io": "shared::config_system::utility_config_io",  # keep

    # ── import-rules module ──
    "shared::import_rules::contract_cycle_import_protocol::": "shared::import_rules::",
    "shared::import_rules::contract_dummy_import_protocol::": "shared::import_rules::",
    "shared::import_rules::contract_import_forbidden_protocol::": "shared::import_rules::",
    "shared::import_rules::contract_import_mandatory_protocol::": "shared::import_rules::",
    "shared::import_rules::contract_import_runner_aggregate::": "shared::import_rules::",
    "shared::import_rules::contract_unused_import_protocol::": "shared::import_rules::",
    "shared::import_rules::taxonomy_dependency_edge_vo::": "shared::import_rules::",
    "shared::import_rules::taxonomy_forbidden_rule_config_vo::": "shared::import_rules::",
    "shared::import_rules::taxonomy_graph_color_vo::": "shared::import_rules::",
    "shared::import_rules::taxonomy_import_constant::": "shared::import_rules::",
    "shared::import_rules::taxonomy_import_error::": "shared::import_rules::",
    "shared::import_rules::taxonomy_resolved_import_vo::": "shared::import_rules::",
    "shared::import_rules::taxonomy_violation_import_vo::": "shared::import_rules::",
    "shared::import_rules::utility_cycle_detector": "shared::import_rules::utility_cycle_detector",  # keep
    "shared::import_rules::utility_dummy_detector": "shared::import_rules::utility_dummy_detector",  # keep
    "shared::import_rules::utility_import_module_parser": "shared::import_rules::utility_import_module_parser",  # keep
    "shared::import_rules::utility_import_resolver": "shared::import_rules::utility_import_resolver",  # keep
    "shared::import_rules::utility_import_symbol_extractor": "shared::import_rules::utility_import_symbol_extractor",  # keep
    "shared::import_rules::utility_path_normalizer": "shared::import_rules::utility_path_normalizer",  # keep

    # ── naming-rules module ──
    "shared::naming_rules::contract_naming_checker_protocol::": "shared::naming_rules::",
    "shared::naming_rules::contract_naming_runner_aggregate::": "shared::naming_rules::",
    "shared::naming_rules::taxonomy_naming_constant::": "shared::naming_rules::",
    "shared::naming_rules::taxonomy_naming_violation_vo::": "shared::naming_rules::",
    "shared::naming_rules::utility_naming_checker": "shared::naming_rules::utility_naming_checker",  # keep
    "shared::naming_rules::utility_file_filter": "shared::naming_rules::utility_file_filter",  # keep
    "shared::naming_rules::utility_naming_filesystem": "shared::naming_rules::utility_naming_filesystem",  # keep

    # ── role-rules module ──
    "shared::role_rules::contract_agent_role_protocol::": "shared::role_rules::",
    "shared::role_rules::contract_capabilities_role_protocol::": "shared::role_rules::",
    "shared::role_rules::contract_role_contract_protocol::": "shared::role_rules::",
    "shared::role_rules::contract_role_runner_aggregate::": "shared::role_rules::",
    "shared::role_rules::contract_surface_role_protocol::": "shared::role_rules::",
    "shared::role_rules::contract_taxonomy_role_protocol::": "shared::role_rules::",
    "shared::role_rules::contract_utility_role_protocol::": "shared::role_rules::",
    "shared::role_rules::taxonomy_layer_names_constant::": "shared::role_rules::",
    "shared::role_rules::taxonomy_layer_names_vo::": "shared::role_rules::",
    "shared::role_rules::taxonomy_violation_role_vo::": "shared::role_rules::",

    # ── orphan-detector module ──
    "shared::orphan_detector::contract_orphan_aggregate::": "shared::orphan_detector::",
    "shared::orphan_detector::contract_orphan_graph_resolver_protocol::": "shared::orphan_detector::",
    "shared::orphan_detector::contract_orphan_protocol::": "shared::orphan_detector::",
    "shared::orphan_detector::taxonomy_orphan_contract_vo::": "shared::orphan_detector::",
    "shared::orphan_detector::taxonomy_violation_orphan_vo::": "shared::orphan_detector::",
    "shared::orphan_detector::utility_orphan_filename": "shared::orphan_detector::utility_orphan_filename",  # keep
    "shared::orphan_detector::utility_orphan_io": "shared::orphan_detector::utility_orphan_io",  # keep
    "shared::orphan_detector::utility_orphan_detector": "shared::orphan_detector::utility_orphan_detector",  # keep
    "shared::orphan_detector::utility_orphan_path": "shared::orphan_detector::utility_orphan_path",  # keep
    "shared::orphan_detector::utility_file_cache": "shared::orphan_detector::utility_file_cache",  # keep
    "shared::orphan_detector::utility_workspace_scanner": "shared::orphan_detector::utility_workspace_scanner",  # keep

    # ── git-hooks module ──
    "shared::git_hooks::contract_diff_protocol::": "shared::git_hooks::",
    "shared::git_hooks::contract_git_hooks_aggregate::": "shared::git_hooks::",
    "shared::git_hooks::contract_hook_protocol::": "shared::git_hooks::",
    "shared::git_hooks::contract_manager_protocol::": "shared::git_hooks::",
    "shared::git_hooks::contract_orchestrator_aggregate::": "shared::git_hooks::",
    "shared::git_hooks::taxonomy_git_diff_data_vo::": "shared::git_hooks::",
    "shared::git_hooks::taxonomy_hook_error::": "shared::git_hooks::",
    "shared::git_hooks::utility_git_io": "shared::git_hooks::utility_git_io",  # keep

    # ── auto-fix module ──
    "shared::auto_fix::contract_file_adapter_protocol::": "shared::auto_fix::",
    "shared::auto_fix::contract_fix_aggregate::": "shared::auto_fix::",
    "shared::auto_fix::contract_fix_protocol::": "shared::auto_fix::",
    "shared::auto_fix::taxonomy_fix_applied_event::": "shared::auto_fix::",
    "shared::auto_fix::taxonomy_fix_vo::": "shared::auto_fix::",
    "shared::auto_fix::utility_symbol_renamer": "shared::auto_fix::utility_symbol_renamer",  # keep

    # ── cli-commands module ──
    "shared::cli_commands::taxonomy_format_vo::": "shared::cli_commands::",
    "shared::cli_commands::taxonomy_result_vo::": "shared::cli_commands::",
    "shared::cli_commands::taxonomy_command_catalog_vo::": "shared::cli_commands::",
    "shared::cli_commands::taxonomy_cli_vo::": "shared::cli_commands::",
    "shared::cli_commands::taxonomy_position_vo::": "shared::cli_commands::",
    "shared::cli_commands::taxonomy_protocol_vo::": "shared::cli_commands::",
    "shared::cli_commands::taxonomy_scan_report_vo::": "shared::cli_commands::",
    "shared::cli_commands::taxonomy_scan_request_vo::": "shared::cli_commands::",
    "shared::cli_commands::utility_path_resolver": "shared::cli_commands::utility_path_resolver",  # keep

    # ── external-lint module ──
    "shared::external_lint::contract_external_lint_aggregate::": "shared::external_lint::",
    "shared::external_lint::contract_external_lint_executor_protocol::": "shared::external_lint::",
    "shared::external_lint::contract_external_lint_selector_protocol::": "shared::external_lint::",
    "shared::external_lint::contract_external_lint_utility_protocol::": "shared::external_lint::",
    "shared::external_lint::utility_external_lint": "shared::external_lint::utility_external_lint",  # keep

    # ── file-watch module ──
    "shared::file_watch::contract_change_analyzer_protocol::": "shared::file_watch::",
    "shared::file_watch::contract_provider_protocol::": "shared::file_watch::",
    "shared::file_watch::contract_watch_aggregate::": "shared::file_watch::",
    "shared::file_watch::taxonomy_diff_result_vo::": "shared::file_watch::",
    "shared::file_watch::taxonomy_service_error::": "shared::file_watch::",
    "shared::file_watch::taxonomy_watch_config_vo::": "shared::file_watch::",
    "shared::file_watch::taxonomy_watch_event_vo::": "shared::file_watch::",

    # ── tui module ──
    "shared::tui::contract_action_handler_protocol::": "shared::tui::",
    "shared::tui::contract_lint_executor_protocol::": "shared::tui::",
    "shared::tui::contract_report_formatter_protocol::": "shared::tui::",
    "shared::tui::contract_tui_aggregate::": "shared::tui::",
    "shared::tui::taxonomy_action_flags_vo::": "shared::tui::",
    "shared::tui::taxonomy_adapter_info_vo::": "shared::tui::",
    "shared::tui::taxonomy_file_entry_vo::": "shared::tui::",
    "shared::tui::taxonomy_lint_result_vo::": "shared::tui::",
    "shared::tui::taxonomy_scan_update_vo::": "shared::tui::",
    "shared::tui::taxonomy_state_vo::": "shared::tui::",
    "shared::tui::taxonomy_tui_event::": "shared::tui::",
    "shared::tui::taxonomy_watch_message_vo::": "shared::tui::",
    "shared::tui::utility_tui_io": "shared::tui::utility_tui_io",  # keep

    # ── maintenance module ──
    "shared::maintenance::contract_maintenance_aggregate::": "shared::maintenance::",
    "shared::maintenance::contract_maintenance_protocol::": "shared::maintenance::",
    "shared::maintenance::taxonomy_doctor_vo::": "shared::maintenance::",
    "shared::maintenance::taxonomy_stats_vo::": "shared::maintenance::",
    "shared::maintenance::utility_dependency_io": "shared::maintenance::utility_dependency_io",  # keep

    # ── project-setup module ──
    "shared::project_setup::contract_setup_aggregate::": "shared::project_setup::",
    "shared::project_setup::contract_setup_protocol::": "shared::project_setup::",
    "shared::project_setup::contract_tool_executor_protocol::": "shared::project_setup::",
    "shared::project_setup::taxonomy_language_vo::": "shared::project_setup::",
    "shared::project_setup::taxonomy_setup_contract_vo::": "shared::project_setup::",
    "shared::project_setup::utility_filesystem_checker": "shared::project_setup::utility_filesystem_checker",  # keep
    "shared::project_setup::utility_setup_io": "shared::project_setup::utility_setup_io",  # keep

    # ── mcp-server module ──
    "shared::mcp_server::contract_mcp_server_aggregate::": "shared::mcp_server::",
    "shared::mcp_server::taxonomy_mcp_tool_args_vo::": "shared::mcp_server::",

    # ── report-formatter module ──
    "shared::report_formatter::contract_report_formatter_aggregate::": "shared::report_formatter::",
    "shared::report_formatter::contract_report_formatter_protocol::": "shared::report_formatter::",
}

# Direct type-level substitutions (specific type renames not caught by module-level patterns)
TYPE_SUBSTITUTIONS = {
    "shared::taxonomy_common_vo::": "shared::common::",
    "shared::taxonomy_definition_vo::": "shared::common::",
    "shared::taxonomy_layer_vo::": "shared::common::",
    "shared::taxonomy_message_vo::": "shared::common::",
    "shared::taxonomy_name_vo::": "shared::common::",
    "shared::taxonomy_error_vo::": "shared::common::",
    "shared::taxonomy_source_vo::": "shared::common::",
    "shared::taxonomy_severity_vo::": "shared::common::",
    "shared::taxonomy_lint_vo::": "shared::common::",
    "shared::taxonomy_suggestion_vo::": "shared::common::",
    "shared::taxonomy_path_vo::": "shared::common::",
    "shared::taxonomy_paths_vo::": "shared::common::",
    "shared::taxonomy_adapter_name_vo::": "shared::common::",
    "shared::taxonomy_common_error::": "shared::common::",
    "shared::taxonomy_duration_vo::": "shared::common::",
    "shared::taxonomy_display_content_vo::": "shared::common::",
    "shared::taxonomy_threshold_vo::": "shared::common::",
}

# Specifically keep these as-is (utility functions accessed from shared::* module::utility_*)
KEEP_PATTERNS = [
    "shared::common::utility_layer_detector",
    "shared::common::utility_file_handler",
    "shared::common::utility_language_detector",
    "shared::common::utility_compliance_score",
    "shared::common::utility_signature_parser",
    "shared::common::utility_command_runner",
    "shared::common::utility_value_object_generator",
    "shared::code_analysis::utility_file_reader",
    "shared::code_analysis::utility_target_resolver",
    "shared::code_analysis::utility_bypass_detector",
    "shared::code_analysis::utility_mandatory_checker",
    "shared::code_analysis::utility_column_index",
    "shared::code_analysis::utility_language_mapper",
    "shared::code_analysis::utility_code_duplication_detector",
    "shared::config_system::utility_config_defaults",
    "shared::config_system::utility_config_parser",
    "shared::config_system::utility_config_io",
    "shared::import_rules::utility_cycle_detector",
    "shared::import_rules::utility_dummy_detector",
    "shared::import_rules::utility_import_module_parser",
    "shared::import_rules::utility_import_resolver",
    "shared::import_rules::utility_import_symbol_extractor",
    "shared::import_rules::utility_path_normalizer",
    "shared::naming_rules::utility_naming_checker",
    "shared::naming_rules::utility_file_filter",
    "shared::naming_rules::utility_naming_filesystem",
    "shared::orphan_detector::utility_orphan_filename",
    "shared::orphan_detector::utility_orphan_io",
    "shared::orphan_detector::utility_orphan_detector",
    "shared::orphan_detector::utility_orphan_path",
    "shared::orphan_detector::utility_file_cache",
    "shared::orphan_detector::utility_workspace_scanner",
    "shared::git_hooks::utility_git_io",
    "shared::auto_fix::utility_symbol_renamer",
    "shared::cli_commands::utility_path_resolver",
    "shared::external_lint::utility_external_lint",
    "shared::tui::utility_tui_io",
    "shared::maintenance::utility_dependency_io",
    "shared::project_setup::utility_filesystem_checker",
    "shared::project_setup::utility_setup_io",
]

# The feature crate directories to scan
FEATURE_CRATES = [
    "crates/import-rules/src",
    "crates/naming-rules/src",
    "crates/role-rules/src",
    "crates/orphan-detector/src",
    "crates/code-analysis/src",
    "crates/auto-fix/src",
    "crates/config-system/src",
    "crates/cli-commands/src",
    "crates/external-lint/src",
    "crates/file-watch/src",
    "crates/git-hooks/src",
    "crates/mcp-server/src",
    "crates/project-setup/src",
    "crates/tui/src",
    "crates/maintenance/src",
    "crates/report-formatter/src",
]

# Also scan test and tests directories
TEST_DIRS = [
    "crates/import-rules/tests",
    "crates/naming-rules/tests",
    "crates/role-rules/tests",
    "crates/orphan-detector/tests",
    "crates/code-analysis/tests",
    "crates/auto-fix/tests",
    "crates/config-system/tests",
    "crates/cli-commands/tests",
    "crates/external-lint/tests",
    "crates/file-watch/tests",
    "crates/git-hooks/tests",
    "crates/mcp-server/tests",
    "crates/project-setup/tests",
    "crates/tui/tests",
    "crates/maintenance/tests",
    "crates/report-formatter/tests",
]


def apply_substitutions(line):
    """Apply module-level substitutions to an import line."""
    # Skip non-import lines
    stripped = line.strip()
    if not stripped.startswith("use shared::"):
        return line

    # Check if this import uses a utility pattern that should be kept
    for keep in KEEP_PATTERNS:
        if keep in stripped:
            return line

    # Try module-level substitutions first (deep path → barrel path)
    for old_sub, new_sub in SUBSTITUTIONS.items():
        if old_sub in stripped:
            if old_sub.endswith("::"):
                # This is a deep path like `shared::common::taxonomy_path_vo::`
                # Replace with barrel path `shared::common::`
                new_line = stripped.replace(old_sub, new_sub, 1)
                return line.replace(stripped, new_line, 1)
            else:
                # This is a utility module path that should be kept
                return line

    # Try type-level substitutions (shared::taxonomy_X:: → shared::common::)
    # But only if the import path has THREE parts (shared::taxonomy_X::Type)
    # NOT if it already has shared::common:: prefix
    if not stripped.startswith("use shared::common::"):
        for old_path, new_path in TYPE_SUBSTITUTIONS.items():
            if stripped.startswith(f"use {old_path}"):
                new_line = stripped.replace(old_path, new_path, 1)
                return line.replace(stripped, new_line, 1)

    return line


def group_imports(lines):
    """Group imports from the same module into use module::{...} blocks."""
    result = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Check if this is a `use shared::xxx::Type;` single-line import that can be grouped
        # Pattern: `use shared::module_name::TypeName;`
        m = re.match(r'^use (shared::[a-z_]+)::([A-Z][a-zA-Z0-9_]+);$', stripped)
        if m:
            module_path = m.group(1)
            type_name = m.group(2)

            # Look ahead for more imports from the same module
            grouped_types = [type_name]
            j = i + 1
            while j < len(lines):
                next_stripped = lines[j].strip()
                nm = re.match(r'^use (shared::[a-z_]+)::([A-Z][a-zA-Z0-9_]+);$', next_stripped)
                if nm and nm.group(1) == module_path:
                    grouped_types.append(nm.group(2))
                    j += 1
                else:
                    break

            if len(grouped_types) > 1:
                # Group them
                indent = " " * (len(line) - len(line.lstrip()))
                grouped = f"{indent}use {module_path}:{{\n"
                for gt in grouped_types:
                    grouped += f"{indent}    {gt},\n"
                grouped += f"{indent}}};\n"
                result.append(grouped)
                i = j
                continue
            else:
                result.append(line)
                i += 1
                continue

        # Check for grouped `use shared::module::{...}` that isn't using barrel imports
        # Or for `use shared::module::submodule::Type` that can be barrel-imported
        result.append(line)
        i += 1
    return result


def transform_file(filepath):
    """Transform the imports in a single file."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    lines = content.split('\n')
    
    # Step 1: Apply substitutions to each line
    new_lines = []
    for line in lines:
        new_line = apply_substitutions(line)
        new_lines.append(new_line)
    
    # Step 2: Group imports from the same module
    grouped_lines = group_imports(new_lines)
    
    new_content = '\n'.join(grouped_lines)
    
    if new_content != original:
        with open(filepath, 'w') as f:
            f.write(new_content)
        return True
    return False


def main():
    changed_files = []
    
    # Process feature crate source directories
    for crate_dir in FEATURE_CRATES:
        if not os.path.isdir(crate_dir):
            continue
        for root, dirs, files in os.walk(crate_dir):
            for f in files:
                if f.endswith('.rs'):
                    filepath = os.path.join(root, f)
                    if transform_file(filepath):
                        changed_files.append(filepath)
                        print(f"  Modified: {filepath}")
    
    # Process test directories
    for test_dir in TEST_DIRS:
        if not os.path.isdir(test_dir):
            continue
        for root, dirs, files in os.walk(test_dir):
            for f in files:
                if f.endswith('.rs'):
                    filepath = os.path.join(root, f)
                    if transform_file(filepath):
                        changed_files.append(filepath)
                        print(f"  Modified: {filepath}")

    print(f"\nTotal files modified: {len(changed_files)}")


if __name__ == "__main__":
    main()
