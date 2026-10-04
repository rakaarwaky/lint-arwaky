// PURPOSE: CrosslinkChecker — AES604: the crosslinks a document owes, and the
// single home the state vocabulary may live in.
//
// The one capability that answers AES604. Two question families: does a
// requirement document's Reference section link the documents it depends on,
// and does a feature backlog restate a section that belongs to the root master
// alone? The second question needs workspace context — the master text — which
// the audit context carries.
use shared_doc_rules::contract_doc_protocol::ICrosslinkProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
use shared_doc_rules::taxonomy_doc_section_vo::Section;

use shared_doc_rules::utility_markdown_scanner::{normalize_heading, sections};

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES604 auditor: Reference crosslinks and state-vocabulary placement.
pub struct CrosslinkChecker {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ICrosslinkProtocol for CrosslinkChecker {
    /// Report every missing crosslink and every restated master-only section in
    /// the documents of *context*.
    fn audit_crosslinks(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            let name = context.document_name(document);
            // The master owns the state vocabulary, so it is the one document
            // allowed to carry those sections; a feature backlog restating them
            // is the violation.
            let is_root = context.is_root_document(document);
            let carries_reference = name == consts::FRD_DOC || name == consts::DATA_DOC;
            let restates_master = name == consts::BACKLOG_DOC && !is_root;
            if !carries_reference && !restates_master {
                continue;
            }
            let parsed = sections(&document.text);
            let mut own = Vec::new();
            if carries_reference {
                self.check_reference_crosslink(&parsed, &mut own);
            }
            if restates_master {
                self.check_state_vocab_restated(&parsed, context, &mut own);
            }
            context.stamp(document, &mut own);
            findings.extend(own);
        }
        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for CrosslinkChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl CrosslinkChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// A requirement document must link its BACKLOG and PRD in the Reference
    /// section, or a reader cannot reach the report that says whether the
    /// promise was kept.
    fn check_reference_crosslink(&self, parsed: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(section) = parsed
            .iter()
            .find(|s| normalize_heading(&s.title) == "reference")
        else {
            return;
        };
        if !section.body.contains(consts::BACKLOG_DOC) {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    consts::RULE_CODE_CROSSLINKS,
                    consts::CROSSLINKS_VIOLATION_NO_BACKLOG_LINK,
                    format!(
                        "line {} Reference section must link {}",
                        section.line,
                        consts::BACKLOG_DOC
                    ),
                )
                .with_reason(
                    "A reader of the requirement cannot reach the report that says whether the promise was kept without the backlog link.",
                    format!("Add a link to {} in the Reference section.", consts::BACKLOG_DOC),
                ),
            );
        }
        if !section.body.contains(consts::PRD_DOC) {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    consts::RULE_CODE_CROSSLINKS,
                    consts::CROSSLINKS_VIOLATION_NO_PRD_LINK,
                    format!(
                        "line {} Reference section must link {}",
                        section.line,
                        consts::PRD_DOC
                    ),
                )
                .with_reason(
                    "A reader of the requirement cannot reach the product intent that justifies it without the PRD link.",
                    format!("Add a link to {} in the Reference section.", consts::PRD_DOC),
                ),
            );
        }
    }

    /// State vocabulary must live only in the root master, not in sub-docs.
    ///
    /// Duplicating the vocabulary is how two documents come to disagree about
    /// what `In Progress` means, so a sub-doc restating a master-only section
    /// is reported when a master exists to be the single home.
    fn check_state_vocab_restated(
        &self,
        parsed: &[Section],
        context: &DocAuditContext,
        findings: &mut Vec<DocFinding>,
    ) {
        let Some(section) = parsed
            .iter()
            .find(|s| normalize_heading(&s.title) == "current condition")
        else {
            return;
        };
        let restated: Vec<&str> = consts::MASTER_ONLY_SECTIONS
            .iter()
            .copied()
            .filter(|title| {
                parsed
                    .iter()
                    .any(|s| normalize_heading(&s.title) == normalize_heading(title))
            })
            .collect();
        if restated.is_empty() {
            return;
        }
        if context.master().is_some() {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    consts::RULE_CODE_CROSSLINKS,
                    consts::CROSSLINKS_VIOLATION_STATE_VOCAB_RESTATED,
                    format!(
                        "line {} feature backlog carries {} section(s)",
                        section.line,
                        restated.join(", ")
                    ),
                )
                .with_reason(
                    "Duplicating the vocabulary is how two documents come to disagree about what `In Progress` means; a sub-doc restating a master-only section breaks the single home.",
                    "Remove the section(s) from the feature backlog; they live once, in the root master.",
                ),
            );
        }
    }
}
