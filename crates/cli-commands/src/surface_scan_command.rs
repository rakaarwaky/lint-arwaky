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

use crate::utility_output_text_formatter::output_violations;

/// Parameters for the `scan` command.
pub struct ScanCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filter: Option<String>,
    pub member: Option<String>,
}

/// Parameters for the `import` command.
pub struct ImportCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub import_orchestrator: Arc<dyn shared::import_rules::IImportRunnerAggregate>,
    pub report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filter: Option<String>,
    pub ignored_paths: Vec<String>,
}

/// Parameters for the `naming` command.
pub struct NamingCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub naming_orchestrator: Arc<dyn shared::naming_rules::INamingRunnerAggregate>,
    pub report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filter: Option<String>,
    pub ignored_paths: Vec<String>,
}

/// Parameters for the `role` command.
pub struct RoleCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub role_orchestrator: Arc<dyn shared::role_rules::IRoleRunnerAggregate>,
    pub report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filter: Option<String>,
    pub ignored_paths: Vec<String>,
}

/// Parameters for the `orphan` command.
pub struct OrphanCommandParams {
    pub path: Option<FilePath>,
    pub member: Option<String>,
    pub format: Format,
    pub orphan_orchestrator: Arc<dyn shared::orphan_rules::IOrphanAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filter: Option<String>,
    pub fs_factory: Arc<dyn Fn() -> Arc<dyn IFilesystemAggregate> + Send + Sync>,
    pub orphan_factory: Arc<
        dyn Fn(
                shared::config_system::taxonomy_config_vo::ArchitectureConfig,
                Arc<dyn IFilesystemAggregate>,
            ) -> Arc<dyn shared::orphan_rules::IOrphanAggregate>
            + Send
            + Sync,
    >,
}

/// Parameters for the `external` command.
pub struct ExternalCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub external_lint: Arc<dyn shared::external_lint::IExternalLintAggregate>,
    pub report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub config_parser: Arc<dyn shared::config_system::IConfigParserProtocol>,
    pub filter: Option<String>,
    pub ignored_paths: Vec<String>,
}

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
pub fn handle_scan(params: ScanCommandParams) -> ExitCode {
    let member_flag = is_member(&params.path, params.filesystem.as_ref());
    let opts = dispatcher::surface_check_action::ScanOptions {
        path: params.path,
        multi_project_orchestrator: params.config_orchestrator,
        filter: params.filter,
        member: params.member,
        filesystem: params.filesystem.clone(),
    };
    let root = resolve_root(&opts.path);
    match dispatcher::surface_check_action::collect_scan(opts) {
        Ok(violations) => {
            output_violations(&violations, &root, params.format, member_flag);
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
pub fn handle_import(params: ImportCommandParams) -> ExitCode {
    let root = resolve_root(&params.path);
    match dispatcher::surface_import_action::collect_import(
        params.path.clone(),
        params.import_orchestrator,
        params.filter,
        params.filesystem.clone(),
        &params.ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                params.format,
                is_member(&params.path, params.filesystem.as_ref()),
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
pub fn handle_naming(params: NamingCommandParams) -> ExitCode {
    let root = resolve_root(&params.path);
    match dispatcher::surface_naming_action::collect_naming(
        params.path.clone(),
        params.naming_orchestrator,
        params.filter,
        params.filesystem.clone(),
        &params.ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                params.format,
                is_member(&params.path, params.filesystem.as_ref()),
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
pub fn handle_role(params: RoleCommandParams) -> ExitCode {
    let root = resolve_root(&params.path);
    match dispatcher::surface_role_action::collect_role_direct(
        params.role_orchestrator,
        params.filter,
        params.filesystem.clone(),
        &root,
        &params.ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                params.format,
                is_member(&params.path, params.filesystem.as_ref()),
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
pub fn handle_orphan(params: OrphanCommandParams) -> ExitCode {
    let root = resolve_root(&params.path);
    match dispatcher::surface_orphan_action::collect_orphan(
        params.path.clone(),
        params.member,
        dispatcher::surface_orphan_action::OrphanScanDeps::new(
            params.orphan_orchestrator,
            params.config_orchestrator,
            params.filesystem.clone(),
            params.fs_factory,
            params.orphan_factory,
        ),
        params.filter,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                params.format,
                is_member(&params.path, params.filesystem.as_ref()),
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
pub fn handle_external(params: ExternalCommandParams) -> ExitCode {
    let root = resolve_root(&params.path);
    match dispatcher::surface_external_action::collect_external_direct(
        params.path.clone(),
        params.external_lint,
        params.filesystem,
        params.config_parser,
        params.filter,
        &params.ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(&violations, &root, params.format, false);
            exit_for(violations.len())
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}
