// Regression tests for resolve_feature_protocol_count on non-Rust layouts.
//
// The Python modules layout puts contracts flat in `modules/shared/src/`
// (named `contract_<feature>_protocol.py`) instead of nesting them under
// `crates/shared/src/<feature>/`. The resolver must recognize that layout so
// a single-subsystem feature (count == 1, injected == 1) is not flagged.

use shared_role_rules::utility_agent_role_checker::resolve_feature_protocol_count;
use std::fs;
use std::path::Path;

fn make_modules_layout(root: &Path) {
    // modules/shared/src/contract_mcp_protocol.py
    let shared = root.join("modules/shared/src");
    fs::create_dir_all(&shared).unwrap();
    fs::write(
        shared.join("contract_mcp_protocol.py"),
        "class IMcpProtocol:\n    pass\n",
    )
    .unwrap();
    // modules/mcp/src/agent_mcp_orchestrator.py
    let mcp_src = root.join("modules/mcp/src");
    fs::create_dir_all(&mcp_src).unwrap();
    fs::write(
        mcp_src.join("agent_mcp_orchestrator.py"),
        "class McpOrchestrator:\n    def __init__(self, generator: IMcpProtocol):\n        pass\n",
    )
    .unwrap();
}

#[test]
fn python_modules_layout_resolves_protocol_count() {
    let tmp = tempfile::TempDir::new().unwrap();
    make_modules_layout(tmp.path());
    let agent = tmp.path().join("modules/mcp/src/agent_mcp_orchestrator.py");
    let count = resolve_feature_protocol_count(&agent);
    assert_eq!(
        count, 1,
        "single-subsystem Python feature must resolve to 1, not the unknown-layout sentinel"
    );
}

#[test]
fn python_modules_layout_multi_protocol_resolves() {
    let tmp = tempfile::TempDir::new().unwrap();
    make_modules_layout(tmp.path());
    // A second protocol in the same flat shared dir that the mcp feature does
    // not reference — dead for this feature, so it must not raise the count.
    let shared = tmp.path().join("modules/shared/src");
    fs::write(
        shared.join("contract_other_protocol.py"),
        "class IOtherProtocol:\n    pass\n",
    )
    .unwrap();
    let agent = tmp.path().join("modules/mcp/src/agent_mcp_orchestrator.py");
    assert_eq!(resolve_feature_protocol_count(&agent), 1);
}

#[test]
fn python_modules_layout_split_contract_siblings_resolve() {
    let tmp = tempfile::TempDir::new().unwrap();
    let shared = tmp.path().join("modules/shared/src");
    fs::create_dir_all(&shared).unwrap();
    fs::write(
        shared.join("contract_skill_protocol.py"),
        "class ISkillProvisionProtocol:\n    pass\n\nclass ISkillRegistryProtocol:\n    pass\n",
    )
    .unwrap();
    fs::write(
        shared.join("contract_skill_update_protocol.py"),
        "class ISkillUpdateProtocol:\n    pass\n",
    )
    .unwrap();
    let skill_src = tmp.path().join("modules/skill/src");
    fs::create_dir_all(&skill_src).unwrap();
    fs::write(
        skill_src.join("agent_skill_orchestrator.py"),
        "class SkillOrchestrator:\n    def __init__(self, provision: ISkillProvisionProtocol, registry: ISkillRegistryProtocol, update: ISkillUpdateProtocol):\n        pass\n",
    )
    .unwrap();
    let agent = skill_src.join("agent_skill_orchestrator.py");
    assert_eq!(
        resolve_feature_protocol_count(&agent),
        3,
        "a split-contract layout must count protocols in every sibling file"
    );
}

#[test]
fn python_modules_layout_split_contract_siblings_filter_dead_references() {
    let tmp = tempfile::TempDir::new().unwrap();
    let shared = tmp.path().join("modules/shared/src");
    fs::create_dir_all(&shared).unwrap();
    fs::write(
        shared.join("contract_skill_protocol.py"),
        "class ISkillProvisionProtocol:\n    pass\n",
    )
    .unwrap();
    fs::write(
        shared.join("contract_skill_update_protocol.py"),
        "class ISkillUpdateProtocol:\n    pass\n\nclass ISkillUpdateDeadProtocol:\n    pass\n",
    )
    .unwrap();
    let skill_src = tmp.path().join("modules/skill/src");
    fs::create_dir_all(&skill_src).unwrap();
    // The dead protocol is referenced nowhere in the feature, so only the two
    // live ones count.
    fs::write(
        skill_src.join("agent_skill_orchestrator.py"),
        "class SkillOrchestrator:\n    def __init__(self, provision: ISkillProvisionProtocol, update: ISkillUpdateProtocol):\n        pass\n",
    )
    .unwrap();
    let agent = skill_src.join("agent_skill_orchestrator.py");
    assert_eq!(resolve_feature_protocol_count(&agent), 2);
}

#[test]
fn python_modules_layout_feature_prefix_does_not_overmatch() {
    let tmp = tempfile::TempDir::new().unwrap();
    let shared = tmp.path().join("modules/shared/src");
    fs::create_dir_all(&shared).unwrap();
    fs::write(
        shared.join("contract_skill_protocol.py"),
        "class ISkillProtocol:\n    pass\n",
    )
    .unwrap();
    let skill_update_src = tmp.path().join("modules/skill_update/src");
    fs::create_dir_all(&skill_update_src).unwrap();
    // The `skill_update` feature has no contract file of its own; the
    // `contract_skill_protocol.py` sibling belongs to the `skill` feature and
    // must not be counted for it.
    fs::write(
        skill_update_src.join("agent_skill_update_orchestrator.py"),
        "class SkillUpdateOrchestrator:\n    def __init__(self, skill: ISkillProtocol):\n        pass\n",
    )
    .unwrap();
    let agent = skill_update_src.join("agent_skill_update_orchestrator.py");
    assert_eq!(resolve_feature_protocol_count(&agent), 0);
}

#[test]
fn rust_crates_layout_still_resolves() {
    let tmp = tempfile::TempDir::new().unwrap();
    let shared = tmp.path().join("crates/shared/src/mcp");
    fs::create_dir_all(&shared).unwrap();
    fs::write(shared.join("protocol.rs"), "pub trait IMcpProtocol {}\n").unwrap();
    let agent_src = tmp.path().join("crates/mcp/src");
    fs::create_dir_all(&agent_src).unwrap();
    let agent = agent_src.join("agent_mcp_orchestrator.rs");
    fs::write(&agent, "struct A;\n").unwrap();
    assert_eq!(resolve_feature_protocol_count(&agent), 1);
}

#[test]
fn unknown_layout_returns_sentinel() {
    let tmp = tempfile::TempDir::new().unwrap();
    let agent = tmp.path().join("agent.py");
    fs::write(&agent, "class A:\n    pass\n").unwrap();
    let count = resolve_feature_protocol_count(&agent);
    assert_eq!(
        count,
        usize::MAX,
        "no layout must keep the sentinel so the skip never fires"
    );
}
