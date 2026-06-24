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
- [crates/cli-commands/src/infrastructure_language_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/infrastructure_language_detector.rs)
- [crates/cli-commands/src/infrastructure_path_normalization.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/infrastructure_path_normalization.rs)
- [crates/cli-commands/src/infrastructure_scanner_provider.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/infrastructure_scanner_provider.rs)
- [crates/cli-commands/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/lib.rs)
- [crates/cli-commands/src/surface_check_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_check_command.rs)
- [crates/cli-commands/src/surface_config_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_config_command.rs)
- [crates/cli-commands/src/surface_fix_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_fix_command.rs)
- [crates/cli-commands/src/surface_git_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_git_command.rs)
- [crates/cli-commands/src/surface_maintenance_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_maintenance_command.rs)
- [crates/cli-commands/src/surface_plugin_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_plugin_command.rs)
- [crates/cli-commands/src/surface_setup_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_setup_command.rs)
- [crates/cli-commands/src/surface_tui_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_tui_command.rs)
- [crates/cli-commands/src/surface_watch_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_watch_command.rs)
- [crates/shared/src/auto-fix/contract_fix_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/contract_fix_aggregate.rs)
- [crates/shared/src/auto-fix/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/mod.rs)
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
- [crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs)
- [crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs)
- [crates/shared/src/config-system/contract_orchestration_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_orchestration_aggregate.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
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
- [crates/shared/src/source-parsing/contract_language_detector_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_language_detector_port.rs)
- [crates/shared/src/source-parsing/contract_path_normalization_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_path_normalization_port.rs)
- [crates/shared/src/source-parsing/contract_scanner_provider_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_scanner_provider_port.rs)
- [crates/shared/src/source-parsing/infrastructure_file_collector_provider.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/infrastructure_file_collector_provider.rs)
- [crates/shared/src/source-parsing/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/mod.rs)
- [crates/shared/src/source-parsing/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_adapter_error.rs)
- [crates/shared/src/source-parsing/taxonomy_language_detector_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_language_detector_helper.rs)
- [crates/shared/src/source-parsing/taxonomy_naming_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_naming_error.rs)
- [crates/shared/src/source-parsing/taxonomy_naming_list_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_naming_list_vo.rs)
- [crates/shared/src/source-parsing/taxonomy_parser_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_parser_error.rs)
- [crates/shared/src/source-parsing/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_path_vo.rs)
- [crates/shared/src/source-parsing/taxonomy_paths_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_paths_vo.rs)
- [crates/shared/src/source-parsing/taxonomy_semantic_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_semantic_error.rs)

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
serde_yaml.workspace = true
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
```

---

## File: crates/cli-commands/src/infrastructure_language_detector.rs

```rust
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::contract_language_detector_port::Language;
use shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector;
use shared::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/cli-commands/src/infrastructure_path_normalization.rs

```rust
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct CliPathNormalizationProvider;

impl CliPathNormalizationProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CliPathNormalizationProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl IPathNormalizationPort for CliPathNormalizationProvider {
    fn normalize_path(&self, path: FilePath) -> FilePath {
        path
    }

    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        _context_path: Option<FilePath>,
    ) -> FilePath {
        path
    }
}
```

---

## File: crates/cli-commands/src/infrastructure_scanner_provider.rs

```rust
// Re-export FileCollectorProvider from shared so cli-commands has a local alias
pub use shared::source_parsing::infrastructure_file_collector_provider::FileCollectorProvider as CliScannerProvider;
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
pub mod surface_fix_command;
pub use surface_fix_command::FixCommandsSurface;
pub mod surface_maintenance_command;
pub use surface_maintenance_command::MaintenanceCommandsSurface;
pub mod surface_git_command;
pub mod surface_plugin_command;
pub mod surface_setup_command;
pub mod surface_tui_command;
pub use surface_tui_command::TuiCommandSurface;
pub mod surface_watch_command;
pub use surface_watch_command::WatchCommandsSurface;
pub mod infrastructure_language_detector;
pub mod infrastructure_path_normalization;
pub mod infrastructure_scanner_provider;
pub mod surface_config_command;
```

---

## File: crates/cli-commands/src/surface_check_command.rs

```rust
// PURPOSE: Command: CLI surface for check/scan — runs AES analysis on target path
use std::collections::HashMap;
use std::sync::Arc;

use std::process::ExitCode;

use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use shared::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath};

pub type OrchestratorFactory = Arc<
    dyn Fn(shared::config_system::taxonomy_config_vo::ArchitectureConfig) -> CheckContext
        + Send
        + Sync,
>;

pub struct CheckContext {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider: Arc<dyn IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub language_detector: Arc<dyn ILanguageDetectorPort>,
}

impl CheckContext {
    pub fn new_default() -> Self {
        let import_container =
            import_rules::root_import_rules_container::ImportContainer::new_default();
        let analyzer = import_container.analyzer();
        let import_orchestrator = import_container.orchestrator();

        let checker_container =
            code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(
                analyzer.clone(),
            );
        code_analysis::agent_code_analysis_orchestrator::init_global_checker(Arc::new(
            checker_container,
        ));

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::new(analyzer.clone());
        let naming_orchestrator = naming_container.orchestrator();
        let role_container = role_rules::root_role_rules_container::RoleContainer::new();
        let role_orchestrator = role_container.orchestrator();
        let code_analysis_container =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new_with_analyzer(
                analyzer,
            );
        let code_analysis_linter = code_analysis_container.code_analysis_linter();
        let external_lint_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = external_lint_container.aggregate();
        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let orphan_orchestrator = orphan_container.analyzer();
        let layer_detector = orphan_container.layer_detector();
        let scanner_provider: Arc<dyn IScannerProviderPort> =
            Arc::new(crate::infrastructure_scanner_provider::CliScannerProvider::new());
        let language_detector: Arc<dyn ILanguageDetectorPort> =
            Arc::new(crate::infrastructure_language_detector::CliLanguageDetector::new());
        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            external_lint,
            role_orchestrator,
            scanner_provider,
            orphan_orchestrator,
            layer_detector,
            language_detector,
        }
    }
}

pub struct CheckCommandsSurface {
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider: Arc<dyn IScannerProviderPort>,
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
    pub fn scan(&self, path: &str, filter: Option<&str>, config: ArchitectureConfig) {
        let path_obj = match FilePath::new(path.to_string()) {
            Ok(fp) => fp,
            Err(_) => FilePath::default(),
        };
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => {
                eprintln!("[error] failed to create tokio runtime");
                return;
            }
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
        let path_obj2 = match FilePath::new(path.to_string()) {
            Ok(fp) => fp,
            Err(_) => FilePath::default(),
        };
        let external_results = rt.block_on(self.external_lint.scan_all(&path_obj2));
        all_results.extend(external_results.values);

        // 4. Run role-rules audit (AES401, AES402, AES403, AES404, AES405, AES406)
        let role_results = rt.block_on(role_orchestrator.run_audit(&path_obj));
        all_results.extend(role_results);

        // 5. Run orphan detection — always scan entire workspace for cross-folder import graph
        let dir_path = match DirectoryPath::new(".".to_string()) {
            Ok(dp) => dp,
            Err(_) => DirectoryPath::default(),
        };
        let source_files = match self.scanner_provider.scan_directory(&dir_path) {
            Ok(list) => list.values,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        let orphan_results =
            self.orphan_orchestrator
                .check_orphans(self.layer_detector.as_ref(), &file_strs, ".");
        all_results.extend(orphan_results);

        let canonical_scan_path = match std::path::Path::new(path).canonicalize() {
            Ok(p) => p,
            Err(_) => std::path::PathBuf::from(path),
        }
        .to_string_lossy()
        .to_string();
        let filtered_results: Vec<_> = if let Some(code) = filter {
            all_results
                .into_iter()
                .filter(|r| {
                    r.code.to_string().contains(code)
                        && r.file.value.starts_with(&canonical_scan_path)
                })
                .collect()
        } else {
            all_results
                .into_iter()
                .filter(|r| r.file.value.starts_with(&canonical_scan_path))
                .collect()
        };
        let results_list = LintResultList::new(filtered_results);
        println!(
            "{}",
            code_analysis_linter.format_report(&results_list, path)
        );
    }

    /// Check if a single file is an orphan.
    /// Still needs to scan all files to build import graph for reachability analysis.
    pub fn check_orphan_single_file(&self, file_path: &str) {
        let path_obj = std::path::Path::new(file_path);

        // Collect all source files from workspace root for cross-folder graph building
        let dir_path = match DirectoryPath::new(".".to_string()) {
            Ok(dp) => dp,
            Err(_) => DirectoryPath::default(),
        };
        let source_files = match self.scanner_provider.scan_directory(&dir_path) {
            Ok(list) => list.values,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();

        // Normalize the target file path
        let target_path = if path_obj.is_absolute() {
            file_path.to_string()
        } else {
            let cwd = match std::env::current_dir() {
                Ok(d) => d,
                Err(_) => std::path::PathBuf::new(),
            };
            cwd.join(file_path).to_string_lossy().to_string()
        };

        // Run orphan detection
        let all_results =
            self.orphan_orchestrator
                .check_orphans(self.layer_detector.as_ref(), &file_strs, ".");

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
    /// If >1 workspaces found, show summary per workspace with violations grouped by code.
    pub fn scan_with_discovery(&self, path: &str, filter: Option<&str>) {
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

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => {
                eprintln!("[error] failed to create tokio runtime");
                return;
            }
        };
        let workspaces = rt.block_on(orchestrator.discover_workspaces(&path_obj));

        if workspaces.len() <= 1 {
            let default_config = ArchitectureConfig::default();
            self.scan(path, filter, default_config);
            return;
        }

        println!(
            "Lint Arwaky v{} (Multi-Workspace Mode)",
            env!("CARGO_PKG_VERSION")
        );
        println!("Found {} workspaces in {path}", workspaces.len());
        println!();

        let mut global_all_results = Vec::new();

        // Collect ALL source files from scan root for cross-workspace orphan detection
        let all_source_files: Vec<String> =
            shared::source_parsing::collect_all_source_files(std::path::Path::new(path))
                .iter()
                .map(|f| f.value.clone())
                .collect();

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
            // can find their implementations in other crates
            let orphan_results = self.orphan_orchestrator.check_orphans(
                self.layer_detector.as_ref(),
                &all_source_files,
                &ws.path.value,
            );
            all_results.extend(orphan_results);

            let filtered_results: Vec<_> = if let Some(code) = filter {
                all_results
                    .into_iter()
                    .filter(|r| r.code.to_string().contains(code))
                    .collect()
            } else {
                all_results
            };

            global_all_results.extend(filtered_results.clone());

            let mut code_counts: HashMap<String, usize> = HashMap::new();
            for r in &filtered_results {
                *code_counts.entry(r.code.to_string()).or_insert(0) += 1;
            }
            let total = filtered_results.len();

            println!("── [{ws_type}] {ws_name} — {total} violations ──");
            if !code_counts.is_empty() {
                let mut sorted: Vec<_> = code_counts.into_iter().collect();
                sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
                for (code, count) in &sorted {
                    println!("   {code}: {count}");
                }
            } else {
                println!("   (clean)");
            }
            println!();
        }

        // Print combined summary
        let mut global_code_counts: HashMap<String, usize> = HashMap::new();
        for r in &global_all_results {
            *global_code_counts.entry(r.code.to_string()).or_insert(0) += 1;
        }
        let global_total = global_all_results.len();
        let global_unique_codes = global_code_counts.len();

        println!("============================================================");
        println!("  Combined Multi-Workspace Report Summary");
        println!("============================================================");
        println!("  Total Workspace Members: {}", workspaces.len());
        println!("  Total Unique AES Codes: {}", global_unique_codes);
        println!("  Total Violations: {}", global_total);
        if !global_code_counts.is_empty() {
            println!("------------------------------------------------------------");
            let mut sorted: Vec<_> = global_code_counts.into_iter().collect();
            sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
            for (code, count) in &sorted {
                println!("  {code}: {count}");
            }
        }
        println!("============================================================");
        println!();

        println!("To scan a specific workspace:");
        for ws in &workspaces {
            println!("  scan {}", ws.path.value);
        }
    }
}

/// check = self-lint (AES analysis on current project, same algorithm as scan)
pub fn handle_check(
    path: Option<String>,
    git_diff: bool,
    ctx: CheckContext,
    filter: Option<String>,
    git_aggregate: Option<Arc<dyn GitHooksAggregate>>,
    config: ArchitectureConfig,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    if git_diff {
        let git_agg = match git_aggregate {
            Some(g) => g,
            None => {
                eprintln!("[error] git hooks not available");
                return ExitCode::FAILURE;
            }
        };
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r,
            Err(_) => {
                eprintln!("[error] failed to create tokio runtime");
                return ExitCode::FAILURE;
            }
        };
        rt.block_on(crate::surface_git_command::handle_git_diff(
            git_agg,
            ctx.code_analysis_linter.clone(),
            ctx.language_detector.clone(),
            "HEAD".to_string(),
        ))
    } else {
        let surface = CheckCommandsSurface::new(ctx);
        surface.scan(&root, filter.as_deref(), config);
        ExitCode::SUCCESS
    }
}

/// scan = AES analysis on external project + external adapters
pub fn handle_scan(
    path: Option<String>,
    ctx: CheckContext,
    multi_project_orchestrator: Option<Arc<dyn MultiProjectOrchestratorAggregate>>,
    factory: OrchestratorFactory,
    filter: Option<String>,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let surface = CheckCommandsSurface::new_with_factory(ctx, multi_project_orchestrator, factory);
    surface.scan_with_discovery(&root, filter.as_deref());
    ExitCode::SUCCESS
}

pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<String>,
    threshold: u32,
) -> ExitCode {
    use shared::cli_commands::taxonomy_severity_vo::Severity;
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
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
        match shared::source_parsing::taxonomy_path_vo::FilePath::new(".".to_string()) {
            Ok(fp) => fp,
            Err(_) => shared::source_parsing::taxonomy_path_vo::FilePath::default(),
        };

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
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
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
    let fp = match FilePath::new(root) {
        Ok(fp) => fp,
        Err(_) => FilePath::default(),
    };
    fix_surface.run_fix(fp, dry_run);
    ExitCode::SUCCESS
}
```

---

## File: crates/cli-commands/src/surface_git_command.rs

```rust
// PURPOSE: GitCommandsSurface — CLI surface for git integration
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
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
        dyn shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort,
    >,
    base: String,
) -> ExitCode {
    println!("Lint Arwaky v{} (Git-Diff Mode)", env!("CARGO_PKG_VERSION"));

    let project_path = match FilePath::new(".".to_string()) {
        Ok(fp) => fp,
        Err(_) => FilePath::default(),
    };

    let changed_files = git_aggregate
        .diff_protocol()
        .get_changed_files(&project_path)
        .await;

    let files: Vec<&shared::source_parsing::taxonomy_path_vo::FilePath> = changed_files
        .values
        .iter()
        .filter(|fp| {
            shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort::is_lintable(language_detector.as_ref(), fp)
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
    let fp = match shared::source_parsing::taxonomy_path_vo::FilePath::new(target.clone()) {
        Ok(fp) => fp,
        Err(_) => shared::source_parsing::taxonomy_path_vo::FilePath::default(),
    };
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
    let fp = match shared::source_parsing::taxonomy_path_vo::FilePath::new(target.clone()) {
        Ok(fp) => fp,
        Err(_) => shared::source_parsing::taxonomy_path_vo::FilePath::default(),
    };
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
    let json_str = match serde_json::to_string_pretty(&config) {
        Ok(s) => s,
        Err(_) => String::new(),
    };
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

## File: crates/cli-commands/src/surface_tui_command.rs

```rust
// PURPOSE: TuiCommandSurface — interactive menu-driven TUI for lint-arwaky-tui binary
use console::{style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::process::ExitCode;

pub struct TuiCommandSurface;

impl TuiCommandSurface {
    pub fn run() -> ExitCode {
        run_tui_loop()
    }
}

fn cli_binary() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-cli");
            if sibling.exists() {
                return sibling.to_string_lossy().to_string();
            }
        }
    }
    "lint-arwaky-cli".to_string()
}

fn print_header(term: &Term) {
    let _ = term.clear_screen();
    println!(
        "{}",
        style("╔══════════════════════════════════════════════════╗")
            .cyan()
            .bold()
    );
    println!(
        "{}  {}  {}",
        style("║").cyan().bold(),
        style("  Lint Arwaky TUI  -- Code Quality Gateway")
            .white()
            .bold(),
        style("║").cyan().bold()
    );
    println!(
        "{}",
        style("╚══════════════════════════════════════════════════╝")
            .cyan()
            .bold()
    );
    println!();
}

fn ask_path(prompt: &str, default: &str) -> String {
    match Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_string())
        .interact_text()
    {
        Ok(input) => input,
        Err(_) => default.to_string(),
    }
}

fn run_cmd(args: &[&str]) {
    let cli = cli_binary();
    println!(
        "\n{} {} {}\n",
        style(">").green().bold(),
        style("Running:").dim(),
        style(format!("{} {}", cli, args.join(" "))).yellow()
    );
    let status = std::process::Command::new(&cli).args(args).status();
    match status {
        Ok(s) if s.success() => {
            println!(
                "\n{} {}",
                style("OK").green().bold(),
                style("Done.").green()
            )
        }
        Ok(s) => {
            let code = match s.code() {
                Some(c) => c,
                None => -1,
            };
            println!("\n{} Exit code: {}", style("FAIL").red().bold(), code)
        }
        Err(e) => println!("\n{} Failed to run binary: {e}", style("FAIL").red().bold()),
    }
}

fn pause() {
    let _ = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(style("Press Enter to return to menu").dim().to_string())
        .default(String::new())
        .allow_empty(true)
        .interact_text();
}

#[derive(Clone, Copy, PartialEq)]
enum MenuKind {
    Action,
    Separator,
}

struct MenuItem {
    label: &'static str,
    id: &'static str,
    kind: MenuKind,
}

const MENU: &[MenuItem] = &[
    MenuItem {
        label: "[check]   AES self-lint audit",
        id: "check",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[scan]    Full multi-adapter scan",
        id: "scan",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[fix]     Apply safe automatic fixes",
        id: "fix",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[ci]      CI mode (exit 1 if score < N)",
        id: "ci",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "[orphan]        Check orphan files (AES501-506)",
        id: "orphan",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[security]      Vulnerability scan",
        id: "security",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[duplicates]    Duplication detection",
        id: "duplicates",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[dependencies]  Library vulnerability scan",
        id: "dependencies",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "[watch]  Watch and lint on changes",
        id: "watch",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "[doctor]      Diagnose environment",
        id: "doctor",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[init]        Create default config",
        id: "init",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[install]     Install adapter deps",
        id: "install",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[mcp-config]  Print MCP config",
        id: "mcp-config",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[config-show] Show active config",
        id: "config-show",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "[install-hook]   Install git pre-commit",
        id: "install-hook",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[uninstall-hook] Remove git pre-commit",
        id: "uninstall-hook",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "[adapters]  List active adapters",
        id: "adapters",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[version]   Show version",
        id: "version",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "Exit",
        id: "exit",
        kind: MenuKind::Action,
    },
];

pub fn run_tui_loop() -> ExitCode {
    let term = Term::stdout();

    loop {
        print_header(&term);

        let selectable: Vec<(usize, &MenuItem)> = MENU
            .iter()
            .enumerate()
            .filter(|(_, m)| m.kind == MenuKind::Action)
            .collect();

        let display_labels: Vec<&str> = selectable.iter().map(|(_, m)| m.label).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select command")
            .items(&display_labels)
            .default(0)
            .interact_on_opt(&term);

        let pick = match selection {
            Ok(Some(i)) => i,
            Ok(None) | Err(_) => break,
        };

        let item = selectable[pick].1;
        println!();

        match item.id {
            "exit" => break,
            "check" => {
                let p = ask_path("Path", ".");
                run_cmd(&["check", &p]);
                pause();
            }
            "scan" => {
                let p = ask_path("Path", ".");
                run_cmd(&["scan", &p]);
                pause();
            }
            "fix" => {
                let p = ask_path("Path", ".");
                run_cmd(&["fix", &p]);
                pause();
            }
            "ci" => {
                let p = ask_path("Path", ".");
                let t: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Threshold")
                    .default("80".to_string())
                    .interact_text()
                {
                    Ok(input) => input,
                    Err(_) => "80".to_string(),
                };
                run_cmd(&["ci", &p, "--threshold", &t]);
                pause();
            }
            "orphan" => {
                let p = ask_path("Path", ".");
                run_cmd(&["orphan", &p]);
                pause();
            }
            "security" => {
                let p = ask_path("Path", ".");
                run_cmd(&["security", &p]);
                pause();
            }
            "duplicates" => {
                let p = ask_path("Path", ".");
                run_cmd(&["duplicates", &p]);
                pause();
            }
            "dependencies" => {
                let p = ask_path("Path", ".");
                run_cmd(&["dependencies", &p]);
                pause();
            }
            "watch" => {
                let p = ask_path("Path", ".");
                run_cmd(&["watch", &p]);
                pause();
            }
            "doctor" => {
                run_cmd(&["doctor"]);
                pause();
            }
            "init" => {
                run_cmd(&["init"]);
                pause();
            }
            "install" => {
                run_cmd(&["install"]);
                pause();
            }
            "mcp-config" => {
                run_cmd(&["mcp-config"]);
                pause();
            }
            "config-show" => {
                run_cmd(&["config-show"]);
                pause();
            }
            "install-hook" => {
                run_cmd(&["install-hook"]);
                pause();
            }
            "uninstall-hook" => {
                run_cmd(&["uninstall-hook"]);
                pause();
            }
            "adapters" => {
                run_cmd(&["adapters"]);
                pause();
            }
            "version" => {
                run_cmd(&["version"]);
                pause();
            }
            _ => {}
        }
    }

    println!("\n{}", style("Bye!").cyan().bold());
    ExitCode::SUCCESS
}
```

---

## File: crates/cli-commands/src/surface_watch_command.rs

```rust
// PURPOSE: WatchCommandsSurface — CLI surface for file watching with auto-lint on changes
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
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
pub mod taxonomy_governance_entity;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
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

