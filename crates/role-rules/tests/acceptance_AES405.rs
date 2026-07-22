// PURPOSE: Acceptance test for AES405 — agent layer size/type annotations.
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;

fn checker() -> AgentRoleChecker {
    AgentRoleChecker::new()
}

// ─── Acceptance: Default trait ──

#[test]
fn acceptance_aes405_default_agent_checker() {
    let _ = AgentRoleChecker::default();
}
