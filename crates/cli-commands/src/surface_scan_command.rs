// PURPOSE: Check/scan commands — CLI formatting + exit-code mapping.
// Calls dispatcher for business logic, only adds CLI output.
use shared::common::ExitCode;
use std::sync::Arc;
use tracing::error;

use shared::cli_commands::Format;
use shared::common::FilePath;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;

use crate::surface_formatting::output_violations;

fn resolve_root(path: &Option<FilePath>) -> String {
    match path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    }
}

fn is_member(path: &Option<FilePath>, fs: &dyn IFilesystemAggregate) -> bool {
    dispatcher::surface_check_action::is_member_path(
        &FilePath::new(resolve_root(path)).unwrap_or_default(),
        fs,
    )
}

fn exit_for(violations: usize) -> ExitCode {
    if violations == 0 {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}

/// `scan` — run all 6 linters via subprocesses (dispatcher self-invocation).
#[allow(clippy::too_many_arguments)]
pub fn handle_scan(
    path: Option<FilePath>,
    format: Format,
    filesystem: Arc<dyn IFilesystemAggregate>,
    config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    filter: Option<String>,
    member: Option<String>,
) -> ExitCode {
    let member_flag = is_member(&path, filesystem.as_ref());
    let opts = dispatcher::surface_check_action::ScanOptions {
        path,
        multi_project_orchestrator: config_orchestrator,
        filter,
        member,
        filesystem: filesystem.clone(),
    };
    let root = resolve_root(&opts.path);
    match dispatcher::surface_check_action::collect_scan(opts) {
        Ok(violations) => {
            output_violations(&violations, &root, format, member_flag);
            exit_for(violations.len())
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}

/// `check` — quality scan (single linter).
pub fn handle_check(
    path: Option<FilePath>,
    format: Format,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    _config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    filter: Option<String>,
) -> ExitCode {
    handle_quality(
        path,
        format,
        code_analysis_linter,
        filesystem,
        filter,
        Vec::new(),
    )
}

/// `quality` — quality rules scan.
pub fn handle_quality(
    path: Option<FilePath>,
    format: Format,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    filter: Option<String>,
    ignored_paths: Vec<String>,
) -> ExitCode {
    let root = resolve_root(&path);
    match dispatcher::surface_quality_action::collect_quality(
        path.clone(),
        code_analysis_linter,
        filter,
        filesystem.clone(),
        &ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                format,
                is_member(&path, filesystem.as_ref()),
            );
            exit_for(violations.len())
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}

/// `import` — import rules scan.
#[allow(clippy::too_many_arguments)]
pub fn handle_import(
    path: Option<FilePath>,
    format: Format,
    import_orchestrator: Arc<dyn shared::import_rules::IImportRunnerAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    filter: Option<String>,
    ignored_paths: Vec<String>,
) -> ExitCode {
    let root = resolve_root(&path);
    match dispatcher::surface_import_action::collect_import(
        path.clone(),
        import_orchestrator,
        filter,
        filesystem.clone(),
        &ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                format,
                is_member(&path, filesystem.as_ref()),
            );
            exit_for(violations.len())
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}

/// `naming` — naming rules scan.
#[allow(clippy::too_many_arguments)]
pub fn handle_naming(
    path: Option<FilePath>,
    format: Format,
    naming_orchestrator: Arc<dyn shared::naming_rules::INamingRunnerAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    filter: Option<String>,
    ignored_paths: Vec<String>,
) -> ExitCode {
    let root = resolve_root(&path);
    match dispatcher::surface_naming_action::collect_naming(
        path.clone(),
        naming_orchestrator,
        filter,
        filesystem.clone(),
        &ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                format,
                is_member(&path, filesystem.as_ref()),
            );
            exit_for(violations.len())
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}

/// `role` — role rules scan (direct aggregate; subprocess variant used by `scan`).
#[allow(clippy::too_many_arguments)]
pub fn handle_role(
    path: Option<FilePath>,
    format: Format,
    role_orchestrator: Arc<dyn shared::role_rules::IRoleRunnerAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    filter: Option<String>,
    ignored_paths: Vec<String>,
) -> ExitCode {
    let root = resolve_root(&path);
    match dispatcher::surface_role_action::collect_role_direct(
        role_orchestrator,
        filter,
        filesystem.clone(),
        &root,
        &ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                format,
                is_member(&path, filesystem.as_ref()),
            );
            exit_for(violations.len())
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}

/// `orphan` — orphan scan.
#[allow(clippy::too_many_arguments)]
pub fn handle_orphan(
    path: Option<FilePath>,
    member: Option<String>,
    format: Format,
    orphan_orchestrator: Arc<dyn shared::orphan_rules::IOrphanAggregate>,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    filter: Option<String>,
) -> ExitCode {
    let root = resolve_root(&path);
    match dispatcher::surface_orphan_action::collect_orphan(
        path.clone(),
        member,
        dispatcher::surface_orphan_action::OrphanScanDeps {
            orphan_orchestrator,
            config_orchestrator,
            fs_agg: filesystem.clone(),
        },
        filter,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                format,
                is_member(&path, filesystem.as_ref()),
            );
            exit_for(violations.len())
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}

/// `external` — external lint scan (direct aggregate; subprocess variant used by `scan`).
#[allow(clippy::too_many_arguments)]
pub fn handle_external(
    path: Option<FilePath>,
    format: Format,
    external_lint: Arc<dyn shared::external_lint::IExternalLintAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    _filesystem: Arc<dyn IFilesystemAggregate>,
    filter: Option<String>,
) -> ExitCode {
    let root = resolve_root(&path);
    match dispatcher::surface_external_action::collect_external_direct(
        path.clone(),
        external_lint,
        _filesystem,
        filter,
    ) {
        Ok(violations) => {
            output_violations(&violations, &root, format, false);
            exit_for(violations.len())
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}
