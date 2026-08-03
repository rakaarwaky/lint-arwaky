// Acceptance tests — AES505: Agent orphan detection.
use orphan_rules_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::IAgentOrphanProtocol;
use std::collections::HashMap;

fn agent_analyzer() -> AgentOrphanAnalyzer {
    AgentOrphanAnalyzer::new()
}

#[test]
fn aes505_agent_with_aggregate_trait_not_used_by_surface_is_orphan() {
    let analyzer = agent_analyzer();
    let fp =
        FilePath::new("crates/orphan-rules/src/agent_foo_orchestrator.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    // File with an Aggregate trait impl that's not referenced by any surface
    content_map.insert(
        fp.value().to_string(),
        "impl IFooAggregate for FooOrchestrator {\n    fn run(&self) {}\n}".to_string(),
    );
    // No surface or container files reference the aggregate
    content_map.insert(
        "crates/shared/src/unrelated.rs".to_string(),
        "fn unrelated() {}".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/shared/src/unrelated.rs".to_string(),
    ];

    let result = analyzer.is_agent_orphan(&fp, &root, &all_files, &content_map);
    assert!(
        result.is_orphan,
        "Agent file with unreferenced aggregate should be orphan"
    );
    assert_eq!(result.severity, Severity::HIGH);
    assert!(result.reason.contains("IFooAggregate"));
}

#[test]
fn aes505_agent_with_aggregate_used_by_surface_is_not_orphan() {
    let analyzer = agent_analyzer();
    let fp =
        FilePath::new("crates/orphan-rules/src/agent_foo_orchestrator.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "impl IFooAggregate for FooOrchestrator {\n    fn run(&self) {}\n}".to_string(),
    );
    // Surface file references IFooAggregate
    content_map.insert(
        "crates/tui/src/surface_main_screen.rs".to_string(),
        "use agent_foo_orchestrator::IFooAggregate;".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/tui/src/surface_main_screen.rs".to_string(),
    ];

    let result = analyzer.is_agent_orphan(&fp, &root, &all_files, &content_map);
    assert!(
        !result.is_orphan,
        "Agent aggregate used by surface should NOT be orphan"
    );
}

#[test]
fn aes505_agent_with_aggregate_used_by_container_is_not_orphan() {
    let analyzer = agent_analyzer();
    let fp =
        FilePath::new("crates/orphan-rules/src/agent_bar_orchestrator.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "impl IBarAggregate for BarOrchestrator {\n    fn run(&self) {}\n}".to_string(),
    );
    // Container file references IBarAggregate
    content_map.insert(
        "crates/cli/src/root_cli_container.rs".to_string(),
        "use agent_bar_orchestrator::IBarAggregate;".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/cli/src/root_cli_container.rs".to_string(),
    ];

    let result = analyzer.is_agent_orphan(&fp, &root, &all_files, &content_map);
    assert!(
        !result.is_orphan,
        "Agent aggregate used by container should NOT be orphan"
    );
}

#[test]
fn aes505_agent_with_aggregate_used_by_main_is_not_orphan() {
    let analyzer = agent_analyzer();
    let fp =
        FilePath::new("crates/orphan-rules/src/agent_baz_orchestrator.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "impl IBazAggregate for BazOrchestrator {\n    fn run(&self) {}\n}".to_string(),
    );
    // main.rs references IBazAggregate
    content_map.insert(
        "crates/cli/src/main.rs".to_string(),
        "use agent_baz_orchestrator::IBazAggregate;".to_string(),
    );
    let all_files = vec![fp.value().to_string(), "crates/cli/src/main.rs".to_string()];

    let result = analyzer.is_agent_orphan(&fp, &root, &all_files, &content_map);
    assert!(
        !result.is_orphan,
        "Agent aggregate used by main.rs should NOT be orphan"
    );
}

#[test]
fn aes505_empty_content_is_not_orphan() {
    let analyzer = agent_analyzer();
    let fp =
        FilePath::new("crates/orphan-rules/src/agent_foo_orchestrator.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let content_map: HashMap<String, String> = HashMap::new();

    let result = analyzer.is_agent_orphan(&fp, &root, &[], &content_map);
    assert!(!result.is_orphan, "Empty content should not be flagged");
}

#[test]
fn aes505_no_aggregate_traits_is_not_orphan() {
    let analyzer = agent_analyzer();
    let fp =
        FilePath::new("crates/orphan-rules/src/agent_foo_orchestrator.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    // Regular functions, no aggregate trait impls
    content_map.insert(
        fp.value().to_string(),
        "fn helper() -> i32 { 42 }\nfn run() { helper(); }".to_string(),
    );
    let all_files = vec![fp.value().to_string()];

    let result = analyzer.is_agent_orphan(&fp, &root, &all_files, &content_map);
    assert!(
        !result.is_orphan,
        "File without aggregate traits should not be orphan"
    );
}

#[test]
fn aes505_agent_violation_display_message() {
    use shared::orphan_rules::AesOrphanViolation;
    let violation = AesOrphanViolation::AgentOrphan {
        agg_name: "IFooAggregate".to_string(),
        reason: Some(shared::common::taxonomy_message_vo::LintMessage::new(
            "Agent file aggregate trait is not used by any surface, container, entry, or main file.".to_string(),
        )),
    };
    let msg = violation.to_string();
    assert!(msg.contains("AES505"));
    assert!(msg.contains("IFooAggregate"));
    assert!(msg.contains("unreachable"));
}
