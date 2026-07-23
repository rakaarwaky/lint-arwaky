// PURPOSE: Acceptance test for AES402 — protocol traits.
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;

// ─── Acceptance: Default trait ──

#[test]
fn acceptance_aes402_default_contract_checker() {
    let _ = ContractRoleChecker::default();
}
