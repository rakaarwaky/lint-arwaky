use std::collections::HashMap;
// PURPOSE: Acceptance test — FR-008 Agent Orphan Checker (AES505).
// Requirement: Agent orchestrator files must be called by surface layer files or binary entry points.

use orphan_detector_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use shared::common::FilePath;
use shared::orphan_detector::IAgentOrphanProtocol;
use std::fs;

/// AES505: Agent aggregate called by a container is NOT orphan.
fn build_content_map(files: &[String]) -> HashMap<String, String> {
    files
        .iter()
        .filter_map(|f| std::fs::read_to_string(f).ok().map(|c| (f.clone(), c)))
        .collect()
}

#[test]
fn fr008_agent_called_by_container_not_orphan() {
    let a = AgentOrphanAnalyzer::default();
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

    let result = a.is_agent_orphan(&f, &root, &all, &build_content_map(&all));
    assert!(
        !result.is_orphan,
        "AES505 FAIL: agent called by container should not be orphan"
    );
}

/// AES505: Agent aggregate NOT called by any surface/container IS orphan.
#[test]
fn fr008_agent_not_called_is_orphan() {
    let a = AgentOrphanAnalyzer::default();
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

    let result = a.is_agent_orphan(&f, &root, &all, &build_content_map(&all));
    assert!(
        result.is_orphan,
        "AES505 FAIL: agent not called by any surface/container must be flagged"
    );
}
