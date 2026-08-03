// PURPOSE: CI entry point — CI threshold validation business logic, no formatting.
use std::sync::Arc;

use shared::common::{FilePath, Severity, Threshold};
use shared::config_system::IConfigOrchestratorAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
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

#[allow(clippy::too_many_arguments)]
pub fn collect_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> Result<CiReport, String> {
    let root_str = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !filesystem.path_exists(std::path::Path::new(&root_str)) {
        return Err(format!("Error: path '{}' does not exist", root_str));
    }
    let root = FilePath::new(root_str).map_err(|_| "invalid path".to_string())?;

    // Build file index once — all rule checkers consume fresh data
    let root_path = std::path::Path::new(root.value());
    filesystem.build_file_index(root_path);

    // Quality analysis (sync)
    let mut results = code_analysis_linter.run_code_analysis_path(&root);

    // Import rules — pass pre-fetched FileEntry data
    let file_list = filesystem.file_list();
    let import_res = import_orchestrator.run_audit_with_entries(file_list);
    results.extend(import_res);

    // Naming rules — pass pre-fetched FileEntry data
    let naming_res = naming_orchestrator.run_audit_with_entries(filesystem.file_list());
    results.extend(naming_res);

    // Orphan detection (sync)
    let ignored = config_orchestrator.ignored_paths(&root);
    let (_, orphan_res) = orphan_orchestrator.scan_orphans(&root, &ignored.values);
    results.extend(orphan_res);

    let score = code_analysis_linter.calc_score(&results);
    let has_crit = code_analysis_linter.check_critical(&results);
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
