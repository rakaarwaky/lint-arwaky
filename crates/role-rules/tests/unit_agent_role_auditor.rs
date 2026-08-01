// PURPOSE: Unit tests for AgentRoleChecker (AES405) — agent type composition.
// Layer: Capabilities (AgentRoleChecker)

use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use std::path::PathBuf;
use shared::role_rules::IAgentRoleChecker;

fn checker() -> AgentRoleChecker {
    AgentRoleChecker::new()
}

fn make_file(path: &str, content: &str) -> FileEntry {
    let ext = path.rsplit('.').next().unwrap_or("rs").to_string();
    FileEntry {
        path: PathBuf::from(path),
        extension: ext,
        language: Language::Rust,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    }
}

// ─── check_agent_routing: Rust happy path ────────────

#[test]
fn rust_agent_with_aggregate_import_passes() {
    let content = r#"
use shared::role_rules::contract_role_aggregate::IRoleAggregate;

pub struct MyOrchestrator {}

impl IRoleAggregate for MyOrchestrator {
    fn run(&self) {}
}
"#;
    let source = make_file("agent_my_orchestrator.rs", content);
    let mut violations = Vec::new();
    checker().check_agent_routing(&source, "agent", &mut violations);
    assert!(
        violations.is_empty(),
        "Expected no violations, got: {violations:?}"
    );
}

// ─── check_agent_routing: Rust — AgentNoImplementor ──

#[test]
fn rust_agent_no_implementor_flagged() {
    let content = r#"
use shared::role_rules::contract_role_aggregate::IRoleAggregate;

pub struct InternalHelper {}

impl InternalHelper {
    pub fn helper(&self) {}
}
"#;
    let source = make_file("agent_my_orchestrator.rs", content);
    let mut violations = Vec::new();
    checker().check_agent_routing(&source, "agent", &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES405");
}

// ─── check_agent_routing: layer guard ────────────────

#[test]
fn non_agent_layer_is_skipped() {
    let content = "no aggregate import at all";
    let source = make_file("capabilities_foo.rs", content);
    let mut violations = Vec::new();
    checker().check_agent_routing(&source, "capabilities", &mut violations);
    assert!(violations.is_empty());
}

// ─── check_agent_routing: Rust — AgentTooManyTypes ───

#[test]
fn rust_agent_too_many_types_flagged() {
    let content = r#"
use shared::role_rules::contract_role_aggregate::IRoleAggregate;

pub struct TypeA {}
pub struct TypeB {}
pub struct TypeC {}
pub struct TypeD {}

impl IRoleAggregate for TypeA {}
"#;
    let source = make_file("agent_my_orchestrator.rs", content);
    let mut violations = Vec::new();
    checker().check_agent_routing(&source, "agent", &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES405");
}

// ─── Default trait ──────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _ = AgentRoleChecker::default();
}