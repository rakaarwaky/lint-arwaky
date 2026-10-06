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
