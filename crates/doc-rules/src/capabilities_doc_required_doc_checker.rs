// PURPOSE: RequiredDocChecker — AES606 (second half): required-document presence.
//
// The HOW-TO-MAKE-* templates require a set of documents to exist at the
// workspace root. A root that is missing one of them — `PRD.md`, `ROADMAP.md`,
// `README.md`, `AGENTS.md`, `ARCHITECTURE.md`, or `CONTRIBUTING.md` — reports
// one finding per missing document.
//
// The check is **root-gated**: it fires only when the audit root is a real
// workspace root (detected by the presence of `Cargo.toml`, `pyproject.toml`,
// or `package.json` at that path). A sub-directory audit (e.g. `docs crates/`)
// is self-contained and does not owe the root document set, so the check stays
// silent there.
use shared_doc_rules::contract_doc_protocol::IRequiredDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES606 required-document auditor: every root document the template
/// mandates must exist at a real workspace root.
pub struct RequiredDocChecker {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IRequiredDocProtocol for RequiredDocChecker {
    /// Report one finding per required root document that does not exist.
    fn audit_required_docs(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        // Root-gate: only fire when the audit root is a real workspace root.
        if !context.is_workspace_root() {
            return Vec::new();
        }
        // Find which required docs are absent from the collected document set.
        let present: std::collections::BTreeSet<&str> = context
            .documents()
            .iter()
            .map(|d| d.path.file_name().and_then(|n| n.to_str()).unwrap_or(""))
            .collect();
        let mut findings = Vec::new();
        for doc in consts::REQUIRED_ROOT_DOCS.iter() {
            if !present.contains(doc) {
                let t = template_name(doc);
                findings.push(
                    DocFinding::new_with_line(
                        doc.to_string(),
                        0,
                        consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                        consts::MISSING_DOC_VIOLATION,
                        format!("`{doc}` is required at the workspace root but does not exist."),
                    )
                    .with_reason(
                        format!(
                            "The HOW-TO-MAKE-{t} template requires this document; a missing document is a gap the reader cannot bridge."
                        ),
                        format!(
                            "Create `{doc}` at the workspace root following the HOW-TO-MAKE-{t} template."
                        ),
                    ),
                );
            }
        }
        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for RequiredDocChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl RequiredDocChecker {
    pub fn new() -> Self {
        Self {}
    }
}

/// Strip the `.md` extension so `PRD.md` → `PRD` for the template name.
fn template_name(doc: &str) -> &str {
    doc.strip_suffix(".md").unwrap_or(doc)
}
