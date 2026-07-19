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
///
/// Priority: multi-crate workspace markers (`crates/`, `packages/`, `modules/`) are
/// checked FIRST. Only if none are found do we fall back to a standalone `Cargo.toml`.
/// This prevents `crates/shared/Cargo.toml` from being mistaken for the workspace root
/// when the actual workspace root is several levels above.
pub fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    let mut dir = std::path::Path::new(path).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    if dir.is_file() {
        dir.pop();
    }
    let mut cargo_toml_candidate: Option<std::path::PathBuf> = None;
    loop {
        // Prefer workspace-root markers (multi-crate directories)
        if dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
        }
        // Remember the first Cargo.toml we see as a fallback for standalone projects
        if cargo_toml_candidate.is_none() && dir.join("Cargo.toml").exists() {
            cargo_toml_candidate = Some(dir.clone());
        }
        if !dir.pop() {
            return cargo_toml_candidate;
        }
    }
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
    surface.scan_with_discovery(&root, filter.as_deref(), member.as_deref(), format)
}

pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<String>,
    threshold: u32,
) -> ExitCode {
    crate::surface_common_command::run_ci_analysis(code_analysis_linter, path, threshold)
}
