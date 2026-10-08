// PURPOSE: CI entry point — CI threshold validation business logic, no formatting.
use std::sync::Arc;

use crate::surface_test_entries;
use shared_common::{FilePath, Severity, Threshold};
use shared_config_system::{ConfigRequest, IConfigOrchestratorAggregate};
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
// CiScanDeps uses IFilesystemAggregate for all filesystem operations.
use shared_import_rules::IImportRunnerAggregate;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_naming_rules::INamingRunnerAggregate;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_orphan_rules::IOrphanAggregate;
use shared_orphan_rules::OrphanRequest;
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::ICodeAnalysisAggregate;

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

/// Run a capability's aggregate inside `catch_unwind`. A panicking linter is
/// logged to `panics` while the rest of the pipeline continues; the caller
/// never unwinds through the dispatcher.
pub(crate) fn run_isolated<T, F>(capability: &str, panics: &mut Vec<String>, audit: F) -> T
where
    T: Default,
    F: FnOnce() -> T,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(audit)) {
        Ok(result) => result,
        Err(payload) => {
            let detail = payload
                .downcast_ref::<String>()
                .map(|s| s.as_str())
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("unknown panic payload");
            panics.push(format!("[{capability}] capability panicked: {detail}"));
            T::default()
        }
    }
}

/// DI container for all aggregates needed by CI validation.
pub struct CiScanDeps {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

pub fn collect_ci(
    deps: CiScanDeps,
    path: Option<FilePath>,
    threshold: Threshold,
) -> Result<CiReport, String> {
    Threshold::try_new(threshold.value())?;
    let root_str = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !deps
        .filesystem
        .execute(FilesystemRequest::path_exists(std::path::Path::new(
            &root_str,
        )))
        .into_path_exists()
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

    let mut panics: Vec<String> = Vec::new();

    // Quality analysis (sync) — isolated, so a panic here cannot unwind
    // through the CI path and kill the host process.
    // Pass the pre-built file index so AES3xx violations reach the CI score (#908).
    let file_list = deps
        .filesystem
        .execute(FilesystemRequest::FileList)
        .into_file_list();
    let mut results = run_isolated("quality", &mut panics, || {
        deps.code_analysis_linter
            .execute(CodeAnalysisRequest::run_analysis(&file_list))
            .into_violations()
    });

    // Import rules — pass pre-fetched FileEntry data; isolated the same way.
    let import_res = run_isolated("import", &mut panics, || {
        deps.import_orchestrator
            .execute(ImportRequest::audit_with_entries(&file_list))
            .into_violations()
    });
    results.extend(import_res);

    // Naming rules — pass pre-fetched FileEntry data.
    //
    // Two file sets, because AES101/AES102 read production source and AES103
    // reads `tests/`/`benches/`, which the index build above prunes. Routing
    // CI through the source-only variant would leave every AES103 violation out
    // of the CI score and its threshold gate.
    let naming_source = deps
        .filesystem
        .execute(FilesystemRequest::FileList)
        .into_file_list();
    let naming_tests =
        surface_test_entries::build_test_entries(&deps.filesystem, root_path, &ignored.values);
    let naming_res = run_isolated("naming", &mut panics, || {
        deps.naming_orchestrator
            .execute(NamingRequest::audit_with_tests(
                &naming_source,
                &naming_tests,
            ))
            .into_violations()
    });
    // CI mixes all rule violations into one Vec<LintResult>; convert naming
    // ViolationItems back — the scoring helpers only read code/severity/message.
    results.extend(
        naming_res
            .into_iter()
            .map(|v| {
                use shared_common::taxonomy_adapter_name_vo::AdapterName;
                use shared_common::taxonomy_lint_vo::{LintResult, LocationList};
                LintResult {
                    file: v.file,
                    line: v.line,
                    column: v.column,
                    code: v.code,
                    message: v.message,
                    source: Some(AdapterName::raw("lint-arwaky")),
                    severity: v.severity,
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                    violation_name: v.violation_name,
                    why: v.why,
                    fix: v.fix,
                }
            })
            .collect::<Vec<_>>(),
    );

    // Orphan detection (sync) — reuse already-fetched ignored paths; isolated
    // like the other capabilities so one panic cannot take down the CI run.
    let orphan_res = run_isolated("orphan", &mut panics, || {
        deps.orphan_orchestrator
            .execute(OrphanRequest::scan(&root, &ignored))
            .into_scan_outcome()
            .1
    });
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

    if !panics.is_empty() {
        for line in &panics {
            eprintln!("{line}");
        }
    }

    Ok(CiReport {
        version: shared_common::RELEASE_VERSION.to_string(),
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
