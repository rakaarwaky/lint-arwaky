// PURPOSE: Acceptance test for AES403 — capability routing (protocol/port imports).
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;

// ─── Acceptance: Default trait ──

#[test]
fn acceptance_aes403_default_capabilities_checker() {
    let _ = CapabilitiesRoleChecker::default();
}
