// PURPOSE: Acceptance test for AES401 — entity structure/enamed types in taxonomy layer.
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;

// ─── Acceptance: Default trait ──

#[test]
fn acceptance_aes401_default_taxonomy_checker() {
    let _ = TaxonomyRoleChecker::default();
}
