// PURPOSE: Per-doc capability for ROADMAP.md — AES606
//
// Answers ALL invariants scoped to ROADMAP.md behind IRoadmapDocProtocol.
// PR-ID parity, crosslinks, heading structure, and entry-table
// conformance are stamped with RULE_CODE_ROADMAP_DOC ("AES606").
// The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IRoadmapDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_vo::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_doc_common_checks::{
    HeadingDefect, detect_doc_heading, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_fr_format_checks::{PrIdParityDefect, detect_pr_id_parity};
use shared_doc_rules::utility_doc_line_drift::detect_doc_line_drift;
use shared_doc_rules::utility_markdown_scanner::doc_h2_contract;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct RoadmapCapability;

// ─── Block 2: Protocol Implementation ─────────────────────

impl IRoadmapDocProtocol for RoadmapCapability {
    fn audit_roadmap(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::ROADMAP_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_ROADMAP_DOC;
            }
        }
        findings
    }
}

// ─── Block 3: Inherent Implementation and Helpers ─────────

impl Default for RoadmapCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl RoadmapCapability {
    pub fn new() -> Self {
        Self
    }
}

impl RoadmapCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;

        // Template body conformance.
        let drift = detect_doc_line_drift(text, consts::ROADMAP_TEMPLATE_LINES);
        findings.extend(
            shared_doc_rules::utility_doc_line_drift::line_drift_findings(
                drift,
                consts::ROADMAP_DOC,
                consts::RULE_CODE_ROADMAP_DOC,
            ),
        );

        // PR-ID parity (PRD ↔ ROADMAP).
        if let Some(prd) = context
            .documents()
            .iter()
            .find(|d| context.document_name(d) == consts::PRD_DOC)
        {
            for defect in detect_pr_id_parity(prd, document) {
                let (violation, message, reason, fix) = match &defect {
                    PrIdParityDefect::PrMissingFromRoadmap { line, id } => (
                        DOC_VIOLATION_NAMES.pr_roadmap_id_missing(),
                        format!(
                            "line {} PR `{}` declared in PRD has no ROADMAP `Roadmap Status` entry",
                            line, id
                        ),
                        format!("Add `{}` to ROADMAP's `Roadmap Status` table.", id),
                        format!("Add `{}` under `Roadmap Status` in ROADMAP.md.", id),
                    ),
                    PrIdParityDefect::RoadmapNotInPrd { line, id, valid } => (
                        DOC_VIOLATION_NAMES.roadmap_pr_id_not_pr(),
                        format!(
                            "line {} ROADMAP `PR-` item `{}` is not declared in the PRD",
                            line, id
                        ),
                        format!(
                            "The PRD declares `Project Requirements`; `{}` must appear there first.",
                            id
                        ),
                        format!(
                            "Declare `{}` in PRD's `Project Requirements`. Valid ids: {}",
                            id,
                            valid.join(", ")
                        ),
                    ),
                };
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        defect.line(),
                        consts::RULE_CODE_ROADMAP_DOC,
                        violation,
                        message,
                    )
                    .with_reason(reason, fix),
                );
            }
        }

        // Crosslinks (PRD, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_ROADMAP_DOC, findings);

        // Heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::ROADMAP_DOC) {
            for defect in detect_doc_heading(text, consts::ROADMAP_DOC, required, allowed) {
                let (violation, message, reason, fix) = match &defect {
                    HeadingDefect::H1Count { line: _, count } => (
                        DOC_VIOLATION_NAMES.h1_count_violation(),
                        format!("{} H1 headings; template requires exactly one", count),
                        format!(
                            "{} has {} H1 headings; the template fixes exactly one H1 at the top.",
                            consts::ROADMAP_DOC,
                            count
                        ),
                        format!(
                            "Remove the extra H1 heading(s) so {} keeps exactly one H1.",
                            consts::ROADMAP_DOC
                        ),
                    ),
                    HeadingDefect::H2Missing { line: _, missing } => (
                        DOC_VIOLATION_NAMES.h2_missing(),
                        format!("missing required H2 section(s): {}", missing.join(", ")),
                        format!(
                            "{} is missing required H2 section(s): {}",
                            consts::ROADMAP_DOC,
                            missing.join(", ")
                        ),
                        format!("Add the missing H2 section(s): {}", missing.join(", ")),
                    ),
                    HeadingDefect::H2Unexpected {
                        line: _,
                        unexpected,
                    } => (
                        DOC_VIOLATION_NAMES.h2_unexpected(),
                        format!("unexpected H2 section(s): {}", unexpected.join(", ")),
                        format!(
                            "{} carries H2 section(s) outside the template: {}",
                            consts::ROADMAP_DOC,
                            unexpected.join(", ")
                        ),
                        format!(
                            "Remove the off-template H2 section(s): {}",
                            unexpected.join(", ")
                        ),
                    ),
                };
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        defect.line(),
                        consts::RULE_CODE_ROADMAP_DOC,
                        violation,
                        message,
                    )
                    .with_reason(reason, fix),
                );
            }
        }

        // ROADMAP entries use a single | Aspect | Detail | table, not H4
        // aspect subsections; only the line-drift + H2 contract checks apply.
    }

    /// Crosslink check: every required link target must appear as a link in
    /// the document.
    fn check_crosslinks(
        document: &DocSource,
        context: &DocAuditContext,
        code: &'static str,
        findings: &mut Vec<DocFinding>,
    ) {
        let name = context.document_name(document);
        for &link in required_links(name) {
            if !document.text.contains(&format!("[{link}")) {
                let violation_type = missing_link_violation_type(link);
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        0,
                        code,
                        violation_type,
                        format!("{name} does not link its required {link}"),
                    )
                    .with_reason(
                        "Every document must cross-link its required artifacts so the chain stays navigable.",
                        format!("Add a link to {link} in the Reference section."),
                    ),
                );
            }
        }
    }
}
