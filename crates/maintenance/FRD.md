# FRD — maintenance

## Feature Goal
The maintenance crate provides maintenance operations for the lint_arwaky system: dependency updates, security audits, configuration drift detection, and AES rule catalog refresh. It keeps the codebase up-to-date and compliant with standards.

## Requirements & Scope
- In scope:
  - dep-update — update Rust/Python/JS dependencies across the workspace.
  - audit — run security audits using cargo-audit, bandit, or external tools.
  - drift-check — check drift between code and defined AES rules.
  - rules-refresh — update the AES rule catalog from external sources.
- Out of scope:
  - Enforcing AES rule evaluation — this crate reports drift and refreshes catalogs; the rules themselves are checked by the analysis crates.
  - Scaffolding new projects — template/structure generation is a separate concern.

## Success Indicators
- [ ] Update accuracy — dependencies updated with compatible versions.
- [ ] Audit coverage — all vulnerabilities detected and reported.
- [ ] Drift detection — differences between code and rules accurately detected.
- [ ] Rule conformance — the crate's own source complies with AES rules when complete.
