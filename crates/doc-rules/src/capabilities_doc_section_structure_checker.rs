// PURPOSE: SectionStructureChecker — AES602: template section order.
//
// The one capability that answers AES602. An FRD must read in template order,
// and a DATA document is held to the same order rule — it follows the same
// template family. Level-3 headings are free-form under the new FRD layout:
// the only sanctioned H3s are `FR-<FEATURE>-NNN:` requirement headings, which
// the AES601 FR-format check owns via the shared FR-ID pattern.
use shared_doc_rules::contract_doc_protocol::ISectionStructureProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
use shared_doc_rules::taxonomy_doc_section_vo::Section;

use shared_doc_rules::utility_markdown_scanner::{normalize_heading, sections};

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES602 auditor: FRD section order.
pub struct SectionStructureChecker {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ISectionStructureProtocol for SectionStructureChecker {
    /// Report every section-order violation in the documents of *context*.
    fn audit_section_structure(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            let name = context.document_name(document);
            let is_requirement_doc = name == consts::FRD_DOC || name == consts::DATA_DOC;
            if !is_requirement_doc {
                continue;
            }
            let parsed = sections(&document.text);
            let mut own = Vec::new();
            self.check_section_order(&parsed, &mut own);
            context.stamp(document, &mut own);
            findings.extend(own);
        }
        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for SectionStructureChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SectionStructureChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// FRD and DATA sections must follow template order.
    fn check_section_order(&self, parsed: &[Section], findings: &mut Vec<DocFinding>) {
        let mut found: Vec<(usize, usize)> = Vec::new();
        for section in parsed {
            if section.level != 2 {
                continue;
            }
            let normalized = normalize_heading(&section.title);
            for (index, want) in consts::FRD_SECTION_ORDER.iter().enumerate() {
                let want_norm = normalize_heading(want);
                if normalized == want_norm || normalized.starts_with(&want_norm) {
                    found.push((index, section.line));
                    break;
                }
            }
        }
        let ordered: Vec<usize> = found.iter().map(|(index, _)| *index).collect();
        let mut ascending = ordered.clone();
        ascending.sort_unstable();
        if ordered != ascending {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    found.first().map_or(0, |(_, line)| *line),
                    consts::RULE_CODE_SECTION_STRUCTURE,
                    consts::SECTION_STRUCTURE_VIOLATION_ORDER,
                    format!(
                        "line {} FRD sections appear out of template order",
                        found.first().map_or(0, |(_, line)| *line)
                    ),
                )
                .with_reason(
                    "The FRD must read in template order so a reader can navigate it against the reference shape.",
                    format!(
                        "Reorder the FRD sections to the template order: {}.",
                        consts::FRD_SECTION_ORDER.join(", ")
                    ),
                ),
            );
        }
    }
}
