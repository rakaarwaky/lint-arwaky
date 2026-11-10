// Acceptance tests — AES506: Surface orphan detection.
use orphan_rules_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_orphan_rules::ISurfacesOrphanProtocol;
use shared_quality_rules::taxonomy_quality_rules_vo::{InboundLinkMap, ReachabilityResult};
use std::collections::{HashMap, HashSet};

fn empty_reachability() -> ReachabilityResult {
    ReachabilityResult::new(HashSet::new())
}

fn reachable_for(fp: &FilePath) -> ReachabilityResult {
    ReachabilityResult::new(HashSet::from([fp.clone()]))
}

fn surfaces_analyzer() -> SurfacesOrphanAnalyzer {
    SurfacesOrphanAnalyzer::new()
}

#[test]
fn aes506_surface_not_reachable_is_orphan() {
    let analyzer = surfaces_analyzer();
    let fp = FilePath::new("crates/tui/src/surface_main_screen.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let alive = empty_reachability();

    let result = analyzer.is_surface_orphan(
        &fp,
        &root,
        &alive,
        &InboundLinkMap::new(HashMap::new()),
        None,
    );
    assert!(
        result.is_orphan,
        "surface not reachable from any entry point should be orphan"
    );
    assert_eq!(result.severity, Severity::HIGH);
    assert_eq!(result.subtype, "SURFACE_ORPHAN");
    assert_eq!(
        result.message,
        "'surface_main_screen' surface is not reachable from any entry point."
    );
    assert!(result.message.chars().count() <= 72);
    assert!(!result.message.starts_with("AES506"));
    assert!(!result.message.contains("WHY:"));
    assert!(!result.message.contains("FIX:"));
    assert!(result.message.ends_with('.'));
    assert!(!result.why.is_empty());
    assert!(!result.fix.is_empty());
}

#[test]
fn aes506_surface_reachable_is_not_orphan() {
    let analyzer = surfaces_analyzer();
    let fp = FilePath::new("crates/tui/src/surface_main_screen.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let alive = reachable_for(&fp);

    let result = analyzer.is_surface_orphan(
        &fp,
        &root,
        &alive,
        &InboundLinkMap::new(HashMap::new()),
        None,
    );
    assert!(
        !result.is_orphan,
        "surface reachable from entry point should NOT be orphan: {}",
        result.message
    );
    assert!(result.subtype.is_empty());
    assert_eq!(result.severity, Severity::LOW);
    assert!(result.message.chars().count() <= 72);
    assert!(result.why.is_empty());
    assert!(result.fix.is_empty());
}

#[test]
fn aes506_surface_python_file_is_orphan_high() {
    let analyzer = surfaces_analyzer();
    let fp = FilePath::new("crates/tui/src/surface_dashboard.py".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let alive = empty_reachability();

    let result = analyzer.is_surface_orphan(
        &fp,
        &root,
        &alive,
        &InboundLinkMap::new(HashMap::new()),
        None,
    );
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::HIGH);
    assert_eq!(result.subtype, "SURFACE_ORPHAN");
    assert_eq!(
        result.message,
        "'surface_dashboard' surface is not reachable from any entry point."
    );
    assert!(result.message.chars().count() <= 72);
    assert!(!result.why.is_empty());
    assert!(!result.fix.is_empty());
}

#[test]
fn aes506_surface_markdown_file_is_orphan_high() {
    let analyzer = surfaces_analyzer();
    let fp = FilePath::new("crates/docs/surface_readme.md".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let alive = empty_reachability();

    let result = analyzer.is_surface_orphan(
        &fp,
        &root,
        &alive,
        &InboundLinkMap::new(HashMap::new()),
        None,
    );
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::HIGH);
    assert_eq!(result.subtype, "SURFACE_ORPHAN");
    assert_eq!(
        result.message,
        "'surface_readme' surface is not reachable from any entry point."
    );
    assert!(result.message.chars().count() <= 72);
    assert!(!result.why.is_empty());
    assert!(!result.fix.is_empty());
}

#[test]
fn aes506_surface_smart_suffix_is_high() {
    let analyzer = surfaces_analyzer();
    let fp = FilePath::new("crates/tui/src/surface_screen_rs.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let alive = empty_reachability();

    let result = analyzer.is_surface_orphan(
        &fp,
        &root,
        &alive,
        &InboundLinkMap::new(HashMap::new()),
        None,
    );
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::HIGH);
    assert_eq!(result.subtype, "SURFACE_ORPHAN");
    assert_eq!(
        result.message,
        "'surface_screen_rs' surface is not reachable from any entry point."
    );
    assert!(result.message.chars().count() <= 72);
    assert!(!result.why.is_empty());
    assert!(!result.fix.is_empty());
}

#[test]
fn aes506_message_is_one_sentence_no_prefix() {
    let analyzer = surfaces_analyzer();
    let fp = FilePath::new("crates/tui/src/surface_main_screen.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let alive = empty_reachability();

    let result = analyzer.is_surface_orphan(
        &fp,
        &root,
        &alive,
        &InboundLinkMap::new(HashMap::new()),
        None,
    );
    assert!(
        !result.message.starts_with("AES506"),
        "message must not have code prefix: {}",
        result.message
    );
    assert!(
        !result.message.contains("WHY:"),
        "message must not contain WHY: prefix: {}",
        result.message
    );
    assert!(
        !result.message.contains("FIX:"),
        "message must not contain FIX: prefix: {}",
        result.message
    );
    assert!(
        result.message.ends_with('.'),
        "message should be a single sentence ending with '.': {}",
        result.message
    );
    assert_eq!(result.subtype, "SURFACE_ORPHAN");
    assert_eq!(result.severity, Severity::HIGH);
    assert_eq!(
        result.message,
        "'surface_main_screen' surface is not reachable from any entry point."
    );
    assert!(result.message.chars().count() <= 72);
    assert!(!result.why.is_empty());
    assert!(!result.fix.is_empty());
}

#[test]
fn aes506_single_finding_count() {
    let analyzer = surfaces_analyzer();
    let fp = FilePath::new("crates/tui/src/surface_main_screen.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let alive = empty_reachability();

    let result = analyzer.is_surface_orphan(
        &fp,
        &root,
        &alive,
        &InboundLinkMap::new(HashMap::new()),
        None,
    );
    assert!(result.is_orphan);
    // Exactly one finding per file check
    assert_eq!(result.subtype, "SURFACE_ORPHAN");
    assert_eq!(result.severity, Severity::HIGH);
    assert_eq!(
        result.message,
        "'surface_main_screen' surface is not reachable from any entry point."
    );
}
