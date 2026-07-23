// PURPOSE: Acceptance test for AES401 — entity structure/enamed types in taxonomy layer.
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

fn checker() -> TaxonomyRoleChecker {
    TaxonomyRoleChecker::new()
}

// ─── Acceptance: check_vo returns violations (no-op method) ──

#[test]
fn acceptance_aes401_check_vo_returns_empty() {
    // FRD requirement: check_vo is a no-op that returns empty Vec
    let checker: &dyn ITaxonomyRoleChecker = &checker();
    let violations = checker.check_vo();
    assert!(
        violations.is_empty(),
        "AES401: check_vo should return empty"
    );
}

// ─── Acceptance: Default trait ──

#[test]
fn acceptance_aes401_default_taxonomy_checker() {
    let _ = TaxonomyRoleChecker::default();
}
