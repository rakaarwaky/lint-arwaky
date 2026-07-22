// PURPOSE: Unit tests for UtilityOrphanAnalyzer — import pattern detection
// TDD: Test that cross-crate imports are detected correctly

use orphan_detector_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IUtilityOrphanProtocol;

fn make_utility_analyzer() -> UtilityOrphanAnalyzer {
    UtilityOrphanAnalyzer::new()
}

#[test]
fn test_utility_imported_by_cross_crate_use_statement_should_not_flag_aes504() {
    // RED: This test should FAIL because current implementation
    // doesn't detect `use shared::code_analysis::utility_target_resolver::detect_source_dir`
    let utility_path = "shared/src/code-analysis/utility_target.rs";
    let consumer_path = "code-analysis/src/agent_code_analysis_orchestrator.rs";

    // Simulate import graph with cross-crate import
    let mut inbound_links = InboundLinkMap::new(std::collections::HashMap::new());
    inbound_links
        .mapping
        .insert(utility_path.to_string(), vec![consumer_path.to_string()]);

    let all_files = vec![utility_path.to_string(), consumer_path.to_string()];

    let analyzer = make_utility_analyzer();
    let result = analyzer.is_utility_orphan(
        &FilePath::new(utility_path.to_string()).unwrap(),
        &FilePath::new(".".to_string()).unwrap(),
        &all_files,
        &inbound_links,
    );

    // Should NOT be flagged as orphan because it's imported by a consumer layer
    assert!(
        !result.is_orphan,
        "Utility imported by cross-crate use statement should not be flagged as AES504"
    );
}

#[test]
fn test_utility_imported_by_nested_use_path_should_not_flag_aes504() {
    // Test that `use shared::code_analysis::utility_target` is detected
    let content = r#"
use shared::code_analysis::utility_target_resolver::detect_source_dir;
use shared::code_analysis::utility_target_resolver::collect_source_files;

pub fn some_function() {
    let dir = detect_source_dir(std::path::Path::new("."));
    let files = collect_source_files(&dir, &Default::default(), &[]);
}
"#;

    let analyzer = make_utility_analyzer();
    let result = analyzer.check_import_pattern(content, "utility_target");

    assert!(
        result,
        "Nested use path like `use shared::code_analysis::utility_target` should be detected"
    );
}

#[test]
fn test_utility_imported_by_crate_path_should_not_flag_aes504() {
    // Test that `use crate::code_analysis::utility_target` is detected
    let content = r#"
use crate::code_analysis::utility_target_resolver::detect_source_dir;

pub fn some_function() {
    let dir = detect_source_dir(std::path::Path::new("."));
}
"#;

    let analyzer = make_utility_analyzer();
    let result = analyzer.check_import_pattern(content, "utility_target");

    assert!(
        result,
        "Crate path like `use crate::code_analysis::utility_target` should be detected"
    );
}

#[test]
fn test_utility_not_imported_should_flag_aes504() {
    // Test that utility with no importers is flagged
    let content = r#"
pub fn some_function() {
    println!("hello");
}
"#;

    let analyzer = make_utility_analyzer();
    let result = analyzer.check_import_pattern(content, "utility_target");

    assert!(
        !result,
        "Utility not imported should not match import pattern"
    );
}
