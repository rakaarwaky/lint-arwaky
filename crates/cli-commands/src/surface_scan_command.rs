// PURPOSE: Check/scan commands — CLI formatting + exit-code mapping.
// Calls dispatcher for business logic, only adds CLI output.
use dispatcher::orchestrator_orphan_pipeline::OrphanFactory;
use shared_common::ExitCode;
use std::sync::Arc;
use tracing::error;

use dispatcher::orchestrator_check_pipeline::FilesystemSeam;
use shared_cli_commands::Format;
use shared_common::FilePath;
use shared_config_system::{IConfigMergeProtocol, IConfigOrchestratorAggregate};
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_quality_rules::ICodeAnalysisAggregate;
use shared_structure_rules::IStructureAggregate;

use crate::utility_output_text_formatter::output_violations;

/// Parameters for the `scan` command.
pub struct ScanCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_seam: FilesystemSeam,
    pub config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filter: Option<String>,
    pub member: Option<String>,
    /// In-process aggregate bundle (W10); `None` falls back to subprocess.
    pub scan_aggregates: Option<dispatcher::orchestrator_check_pipeline::ScanAggregates>,
}

/// Parameters for the `import` command.
pub struct ImportCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub import_orchestrator: Arc<dyn shared_import_rules::IImportRunnerAggregate>,
    pub report_formatter: Arc<dyn shared_report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_seam: FilesystemSeam,
    pub filter: Option<String>,
    pub ignored_paths: Vec<String>,
}

/// Parameters for the `naming` command.
pub struct NamingCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub naming_orchestrator: Arc<dyn shared_naming_rules::INamingRunnerAggregate>,
    pub report_formatter: Arc<dyn shared_report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_seam: FilesystemSeam,
    pub filter: Option<String>,
    pub ignored_paths: Vec<String>,
}

/// Parameters for the `role` command.
pub struct RoleCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub role_orchestrator: Arc<dyn shared_role_rules::IRoleRunnerAggregate>,
    pub report_formatter: Arc<dyn shared_report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_seam: FilesystemSeam,
    pub filter: Option<String>,
    pub ignored_paths: Vec<String>,
}

/// Parameters for the `orphan` command.
pub struct OrphanCommandParams {
    pub path: Option<FilePath>,
    pub member: Option<String>,
    pub format: Format,
    pub orphan_orchestrator: Arc<dyn shared_orphan_rules::IOrphanAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub report_formatter: Arc<dyn shared_report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_seam: FilesystemSeam,
    pub filter: Option<String>,
    pub fs_factory: Arc<dyn Fn() -> FilesystemSeam + Send + Sync>,
    pub orphan_factory: Arc<OrphanFactory>,
}

/// Parameters for the `external` command.
pub struct ExternalCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub external_lint: Arc<dyn shared_external_lint::IExternalLintAggregate>,
    pub report_formatter: Arc<dyn shared_report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_seam: FilesystemSeam,
    pub config_parser: Arc<dyn IConfigMergeProtocol>,
    pub filter: Option<String>,
    pub ignored_paths: Vec<String>,
}

/// Parameters for the `docs` command.
pub struct DocsCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub doc_orchestrator: Arc<dyn shared_doc_rules::IDocRunnerAggregate>,
}

/// Parameters for the `structure` command.
pub struct StructureCommandParams {
    pub path: Option<FilePath>,
    pub format: Format,
    pub structure_orchestrator: Arc<dyn IStructureAggregate>,
    pub report_formatter: Arc<dyn shared_report_formatter::IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_seam: FilesystemSeam,
    pub filter: Option<String>,
    pub ignored_paths: Vec<String>,
}

fn resolve_root(path: &Option<FilePath>) -> String {
    match path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    }
}

fn is_member(path: &Option<FilePath>, seam: &FilesystemSeam) -> bool {
    dispatcher::orchestrator_check_pipeline::is_member_path(
        &FilePath::new(resolve_root(path)).unwrap_or_default(),
        seam.workspace.as_ref(),
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
    let member_flag = is_member(&params.path, &params.filesystem_seam);
    let opts = dispatcher::orchestrator_check_pipeline::ScanOptions {
        path: params.path,
        multi_project_orchestrator: params.config_orchestrator,
        filter: params.filter,
        member: params.member,
        filesystem: Arc::new(params.filesystem_seam.clone()),
        scan_aggregates: params.scan_aggregates.clone(),
    };
    let root = resolve_root(&opts.path);
    match dispatcher::orchestrator_check_pipeline::collect_scan(opts) {
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

/// `check` — 1:1 alias of `scan` per FR-001.
pub fn handle_check(params: ScanCommandParams) -> ExitCode {
    handle_scan(params)
}

/// `quality` — quality rules scan.
pub fn handle_quality(
    path: Option<FilePath>,
    format: Format,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    filesystem_seam: FilesystemSeam,
    filter: Option<String>,
    ignored_paths: Vec<String>,
) -> ExitCode {
    let root = resolve_root(&path);
    match dispatcher::orchestrator_quality_pipeline::collect_quality(
        path.clone(),
        code_analysis_linter,
        filter,
        filesystem.clone(),
        filesystem_seam.io.clone(),
        &ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                format,
                is_member(&path, &filesystem_seam),
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
    match dispatcher::orchestrator_import_pipeline::collect_import(
        params.path.clone(),
        params.import_orchestrator,
        params.filter,
        params.filesystem.clone(),
        params.filesystem_seam.io.clone(),
        &params.ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                params.format,
                is_member(&params.path, &params.filesystem_seam),
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
    match dispatcher::orchestrator_naming_pipeline::collect_naming(
        params.path.clone(),
        params.naming_orchestrator,
        params.filter,
        params.filesystem.clone(),
        params.filesystem_seam.io.clone(),
        &params.ignored_paths,
    ) {
        Ok(violations) => {
            output_violations(
                &violations,
                &root,
                params.format,
                is_member(&params.path, &params.filesystem_seam),
            );
            exit_for(violations.len())
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}

/// `docs` — doc invariant audit (AES601–AES605) over the workspace document
/// chain. This is a markdown-only check, so it bypasses the filesystem index
/// and calls the doc orchestrator directly on the target path.
pub fn handle_docs(params: DocsCommandParams) -> ExitCode {
    let root = resolve_root(&params.path);
    match dispatcher::orchestrator_docs_pipeline::collect_docs(&root, params.doc_orchestrator) {
        Ok(findings) => {
            let violations =
                dispatcher::orchestrator_docs_pipeline::docs_findings_to_violations(&findings);
            output_violations(&violations, &root, params.format, false);
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
    match dispatcher::orchestrator_role_pipeline::collect_role_direct(
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
                is_member(&params.path, &params.filesystem_seam),
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
    match dispatcher::orchestrator_orphan_pipeline::collect_orphan(
        params.path.clone(),
        params.member,
        dispatcher::orchestrator_orphan_pipeline::OrphanScanDeps::new(
            params.orphan_orchestrator,
            params.config_orchestrator,
            params.filesystem.clone(),
            params.filesystem_seam.io.clone(),
            params.filesystem_seam.workspace.clone(),
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
                is_member(&params.path, &params.filesystem_seam),
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
    match dispatcher::orchestrator_external_pipeline::collect_external_direct(
        params.path.clone(),
        params.external_lint,
        params.filesystem,
        params.filesystem_seam.io.clone(),
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

/// `structure` — folder-layout audit (AES701–AES703).
pub fn handle_structure(params: StructureCommandParams) -> ExitCode {
    let root = resolve_root(&params.path);
    match dispatcher::orchestrator_structure_pipeline::collect_structure(
        &root,
        params.structure_orchestrator,
    ) {
        Ok(mut violations) => {
            if let Some(ref filter_str) = params.filter {
                let filter_upper = filter_str.to_uppercase();
                violations.retain(|v| v.code.code().contains(&filter_upper));
            }
            output_violations(&violations, &root, params.format, false);
            exit_for(violations.len())
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}
