// PURPOSE: Acceptance test for AES403 — capability routing (protocol/port imports).
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultCode;
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn checker() -> CapabilitiesRoleChecker {
    CapabilitiesRoleChecker::new()
}

fn make_source(file: &str, content: &str) -> SourceContentVO {
    let fp = shared::common::taxonomy_path_vo::FilePath::new(file.to_string()).unwrap();
    let cs = ContentString::new(content.to_string());
    SourceContentVO::new(fp, cs, "rust")
}

// ─── Acceptance: Capabilities with protocol import passes ──

#[test]
fn acceptance_aes403_capabilities_imports_pass() {
    // FRD requirement: Capability files must import from contract layer protocols/ports
    let content = r#"
use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;

pub struct MyCapability;

impl IAgentRoleChecker for MyCapability {
    fn check_container(&self, _s: &str, _v: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>) {}
}
"#;
    let source = make_source("capabilities_my_capability.rs", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert!(violations.is_empty(), "AES403: Capabilities with protocol imports should pass");
}

// ─── Acceptance: Capabilities without protocol import flagged ──

#[test]
fn acceptance_aes403_capabilities_no_import_flagged() {
    // FRD requirement: Capability files without protocol imports should be flagged
    let content = r#"
pub struct StandaloneChecker;

impl StandaloneChecker {
    pub fn do_work(&self) {}
}
"#;
    let source = make_source("capabilities_standalone.rs", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES403");
}

// ─── Acceptance: Multi-language capability routing ──

#[test]
fn acceptance_aes403_python_capability_passes() {
    // FRD requirement: Python capabilities must import from shared contracts
    let content = r#"
from shared.role_rules.contract_agent_role_protocol import IAgentRoleChecker

class MyCapability(IAgentRoleChecker):
    def check_container(self, source, violations):
        pass
"#;
    let source = make_source("capabilities_my_capability.py", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert!(violations.is_empty(), "AES403: Python capabilities with imports should pass");
}

#[test]
fn acceptance_aes403_typescript_capability_passes() {
    // FRD requirement: TypeScript capabilities must import from shared contracts
    let content = r#"
import { IAgentRoleChecker } from 'shared/role_rules/contract_agent_role_protocol';

export class MyCapability implements IAgentRoleChecker {
    checkContainer(source: string, violations: any[]) {}
}
"#;
    let source = make_source("capabilities_my_capability.ts", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert!(violations.is_empty(), "AES403: TypeScript capabilities with imports should pass");
}
