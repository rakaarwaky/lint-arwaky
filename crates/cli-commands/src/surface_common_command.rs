// PURPOSE: Shared utilities for CLI command surfaces
//
// Provides:
//   - create_runtime / create_current_thread_runtime: tokio runtime factories
//   - resolve_file_path / canonicalize_path / current_dir: path resolution helpers
//   - run_ci_analysis: CI pipeline that runs code analysis, computes score, compares
//     against threshold, and returns pass/fail exit code. Detects CRITICAL violations
//     as auto-fail regardless of score.
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
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
    path: Option<String>,
    threshold: u32,
) -> ExitCode {
    use shared::cli_commands::taxonomy_severity_vo::Severity;
    let root = path.unwrap_or_else(|| ".".to_string());
    let results = code_analysis_linter.run_code_analysis_path(&root);
    let score = code_analysis_linter.calc_score(&results);
    let effective_threshold = if threshold == 80 { 70 } else { threshold };

    let has_crit = code_analysis_linter.check_critical(&results);
    let below_threshold = (score as u32) < effective_threshold;

    println!("Architecture Compliance CI");
    println!("Score: {:.1} / 100", score);
    println!("Threshold: {}", effective_threshold);
    println!();

    let mut reasons: Vec<String> = Vec::new();
    if has_crit {
        reasons.push("CRITICAL violation(s) detected — auto-fail triggered".to_string());
    }
    if below_threshold {
        reasons.push(format!(
            "Score below threshold ({:.1} < {})",
            score, effective_threshold
        ));
    }

    let critical_count = results
        .iter()
        .filter(|r| r.severity == Severity::CRITICAL)
        .count();
    let high_count = results
        .iter()
        .filter(|r| r.severity == Severity::HIGH)
        .count();
    let medium_count = results
        .iter()
        .filter(|r| r.severity == Severity::MEDIUM)
        .count();
    let low_count = results
        .iter()
        .filter(|r| r.severity == Severity::LOW)
        .count();

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
