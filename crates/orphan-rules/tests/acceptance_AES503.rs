// Acceptance tests — AES503: Capabilities orphan detection.
#[path = "mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::mock_filesystem;
use orphan_rules_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::ICapabilitiesOrphanProtocol;
use shared::quality_rules::taxonomy_analysis_vo::ReachabilityResult;
use std::collections::HashSet;

fn capabilities_analyzer() -> CapabilitiesOrphanAnalyzer {
    CapabilitiesOrphanAnalyzer::new(mock_filesystem())
}

#[test]
fn aes503_reachable_file_is_not_orphan() {
    let analyzer = capabilities_analyzer();
    let fp = FilePath::new("crates/orphan-rules/src/capabilities_foo.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let mut reachable_set = HashSet::new();
    reachable_set.insert(fp.clone());
    let alive = ReachabilityResult::new(reachable_set);

    let result = analyzer.is_capabilities_orphan(&fp, &root, &alive);
    assert!(
        !result.is_orphan,
        "Reachable capabilities file should NOT be orphan"
    );
}

#[test]
fn aes503_unreachable_file_is_orphan() {
    let analyzer = capabilities_analyzer();
    let fp = FilePath::new("crates/orphan-rules/src/capabilities_foo.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let alive = ReachabilityResult::new(HashSet::new());

    let result = analyzer.is_capabilities_orphan(&fp, &root, &alive);
    // With mock filesystem that doesn't read files or check wiring, this should be orphan
    assert!(
        result.is_orphan,
        "Unreachable capabilities file should be orphan"
    );
    assert_eq!(result.severity, Severity::MEDIUM);
    assert!(!result.reason.is_empty());
}

#[test]
fn aes503_unreachable_file_reason_mentions_not_wired() {
    let analyzer = capabilities_analyzer();
    let fp = FilePath::new("crates/orphan-rules/src/capabilities_bar.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let alive = ReachabilityResult::new(HashSet::new());

    let result = analyzer.is_capabilities_orphan(&fp, &root, &alive);
    assert!(result.is_orphan);
    // The reason should mention that the struct/trait is not wired
    assert!(
        result.reason.contains("not wired") || result.reason.contains("Not reachable"),
        "Reason should mention wiring or reachability: {}",
        result.reason
    );
}

#[test]
fn aes503_multiple_files_one_reachable() {
    let analyzer = capabilities_analyzer();
    let root = FilePath::new(".".to_string()).unwrap();

    let fp_reachable =
        FilePath::new("crates/orphan-rules/src/capabilities_handler.rs".to_string()).unwrap();
    let fp_orphan =
        FilePath::new("crates/orphan-rules/src/capabilities_legacy.rs".to_string()).unwrap();

    let mut reachable_set = HashSet::new();
    reachable_set.insert(fp_reachable.clone());
    let alive = ReachabilityResult::new(reachable_set);

    let result_reachable = analyzer.is_capabilities_orphan(&fp_reachable, &root, &alive);
    let result_orphan = analyzer.is_capabilities_orphan(&fp_orphan, &root, &alive);

    assert!(
        !result_reachable.is_orphan,
        "Reachable should not be orphan"
    );
    assert!(result_orphan.is_orphan, "Unreachable should be orphan");
}

#[test]
fn aes503_capabilities_violation_display_message() {
    use shared::orphan_rules::AesOrphanViolation;
    let violation = AesOrphanViolation::CapabilitiesOrphan {
        stem: "capabilities_handler".to_string(),
        reason: Some(shared::common::taxonomy_message_vo::LintMessage::new(
            "Not wired in container.".to_string(),
        )),
    };
    let msg = violation.to_string();
    assert!(msg.contains("AES503"));
    assert!(msg.contains("capabilities_handler"));
    assert!(msg.contains("not wired"));
}
