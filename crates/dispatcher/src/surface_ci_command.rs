// PURPOSE: CI entry point — surface action for CI threshold validation
use shared::common::ExitCode;
use std::sync::Arc;

use shared::common::{FilePath, Severity, Threshold};
use shared::config_system::IConfigOrchestratorAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;

#[allow(clippy::too_many_arguments)]
pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    let root_str = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !filesystem.path_exists(std::path::Path::new(&root_str)) {
        eprintln!("Error: path '{}' does not exist", root_str);
        return ExitCode::RUNTIME_ERROR;
    }
    let root = match FilePath::new(root_str) {
        Ok(fp) => fp,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };

    // Quality analysis (sync)
    let mut results = code_analysis_linter.run_code_analysis_path(&root);

    // Import rules (sync in new API)
    if let Ok(import_res) = import_orchestrator.run_audit(&root) {
        results.extend(import_res);
    }

    // Naming rules
    let naming_res = naming_orchestrator.run_audit_with_entries(filesystem.file_list());
    results.extend(naming_res);

    // Orphan detection (sync)
    let ignored = config_orchestrator.ignored_paths(&root);
    let (_, orphan_res) = orphan_orchestrator.scan_orphans(&root, &ignored.values);
    results.extend(orphan_res);

    let score = code_analysis_linter.calc_score(&results);
    let has_crit = code_analysis_linter.check_critical(&results);
    let below_threshold = score.value() < threshold.value() as f64;

    let ver = env!("CARGO_PKG_VERSION");
    println!("Lint Arwaky v{ver} — CI Architecture Compliance");
    println!("Score: {:.1} / 100", score.value());
    println!("Threshold: {}", threshold.value());
    println!();

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

    println!(
        "CRITICAL: {} | HIGH: {} | MEDIUM: {} | LOW: {}",
        critical_count, high_count, medium_count, low_count
    );
    println!();

    if reasons.is_empty() {
        println!("Result: PASS (exit code 0)");
        ExitCode::OK
    } else {
        for r in &reasons {
            eprintln!("  {}", r);
        }
        eprintln!("Result: FAIL (exit code 1)");
        ExitCode::POLICY_FAIL
    }
}
