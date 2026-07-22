// PURPOSE: Unit tests for AgentOrphanAnalyzer — AES505 agent orphan detection.
// Layer: Capabilities (AgentOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IAgentOrphanProtocol;

fn analyzer() -> AgentOrphanAnalyzer {
    AgentOrphanAnalyzer::new()
}

// ─── Happy path: agent aggregate called by container ──────

#[test]
fn agent_with_aggregate_called_by_container_is_not_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();

    let agent_path = dir.path().join("agent_orphan_orchestrator.rs");
    std::fs::write(
        &agent_path,
        "impl IOrphanAggregate for ArchOrphanAnalyzer {\n    fn check_orphans(&self) {}\n}\n",
    )
    .unwrap();

    let container_path = dir.path().join("root_orphan_detector_container.rs");
    std::fs::write(
        &container_path,
        "use crate::agent_orphan_orchestrator::ArchOrphanAnalyzer;\nlet x = IOrphanAggregate;\n",
    )
    .unwrap();

    let f = FilePath::new(agent_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![
        agent_path.to_str().unwrap().to_string(),
        container_path.to_str().unwrap().to_string(),
    ];

    let result = a.is_agent_orphan(&f, &root, &all_files);
    assert!(!result.is_orphan);
}

// ─── Orphan: agent aggregate not called ───────────────────

#[test]
fn agent_with_aggregate_not_called_is_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();

    let agent_path = dir.path().join("agent_dead_orchestrator.rs");
    std::fs::write(
        &agent_path,
        "impl IDeadAggregate for DeadOrchestrator {\n    fn run(&self) {}\n}\n",
    )
    .unwrap();

    let other_path = dir.path().join("capabilities_foo.rs");
    std::fs::write(&other_path, "pub struct Foo;\n").unwrap();

    let f = FilePath::new(agent_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![
        agent_path.to_str().unwrap().to_string(),
        other_path.to_str().unwrap().to_string(),
    ];

    let result = a.is_agent_orphan(&f, &root, &all_files);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::HIGH);
    assert!(result.reason.contains("IDeadAggregate"));
}

// ─── No aggregate traits → not orphan ─────────────────────

#[test]
fn agent_without_aggregate_traits_is_not_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();

    let agent_path = dir.path().join("agent_simple_orchestrator.rs");
    std::fs::write(
        &agent_path,
        "pub struct SimpleOrch;\nimpl SimpleOrch { pub fn new() -> Self { Self } }\n",
    )
    .unwrap();

    let f = FilePath::new(agent_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![agent_path.to_str().unwrap().to_string()];

    let result = a.is_agent_orphan(&f, &root, &all_files);
    assert!(!result.is_orphan);
}

// ─── Empty file → not orphan ──────────────────────────────

#[test]
fn empty_agent_file_is_not_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();
    let agent_path = dir.path().join("agent_empty_orchestrator.rs");
    std::fs::write(&agent_path, "").unwrap();

    let f = FilePath::new(agent_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![agent_path.to_str().unwrap().to_string()];

    let result = a.is_agent_orphan(&f, &root, &all_files);
    assert!(!result.is_orphan);
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = AgentOrphanAnalyzer::default();
}
