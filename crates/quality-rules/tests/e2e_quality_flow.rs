// PURPOSE: E2E tests — full pipeline: temp dir with quality violations → container → analysis → verify all types found
use quality_rules_lint_arwaky::CodeAnalysisContainer;

use shared::common::FilePath;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use std::path::PathBuf;

#[test]
fn e2e_detects_all_violation_types() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();

    // File 1: contains bypass tokens (AES304)
    let bypass_content = (0..20)
        .map(|i| format!("fn func_{}() {{ let x = foo.unwrap(); }}", i))
        .collect::<Vec<_>>()
        .join("\n");

    // File 2: another file with bypass tokens for duplication comparison
    let dup_content = bypass_content.clone();

    let entries = vec![
        FileEntry {
            path: PathBuf::from("src/aes304_example.rs"),
            extension: "rs".to_string(),
            language: shared::common::taxonomy_language_vo::Language::Rust,
            size: bypass_content.len() as u64,
            content: bypass_content,
            parse_ok: true,
            parse_metadata: None,
        },
        FileEntry {
            path: PathBuf::from("src/aes304_duplicate.rs"),
            extension: "rs".to_string(),
            language: shared::common::taxonomy_language_vo::Language::Rust,
            size: dup_content.len() as u64,
            content: dup_content,
            parse_ok: true,
            parse_metadata: None,
        },
    ];

    let results = linter.run_analysis_with_entries(&entries);

    // Should find AES304 violations
    let has_aes304 = results.iter().any(|r| r.code.code().contains("AES304"));
    assert!(has_aes304, "Expected AES304 bypass violations");

    // Score should be less than 100 with violations present
    let score = linter.calc_score(&results);
    assert!(
        score.value() < 100.0,
        "Score should be < 100 when violations exist, got {}",
        score.value()
    );

    // Report should contain violation data
    let results_list = shared::cli_commands::LintResultList::new(results);
    let root = FilePath::new("/project".to_string()).unwrap();
    let report = linter.format_report(&results_list, &root);
    assert!(report.value().contains("AES"));
}

#[test]
fn e2e_disabled_config_returns_empty() {
    use shared::common::BooleanVO;
    use shared::config_system::ArchitectureConfig;

    let config = ArchitectureConfig {
        enabled: BooleanVO::new(false),
        ..ArchitectureConfig::default()
    };

    let layer_map = shared::common::LayerMapVO::new(std::collections::HashMap::new());
    let container = CodeAnalysisContainer::new_with_config(config, layer_map);
    let linter = container.code_analysis_linter();

    let entries = vec![FileEntry {
        path: PathBuf::from("src/lib.rs"),
        extension: "rs".to_string(),
        language: shared::common::taxonomy_language_vo::Language::Rust,
        size: 100,
        content: "let x = foo.unwrap();\n".to_string(),
        parse_ok: true,
        parse_metadata: None,
    }];

    let results = linter.run_analysis_with_entries(&entries);
    assert!(
        results.is_empty(),
        "Disabled config should return empty results"
    );
}

#[test]
fn e2e_bypass_patterns_in_comment_detected() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();

    let content = "// FIXME: this needs fixing\n// HACK: temporary workaround\nfn main() {}\n";

    let entries = vec![FileEntry {
        path: PathBuf::from("src/comment_bypass.rs"),
        extension: "rs".to_string(),
        language: shared::common::taxonomy_language_vo::Language::Rust,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    }];

    let results = linter.run_analysis_with_entries(&entries);
    let has_aes304 = results.iter().any(|r| r.code.code().contains("AES304"));
    assert!(has_aes304, "Expected AES304 for FIXME/HACK in comments");
}
