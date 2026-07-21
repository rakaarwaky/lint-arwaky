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
use shared::common::taxonomy_git_vo::GitBranchName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_threshold_vo::Threshold;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;

use crate::surface_check_command::CheckContext;
use crate::surface_check_command::{CheckCommandsSurface, OrchestratorFactory};

/// Walk up from `path` to find the workspace root (parent of `crates/`, `packages/`, or `modules/`).
pub fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    shared::common::find_workspace_root(path)
}

/// check = self-lint (AES analysis on current project, same algorithm as scan)
pub fn handle_check(
    path: Option<FilePath>,
    git_diff: bool,
    ctx: CheckContext,
    filter: Option<String>,
    git_aggregate: Option<Arc<dyn GitHooksAggregate>>,
    config: ArchitectureConfig,
    format: Format,
) -> ExitCode {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    // Validate path exists before scanning
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::from(2);
    }
    if git_diff {
        let git_agg = match git_aggregate {
            Some(g) => g,
            None => {
                eprintln!("[error] git hooks not available");
                return ExitCode::from(2);
            }
        };
        // P2.5: pass user-provided path and filter to git-diff handler
        let rt = match crate::surface_common_command::create_current_thread_runtime() {
            Ok(r) => r,
            Err(_) => return ExitCode::from(2),
        };
        rt.block_on(crate::surface_git_command::handle_git_diff(
            git_agg,
            ctx.code_analysis_linter.clone(),
            GitBranchName::new("HEAD"),
            Some(&root),
            filter.as_deref(),
        ))
    } else {
        let surface = CheckCommandsSurface::new(ctx);
        surface.scan(&root, filter.as_deref(), config, format)
    }
}

/// scan = AES analysis on external project + external adapters
pub fn handle_scan(
    path: Option<FilePath>,
    ctx: CheckContext,
    multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    factory: OrchestratorFactory,
    filter: Option<String>,
    member: Option<String>,
    format: Format,
) -> ExitCode {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    // Validate path exists before scanning
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::from(2);
    }
    let surface = CheckCommandsSurface::new_with_factory(ctx, multi_project_orchestrator, factory);
    surface.scan_with_discovery(&root, filter.as_deref(), member.as_deref(), format)
}

pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    crate::surface_common_command::run_ci_analysis(code_analysis_linter, path, threshold)
}
