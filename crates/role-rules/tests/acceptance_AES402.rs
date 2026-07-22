// PURPOSE: Acceptance test for AES402 — protocol traits and port types.
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultCode, Severity};
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

// ─── Acceptance: Protocol traits pass in contract layer ──

#[test]
fn acceptance_aes402_protocol_trait_passes() {
    // FRD requirement: Contract layer traits (protocols) should not be flagged
    let content = r#"
pub trait MyProtocol {
    fn do_work(&self) -> String;
}
"#;
    let source = make_source("contract_my_protocol.rs", content);
    let mut violations = Vec::new();
    checker().check_protocol(&source, &mut violations);
    assert!(violations.is_empty(), "AES402: Protocol traits should pass");
}

// ─── Acceptance: Structs in contract layer flagged ──

#[test]
fn acceptance_aes402_struct_in_contract_flagged() {
    // FRD requirement: If structs appear in contract layer, flag them
    let content = r#"
pub struct MyContractStruct;
"#;
    let source = make_source("contract_wrong_type.rs", content);
    let mut violations = Vec::new();
    checker().check_protocol(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES402");
}

// ─── Acceptance: Port type (trait alias) passes ──

#[test]
fn acceptance_aes402_port_trait_passes() {
    // FRD requirement: Port types (trait objects, trait aliases) should pass
    let content = r#"
pub trait MyPort {
    fn handle(&self, data: &str);
}

impl MyPort for MyImpl {
    fn handle(&self, _data: &str) {}
}
"#;
    let source = make_source("contract_my_port.rs", content);
    let mut violations = Vec::new();
    checker().check_port(&source, &mut violations);
    assert!(violations.is_empty(), "AES402: Port traits should pass");
}
