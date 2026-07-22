// PURPOSE: Acceptance test for AES405 — agent layer size/type annotations.
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultCode;
use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn checker() -> AgentRoleChecker {
    AgentRoleChecker::new()
}

fn make_source(file: &str, content: &str) -> SourceContentVO {
    let fp = shared::common::taxonomy_path_vo::FilePath::new(file.to_string()).unwrap();
    let cs = ContentString::new(content.to_string());
    SourceContentVO::new(fp, cs, "rust")
}

// ─── Acceptance: Agent files with proper annotations pass ──

#[test]
fn acceptance_aes405_agent_with_type_annotations_passes() {
    // FRD requirement: Agent files should have type annotations (no Any)
    let content = r#"
pub struct MyAgent;

impl MyAgent {
    pub fn do_work(&self, input: &str) -> String {
        input.to_string()
    }
}
"#;
    let source = make_source("agent_my_agent.rs", content);
    let mut violations = Vec::new();
    checker().check_any_type_annotation(&source, &mut violations);
    assert!(violations.is_empty(), "AES405: Agent with type annotations should pass");
}

// ─── Acceptance: Agent files with Any annotation flagged ──

#[test]
fn acceptance_aes405_agent_with_any_flagged() {
    // FRD requirement: Agent files should not use Any types
    let content = r#"
pub struct MyAgent;

impl MyAgent {
    pub fn do_work(&self, input: &dyn std::any::Any) -> &dyn std::any::Any {
        input
    }
}
"#;
    let source = make_source("agent_any_agent.rs", content);
    let mut violations = Vec::new();
    checker().check_any_type_annotation(&source, &mut violations);
    assert!(violations.len() >= 1, "AES405: Agent with Any should be flagged");
}

// ─── Acceptance: File size limit check ──

#[test]
fn acceptance_aes405_agent_file_size_within_limit() {
    // FRD requirement: Agent files should not exceed size limit (500 lines)
    let content = r#"
pub struct MyAgent;

impl MyAgent {
    pub fn do_work(&self, input: &str) -> String {
        input.to_string()
    }
}
"#;
    let source = make_source("agent_small_agent.rs", content);
    let mut violations = Vec::new();
    checker().check_file_size_limit(&source, &mut violations);
    assert!(violations.is_empty(), "AES405: Small agent files should pass");
}

#[test]
fn acceptance_aes405_large_agent_flagged() {
    // FRD requirement: Agent files exceeding size limit should be flagged
    let mut content = String::from("pub struct MyAgent;\n");
    for i in 0..600 {
        content.push_str(&format!("    pub fn method_{:03}(&self) {{}}\n", i));
    }
    let source = make_source("agent_large_agent.rs", &content);
    let mut violations = Vec::new();
    checker().check_file_size_limit(&source, &mut violations);
    assert!(violations.len() >= 1, "AES405: Large agent files should be flagged");
}
