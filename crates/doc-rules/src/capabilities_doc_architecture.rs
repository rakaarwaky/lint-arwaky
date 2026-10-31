// PURPOSE: Per-doc capability for ARCHITECTURE.md — AES603
//
// Answers ALL invariants scoped to ARCHITECTURE.md behind
// IArchitectureDocProtocol. Heading structure and template body
// conformance are stamped with RULE_CODE_ARCHITECTURE_DOC
// ("AES603"). The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IArchitectureDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_names::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_doc_common_checks::{
    detect_doc_heading, detect_doc_line_drift,
};
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
pub struct ArchitectureCapability;

impl ArchitectureCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IArchitectureDocProtocol for ArchitectureCapability {
    fn audit_architecture(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::ARCHITECTURE_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_ARCHITECTURE_DOC;
            }
        }
        findings
    }
}

impl ArchitectureCapability {
    fn audit_one(document: &DocSource, _context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // Template body conformance.
        let drift = detect_doc_line_drift(text, consts::ARCHITECTURE_TEMPLATE_LINES);
        findings.extend(crate::utility_doc_line_drift::line_drift_findings(
            drift,
            consts::ARCHITECTURE_DOC,
            consts::RULE_CODE_ARCHITECTURE_DOC,
        ));

        // Heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::ARCHITECTURE_DOC) {
            for defect in detect_doc_heading(text, consts::ARCHITECTURE_DOC, required, allowed) {
                let (violation, message, reason, fix) = match &defect {
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H1Count {
                        line: _,
                        count,
                    } => (
                        DOC_VIOLATION_NAMES.h1_count_violation(),
                        format!(
                            "{} opens with {} level-1 headings",
                            consts::ARCHITECTURE_DOC,
                            count
                        ),
                        format!(
                            "{} must open with exactly one H1 naming the project",
                            consts::ARCHITECTURE_DOC
                        ),
                        format!(
                            "Keep exactly one `# {}` line at the top of {}",
                            consts::ARCHITECTURE_DOC,
                            consts::ARCHITECTURE_DOC
                        ),
                    ),
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H2Missing {
                        line: _,
                        missing,
                    } => (
                        DOC_VIOLATION_NAMES.h2_missing(),
                        format!(
                            "{} has no H2 heading for {}",
                            consts::ARCHITECTURE_DOC,
                            missing.join(", ")
                        ),
                        format!(
                            "{} needs a level-2 heading for every template-required section",
                            consts::ARCHITECTURE_DOC
                        ),
                        format!(
                            "Add the missing `## {}` section(s) to {}",
                            missing.join(" `## `"),
                            consts::ARCHITECTURE_DOC
                        ),
                    ),
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H2Unexpected {
                        line: _,
                        unexpected,
                    } => (
                        DOC_VIOLATION_NAMES.h2_unexpected(),
                        format!(
                            "{} carries H2 heading(s) outside the template: {}",
                            consts::ARCHITECTURE_DOC,
                            unexpected.join(", ")
                        ),
                        format!(
                            "{} must not carry H2 headings the template does not declare",
                            consts::ARCHITECTURE_DOC
                        ),
                        format!(
                            "Remove or rename the unexpected heading(s): {}",
                            unexpected.join(", ")
                        ),
                    ),
                };
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        defect.line(),
                        consts::RULE_CODE_ARCHITECTURE_DOC,
                        violation,
                        message,
                    )
                    .with_reason(reason, fix),
                );
            }
        }

        // Template body conformance is exercised through the
        // required H2 set above; ARCHITECTURE.md carries no entry tables.
        let _ = parsed;
    }
}
