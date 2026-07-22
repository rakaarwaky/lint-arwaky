// PURPOSE: Unit tests for TaxonomyRoleChecker (AES401) — entity, error, event, constant checks.
// Layer: Capabilities (TaxonomyRoleChecker)

use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

fn checker() -> TaxonomyRoleChecker {
    TaxonomyRoleChecker::new()
}

// ─── Default trait ──

#[test]
fn default_creates_valid_instance() {
    let _ = TaxonomyRoleChecker::default();
}

// ─── check_vo: Returns empty Vec via trait ──

#[test]
fn check_vo_returns_empty() {
    let checker: &dyn ITaxonomyRoleChecker = &checker();
    let violations = checker.check_vo();
    assert!(violations.is_empty());
}
