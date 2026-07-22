// PURPOSE: Unit tests for ContractRoleChecker (AES402) — port/protocol primitive detection.
// Layer: Capabilities (ContractRoleChecker)

use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::role_rules::contract_role_protocol::IContractRoleChecker;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn checker() -> ContractRoleChecker {
    ContractRoleChecker::new()
}

fn make_source(file: &str, content: &str) -> SourceContentVO {
    let fp = shared::common::taxonomy_path_vo::FilePath::new(file.to_string()).unwrap();
    let cs = ContentString::new(content.to_string());
    SourceContentVO::new(fp, cs, "rust")
}

// ─── check_protocol: Happy Path ──────────────────────

#[test]
fn protocol_with_trait_not_flagged() {
    let content = "pub trait IMyProtocol {\n    fn do_thing(&self);\n}";
    let source = make_source("contract_my_protocol.rs", content);
    let violations = checker().check_protocol(&source);
    assert!(violations.is_empty());
}

#[test]
fn protocol_with_trait_and_methods_not_flagged() {
    let content = "pub trait IService {\n    fn run(&self) -> Result<(), Error>;\n    fn stop(&self);\n}";
    let source = make_source("contract_service_protocol.rs", content);
    let violations = checker().check_protocol(&source);
    assert!(violations.is_empty());
}

// ─── check_protocol: AES402 Violation ────────────────

#[test]
fn protocol_with_struct_flagged() {
    let content = "pub struct IMyProtocol;\nimpl IMyProtocol {}";
    let source = make_source("contract_my_protocol.rs", content);
    let violations = checker().check_protocol(&source);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES402");
}

// ─── check_port: Happy Path ─────────────────────────

#[test]
fn port_with_trait_not_flagged() {
    let content = "pub trait IMyPort {\n    fn handle(&self, data: &str);\n}";
    let source = make_source("contract_my_port.rs", content);
    let violations = checker().check_port(&source);
    assert!(violations.is_empty());
}

// ─── check_port: AES402 Violation ───────────────────

#[test]
fn port_with_struct_flagged() {
    let content = "pub struct DataPort;\nimpl DataPort {}";
    let source = make_source("contract_data_port.rs", content);
    let violations = checker().check_port(&source);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES402");
}

// ─── Default trait ──────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _ = ContractRoleChecker::default();
}
