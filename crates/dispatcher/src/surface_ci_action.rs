// PURPOSE: CI entry point — CI threshold validation business logic, no formatting.
use std::sync::Arc;

use shared::common::{FilePath, Severity, Threshold};
use shared::config_system::{ConfigRequest, IConfigOrchestratorAggregate};
use shared::filesystem::FilesystemRequest;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::import_rules::IImportRunnerAggregate;
use shared::import_rules::taxonomy_import_request_vo::ImportRequest;
use shared::naming_rules::INamingRunnerAggregate;
use shared::naming_rules::taxonomy_naming_request_vo::NamingRequest;
use shared::orphan_rules::IOrphanAggregate;
use shared::orphan_rules::OrphanRequest;
use shared::quality_rules::CodeAnalysisRequest;
use shared::quality_rules::ICodeAnalysisAggregate;

/// CI evaluation result — formatted by CLI/MCP surfaces.
#[derive(Debug, Clone)]
pub struct CiReport {
    pub version: String,
    pub score: f64,
    pub threshold: u32,
    pub pass: bool,
    pub reasons: Vec<String>,
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub total_violations: usize,
}

/// DI container for all aggregates needed by CI validation.
pub struct CiScanDeps {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_io: Arc<dyn IFileSystemIOProtocol>,
}

pub fn collect_ci(
    deps: CiScanDeps,
    path: Option<FilePath>,
    threshold: Threshold,
) -> Result<CiReport, String> {
    let root_str = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !deps
        .filesystem_io
        .path_exists(std::path::Path::new(&root_str))
    {
        return Err(format!("Error: path '{}' does not exist", root_str));
    }
    let root = FilePath::new(root_str).map_err(|_| "invalid path".to_string())?;

    // Build file index once — all rule checkers consume fresh data (respects config ignored_paths)
    let root_path = std::path::Path::new(root.value());
    let ignored = deps
        .config_orchestrator
        .execute(ConfigRequest::ignored_paths(&root))
        .into_patterns();
    deps.filesystem
        .execute(FilesystemRequest::build_file_index_with_ignored(
            root_path,
            &ignored.values,
        ));

    // Quality analysis (sync)
    let mut results = deps
        .code_analysis_linter
        .execute(CodeAnalysisRequest::run_analysis(&[]))
        .into_violations();

    // Import rules — pass pre-fetched FileEntry data
    let file_list = deps
        .filesystem
        .execute(FilesystemRequest::FileList)
        .into_file_list();
    let import_res = deps
        .import_orchestrator
        .execute(ImportRequest::audit_with_entries(&file_list))
        .into_violations();
    results.extend(import_res);

    // Naming rules — pass pre-fetched FileEntry data
    let naming_res = deps
        .naming_orchestrator
        .execute(NamingRequest::audit(
            &deps
                .filesystem
                .execute(FilesystemRequest::FileList)
                .into_file_list(),
        ))
        .into_violations();
    results.extend(naming_res);

    // Orphan detection (sync) — reuse already-fetched ignored paths
    let (_, orphan_res) = deps
        .orphan_orchestrator
        .execute(OrphanRequest::scan(&root, &ignored))
        .into_scan_outcome();
    results.extend(orphan_res);

    let score = deps
        .code_analysis_linter
        .execute(CodeAnalysisRequest::calc_score(&results))
        .into_score();
    let has_crit = deps
        .code_analysis_linter
        .execute(CodeAnalysisRequest::check_critical(&results))
        .into_is_critical();
    let below_threshold = score.value() < threshold.value() as f64;

    let mut reasons: Vec<String> = Vec::new();
    if has_crit.value() {
        reasons.push("CRITICAL violation(s) detected — auto-fail triggered".to_string());
    }
    if below_threshold {
        reasons.push(format!(
            "Score below threshold ({:.1} < {})",
            score.value(),
            threshold.value()
        ));
    }

    let (mut critical_count, mut high_count, mut medium_count, mut low_count) = (0usize, 0, 0, 0);
    for r in &results {
        match r.severity {
            Severity::CRITICAL => critical_count += 1,
            Severity::HIGH => high_count += 1,
            Severity::MEDIUM => medium_count += 1,
            Severity::LOW => low_count += 1,
            _ => {}
        }
    }

    Ok(CiReport {
        version: env!("CARGO_PKG_VERSION").to_string(),
        score: score.value(),
        threshold: threshold.value(),
        pass: reasons.is_empty(),
        reasons,
        critical: critical_count,
        high: high_count,
        medium: medium_count,
        low: low_count,
        total_violations: results.len(),
    })
}
