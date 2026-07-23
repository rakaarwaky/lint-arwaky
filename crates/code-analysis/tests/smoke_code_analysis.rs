// PURPOSE: Smoke test — verify the code-analysis crate boots, constructs
// its containers, and can execute a basic scan without panicking.
// Must complete in under 5 seconds.

use code_analysis_lint_arwaky::{CodeAnalysisContainer, CodeAnalysisOrchestrator};
use shared::common::taxonomy_path_vo::FilePath;
use std::time::Instant;

#[test]
fn crate_boots_and_scans_without_panic() {
    let start = Instant::now();

    // 1. Construct container
    let container = CodeAnalysisContainer::new();

    // 2. Get aggregate
    let aggregate = container.code_analysis_linter();

    // 3. Run analysis on current directory (should not panic)
    let root = FilePath::new(".".to_string()).unwrap();
    let results = aggregate.run_code_analysis(&root);

    // 4. Calculate score (should not panic)
    let score = aggregate.calc_score(&results.values);
    assert!(score.value >= 0.0 && score.value <= 100.0);

    // 5. Check critical (should not panic)
    let _has_critical = aggregate.check_critical(&results.values);

    // 6. Format report (should not panic)
    let report = aggregate.format_report(&results, &root);
    assert!(report.contains("AES Architecture Compliance Report"));

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test took {:?}, exceeds 5s limit",
        elapsed
    );
}

#[test]
fn orchestrator_boots_and_lint_path_works() {
    let start = Instant::now();

    let orch = CodeAnalysisOrchestrator::new_with_defaults();
    let results = orch.run_self_lint(".");

    // Should return a Vec (possibly empty) without panicking
    let _ = results.len();

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test took {:?}, exceeds 5s limit",
        elapsed
    );
}