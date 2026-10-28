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
            let required = Self::required_links(name);
            let restates_master = name == consts::BACKLOG_DOC && !is_root;
            if required.is_empty() && !restates_master {
                continue;
            }
            let parsed = sections(&document.text);
            let mut own = Vec::new();
            if !required.is_empty() {
                self.check_reference_crosslink(name, required, &parsed, &mut own);
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

impl CrosslinkChecker {
    /// The mandatory reference links per document, one entry per the AES604
    /// target state; empty for README, which carries a placeholder table.
    fn required_links(name: &str) -> &'static [&'static str] {
        match name {
            consts::AGENTS_DOC => &[
                consts::README_DOC,
                consts::CONTRIBUTING_DOC,
                consts::ARCHITECTURE_DOC,
                consts::ROADMAP_DOC,
                consts::PRD_DOC,
            ],
            consts::PRD_DOC => &[consts::ROADMAP_DOC, consts::ARCHITECTURE_DOC],
            consts::ROADMAP_DOC => &[consts::PRD_DOC, consts::ARCHITECTURE_DOC],
            consts::BACKLOG_DOC => &[
                consts::PRD_DOC,
                consts::FRD_DOC,
                consts::ROADMAP_DOC,
                consts::ARCHITECTURE_DOC,
            ],
            consts::FRD_DOC => &[
                consts::PRD_DOC,
                consts::BACKLOG_DOC,
                consts::ROADMAP_DOC,
                consts::ARCHITECTURE_DOC,
            ],
            consts::DATA_DOC => &[
                consts::PRD_DOC,
                consts::BACKLOG_DOC,
                consts::ROADMAP_DOC,
                consts::ARCHITECTURE_DOC,
            ],
            consts::DESIGN_DOC => &[
                consts::PRD_DOC,
                consts::BACKLOG_DOC,
                consts::ROADMAP_DOC,
                consts::ARCHITECTURE_DOC,
            ],
            consts::CONTRIBUTING_DOC => &[consts::PRD_DOC, consts::ARCHITECTURE_DOC],
            // README.md mandates no links; its Reference section is a
            // placeholder table. ARCHITECTURE.md is out of scope for AES604.
            _ => &[],
        }
    }

    fn missing_link_violation_type(link: &str) -> &'static str {
        match link {
            consts::BACKLOG_DOC => consts::CROSSLINKS_VIOLATION_NO_BACKLOG_LINK,
            consts::PRD_DOC => consts::CROSSLINKS_VIOLATION_NO_PRD_LINK,
            // No dedicated shared constant exists yet for these artifacts;
            // one of the next AES604 PRs adds them.
            consts::ROADMAP_DOC => "no_roadmap_link",
            consts::ARCHITECTURE_DOC => "no_architecture_link",
            consts::README_DOC => "no_readme_link",
            consts::CONTRIBUTING_DOC => "no_contributing_link",
            consts::FRD_DOC => "no_frd_link",
            consts::DATA_DOC => "no_data_link",
            consts::DESIGN_DOC => "no_design_link",
            _ => "no_required_link",
        }
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

    /// A document must link every artifact in its required set in the
    /// Reference section, or a reader cannot reach the document that carries
    /// the detail it depends on.
    fn check_reference_crosslink(
        &self,
        name: &str,
        required: &[&str],
        parsed: &[Section],
        findings: &mut Vec<DocFinding>,
    ) {
        let Some(section) = parsed
            .iter()
            .find(|s| normalize_heading(&s.title) == "reference")
        else {
            return;
        };
        for link in required {
            if section.body.contains(link) {
                continue;
            }
            findings.push(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    consts::RULE_CODE_CROSSLINKS,
                    Self::missing_link_violation_type(link),
                    format!(
                        "line {} Reference section of {} must link {}",
                        section.line, name, link
                    ),
                )
                .with_reason(
                    "A reader cannot reach the artifact this document depends on without the link.",
                    format!("Add a link to {} in the Reference section.", link),
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
            .find(|s| normalize_heading(&s.title) == "progress overview")
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
