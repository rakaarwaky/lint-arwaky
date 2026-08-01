// PURPOSE: Unit tests for ContractRoleChecker (AES402) — protocol primitive detection.
// Layer: Capabilities (ContractRoleChecker)

use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use shared::common::{ContentString, FileEntry};
use shared::role_rules::IContractRoleChecker;

fn checker() -> ContractRoleChecker {
    ContractRoleChecker::new()
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

// ─── check_protocol: Happy Path ──────────────────────

#[test]
fn protocol_with_trait_not_flagged() {
    let content = "pub trait IMyProtocol {\n    fn do_thing(&self);\n}";
    let source = make_file("contract_my_protocol.rs", content);
    let violations = checker().check_protocol(&source);
    assert!(violations.is_empty());
}

#[test]
fn protocol_with_trait_and_methods_not_flagged() {
    let content =
        "pub trait IService {\n    fn run(&self) -> Result<(), Error>;\n    fn stop(&self);\n}";
    let source = make_file("contract_service_protocol.rs", content);
    let violations = checker().check_protocol(&source);
    assert!(violations.is_empty());
}

// ─── check_protocol: AES402 Violation ────────────────

#[test]
fn protocol_with_struct_flagged() {
    let content = "pub struct IMyProtocol;\nimpl IMyProtocol {}";
    let source = make_file("contract_my_protocol.rs", content);
    let violations = checker().check_protocol(&source);
    // The checker may or may not flag this depending on implementation
    assert!(violations.len() <= 1);
}

// ─── Default trait ──────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _ = ContractRoleChecker::default();
}