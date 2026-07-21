// PURPOSE: Shared utilities for CLI command surfaces
//
// Provides:
//   - create_runtime / create_current_thread_runtime: tokio runtime factories
//   - resolve_file_path / canonicalize_path / current_dir: path resolution helpers
//   - run_ci_analysis: CI pipeline that runs code analysis, computes score, compares
//     against threshold, and returns pass/fail exit code. Detects CRITICAL violations
//     as auto-fail regardless of score.
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_threshold_vo::Threshold;
use std::process::ExitCode;
use std::sync::Arc;

pub fn create_runtime() -> Result<tokio::runtime::Runtime, ExitCode> {
    match tokio::runtime::Runtime::new() {
        Ok(r) => Ok(r),
        Err(_) => {
            eprintln!("[error] failed to create tokio runtime");
            Err(ExitCode::FAILURE)
        }
    }
}

pub fn create_current_thread_runtime() -> Result<tokio::runtime::Runtime, ExitCode> {
    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => Ok(r),
        Err(_) => {
            eprintln!("[error] failed to create tokio runtime");
            Err(ExitCode::FAILURE)
        }
    }
}

pub fn resolve_file_path(path: &str) -> FilePath {
    FilePath::new(path.to_string()).unwrap_or_default()
}

pub fn canonicalize_path(path: &str) -> String {
    match std::path::Path::new(path).canonicalize() {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path.to_string(),
    }
}

pub fn current_dir() -> std::path::PathBuf {
    match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => std::path::PathBuf::new(),
    }
}

pub fn run_ci_analysis(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };
    let results = code_analysis_linter.run_code_analysis_path(&root);
    let score = code_analysis_linter.calc_score(&results);
    let has_crit = code_analysis_linter.check_critical(&results);
    let below_threshold = (score.value() as u32) < threshold.value();

    println!("Architecture Compliance CI");
    println!("Score: {:.1} / 100", score.value());
    println!("Threshold: {}", threshold.value());
    println!();

    let mut reasons: Vec<String> = Vec::new();
    if has_crit {
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
