// PURPOSE: CI entry point — CI threshold validation business logic, no formatting.
use crate::surface_entries_store;
use shared_common::{BooleanVO, FilePath, Score, Severity, Threshold};
use shared_config_system::ConfigRequest;
use shared_filesystem::FilesystemRequest;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_orphan_rules::OrphanRequest;
use shared_quality_rules::CodeAnalysisRequest;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;

/// CI evaluation result — formatted by CLI/MCP surfaces.
#[derive(Debug, Clone, Default)]
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
    /// Rule groups this run actually scored. `ci` used to judge four of the
    /// ten groups `scan` runs, so a docs-only or layout-only tree passed the
    /// gate; printing the inventory makes a missing group visible.
    pub groups_run: Vec<String>,
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
///
/// This is the same bundle `scan`/`check`/`report` use. `ci` previously had a
/// private six-field struct that carried only four of the ten rule groups, so
/// AES2xx-layer structural drift and the whole document chain (AES601–AES610)
/// never reached the score or the threshold gate. Sharing the bundle is what
/// keeps the two verbs from drifting apart again.
pub type CiScanDeps = crate::surface_check_action::ScanAggregates;

/// The rule groups `ci` scores, in report order. `tests` is not listed: it is
/// not a group, it is the `tests/`+`benches/` file set AES103 consumes, and it
/// is fed to the naming group alongside the source list.
pub const CI_RULE_GROUPS: &[&str] = &[
    "quality",
    "role",
    "import",
    "naming",
    "orphan",
    "external",
    "structure",
    "doc",
];

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
        .fs_seam
        .aggregate
        .execute(FilesystemRequest::path_exists(std::path::Path::new(
            &root_str,
        )))
        .into_path_exists()
    {
        return Err(format!("Error: path '{}' does not exist", root_str));
    }
    // `root_str` stays owned: the structure and doc groups scope their
    // findings against the target string after the analysis.
    let root = FilePath::new(root_str.clone()).map_err(|_| "invalid path".to_string())?;

    // Build file index once — all rule checkers consume fresh data (respects config ignored_paths)
    let root_path = std::path::Path::new(root.value());
    let ignored = deps
        .config
        .execute(ConfigRequest::ignored_paths(&root))
        .into_patterns();
    deps.fs_seam
        .aggregate
        .execute(FilesystemRequest::build_file_index_with_ignored(
            root_path,
            &ignored.values,
        ));

    let mut panics: Vec<String> = Vec::new();
    let mut groups_run: Vec<String> = Vec::new();

    // One file list for the whole run. The second identical `FileList` request
    // and the per-group body clone were pure overhead on a 10k-file tree.
    let file_list = deps
        .fs_seam
        .aggregate
        .execute(FilesystemRequest::FileList)
        .into_file_list();

    // Quality analysis (sync) — isolated, so a panic here cannot unwind
    // through the CI path and kill the host process.
    // Pass the pre-built file index so AES3xx violations reach the CI score (#908).
    let mut results = run_isolated("quality", &mut panics, || {
        deps.quality
            .execute(CodeAnalysisRequest::RunAnalysis {
                files: file_list.clone(),
            })
            .into_violations()
    });
    groups_run.push("quality".to_string());

    // Role rules (AES401–AES406) — never scored before this change.
    let role_res = run_isolated("role", &mut panics, || {
        deps.role
            .execute(RoleRequest::audit(&file_list))
            .into_violations()
    });
    results.extend(role_res);
    groups_run.push("role".to_string());

    // Import rules — pass pre-fetched FileEntry data; isolated the same way.
    let import_res = run_isolated("import", &mut panics, || {
        deps.import
            .execute(ImportRequest::audit_with_entries(&file_list))
            .into_violations()
    });
    results.extend(import_res);
    groups_run.push("import".to_string());

    // Naming rules — pass pre-fetched FileEntry data.
    //
    // Two file sets, because AES101/AES102 read production source and AES103
    // reads `tests/`/`benches/`, which the index build above prunes. Routing
    // CI through the source-only variant would leave every AES103 violation out
    // of the CI score and its threshold gate.
    let naming_tests = surface_entries_store::build_test_entries(
        &deps.fs_seam.aggregate,
        root_path,
        &ignored.values,
    );
    let naming_res = run_isolated("naming", &mut panics, || {
        deps.naming
            .execute(NamingRequest::audit_with_tests(&file_list, &naming_tests))
            .into_violations()
    });
    results.extend(naming_res.into_iter().map(lint_result_from_violation));
    groups_run.push("naming".to_string());

    // Orphan detection (sync) — reuse already-fetched ignored paths; isolated
    // like the other capabilities so one panic cannot take down the CI run.
    let orphan_res = run_isolated("orphan", &mut panics, || {
        deps.orphan
            .execute(OrphanRequest::scan(
                &root,
                &shared_common::taxonomy_common_vo::PatternList::new(ignored.values.clone()),
            ))
            .into_scan_outcome()
            .1
    });
    results.extend(orphan_res);
    groups_run.push("orphan".to_string());

    // External adapters — Clippy/Ruff/ESLint/… were never aggregated into the
    // gate either, so a tree whose only findings came from a native linter
    // reported a clean CI run.
    let external_res = run_isolated("external", &mut panics, || {
        crate::surface_external_action::collect_external_direct(
            Some(root.clone()),
            deps.external.clone(),
            deps.fs_seam.aggregate.clone(),
            deps.config_file_reader.as_ref(),
            None,
            &ignored.values,
        )
        .unwrap_or_default()
    });
    results.extend(external_res.into_iter().map(lint_result_from_violation));
    groups_run.push("external".to_string());

    // Structure (AES701–AES706) and the document chain (AES601–AES610) name
    // folders and documents rather than source files, so they are converted
    // from their own finding types after the file-index groups.
    let structure_res = run_isolated("structure", &mut panics, || {
        crate::surface_check_scope_action::structure_violations_in_scope(&root_str, &deps)
    });
    results.extend(structure_res.into_iter().map(lint_result_from_violation));
    groups_run.push("structure".to_string());

    let doc_res = run_isolated("doc", &mut panics, || {
        crate::surface_check_scope_action::doc_violations_in_scope(&root_str, &deps)
    });
    results.extend(doc_res.into_iter().map(lint_result_from_violation));
    groups_run.push("doc".to_string());

    let score = Score::new(shared_quality_rules::utility_compliance_score::compute_score(&results));
    let has_crit = BooleanVO::new(
        shared_quality_rules::utility_compliance_checker::contains_critical_severity(&results),
    );
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
        groups_run,
    })
}

/// The single `ViolationItem` → `LintResult` bridge for the CI path.
///
/// `structure` and `doc` findings are produced as `ViolationItem`s (their rule
/// code, severity, WHY and FIX are already resolved). The scoring helpers read
/// only code/severity/message, but the report's counters and the JSON payload
/// read everything, so the fields are carried across rather than defaulted.
fn lint_result_from_violation(v: shared_common::ViolationItem) -> shared_common::taxonomy_lint_vo::LintResult
{
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
}
