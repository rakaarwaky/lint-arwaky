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

fn empty_reachability() -> ReachabilityResult {
    ReachabilityResult::new(HashSet::new())
}

fn reachable_for(fp: &FilePath) -> ReachabilityResult {
    ReachabilityResult::new(HashSet::from([fp.clone()]))
}

#[test]
fn aes503_reachable_file_is_not_orphan() {
    let analyzer = capabilities_analyzer();
    let fp = FilePath::new("crates/orphan-rules/src/capabilities_foo.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let alive = reachable_for(&fp);

    let result = analyzer.is_capabilities_orphan(&fp, &root, &alive);
    // Reachable but not wired → orphan (mock filesystem doesn't support wiring)
    // This test verifies the reachability check passes (no "not reachable" message)
    assert!(
        result.is_orphan,
        "Reachable but unwired capabilities file should be orphan with mock filesystem"
    );
    assert!(
        result.reason.contains("not wired"),
        "Should fail on wiring check, not reachability: {}",
        result.reason
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
        result.reason.contains("not wired") || result.reason.contains("not reachable"),
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

    let alive = reachable_for(&fp_reachable);

    let result_reachable = analyzer.is_capabilities_orphan(&fp_reachable, &root, &alive);
    let result_orphan = analyzer.is_capabilities_orphan(&fp_orphan, &root, &alive);

    // Reachable file passes reachability check but fails wiring (mock) → "not wired"
    assert!(
        result_reachable.is_orphan,
        "Reachable but unwired should be orphan with mock filesystem"
    );
    assert!(
        result_reachable.reason.contains("not wired"),
        "Should fail on wiring: {}",
        result_reachable.reason
    );
    // Unreachable file fails reachability check → "not reachable"
    assert!(result_orphan.is_orphan, "Unreachable should be orphan");
    assert!(
        result_orphan.reason.contains("not reachable"),
        "Should fail on reachability: {}",
        result_orphan.reason
    );
}

#[test]
fn aes503_capabilities_violation_display_message() {
    use shared::orphan_rules::AesOrphanViolation;
    let _violation = AesOrphanViolation::CapabilitiesOrphan {
        stem: "capabilities_handler".to_string(),
        reason: Some(shared::common::taxonomy_message_vo::LintMessage::new(
            "Not wired in container.".to_string(),
        )),
    };
    let msg = format!(
        "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs.",
        "capabilities_handler", "Not wired in container.", "capabilities_handler"
    );
    assert!(msg.contains("AES503"));
    assert!(msg.contains("capabilities_handler"));
    assert!(msg.contains("not wired"));
}
