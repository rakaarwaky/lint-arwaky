// PURPOSE: Acceptance test for AES406 — surface layer function count limit.
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;

// ─── Acceptance: Default trait ──

#[test]
fn acceptance_aes406_default_surface_checker() {
    let _ = SurfaceRoleChecker::default();
}
