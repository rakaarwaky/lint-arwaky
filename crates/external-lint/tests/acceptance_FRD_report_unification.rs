// PURPOSE: Acceptance test — FRD: "Seamless report unification — AES and external
// violations combined in a single unified report or MCP response."

use external_lint_lint_arwaky::ExternalLintContainer;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use std::fs;

/// FRD-EXT-004: scan_all returns a unified LintResultList combining all adapter outputs.
#[tokio::test]
async fn frd_004_scan_all_returns_unified_result_list() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    fs::write(dir.path().join("app.py"), "x = 1").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results: LintResultList = aggregate.scan_all(&path).await;

    // The return type IS the unified list — type-level proof of unification
    // Each result has file, line, column, code, message, source, severity
    for result in results.iter() {
        assert!(
            !result.file.value().is_empty(),
            "Each result must have a file"
        );
        assert!(
            result.source.is_some(),
            "Each result must have a source adapter"
        );
    }
}

/// FRD-EXT-005: Each LintResult carries the source adapter name for traceability.
#[tokio::test]
async fn frd_005_results_carry_adapter_source() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("script.py"), "import os\n").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results = aggregate.scan_all(&path).await;

    // If any results exist, they must have a source
    for result in results.iter() {
        let source = result.source.as_ref().expect("source must be present");
        let name = source.value();
        assert!(
            [
                "ruff",
                "mypy",
                "bandit",
                "eslint",
                "prettier",
                "tsc",
                "clippy",
                "rustfmt",
                "cargo-audit"
            ]
            .contains(&name),
            "Source '{}' must be a known adapter",
            name
        );
    }
}

/// FRD-EXT-006: adapter_names() exposes the full registered adapter list.
#[tokio::test]
async fn frd_006_adapter_names_exposes_all_registered() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();

    let expected = [
        "clippy",
        "rustfmt",
        "cargo-audit",
        "ruff",
        "mypy",
        "bandit",
        "eslint",
        "prettier",
        "tsc",
    ];
    for name in &expected {
        assert!(
            names.contains(&name.to_string()),
            "Expected adapter '{}' to be registered",
            name
        );
    }
}
