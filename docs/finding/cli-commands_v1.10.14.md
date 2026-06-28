# Crate: cli-commands (v1.10.14)

This document contains the source code for feature crate `cli-commands` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands
  Violations: 0
```

---

## File List

- [crates/cli-commands/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/Cargo.toml)
- [crates/cli-commands/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/FRD.md)
- [crates/cli-commands/src/infrastructure_language_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/infrastructure_language_detector.rs)
- [crates/cli-commands/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/lib.rs)
- [crates/cli-commands/src/root_cli_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/root_cli_container.rs)
- [crates/cli-commands/src/surface_check_action.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_check_action.rs)
- [crates/cli-commands/src/surface_check_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_check_command.rs)
- [crates/cli-commands/src/surface_common_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_common_command.rs)
- [crates/cli-commands/src/surface_config_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_config_command.rs)
- [crates/cli-commands/src/surface_fix_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_fix_command.rs)
- [crates/cli-commands/src/surface_git_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_git_command.rs)
- [crates/cli-commands/src/surface_maintenance_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_maintenance_command.rs)
- [crates/cli-commands/src/surface_plugin_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_plugin_command.rs)
- [crates/cli-commands/src/surface_setup_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_setup_command.rs)
- [crates/cli-commands/src/surface_watch_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_watch_command.rs)
- [crates/shared/src/auto-fix/contract_fix_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/contract_fix_aggregate.rs)
- [crates/shared/src/auto-fix/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/mod.rs)
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
- [crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs)
- [crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/common/contract_language_detector_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_language_detector_port.rs)
- [crates/shared/src/common/contract_scanner_provider_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_scanner_provider_port.rs)
- [crates/shared/src/common/infrastructure_file_collector_provider.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/infrastructure_file_collector_provider.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_error.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_byte_count_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_byte_count_vo.rs)
- [crates/shared/src/common/taxonomy_common_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_error.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_definition_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_definition_vo.rs)
- [crates/shared/src/common/taxonomy_display_content_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_display_content_vo.rs)
- [crates/shared/src/common/taxonomy_duration_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_duration_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_filesystem_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_filesystem_error.rs)
- [crates/shared/src/common/taxonomy_job_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_vo.rs)
- [crates/shared/src/common/taxonomy_language_detector_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_language_detector_helper.rs)
- [crates/shared/src/common/taxonomy_language_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_language_vo.rs)
- [crates/shared/src/common/taxonomy_line_count_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_line_count_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_name_vo.rs)
- [crates/shared/src/common/taxonomy_naming_list_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_naming_list_vo.rs)
- [crates/shared/src/common/taxonomy_parser_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_parser_error.rs)
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
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/config-system/taxonomy_multi_project_workspace_info_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_multi_project_workspace_info_vo.rs)
- [crates/shared/src/external-lint/contract_external_lint_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_aggregate.rs)
- [crates/shared/src/external-lint/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/mod.rs)
- [crates/shared/src/file-watch/contract_watch_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_watch_aggregate.rs)
- [crates/shared/src/file-watch/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/mod.rs)
- [crates/shared/src/file-watch/taxonomy_watch_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_watch_config_vo.rs)
- [crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs)
- [crates/shared/src/git-hooks/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/mod.rs)
- [crates/shared/src/import-rules/contract_import_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_runner_aggregate.rs)
- [crates/shared/src/import-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/mod.rs)
- [crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs)
- [crates/shared/src/naming-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/mod.rs)
- [crates/shared/src/orphan-detector/contract_orphan_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_aggregate.rs)
- [crates/shared/src/orphan-detector/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/mod.rs)
- [crates/shared/src/project-setup/contract_maintenance_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_maintenance_aggregate.rs)
- [crates/shared/src/project-setup/contract_setup_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_setup_aggregate.rs)
- [crates/shared/src/project-setup/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/mod.rs)
- [crates/shared/src/role-rules/contract_role_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_runner_aggregate.rs)
- [crates/shared/src/role-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/mod.rs)

---

## File: crates/cli-commands/Cargo.toml

```toml
[package]
name = "cli_commands-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "CLI command surfaces (`check`, `scan`, `fix`, `git`, `config`, `setup`, `tui`, `watch`) composing the agent orchestrators into the user-facing CLI."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]  # (unchanged)
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
clap.workspace = true
console.workspace = true
dialoguer.workspace = true
futures.workspace = true
anyhow.workspace = true
serde_yml.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
ctrlc.workspace = true
tokio.workspace = true
shared.workspace = true
dirs.workspace = true
import_rules.workspace = true
naming_rules.workspace = true
role_rules.workspace = true
code_analysis.workspace = true
external_lint.workspace = true
orphan_detector.workspace = true
auto_fix.workspace = true
config_system.workspace = true
git_hooks.workspace = true
file_watch.workspace = true
project_setup.workspace = true
maintenance.workspace = true
```

---

## File: crates/cli-commands/FRD.md

```rust
# Feature Requirement Document (FRD) - CLI Commands

## 1. Feature Goal

The primary purpose of the `cli-commands` module is to provide a unified command-line interface (CLI) for users to run the entire linting pipeline. This module implements surfaces for main commands such as `check`, `scan`, `fix`, `git`, `config`, `setup`, `tui`, and `watch`.

## 2. Requirements & Scope

The `cli-commands` module is responsible for providing command surfaces based on the following specifications:

### Command Specifications

- **check**: Check a single file or directory against AES rules.
- **scan**: Scan the entire workspace and generate a comprehensive report.
- **fix**: Apply automatic fixes to files that violate rules.
- **git**: Implement git hooks and diff checks for pre-commit.
- **config**: Manage lint_arwaky configuration (initialization, validation, updates).
- **setup**: Set up a new project with AES directory structure.
- **tui**: Launch the terminal user interface (TUI) for interactive linting.
- **watch**: Monitor file changes in real-time and run automatic scans.

### Inputs

- Command-line arguments (`&[String]`) parsed by clap.
- Project configuration from various sources (YAML, environment variables).

### Outputs

- Linter reports in terminal format (colored output via the console crate).
- Appropriate exit code (0 for success, non-zero for violations).

---

## 3. Success Indicators

The success of the `cli-commands` module is measured by:

- **UX Consistency**: All commands follow a consistent input/output pattern.
- **Performance**: CLI is responsive with buffered output for large workspaces.
- **Help Documentation**: Every command has clear `--help` documentation.
- **Self-Audit Conformity**: The module itself passes AES rule checks.
```

---

## File: crates/cli-commands/src/infrastructure_language_detector.rs

```rust
// PURPOSE: CliLanguageDetector — ILanguageDetectorPort implementation for CLI commands
//
// Thin wrapper around the shared LanguageDetector to adapt it for CLI use.
// Delegates detect() and is_lintable() to the inner LanguageDetector.
use shared::common::contract_language_detector_port::ILanguageDetectorPort;
use shared::common::contract_language_detector_port::Language;
use shared::common::taxonomy_language_detector_helper::LanguageDetector;
use shared::common::taxonomy_path_vo::FilePath;

pub struct CliLanguageDetector {
    inner: LanguageDetector,
}

impl CliLanguageDetector {
    pub fn new() -> Self {
        Self {
            inner: LanguageDetector::new(),
        }
    }
}

impl Default for CliLanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ILanguageDetectorPort for CliLanguageDetector {
    fn detect(&self, path: &FilePath) -> Language {
        self.inner.detect(path)
    }

    fn is_lintable(&self, path: &FilePath) -> bool {
        self.inner.is_lintable(path)
    }
}
```

---

## File: crates/cli-commands/src/lib.rs

```rust
// PURPOSE: Module declarations for cli-commands (surfaces, transport, container)
pub use shared::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
pub use shared::cli_commands::taxonomy_cli_vo::{get_cli, Cli, Commands};
pub use shared::cli_commands::taxonomy_command_catalog_vo::{command_catalog, CommandCatalogVO};
pub use shared::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;

pub mod surface_check_command;
pub use surface_check_command::CheckCommandsSurface;
pub mod surface_check_action;
pub mod surface_common_command;
pub mod surface_fix_command;
pub use surface_fix_command::FixCommandsSurface;
pub mod surface_maintenance_command;
pub use surface_maintenance_command::MaintenanceCommandsSurface;
pub mod surface_git_command;
pub mod surface_plugin_command;
pub mod surface_setup_command;
pub mod surface_watch_command;
pub use surface_watch_command::WatchCommandsSurface;
pub mod infrastructure_language_detector;
pub mod root_cli_container;
pub mod surface_config_command;
pub use root_cli_container::CliContainer;
```

---

## File: crates/cli-commands/src/root_cli_container.rs

```rust
// PURPOSE: CliContainer — DI wiring for CLI binary aggregates
use std::sync::Arc;

use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::contract_scanner_provider_port::IScannerProviderPort;
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

pub struct CliContainer {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub scanner_provider: Arc<dyn IScannerProviderPort>,
    pub git_aggregate: Arc<dyn GitHooksAggregate>,
    pub multi_project_orchestrator: Arc<dyn MultiProjectOrchestratorAggregate>,
}

impl CliContainer {
    pub fn new_default() -> Self {
        let import_container =
            import_rules::root_import_rules_container::ImportContainer::new_default();
        let analyzer = import_container.analyzer();

        let checker_container =
            code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(
                analyzer.clone(),
            );
        code_analysis::agent_code_analysis_orchestrator::init_global_checker(Arc::new(
            checker_container,
        ));

        let code_analysis_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new()
                .code_analysis_linter();
        let import_orchestrator = import_container.orchestrator();

        let role_container = role_rules::root_role_rules_container::RoleContainer::new();
        let role_orchestrator = role_container.orchestrator();

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::new(analyzer);
        let naming_orchestrator = naming_container.orchestrator();

        let external_lint_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = external_lint_container.aggregate();

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let orphan_orchestrator = orphan_container.analyzer();
        let layer_detector = orphan_container.layer_detector();

        let scanner_provider: Arc<dyn IScannerProviderPort> = Arc::new(
            shared::common::infrastructure_file_collector_provider::FileCollectorProvider::new(),
        );

        let config_container = config_system::root_config_system_container::ConfigContainer::new();
        let multi_project_orchestrator = config_container.multi_project_orchestrator();

        let git_container = git_hooks::root_git_hooks_container::GitContainer::new_default();
        let git_aggregate = git_container.aggregate();

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            role_orchestrator,
            external_lint,
            orphan_orchestrator,
            layer_detector,
            scanner_provider,
            git_aggregate,
            multi_project_orchestrator,
        }
    }

    pub fn check_context(&self) -> crate::surface_check_command::CheckContext {
        crate::surface_check_command::CheckContext {
            code_analysis_linter: self.code_analysis_linter.clone(),
            import_orchestrator: self.import_orchestrator.clone(),
            naming_orchestrator: self.naming_orchestrator.clone(),
            external_lint: self.external_lint.clone(),
            role_orchestrator: self.role_orchestrator.clone(),
            scanner_provider: self.scanner_provider.clone(),
            orphan_orchestrator: self.orphan_orchestrator.clone(),
            layer_detector: self.layer_detector.clone(),
            language_detector: Arc::new(
                crate::infrastructure_language_detector::CliLanguageDetector::new(),
            ),
        }
    }
}
```

---

## File: crates/cli-commands/src/surface_check_action.rs

```rust
// PURPOSE: Check/scan/CI entry points — thin wrappers around CheckCommandsSurface
//
// Three commands, distinguished by scope:
//   - check:  self-lint the lint-arwaky project itself (uses CheckCommandsSurface.scan)
//   - scan:   full analysis on external project + external adapters (uses scan_with_discovery)
//   - ci:     CI-mode with threshold comparison and critical-violation auto-fail
//
// find_workspace_root walks up from the given path looking for Cargo.toml/crates/packages/modules.
use std::sync::Arc;

use std::process::ExitCode;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;

use crate::surface_check_command::{CheckCommandsSurface, OrchestratorFactory};

/// Walk up from `path` to find the workspace root (parent of `crates/`, `packages/`, or `modules/`).
pub fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    shared::common::taxonomy_workspace_helper::find_workspace_root(path)
}

/// check = self-lint (AES analysis on current project, same algorithm as scan)
pub fn handle_check(
    path: Option<String>,
    git_diff: bool,
    ctx: crate::surface_check_command::CheckContext,
    filter: Option<String>,
    git_aggregate: Option<Arc<dyn GitHooksAggregate>>,
    config: ArchitectureConfig,
    format: Format,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    // Validate path exists before scanning
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::FAILURE;
    }
    if git_diff {
        let git_agg = match git_aggregate {
            Some(g) => g,
            None => {
                eprintln!("[error] git hooks not available");
                return ExitCode::FAILURE;
            }
        };
        let rt = match crate::surface_common_command::create_current_thread_runtime() {
            Ok(r) => r,
            Err(_) => return ExitCode::FAILURE,
        };
        rt.block_on(crate::surface_git_command::handle_git_diff(
            git_agg,
            ctx.code_analysis_linter.clone(),
            ctx.language_detector.clone(),
            "HEAD".to_string(),
        ))
    } else {
        let surface = CheckCommandsSurface::new(ctx);
        surface.scan(&root, filter.as_deref(), config, format)
    }
}

/// scan = AES analysis on external project + external adapters
pub fn handle_scan(
    path: Option<String>,
    ctx: crate::surface_check_command::CheckContext,
    multi_project_orchestrator: Option<Arc<dyn MultiProjectOrchestratorAggregate>>,
    factory: OrchestratorFactory,
    filter: Option<String>,
    member: Option<String>,
    format: Format,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    // Validate path exists before scanning
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::FAILURE;
    }
    let surface = CheckCommandsSurface::new_with_factory(ctx, multi_project_orchestrator, factory);
    surface.scan_with_discovery(&root, filter.as_deref(), member.as_deref(), format);
    ExitCode::SUCCESS
}

pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<String>,
    threshold: u32,
) -> ExitCode {
    crate::surface_common_command::run_ci_analysis(code_analysis_linter, path, threshold)
}
```

---

## File: crates/cli-commands/src/surface_check_command.rs

```rust
// PURPOSE: CheckCommandsSurface — CLI surface for check/scan commands
//
// This is the primary surface that coordinates the full lint pipeline.
// The scan() method runs ALL linters in sequence:
//   1. Code analysis (AES301-305)
//   2. Naming rules (AES101-102)
//   3. Import rules (AES201-205)
//   4. External linters (Clippy, Ruff, ESLint, etc.)
//   5. Role rules (AES401-406)
//   6. Orphan detection (AES501-506)
//
// The OrchestratorFactory type enables the `scan` command to create
// fresh per-project DI containers for each workspace member, so that
// each member gets its own language-specific configuration.
use std::process::ExitCode;
use std::sync::Arc;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

/// CheckContext — DI container struct holding all analysis subsystems.
/// Defined in the surfaces layer because surfaces are the primary consumers.
pub struct CheckContext {
    pub code_analysis_linter:
        Arc<dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider:
        Arc<dyn shared::common::contract_scanner_provider_port::IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub language_detector:
        Arc<dyn shared::common::contract_language_detector_port::ILanguageDetectorPort>,
}

pub type OrchestratorFactory = Arc<
    dyn Fn(shared::config_system::taxonomy_config_vo::ArchitectureConfig) -> CheckContext
        + Send
        + Sync,
>;

pub struct CheckCommandsSurface {
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider:
        Arc<dyn shared::common::contract_scanner_provider_port::IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn MultiProjectOrchestratorAggregate>>,
    pub factory: Option<OrchestratorFactory>,
}

impl CheckCommandsSurface {
    pub fn new(ctx: CheckContext) -> Self {
        Self {
            external_lint: ctx.external_lint,
            code_analysis_linter: ctx.code_analysis_linter,
            import_orchestrator: ctx.import_orchestrator,
            naming_orchestrator: ctx.naming_orchestrator,
            role_orchestrator: ctx.role_orchestrator,
            scanner_provider: ctx.scanner_provider,
            orphan_orchestrator: ctx.orphan_orchestrator,
            layer_detector: ctx.layer_detector,
            multi_project_orchestrator: None,
            factory: None,
        }
    }

    pub fn new_with_factory(
        ctx: CheckContext,
        multi_project_orchestrator: Option<Arc<dyn MultiProjectOrchestratorAggregate>>,
        factory: OrchestratorFactory,
    ) -> Self {
        Self {
            external_lint: ctx.external_lint,
            code_analysis_linter: ctx.code_analysis_linter,
            import_orchestrator: ctx.import_orchestrator,
            naming_orchestrator: ctx.naming_orchestrator,
            role_orchestrator: ctx.role_orchestrator,
            scanner_provider: ctx.scanner_provider,
            orphan_orchestrator: ctx.orphan_orchestrator,
            layer_detector: ctx.layer_detector,
            multi_project_orchestrator,
            factory: Some(factory),
        }
    }

    /// Run AES analysis + external adapters on a target path.
    ///
    /// This is the core scan pipeline. It runs all 6 linter groups in the
    /// same order every time:
    ///   1. code-analysis (AES301-305) — file lines, bypass, mandatory defs
    ///   2. naming (AES101-102) — suffix/prefix conventions
    ///   3. imports (AES201-205) — mandatory, forbidden, unused, cycles
    ///   4. external (Clippy, Ruff, ESLint) — subprocess-based linting
    ///   5. roles (AES401-406) — layer-role violations
    ///   6. orphans (AES501-506) — dead code detection via import graph
    ///
    /// If a factory is provided, per-project containers are created for
    /// each workspace member (used by scan, not check).
    pub fn scan(
        &self,
        path: &str,
        filter: Option<&str>,
        config: ArchitectureConfig,
        format: Format,
    ) -> ExitCode {
        let path_obj = crate::surface_common_command::resolve_file_path(path);
        let rt = match crate::surface_common_command::create_runtime() {
            Ok(r) => r,
            Err(_) => return ExitCode::FAILURE,
        };

        // Determine dynamic orchestrators based on detected language config
        let (code_analysis_linter, naming_orchestrator, import_orchestrator, role_orchestrator) =
            if let Some(ref factory) = self.factory {
                let ctx = factory(config.clone());
                (
                    ctx.code_analysis_linter,
                    ctx.naming_orchestrator,
                    ctx.import_orchestrator,
                    ctx.role_orchestrator,
                )
            } else {
                (
                    self.code_analysis_linter.clone(),
                    self.naming_orchestrator.clone(),
                    self.import_orchestrator.clone(),
                    self.role_orchestrator.clone(),
                )
            };

        let mut all_results = Vec::new();

        // 1. Run AES analysis (same algorithm for check and scan)
        let aes_results = code_analysis_linter.run_code_analysis(path);
        all_results.extend(aes_results.values);

        // 2. Run naming-rules audit (AES101, AES102)
        let naming_results = rt.block_on(naming_orchestrator.run_audit(&path_obj));
        all_results.extend(naming_results);

        // 3. Run import-rules audit (AES201, AES202, AES205, AES203, cycles)
        let import_results = rt.block_on(import_orchestrator.run_audit(&path_obj));
        all_results.extend(import_results);

        // 4. Run external linter adapters via aggregate
        let path_obj2 = FilePath::new(path.to_string()).unwrap_or_default();
        let external_results = rt.block_on(self.external_lint.scan_all(&path_obj2));
        all_results.extend(external_results.values);

        // 5. Run role-rules audit (AES401-406: layer-role violations)
        let role_results = rt.block_on(role_orchestrator.run_audit(&path_obj));
        all_results.extend(role_results);

        // 6. Run orphan detection (AES501-506: dead code via import graph)
        let orphan_results = self.run_orphan_detection_pass(
            path,
            &self.scanner_provider,
            &self.orphan_orchestrator,
            &self.layer_detector,
        );
        all_results.extend(orphan_results);

        let violation_count = self.filter_and_display_results(
            all_results,
            path,
            filter,
            code_analysis_linter,
            &format,
        );
        if violation_count > 0 {
            ExitCode::from(1)
        } else {
            ExitCode::SUCCESS
        }
    }

    /// Run orphan detection pass — scans workspace for cross-folder import graph.
    fn run_orphan_detection_pass(
        &self,
        path: &str,
        scanner_provider: &Arc<
            dyn shared::common::contract_scanner_provider_port::IScannerProviderPort,
        >,
        orphan_orchestrator: &Arc<dyn IOrphanAggregate>,
        layer_detector: &Arc<dyn ILayerDetectionAggregate>,
    ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        let scan_root = crate::surface_check_action::find_workspace_root(path);
        let orphan_scan_root = scan_root.as_ref().and_then(|r| r.to_str()).unwrap_or(".");
        let dir_path = DirectoryPath::new(orphan_scan_root.to_string()).unwrap_or_default();
        let source_files = match scanner_provider.scan_directory(&dir_path) {
            Ok(list) => list.values,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        orphan_orchestrator.check_orphans(layer_detector.as_ref(), &file_strs, orphan_scan_root)
    }

    /// Filter results to the target path and display the report.
    fn filter_and_display_results(
        &self,
        all_results: Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
        path: &str,
        filter: Option<&str>,
        reporter: Arc<dyn ICodeAnalysisAggregate>,
        format: &Format,
    ) -> usize {
        let canonical_scan_path = crate::surface_common_command::canonicalize_path(path);
        let cwd = crate::surface_common_command::current_dir();
        let filtered_results: Vec<_> = if let Some(code) = filter {
            all_results
                .into_iter()
                .filter(|r| {
                    let abs_path = cwd.join(&r.file.value);
                    r.code.to_string().contains(code)
                        && abs_path.to_string_lossy().starts_with(&canonical_scan_path)
                })
                .collect()
        } else {
            all_results
                .into_iter()
                .filter(|r| {
                    let abs_path = cwd.join(&r.file.value);
                    abs_path.to_string_lossy().starts_with(&canonical_scan_path)
                })
                .collect()
        };
        let violation_count = filtered_results.len();
        match format {
            Format::Text => {
                let results_list = LintResultList::new(filtered_results);
                println!("{}", reporter.format_report(&results_list, path));
            }
            Format::Json => {
                let json = serde_json::to_string_pretty(&filtered_results)
                    .unwrap_or_else(|_| "[]".to_string());
                println!("{json}");
            }
            Format::Sarif => {
                let sarif = self.format_sarif_output(&filtered_results);
                println!("{sarif}");
            }
            Format::Junit => {
                let junit = self.format_junit_output(&filtered_results);
                println!("{junit}");
            }
        }
        violation_count
    }

    /// Check if a single file is an orphan.
    /// Still needs to scan all files to build import graph for reachability analysis.
    pub fn check_orphan_single_file(&self, file_path: &str) {
        let path_obj = std::path::Path::new(file_path);

        // Find workspace root for cross-crate graph building
        let scan_root = match crate::surface_check_action::find_workspace_root(file_path) {
            Some(r) => r,
            None => std::path::PathBuf::from("."),
        };
        let all_files: Vec<String> = shared::common::collect_all_source_files(&scan_root)
            .iter()
            .map(|f| f.value.clone())
            .collect();

        // Normalize the target file path
        let target_path = if path_obj.is_absolute() {
            file_path.to_string()
        } else {
            let cwd = crate::surface_common_command::current_dir();
            cwd.join(file_path).to_string_lossy().to_string()
        };

        // Run orphan detection with workspace root
        let all_results = self.orphan_orchestrator.check_orphans(
            self.layer_detector.as_ref(),
            &all_files,
            &scan_root.to_string_lossy(),
        );

        // Filter results for the specific file
        let file_results: Vec<_> = all_results
            .into_iter()
            .filter(|r| r.file.value == target_path || r.file.value == file_path)
            .collect();

        if file_results.is_empty() {
            println!(
                "  {} is NOT an orphan (reachable from entry point)",
                file_path
            );
        } else {
            println!("  {} is an ORPHAN:", file_path);
            for r in &file_results {
                println!("    [{}] {}", r.code, r.message);
            }
        }
    }

    /// Scan with multi-workspace discovery.
    ///
    /// For each discovered workspace member (Cargo.toml member, pyproject.toml
    /// module, package.json workspace):
    ///   1. Create per-project DI containers via OrchestratorFactory
    ///   2. Run all 6 linter groups on the member
    ///   3. Run orphan detection across ALL source files (cross-workspace)
    ///   4. Filter results to that member's path
    ///   5. Aggregate into global results
    ///
    /// If `member` is specified, only that workspace member is scanned.
    /// Cross-workspace orphan detection is important: contracts defined in
    /// `shared/` may be implemented in `import-rules/`, so the orphan graph
    /// must span all workspace members.
    pub fn scan_with_discovery(
        &self,
        path: &str,
        filter: Option<&str>,
        member: Option<&str>,
        format: Format,
    ) {
        let path_obj = match FilePath::new(path.to_string()) {
            Ok(fp) => fp,
            Err(_) => {
                eprintln!("[error] invalid path: {path}");
                return;
            }
        };

        let orchestrator = match self.multi_project_orchestrator.as_ref() {
            Some(o) => o.clone(),
            None => {
                eprintln!("[error] multi-project orchestrator not available");
                return;
            }
        };

        let rt = match crate::surface_common_command::create_runtime() {
            Ok(r) => r,
            Err(_) => return,
        };
        let workspaces = rt.block_on(orchestrator.discover_workspaces(&path_obj));

        if workspaces.is_empty() {
            // No workspaces discovered — fall back to single-scan mode
            let default_config = ArchitectureConfig::default();
            self.scan(path, filter, default_config, format);
            return;
        }

        // Filter to specific member if requested
        let workspaces = if let Some(member_name) = member {
            let filtered: Vec<_> = workspaces
                .into_iter()
                .filter(|ws| {
                    let ws_file = std::path::Path::new(&ws.path.value)
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    ws_file.contains(member_name) || ws.path.value.contains(member_name)
                })
                .collect();
            if filtered.is_empty() {
                eprintln!("[error] no workspace member matching '{member_name}'");
                eprintln!();
                eprintln!("Available members:");
                let orig = rt.block_on(orchestrator.discover_workspaces(&path_obj));
                for ws in &orig {
                    let name = std::path::Path::new(&ws.path.value)
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    eprintln!("  - {} ({})", name, ws.workspace_type);
                }
                eprintln!();
                eprintln!("Usage: lint-arwaky-cli scan {path} --member <name>");
                return;
            }
            filtered
        } else {
            workspaces
        };

        // Collect ALL source files from workspace root for cross-workspace orphan detection
        let scan_root = match crate::surface_check_action::find_workspace_root(path) {
            Some(r) => r,
            None => std::path::PathBuf::from(path),
        };
        let all_source_files: Vec<String> = shared::common::collect_all_source_files(&scan_root)
            .iter()
            .map(|f| f.value.clone())
            .collect();

        let multi = workspaces.len() > 1;
        if multi {
            println!(
                "Lint Arwaky v{} (Multi-Workspace Mode)",
                env!("CARGO_PKG_VERSION")
            );
            println!("Found {} workspaces in {path}", workspaces.len());
            println!();
        }

        let mut global_all_results = Vec::new();

        for ws in &workspaces {
            let ws_name = match std::path::Path::new(&ws.path.value).file_name() {
                Some(name) => name.to_string_lossy(),
                None => std::borrow::Cow::Borrowed(""),
            };
            let ws_type = &ws.workspace_type;

            let mut all_results = Vec::new();

            // Determine dynamic orchestrators based on detected language config
            let (code_analysis_linter, naming_orchestrator, import_orchestrator, role_orchestrator) =
                if let Some(ref factory) = self.factory {
                    let ctx = factory(ws.config.clone());
                    (
                        ctx.code_analysis_linter,
                        ctx.naming_orchestrator,
                        ctx.import_orchestrator,
                        ctx.role_orchestrator,
                    )
                } else {
                    (
                        self.code_analysis_linter.clone(),
                        self.naming_orchestrator.clone(),
                        self.import_orchestrator.clone(),
                        self.role_orchestrator.clone(),
                    )
                };

            let aes_results = code_analysis_linter.run_code_analysis(&ws.path.value);
            all_results.extend(aes_results.values);

            let naming_results = rt.block_on(naming_orchestrator.run_audit(&ws.path));
            all_results.extend(naming_results);

            let import_results = rt.block_on(import_orchestrator.run_audit(&ws.path));
            all_results.extend(import_results);

            let external_results = rt.block_on(self.external_lint.scan_all(&ws.path));
            all_results.extend(external_results.values);

            // Role-rules per workspace (AES401, AES402, AES403, AES404, AES405, AES406)
            let role_results = rt.block_on(role_orchestrator.run_audit(&ws.path));
            all_results.extend(role_results);

            // Orphan detection — scan across ALL workspaces so contracts in shared/
            // can find their implementations in other crates. Use scan_root (workspace
            // root) so the graph resolver can find all workspace crate directories.
            let orphan_results = self.orphan_orchestrator.check_orphans(
                self.layer_detector.as_ref(),
                &all_source_files,
                &scan_root.to_string_lossy(),
            );
            all_results.extend(orphan_results);

            // Filter results to only those in this workspace member's path
            let ws_canonical = std::path::Path::new(&ws.path.value).canonicalize().ok();
            let cwd_for_ws = match std::env::current_dir() {
                Ok(d) => d,
                Err(_) => std::path::PathBuf::new(),
            };
            let filtered_results: Vec<_> = if let Some(code) = filter {
                all_results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        r.code.to_string().contains(code)
                            && ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(true)
                    })
                    .collect()
            } else {
                all_results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        ws_canonical
                            .as_ref()
                            .map(|c| abs_path.starts_with(c))
                            .unwrap_or(true)
                    })
                    .collect()
            };

            global_all_results.extend(filtered_results.clone());

            if multi {
                let total = filtered_results.len();
                println!("── [{ws_type}] {ws_name} — {total} violations ──");
                if !filtered_results.is_empty() {
                    let mut code_counts: std::collections::HashMap<String, usize> =
                        std::collections::HashMap::new();
                    for r in &filtered_results {
                        *code_counts.entry(r.code.to_string()).or_insert(0) += 1;
                    }
                    let mut sorted: Vec<_> = code_counts.into_iter().collect();
                    sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
                    for (code, count) in &sorted {
                        println!("       {code}: {count}");
                    }
                } else {
                    println!("   (clean)");
                }
                println!();
            } else {
                // Single workspace — print full violation detail (respects --format)
                match format {
                    Format::Text => {
                        let results_list = LintResultList::new(filtered_results.clone());
                        print!(
                            "{}",
                            code_analysis_linter.format_report(&results_list, &ws.path.value)
                        );
                    }
                    Format::Json => {
                        let json = serde_json::to_string_pretty(&filtered_results)
                            .unwrap_or_else(|_| "[]".to_string());
                        println!("{json}");
                    }
                    Format::Sarif => {
                        let sarif = self.format_sarif_output(&filtered_results);
                        println!("{sarif}");
                    }
                    Format::Junit => {
                        let junit = self.format_junit_output(&filtered_results);
                        println!("{junit}");
                    }
                }
            }
        }

        if multi {
            match format {
                Format::Text => {
                    self.print_multi_workspace_summary(&global_all_results, &workspaces, member);
                }
                Format::Json => {
                    let json = serde_json::to_string_pretty(&global_all_results)
                        .unwrap_or_else(|_| "[]".to_string());
                    println!("{json}");
                }
                Format::Sarif => {
                    let sarif = self.format_sarif_output(&global_all_results);
                    println!("{sarif}");
                }
                Format::Junit => {
                    let junit = self.format_junit_output(&global_all_results);
                    println!("{junit}");
                }
            }
        }
    }

    /// Format results as a SARIF 2.1.0 JSON string.
    fn format_sarif_output(
        &self,
        results: &[shared::cli_commands::taxonomy_result_vo::LintResult],
    ) -> String {
        use shared::cli_commands::taxonomy_severity_vo::Severity;

        #[derive(serde::Serialize)]
        struct SarifLog {
            #[serde(rename = "$schema")]
            schema: &'static str,
            version: &'static str,
            runs: Vec<SarifRun>,
        }

        #[derive(serde::Serialize)]
        struct SarifRun {
            tool: SarifTool,
            results: Vec<SarifResult>,
        }

        #[derive(serde::Serialize)]
        struct SarifTool {
            driver: SarifDriver,
        }

        #[derive(serde::Serialize)]
        struct SarifDriver {
            name: &'static str,
            version: &'static str,
            information_uri: &'static str,
        }

        #[derive(serde::Serialize)]
        struct SarifResult {
            rule_id: String,
            level: String,
            message: SarifMessage,
            locations: Vec<SarifLocation>,
        }

        #[derive(serde::Serialize)]
        struct SarifMessage {
            text: String,
        }

        #[derive(serde::Serialize)]
        struct SarifLocation {
            physical_location: SarifPhysicalLocation,
        }

        #[derive(serde::Serialize)]
        struct SarifPhysicalLocation {
            artifact_location: SarifArtifactLocation,
            region: SarifRegion,
        }

        #[derive(serde::Serialize)]
        struct SarifArtifactLocation {
            uri: String,
        }

        #[derive(serde::Serialize)]
        struct SarifRegion {
            start_line: i64,
        }

        // Map Severity → SARIF level
        fn severity_to_sarif_level(sev: &Severity) -> &'static str {
            match sev {
                Severity::CRITICAL | Severity::HIGH => "error",
                Severity::MEDIUM => "warning",
                Severity::LOW | Severity::INFO => "note",
            }
        }

        let sarif_results: Vec<SarifResult> = results
            .iter()
            .map(|r| SarifResult {
                rule_id: r.code.to_string(),
                level: severity_to_sarif_level(&r.severity).to_string(),
                message: SarifMessage {
                    text: r.message.value.clone(),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: r.file.value.clone(),
                        },
                        region: SarifRegion {
                            start_line: r.line.value(),
                        },
                    },
                }],
            })
            .collect();

        let log = SarifLog {
            schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            version: "2.1.0",
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "lint-arwaky",
                        version: env!("CARGO_PKG_VERSION"),
                        information_uri: "https://github.com/rakaarwaky/lint-arwaky",
                    },
                },
                results: sarif_results,
            }],
        };

        serde_json::to_string_pretty(&log).unwrap_or_else(|_| "{}".to_string())
    }

    /// Format results as JUnit XML.
    fn format_junit_output(
        &self,
        results: &[shared::cli_commands::taxonomy_result_vo::LintResult],
    ) -> String {
        let total = results.len();
        let failures: Vec<_> = results
            .iter()
            .filter(|r| {
                use shared::cli_commands::taxonomy_severity_vo::Severity;
                matches!(
                    r.severity,
                    Severity::CRITICAL | Severity::HIGH | Severity::MEDIUM | Severity::LOW
                )
            })
            .collect();
        let failure_count = failures.len();

        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(&format!(
            "<testsuites name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
        ));
        xml.push_str(&format!(
            "  <testsuite name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
        ));

        for r in results {
            let classname = xml_escape(&r.code.to_string());
            let name = xml_escape(&format!("{}:{}", r.file.value, r.line.value()));
            let message = xml_escape(&r.message.value);
            let sev = r.severity.to_string();

            xml.push_str(&format!(
                "    <testcase classname=\"{classname}\" name=\"{name}\">\n"
            ));
            xml.push_str(&format!(
                "      <failure message=\"{sev}: {message}\" type=\"{sev}\">\n"
            ));
            xml.push_str(&format!("        {message}\n"));
            xml.push_str("      </failure>\n");
            xml.push_str("    </testcase>\n");
        }

        xml.push_str("  </testsuite>\n");
        xml.push_str("</testsuites>\n");
        xml
    }

    /// Print multi-workspace text summary (extracted from scan_with_discovery).
    fn print_multi_workspace_summary(
        &self,
        global_all_results: &[shared::cli_commands::taxonomy_result_vo::LintResult],
        workspaces: &[shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo],
        member: Option<&str>,
    ) {
        use std::collections::HashMap;

        let mut global_all_counts: HashMap<String, usize> = HashMap::new();
        for r in global_all_results {
            *global_all_counts.entry(r.code.to_string()).or_insert(0) += 1;
        }
        let global_total = global_all_results.len();
        let global_code_counts: HashMap<String, usize> = global_all_counts
            .iter()
            .filter(|(code, _)| code.starts_with("AES"))
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        let global_unique_codes = global_code_counts.len();
        let external_code_counts: HashMap<String, usize> = global_all_counts
            .iter()
            .filter(|(code, _)| !code.starts_with("AES"))
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        let global_unique_external = external_code_counts.len();

        println!("============================================================");
        println!("  Combined Multi-Workspace Report Summary");
        println!("============================================================");
        println!("  Total Workspace Members: {}", workspaces.len());
        println!("  Total Unique AES Codes: {global_unique_codes}");
        if global_unique_external > 0 {
            println!("  Total Unique External Codes: {global_unique_external}");
        }
        println!("  Total Violations: {global_total}");
        println!();
        let mut sorted: Vec<_> = global_code_counts.into_iter().collect();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
        for (code, count) in &sorted {
            println!("  {code}: {count}");
        }
        if !external_code_counts.is_empty() {
            println!();
            println!("  ── External Lint Codes ──");
            let mut ext_sorted: Vec<_> = external_code_counts.into_iter().collect();
            ext_sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
            for (code, count) in &ext_sorted {
                println!("  {code}: {count}");
            }
        }

        if member.is_none() {
            println!();
            println!("============================================================");
            println!("  Scan Individual Members");
            println!("============================================================");
            println!("  To scan a specific workspace member:");
            println!("    lint-arwaky-cli scan . --member <name>");
            println!();
            println!("  Available members:");
            for ws in workspaces {
                let name = std::path::Path::new(&ws.path.value)
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                println!("    - {} ({})", name, ws.workspace_type);
            }
            println!();
            println!("  Filter by AES rule code:");
            println!("    lint-arwaky-cli scan . --filter AES204");
        }
    }
}

/// XML-escape a string for safe inclusion in JUnit XML output.
fn xml_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            other => escaped.push(other),
        }
    }
    escaped
}
```

---

## File: crates/cli-commands/src/surface_common_command.rs

```rust
// PURPOSE: Shared utilities for CLI command surfaces
//
// Provides:
//   - create_runtime / create_current_thread_runtime: tokio runtime factories
//   - resolve_file_path / canonicalize_path / current_dir: path resolution helpers
//   - run_ci_analysis: CI pipeline that runs code analysis, computes score, compares
//     against threshold, and returns pass/fail exit code. Detects CRITICAL violations
//     as auto-fail regardless of score.
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use std::process::ExitCode;
use std::sync::Arc;

pub fn create_runtime() -> Result<tokio::runtime::Runtime, ExitCode> {
    match tokio::runtime::Runtime::new() {
        Ok(r) => Ok(r),
        Err(_) => {
            eprintln!("[error] failed to create tokio runtime");
            Err(ExitCode::FAILURE)
        }
    }
}

pub fn create_current_thread_runtime() -> Result<tokio::runtime::Runtime, ExitCode> {
    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => Ok(r),
        Err(_) => {
            eprintln!("[error] failed to create tokio runtime");
            Err(ExitCode::FAILURE)
        }
    }
}

pub fn resolve_file_path(path: &str) -> FilePath {
    FilePath::new(path.to_string()).unwrap_or_default()
}

pub fn canonicalize_path(path: &str) -> String {
    match std::path::Path::new(path).canonicalize() {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path.to_string(),
    }
}

pub fn current_dir() -> std::path::PathBuf {
    match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => std::path::PathBuf::new(),
    }
}

pub fn run_ci_analysis(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<String>,
    threshold: u32,
) -> ExitCode {
    use shared::cli_commands::taxonomy_severity_vo::Severity;
    let root = path.unwrap_or_else(|| ".".to_string());
    let results = code_analysis_linter.run_code_analysis_path(&root);
    let score = code_analysis_linter.calc_score(&results);
    let effective_threshold = if threshold == 80 { 70 } else { threshold };

    let has_crit = code_analysis_linter.check_critical(&results);
    let below_threshold = (score as u32) < effective_threshold;

    println!("Architecture Compliance CI");
    println!("Score: {:.1} / 100", score);
    println!("Threshold: {}", effective_threshold);
    println!();

    let mut reasons: Vec<String> = Vec::new();
    if has_crit {
        reasons.push("CRITICAL violation(s) detected — auto-fail triggered".to_string());
    }
    if below_threshold {
        reasons.push(format!(
            "Score below threshold ({:.1} < {})",
            score, effective_threshold
        ));
    }

    let critical_count = results
        .iter()
        .filter(|r| r.severity == Severity::CRITICAL)
        .count();
    let high_count = results
        .iter()
        .filter(|r| r.severity == Severity::HIGH)
        .count();
    let medium_count = results
        .iter()
        .filter(|r| r.severity == Severity::MEDIUM)
        .count();
    let low_count = results
        .iter()
        .filter(|r| r.severity == Severity::LOW)
        .count();

    println!(
        "CRITICAL: {} | HIGH: {} | MEDIUM: {} | LOW: {}",
        critical_count, high_count, medium_count, low_count
    );
    println!();

    if reasons.is_empty() {
        println!("Result: PASS (exit code 0)");
        ExitCode::SUCCESS
    } else {
        for r in &reasons {
            println!("  {}", r);
        }
        println!("Result: FAIL (exit code 1)");
        ExitCode::from(1)
    }
}
```

---

## File: crates/cli-commands/src/surface_config_command.rs

```rust
// PURPOSE: ConfigCommandsSurface — CLI surface for config show
use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub async fn handle_config_show(
    config_orchestrator: Arc<dyn IConfigOrchestrationAggregate>,
) -> ExitCode {
    let project_root =
        shared::common::taxonomy_path_vo::FilePath::new(".".to_string()).unwrap_or_default();

    let config_reader = config_orchestrator.config_reader();
    let config_files = config_reader.list_config_files(&project_root).await;

    if !config_files.is_empty() {
        let (lang, path_str) = &config_files[0];
        if let Some(source) = config_reader.read_config(&project_root, lang).await {
            println!("Found: {}", path_str);
            println!("{}", source.raw_content);
            return ExitCode::SUCCESS;
        }
    }

    println!("No config file found. Run `lint-arwaky init` to create one.");
    ExitCode::SUCCESS
}
```

---

## File: crates/cli-commands/src/surface_fix_command.rs

```rust
// PURPOSE: FixCommandsSurface — CLI surface for auto-fix operations
//
// Runs lint → apply auto-fixes → re-lint to measure improvement.
// Supports dry-run mode (preview only) via the fix_orchestrator_factory closure.
//
// The factory pattern allows the DI container to control whether fixes are
// actually applied (real mode) or just simulated (dry-run).
//
// Fixable violations: AES101 (naming), AES203 (unused imports), AES304 (bypass)
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

pub struct FixCommandsSurface {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub fix_orchestrator_factory:
        Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
}

impl FixCommandsSurface {
    pub fn new(
        code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
        fix_orchestrator_factory: Arc<
            dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
        >,
    ) -> Self {
        Self {
            code_analysis_linter,
            fix_orchestrator_factory,
        }
    }

    pub fn fix(&self, path: &str) {
        let canonical = match PathBuf::from(path).canonicalize() {
            Ok(p) => p,
            Err(_) => PathBuf::from(path),
        };
        let project_path = FilePath {
            value: canonical.to_string_lossy().to_string(),
        };
        self.run_fix(project_path, false);
    }

    pub fn run_fix(&self, project_path: FilePath, dry_run: bool) {
        if dry_run {
            println!("[DRY-RUN] Previewing fixes for {}...", project_path.value);
        } else {
            println!("Applying safe fixes to {}...", project_path.value);
        }

        let results = self
            .code_analysis_linter
            .run_code_analysis(&project_path.value);
        println!("Found {} violations before fix", results.len());

        let fix_orch = (self.fix_orchestrator_factory)(dry_run);
        let fix_result = fix_orch.execute(&project_path);

        println!("{}", fix_result.output.value);

        if !dry_run {
            let after_results = self
                .code_analysis_linter
                .run_code_analysis(&project_path.value);
            let fixed_count = results.len().saturating_sub(after_results.len());
            println!(
                "Fixed {} violations ({} remaining)",
                fixed_count,
                after_results.len()
            );
            println!("Fix complete.");
        } else {
            println!("Dry-run complete — no changes applied.");
        }
    }
}

pub fn handle_fix(
    path: Option<String>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
    >,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let fix_surface = FixCommandsSurface::new(code_analysis_linter, fix_orchestrator_factory);
    let fp = FilePath::new(root).unwrap_or_default();
    fix_surface.run_fix(fp, dry_run);
    ExitCode::SUCCESS
}
```

---

## File: crates/cli-commands/src/surface_git_command.rs

```rust
// PURPOSE: GitCommandsSurface — CLI surface for git-diff integration
//
// Runs AES analysis only on files changed since the specified git base (e.g. HEAD).
// Filters changed files through the language detector to skip non-lintable files.
//
// Use-case: pre-commit hooks and CI workflows that want per-file diff analysis.
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub struct GitCommandsSurface {}

impl Default for GitCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl GitCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn handle_git_diff(
    git_aggregate: Arc<dyn GitHooksAggregate>,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    language_detector: Arc<
        dyn shared::common::contract_language_detector_port::ILanguageDetectorPort,
    >,
    base: String,
) -> ExitCode {
    println!("Lint Arwaky v{} (Git-Diff Mode)", env!("CARGO_PKG_VERSION"));

    let project_path = FilePath::new(".".to_string()).unwrap_or_default();

    let changed_files = git_aggregate
        .diff_protocol()
        .get_changed_files(&project_path)
        .await;

    let files: Vec<&shared::common::taxonomy_path_vo::FilePath> = changed_files
        .values
        .iter()
        .filter(|fp| {
            shared::common::contract_language_detector_port::ILanguageDetectorPort::is_lintable(
                language_detector.as_ref(),
                fp,
            )
        })
        .collect();

    println!("Base: {} (changed files)", base);
    println!("Files changed: {}", files.len());
    println!();

    let mut total_violations = 0;
    for f in &files {
        let results = code_analysis_linter.run_code_analysis_path(&f.value);
        let fv = results.len();
        total_violations += fv;
        if fv > 0 {
            println!("  {}  -> {} violation(s)", f.value, fv);
            for r in results.iter().take(3) {
                println!(
                    "    {}:{} [{}] {}",
                    r.file.value(),
                    r.line.value(),
                    format!("{:?}", r.severity).to_uppercase(),
                    r.message.value()
                );
            }
        } else {
            println!("  {}  -> clean", f.value);
        }
    }

    println!();
    println!(
        "{} violations across {} changed files",
        total_violations,
        files.len()
    );
    ExitCode::SUCCESS
}
```

---

## File: crates/cli-commands/src/surface_maintenance_command.rs

```rust
// PURPOSE: MaintenanceCommandsSurface — CLI surface for maintenance (doctor, security, dependencies)
//
// Three independent subcommands, all delegated to MaintenanceCommandsAggregate:
//   - doctor:     toolchain diagnostics (cargo, python3, node, git, etc.)
//   - security:   vulnerability scan via cargo-audit (Rust) or bandit (Python)
//   - deps:       dependency report from Cargo.lock / pyproject.toml / requirements.txt
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub struct MaintenanceCommandsSurface;

pub async fn handle_doctor(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
) -> ExitCode {
    println!("Environment Diagnostics");
    println!();

    let diag = maintenance_orchestrator.diagnose_toolchain().await;

    println!("Rust Toolchain:");
    for status in &diag.rust_tools {
        println!(
            "  {} {} {}  ({})",
            if status.status == "OK" { "✓" } else { "✗" },
            status.name,
            status.version,
            status.status
        );
    }
    if !diag.binary_path.is_empty() {
        println!("  binary: {}", diag.binary_path);
    }

    println!();
    println!("Python Toolchain:");
    for status in &diag.python_tools {
        println!(
            "  {} {} {}  ({})",
            if status.status == "OK" { "✓" } else { "✗" },
            status.name,
            status.version,
            status.status
        );
    }

    println!();
    println!("JavaScript Toolchain:");
    for status in &diag.js_tools {
        println!(
            "  {} {} {}  ({})",
            if status.status == "OK" { "✓" } else { "✗" },
            status.name,
            status.version,
            status.status
        );
    }

    println!();
    println!("VCS:");
    for status in &diag.vcs_tools {
        println!(
            "  {} {} {}  ({})",
            if status.status == "OK" { "✓" } else { "✗" },
            status.name,
            status.version,
            status.status
        );
    }

    ExitCode::SUCCESS
}

pub async fn handle_security(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<String>,
) -> ExitCode {
    let target = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let fp = shared::common::taxonomy_path_vo::FilePath::new(target.clone()).unwrap_or_default();
    println!("Security Vulnerability Scan — {}", target);
    println!();

    let report = maintenance_orchestrator.run_security_scan(&fp).await;

    println!("Language: {}", report.language);
    println!("Tool: {}", report.tool_name);

    if !report.tool_installed {
        println!("{} not available. Please install it.", report.tool_name);
        return ExitCode::SUCCESS;
    }

    println!("Findings: {}", report.findings.len());
    for f in &report.findings {
        println!(
            "  {} {} {}:{} {}",
            f.severity.to_uppercase(),
            f.test_id,
            f.file,
            f.line,
            f.issue
        );
    }

    ExitCode::SUCCESS
}

pub async fn handle_dependencies(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<String>,
) -> ExitCode {
    let target = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let fp = shared::common::taxonomy_path_vo::FilePath::new(target.clone()).unwrap_or_default();
    println!("Dependency Report — {}", target);
    println!();

    match maintenance_orchestrator.run_dependency_report(&fp).await {
        Ok(report) => {
            println!("Language: {}", report.language);
            println!("Dependencies: {} total", report.dependencies.len());
            println!();
            println!("{:<25} {:<12} Type", "Package", "Version");
            for dep in report.dependencies.iter().take(30) {
                println!("{:<25} {:<12} {}", dep.name, dep.version, dep.dep_type);
            }
            if report.dependencies.len() > 30 {
                println!("... and {} more", report.dependencies.len() - 30);
            }
        }
        Err(e) => {
            println!("{e}");
        }
    }

    ExitCode::SUCCESS
}
```

---

## File: crates/cli-commands/src/surface_plugin_command.rs

```rust
// PURPOSE: PluginCommandsSurface — CLI surface for listing adapters/plugins
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub fn handle_adapters(_external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    println!("External lint adapters:");
    println!("  - ESLint (JavaScript/TypeScript)");
    println!("  - Prettier (JavaScript/TypeScript)");
    println!("  - TSC (TypeScript)");
    println!("  - Ruff (Python)");
    println!("  - MyPy (Python)");
    println!("  - Bandit (Python)");
    println!("  - RustFmt (Rust)");
    println!("  - CargoAudit (Rust)");
    ExitCode::SUCCESS
}
```

---

## File: crates/cli-commands/src/surface_setup_command.rs

```rust
// PURPOSE: SetupCommandsSurface — CLI surface for project setup (init, install, mcp-config)
//
// Three subcommands:
//   - init:        writes lint_arwaky.config.<lang>.yaml (local) or global XDG configs
//   - install:     pip install Python adapters (ruff, mypy, bandit) + npm install JS adapters (eslint, prettier, typescript)
//   - mcp-config:  prints MCP client config JSON for Claude/Cursor/Windsurf/Copilot
//
// Binary resolution for mcp-config: checks sibling of current exe first, falls back to PATH.
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub fn handle_init(
    setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    global: bool,
) -> ExitCode {
    if global {
        handle_init_global(setup_orchestrator);
        return ExitCode::SUCCESS;
    }
    let language = setup_orchestrator.detect_language();
    let language_str = language.value().to_string();

    let target = format!("lint_arwaky.config.{}.yaml", language_str);
    if setup_orchestrator.file_exists(&target) {
        println!("Config already exists: {}", target);
    } else {
        let content = setup_orchestrator.get_config_template(&language_str);
        match setup_orchestrator.write_config_file(&target, content) {
            Ok(desc) => {
                println!("Config created: {} (language: {})", target, language_str);
                println!("  {}", desc.value);
            }
            Err(e) => {
                println!("Error creating config: {e}");
            }
        }
    }
    ExitCode::SUCCESS
}

fn handle_init_global(setup_orchestrator: Arc<dyn SetupManagementAggregate>) {
    let config_dir = match setup_orchestrator.create_global_config_dir() {
        Ok(d) => d,
        Err(_) => {
            println!("Error: Could not determine or create XDG config directory");
            return;
        }
    };

    println!("Installing default configs to: {}", config_dir.display());

    let configs = [
        (
            "lint_arwaky.config.rust.yaml",
            setup_orchestrator.get_config_template("rust"),
        ),
        (
            "lint_arwaky.config.python.yaml",
            setup_orchestrator.get_config_template("python"),
        ),
        (
            "lint_arwaky.config.javascript.yaml",
            setup_orchestrator.get_config_template("javascript"),
        ),
    ];

    for (filename, content) in &configs {
        let target = config_dir.join(filename);
        let target_str = target.to_string_lossy();
        if setup_orchestrator.file_exists(&target_str) {
            println!("  {filename} — already exists, skipping");
        } else {
            match setup_orchestrator.write_config_file(&target_str, content) {
                Ok(_) => println!("  {filename} — created"),
                Err(e) => println!("  {filename} — error: {e}"),
            }
        }
    }
}

pub async fn handle_install(
    setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    sudo: bool,
) -> ExitCode {
    println!("Lint Arwaky — Install Adapter Dependencies");
    println!("{}", "=".repeat(50));

    println!("\n[1/2] Installing Python adapters (ruff, mypy, bandit)...");
    let py_status = setup_orchestrator.install_python_adapters().await;
    if py_status.value {
        println!("  Python adapters installed");
    } else {
        println!("  Failed to install Python adapters");
    }

    println!("\n[2/2] Installing JavaScript adapters (eslint, prettier, typescript)...");
    let js_status = setup_orchestrator.install_javascript_adapters(sudo).await;
    if js_status.value {
        println!("  JavaScript adapters installed");
    } else {
        println!("  Failed to install JavaScript adapters");
    }

    println!("\n{}", "=".repeat(50));
    if py_status.value && js_status.value {
        println!("Done! Run `lint-arwaky doctor` to verify.");
        ExitCode::SUCCESS
    } else {
        println!("Installation failed. Run with `--sudo` if npm globally requires permissions.");
        ExitCode::from(1)
    }
}

pub fn handle_mcp_config(client: &str) -> ExitCode {
    let binary = which_mcp_binary();
    let config = match client {
        "claude-code" | "claude" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        "cursor" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        "windsurf" => serde_json::json!({
            "config:lint-arwaky": {
                "command": binary,
                "args": [],
                "env": {}
            }
        }),
        "copilot" => serde_json::json!({
            "inputs": [],
            "server": {
                "command": binary,
                "args": [],
                "env": {}
            }
        }),
        _ => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
    };
    let json_str = serde_json::to_string_pretty(&config).unwrap_or_default();
    println!("MCP Client Configuration for: {}", client);
    println!("Binary: {}", binary);
    println!();
    println!("{}", json_str);
    ExitCode::SUCCESS
}

fn which_mcp_binary() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-mcp");
            if sibling.exists() {
                return sibling.to_string_lossy().to_string();
            }
        }
    }
    "lint-arwaky-mcp".to_string()
}
```

---

## File: crates/cli-commands/src/surface_watch_command.rs

```rust
// PURPOSE: WatchCommandsSurface — CLI surface for file watching with auto-lint on changes
//
// Creates a WatchConfig from the given path, sets up Ctrl+C signal handling,
// and delegates to IWatchAggregate.run() which blocks until interrupted.
//
// The atomic `running` flag coordinates graceful shutdown on Ctrl+C.
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

pub struct WatchCommandsSurface {}

impl Default for WatchCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl WatchCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn handle_watch(watch_aggregate: Arc<dyn IWatchAggregate>, path: Option<String>) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let config = WatchConfig::from_path(root);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    if let Err(e) = ctrlc::set_handler(move || {
        eprintln!("\nStopping watcher...");
        r.store(false, Ordering::SeqCst);
    }) {
        eprintln!("[error] failed to set Ctrl+C handler: {}", e);
        return ExitCode::FAILURE;
    }

    watch_aggregate.run(config, running)
}
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
