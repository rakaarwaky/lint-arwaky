// PURPOSE: CI entry point — surface action for CI threshold validation across all 5 linters
use shared::common::taxonomy_common_error::ExitCode;
use std::sync::Arc;

use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_threshold_vo::Threshold;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    let root_str = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root_str).exists() {
        eprintln!("Error: path '{}' does not exist", root_str);
        return ExitCode::from(2);
    }
    let root = match FilePath::new(root_str) {
        Ok(fp) => fp,
        Err(_) => return ExitCode::from(2),
    };

    let rt = match crate::surface_common_action::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::from(2),
    };

    let mut results = code_analysis_linter.run_code_analysis_path(&root);

    if let Ok(import_res) = rt.block_on(import_orchestrator.run_audit(&root)) {
        results.extend(import_res);
    }
    if let Ok(naming_res) = rt.block_on(naming_orchestrator.run_audit(&root)) {
        results.extend(naming_res);
    }
    let role_res = rt.block_on(role_orchestrator.run_audit(&root));
    results.extend(role_res);

    let (_, orphan_res) = orphan_orchestrator.scan_orphans(&root, &[]);
    results.extend(orphan_res);

    let score = code_analysis_linter.calc_score(&results);
    let has_crit = code_analysis_linter.check_critical(&results);
    // P2.7: compare as floats, not truncated u32
    let below_threshold = score.value() < threshold.value() as f64;

    println!("Architecture Compliance CI");
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
        ExitCode::SUCCESS
    } else {
        for r in &reasons {
            println!("  {}", r);
        }
        println!("Result: FAIL (exit code 1)");
        ExitCode::from(1)
    }
}
