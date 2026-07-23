// PURPOSE: Acceptance test for AES404 — agent layer orchestrator delegation.
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::agent_role_orchestrator::RoleOrchestrator;

// ─── Acceptance: Orchestrator is Send + Sync ──

#[test]
fn acceptance_aes404_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RoleOrchestrator>();
}
