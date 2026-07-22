// PURPOSE: Unit tests for CapabilitiesRoleChecker (AES403) — protocol/port import routing.
// Layer: Capabilities (CapabilitiesRoleChecker)

use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
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

// ─── check_capability_routing: Happy Path (Rust) ────

#[test]
fn rust_capabilities_with_protocol_import_passes() {
    let content = r#"
use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;

pub struct MyChecker;

impl IAgentRoleChecker for MyChecker {
    fn check_container(&self, _s: &str, _v: &mut Vec<LintResult>) {}
}
"#;
    let source = make_source("capabilities_my_checker.rs", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn rust_capabilities_with_port_import_passes() {
    let content = r#"
use shared::role_rules::contract_role_protocol::IContractRoleChecker;

pub struct MyChecker;

impl IContractRoleChecker for MyChecker {}
"#;
    let source = make_source("capabilities_my_checker.rs", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert!(violations.is_empty());
}

// ─── check_capability_routing: AES403 Violation (Rust) ──

#[test]
fn rust_capabilities_without_protocol_import_flagged() {
    let content = r#"
pub struct MyChecker;

impl MyChecker {
    pub fn do_work(&self) {}
}
"#;
    let source = make_source("capabilities_my_checker.rs", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES403");
}

// ─── check_capability_routing: Happy Path (Python) ──

#[test]
fn python_capabilities_with_protocol_import_passes() {
    let content = r#"
from shared.role_rules.contract_agent_role_protocol import IAgentRoleChecker

class MyChecker(IAgentRoleChecker):
    def check_container(self, source, violations):
        pass
"#;
    let source = make_source("capabilities_my_checker.py", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert!(violations.is_empty());
}

// ─── check_capability_routing: AES403 Violation (Python) ──

#[test]
fn python_capabilities_without_protocol_import_flagged() {
    let content = r#"
class MyChecker:
    def do_work(self):
        pass
"#;
    let source = make_source("capabilities_my_checker.py", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES403");
}

// ─── check_capability_routing: Happy Path (JS/TS) ────

#[test]
fn typescript_capabilities_with_protocol_import_passes() {
    let content = r#"
import { IAgentRoleChecker } from 'shared/role_rules/contract_agent_role_protocol';

export class MyChecker implements IAgentRoleChecker {
    checkContainer(source: string, violations: any[]) {}
}
"#;
    let source = make_source("capabilities_my_checker.ts", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert!(violations.is_empty());
}

// ─── check_capability_routing: AES403 Violation (JS/TS) ──

#[test]
fn typescript_capabilities_without_protocol_import_flagged() {
    let content = r#"
export class MyChecker {
    doWork() {}
}
"#;
    let source = make_source("capabilities_my_checker.ts", content);
    let mut violations = Vec::new();
    checker().check_capability_routing(&source, "capabilities", &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES403");
}

// ─── Default trait ──────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _ = CapabilitiesRoleChecker::default();
}
