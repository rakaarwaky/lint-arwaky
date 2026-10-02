// PURPOSE: HeadingStructureChecker — AES605: the H1/H2 heading structure each
// recognised document must hold.
//
// The one capability that answers AES605. Every document with a registered H2
// contract must open with exactly one level-1 heading, carry every required
// level-2 section, and hold no level-2 heading outside the agreed set. The
// document set is this capability's own decision: a document with no
// registered contract has no heading shape to hold, so it is skipped rather
// than reported.
use shared_doc_rules::contract_doc_protocol::IDocHeadingProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;

use shared_doc_rules::utility_markdown_scanner::{
    blank_fenced, doc_h2_contract, heading_re, normalize_heading,
};

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES605 auditor: H1 count, required H2 set, and closed H2 set.
pub struct HeadingStructureChecker {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IDocHeadingProtocol for HeadingStructureChecker {
    /// Report every heading-count and heading-contract violation in the
    /// documents of *context*.
    fn audit_doc_heading(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            let name = context.document_name(document);
            // Only a document with a registered H2 contract has a heading
            // shape to hold; anything else is outside the document chain.
            let Some((required, allowed)) = doc_h2_contract(name) else {
                continue;
            };
            let mut own = Vec::new();
            self.check_doc_heading(&document.text, name, required, allowed, &mut own);
            context.stamp(document, &mut own);
            findings.extend(own);
        }
        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for HeadingStructureChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadingStructureChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// A document must carry exactly one H1, every required H2 from its
    /// template, and no H2 outside the agreed set. Level-3 headings are free.
    fn check_doc_heading(
        &self,
        text: &str,
        name: &str,
        required: &[&str],
        allowed: &[&str],
        findings: &mut Vec<DocFinding>,
    ) {
        let Some(re) = heading_re() else {
            return;
        };
        // Headings inside fenced code blocks are ignored: a shell comment
        // such as `# Tests (matches CI "Tests" job)` must not read as a heading.
        let prose = blank_fenced(text);
        let captures: Vec<_> = re.captures_iter(&prose).collect();
        let h1_count: usize = captures
            .iter()
            .filter(|c| c.get(1).is_some_and(|m| m.as_str().len() == 1))
            .count();
        if h1_count != 1 {
            findings.push(DocFinding::new_with_line(
                "",
                0,
                consts::RULE_CODE_DOC_STRUCTURE,
                consts::DOC_STRUCTURE_VIOLATION_H1_COUNT,
                format!(
                    "{name} must open with exactly one level-1 heading, found {}; the template names one H1 at the top of the file",
                    h1_count
                ),
            ));
        }
        let h2: Vec<String> = captures
            .iter()
            .filter(|c| c.get(1).is_some_and(|m| m.as_str().len() == 2))
            .map(|c| normalize_heading(c.get(2).map_or("", |m| m.as_str())))
            .collect();
        let missing: Vec<&str> = required
            .iter()
            .copied()
            .filter(|want| {
                let want = normalize_heading(want);
                !h2.iter()
                    .any(|title| title == &want || title.starts_with(&want))
            })
            .collect();
        if !missing.is_empty() {
            findings.push(DocFinding::new_with_line(
                "",
                0,
                consts::RULE_CODE_DOC_STRUCTURE,
                consts::DOC_STRUCTURE_VIOLATION_H2_MISSING,
                format!(
                    "{name} has no H2 heading for {}; each of these level-2 sections is mandatory — {}",
                    missing.join(", "),
                    required.join(", ")
                ),
            ));
        }
        // The H2 set is closed: a heading at level 2 that is not in the
        // required + allowed union must be demoted to a level-3 heading or removed.
        let permitted: Vec<String> = required
            .iter()
            .copied()
            .chain(allowed.iter().copied())
            .map(normalize_heading)
            .collect();
        let unexpected_h2: Vec<String> = h2
            .into_iter()
            .filter(|title| !permitted.iter().any(|a| title == a || title.starts_with(a)))
            .collect();
        if !unexpected_h2.is_empty() {
            findings.push(DocFinding::new_with_line(
                "",
                0,
                consts::RULE_CODE_DOC_STRUCTURE,
                consts::DOC_STRUCTURE_VIOLATION_H2_UNEXPECTED,
                format!(
                    "{name} carries H2 heading(s) outside the template: {}; move each to a level-3 heading or remove it",
                    unexpected_h2.join(", ")
                ),
            ));
        }
    }
}
