// PURPOSE: Acceptance test — AES505 Agent Orphan Checker.
// Requirement: Agent orchestrator files must be called by surface layer files or binary entry points.

use orphan_detector_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IAgentOrphanProtocol;
use std::fs;

/// AES505: Agent aggregate called by a container is NOT orphan.
#[test]
fn aes505_agent_called_by_container_not_orphan() {
    let a = AgentOrphanAnalyzer::new();
    let dir = tempfile::tempdir().unwrap();

    let agent = dir.path().join("agent_orphan_orchestrator.rs");
    fs::write(
        &agent,
        "impl IOrphanAggregate for ArchOrphanAnalyzer {\n    fn check_orphans(&self) {}\n}\n",
    )
    .unwrap();

    let container = dir.path().join("root_orphan_detector_container.rs");
    fs::write(
        &container,
        "use IOrphanAggregate;\nlet x: Arc<dyn IOrphanAggregate> = ...;\n",
    )
    .unwrap();

    let f = FilePath::new(agent.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all = vec![
        agent.to_str().unwrap().to_string(),
        container.to_str().unwrap().to_string(),
    ];

    let result = a.is_agent_orphan(&f, &root, &all);
    assert!(
        !result.is_orphan,
        "AES505 FAIL: agent called by container should not be orphan"
    );
}

/// AES505: Agent aggregate NOT called by any surface/container IS orphan.
#[test]
fn aes505_agent_not_called_is_orphan() {
    let a = AgentOrphanAnalyzer::new();
    let dir = tempfile::tempdir().unwrap();

    let agent = dir.path().join("agent_dead_orchestrator.rs");
    fs::write(
        &agent,
        "impl IDeadAggregate for DeadOrch {\n    fn run(&self) {}\n}\n",
    )
    .unwrap();

    let other = dir.path().join("capabilities_foo.rs");
    fs::write(&other, "pub struct Foo;\n").unwrap();

    let f = FilePath::new(agent.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all = vec![
        agent.to_str().unwrap().to_string(),
        other.to_str().unwrap().to_string(),
    ];

    let result = a.is_agent_orphan(&f, &root, &all);
    assert!(
        result.is_orphan,
        "AES505 FAIL: agent not called by any surface/container must be flagged"
    );
}
