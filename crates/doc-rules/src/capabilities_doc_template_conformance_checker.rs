// PURPOSE: TemplateConformanceChecker — AES606 (first half): template body-shape
// conformance for the BACKLOG.md and ROADMAP.md document types.
//
// The HOW-TO-MAKE-BACKLOG and HOW-TO-MAKE-ROADMAP templates each mandate a
// specific table shape under a specific H2 section. A section that is present
// but carrying the wrong body — bullets, prose, or a table missing a required
// column — is a violation. This is the gap AES605 cannot close: AES605 sees
// the H2 heading is present, but says nothing about what the section body
// must look like.
use shared_doc_rules::contract_doc_protocol::ITemplateConformanceProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_section_vo::Section;
use shared_doc_rules::utility_markdown_scanner::{
    has_table_with_columns, normalize_heading, sections,
};

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES606 template-conformance auditor: every mandated section of a
/// BACKLOG or ROADMAP document must carry its template table.
pub struct TemplateConformanceChecker {}

/// The body carries a markdown table: a `|`-prefixed header row followed by a
/// table separator row. Header rows alone are not tables in markdown.
fn has_any_table(body: &str) -> bool {
    let lines = body.lines().collect::<Vec<_>>();
    lines.iter().enumerate().any(|(i, line)| {
        if !line.trim().starts_with('|') || i == 0 || !is_table_separator(line) {
            return false;
        }
        lines[..i].iter().any(|prev| prev.trim().starts_with('|'))
    })
}

/// A line that is a markdown table separator row: `| --- |`, `|:---:|`, etc.
fn is_table_separator(line: &str) -> bool {
    let t = line.trim();
    if !t.starts_with('|') {
        return false;
    }
    // Strip the outer `|` delimiters and any whitespace, then check the cell
    // contents: each `|`-separated cell holds only `-` and/or `:` characters.
    let inner = t
        .trim_start_matches(['|', ' '])
        .trim_end_matches(['|', ' '])
        .split('|')
        .map(|cell| cell.trim())
        .collect::<Vec<_>>();
    !inner.is_empty()
        && inner
            .iter()
            .all(|cell| !cell.is_empty() && cell.chars().all(|c| c == '-' || c == ':'))
        && inner.iter().any(|cell| cell.contains('-'))
}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ITemplateConformanceProtocol for TemplateConformanceChecker {
    /// Report every section that is present but does not carry the table shape
    /// its template mandates.
    fn audit_template_conformance(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            let name = context.document_name(document);
            match name {
                consts::BACKLOG_DOC => {
                    let mut own = self.check_backlog(document);
                    context.stamp(document, &mut own);
                    findings.extend(own);
                }
                consts::ROADMAP_DOC => {
                    let mut own = self.check_roadmap(document);
                    context.stamp(document, &mut own);
                    findings.extend(own);
                }
                _ => {}
            }
        }
        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for TemplateConformanceChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateConformanceChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// BACKLOG.md: check the four table-shaped sections mandated by
    /// HOW-TO-MAKE-BACKLOG.
    fn check_backlog(&self, document: &DocSource) -> Vec<DocFinding> {
        let parsed = sections(&document.text);
        let checks: [(&str, &[&str]); 4] = [
            ("Backlog", consts::BACKLOG_TABLE_COLUMNS),
            ("Scenario Evidence", consts::SCENARIO_EVIDENCE_COLUMNS),
            ("Release Readiness", consts::RELEASE_READINESS_COLUMNS),
            ("Change Log", consts::CHANGE_LOG_COLUMNS),
        ];
        checks
            .iter()
            .filter_map(|&(title, columns)| self.check_section_shape(&parsed, title, columns))
            .collect()
    }

    /// ROADMAP.md: check the two table-shaped sections mandated by
    /// HOW-TO-MAKE-ROADMAP.
    fn check_roadmap(&self, document: &DocSource) -> Vec<DocFinding> {
        let parsed = sections(&document.text);
        let checks: [(&str, &[&str]); 2] = [
            ("State Definitions", consts::STATE_DEFINITIONS_COLUMNS),
            ("Feature Roll-up", consts::FEATURE_ROLLUP_COLUMNS),
        ];
        checks
            .iter()
            .filter_map(|&(title, columns)| self.check_section_shape(&parsed, title, columns))
            .collect()
    }

    /// Find the H2 section titled *title* in *parsed*, check that its body
    /// carries a table with every column in *columns*, and return the finding
    /// when the shape is wrong. Returns `None` when the section is absent (a
    /// missing H2 is AES605's question, not this rule's).
    fn check_section_shape(
        &self,
        parsed: &[Section],
        title: &str,
        columns: &[&str],
    ) -> Option<DocFinding> {
        let want = normalize_heading(title);
        let section = parsed
            .iter()
            .find(|s| s.level == 2 && normalize_heading(&s.title).starts_with(&want))?;
        let body = &section.body;
        if has_table_with_columns(body, columns) {
            return None;
        }
        // The section is present but wrong-shaped. Distinguish "no table at all"
        // from "table missing columns" so the fix message can be specific: a
        // markdown table separator line (`| --- |`) is what turns a header row
        // into a parseable table.
        let has_any_table = has_any_table(body);
        let violation = if has_any_table {
            consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS
        } else {
            consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE
        };
        Some(
            DocFinding::new_with_line(
                "",
                section.line,
                consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                violation,
                format!(
                    "line {} `## {}` must be a markdown table with columns {}",
                    section.line,
                    title,
                    columns.join(" | ")
                ),
            )
            .with_reason(
                format!(
                    "The HOW-TO template mandates `{}` under this section; a body carrying bullets, prose, or a table missing required columns is not a report a reader can verify.",
                    title
                ),
                format!(
                    "Replace the body of `## {}` with a table carrying columns {}.",
                    title,
                    columns.join(" | ")
                ),
            ),
        )
    }
}
