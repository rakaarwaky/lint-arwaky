// PURPOSE: TemplateBodyChecker — AES606: template body and placeholder conformance.
//
// Evaluates recognized documents in memory against registered `TemplateProfile`s:
//   1. Checks for deleted required sections (non-deletable H2 sections absent from the body).
//      Reports at most one finding per document summarizing all missing non-optional sections,
//      anchored to line 0.
//   2. Checks for unfilled placeholder tokens from the profile's `placeholder_catalog`
//      surviving in stripped prose (honoring `effective_strict_placeholders`).
//
// Uses the structured scanner in `utility_markdown_scanner` to strip code fences,
// inline backtick spans, autolinks, and HTML tags before catalog matching.
use shared_doc_rules::contract_doc_protocol::ITemplateBodyProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
use shared_doc_rules::utility_markdown_scanner::{
    matches_placeholder_catalog, normalize_heading, sections,
};

// ─── Block 1: Struct Definition ────────────────────────────

/// AES606 template body and placeholder auditor.
pub struct TemplateBodyChecker;

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ITemplateBodyProtocol for TemplateBodyChecker {
    fn audit_template_body(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let strict_placeholders = context.effective_strict_placeholders();
        let mut findings = Vec::new();

        for document in context.documents() {
            // Documents with zero content (0 bytes) are handled by AES605 (h1_count);
            // skip here to avoid redundant findings.
            if document.text.trim().is_empty() {
                continue;
            }

            let doc_name = context.document_name(document);
            let Some(profile) = consts::TEMPLATE_PROFILES
                .iter()
                .find(|p| p.doc_name == doc_name)
            else {
                continue;
            };

            let mut own = Vec::new();

            // 1. Deleted required sections check:
            // Non-deletable H2 sections must not be omitted entirely.
            let doc_sections = sections(&document.text);
            let present_h2s: Vec<String> = doc_sections
                .iter()
                .filter(|s| s.level == 2)
                .map(|s| normalize_heading(&s.title))
                .collect();

            let missing_sections: Vec<&str> = profile
                .required_h2s
                .iter()
                .copied()
                .filter(|&req_h2| {
                    if profile.deletable_sections.contains(&req_h2) {
                        return false;
                    }
                    let norm_req = normalize_heading(req_h2);
                    !present_h2s
                        .iter()
                        .any(|h| *h == norm_req || h.starts_with(&norm_req))
                })
                .collect();

            if !missing_sections.is_empty() {
                own.push(
                    DocFinding::new_with_line(
                        "",
                        0,
                        consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                        consts::TEMPLATE_BODY_VIOLATION_DELETED_SECTION,
                        format!(
                            "Required non-optional template section(s) missing from `{doc_name}`: {}",
                            missing_sections.join(", ")
                        ),
                    )
                    .with_reason(
                        format!(
                            "The `{doc_name}` template marks these section(s) as mandatory; deleting them violates template fidelity."
                        ),
                        format!(
                            "Restore section(s) {} according to `{}`.",
                            missing_sections.join(", "),
                            profile.template_file
                        ),
                    ),
                );
            }

            // 2. Unfilled placeholder check (if strict_placeholders is effectively enabled):
            if strict_placeholders && !profile.placeholder_catalog.is_empty() {
                let matches =
                    matches_placeholder_catalog(&document.text, profile.placeholder_catalog);
                for (line, marker) in matches {
                    own.push(
                        DocFinding::new_with_line(
                            "",
                            line,
                            consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                            consts::TEMPLATE_BODY_VIOLATION_UNFILLED_PLACEHOLDER,
                            format!("Found unpopulated template token `{marker}` on line {line}."),
                        )
                        .with_reason(
                            "Requirements and documentation must provide concrete specifications; placeholder tokens indicate incomplete documentation.",
                            format!(
                                "Replace `{marker}` on line {line} with the actual project/feature value."
                            ),
                        ),
                    );
                }
            }

            context.stamp(document, &mut own);
            findings.extend(own);
        }

        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for TemplateBodyChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateBodyChecker {
    pub fn new() -> Self {
        Self
    }
}
