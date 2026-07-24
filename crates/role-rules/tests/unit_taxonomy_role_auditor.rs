// PURPOSE: Unit tests for TaxonomyRoleChecker (AES401) — entity, error, event, constant checks.
// Layer: Capabilities (TaxonomyRoleChecker)

use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;

// ─── Default trait ──

#[test]
fn default_creates_valid_instance() {
    let _ = TaxonomyRoleChecker::default();
}
