// PURPOSE: TemplateConformanceChecker — AES606: template body-shape conformance
// for the 9-document layout.
//
// Entry-keyed documents (FRD, PRD, ROADMAP, BACKLOG, DATA, DESIGN) mandate
// `| Aspect | Detail |` entry tables under their item H3s; this checker walks
// each item H3 and verifies the table shape and its fixed row labels. H2-level
// sections (AGENTS Runtime, Project Structure, Commands, ...; README Quick
// Start, Reference; CONTRIBUTING Workflow, Commit Convention, Reference; ...)
// carry fixed tables checked directly at H2 level.

use shared_doc_rules::contract_doc_protocol::ITemplateConformanceProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
use shared_doc_rules::taxonomy_doc_section_vo::Section;
use shared_doc_rules::utility_markdown_scanner::{
    h3_subsections, has_table_with_columns, normalize_heading, sections,
};

use std::collections::HashSet;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES606 template-conformance auditor: every mandated section of a
/// recognized document must carry the table shape its template mandates.
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

// ─── Block 2: Entry regexes ────────────────────────────────

/// `FR-<FEATURE>-NNN:` item heading (FRD `Functional Requirements` entries).
fn fr_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"^FR-[A-Za-z0-9_]+-\d+:").ok())
        .as_ref()
}

/// `PR-<PROJECT>-NNN:` item heading (PRD `Project Requirements` and ROADMAP
/// `Roadmap Status` entries).
fn pr_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"^PR-[A-Za-z0-9_]+-\d+:").ok())
        .as_ref()
}

/// `DT-<DATA>-NNN:` item heading (DATA `Data Requirements` entries).
fn dt_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"^DT-[A-Za-z0-9_]+-\d+:").ok())
        .as_ref()
}

/// `DS-<DESIGN>-NNN:` item heading (DESIGN `Design Requirements` entries).
fn ds_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"^DS-[A-Za-z0-9_]+-\d+:").ok())
        .as_ref()
}

/// `{<qualifier>}` single-qualifier heading (BACKLOG `Backlog Status` entries).
fn backlog_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"^\{[A-Za-z0-9_-]+\}$").ok())
        .as_ref()
}

// ─── Block 3: Protocol Trait Implementation ────────────────

impl ITemplateConformanceProtocol for TemplateConformanceChecker {
    /// Report every section that is present but does not carry the table shape
    /// its template mandates.
    fn audit_template_conformance(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            let name = context.document_name(document);
            let parsed = sections(&document.text);
            let mut own = match name {
                consts::FRD_DOC => self.check_frd(&parsed),
                consts::PRD_DOC => self.check_prd(&parsed),
                consts::ROADMAP_DOC => self.check_roadmap(&parsed),
                consts::BACKLOG_DOC => self.check_backlog(&parsed),
                consts::DATA_DOC => self.check_data(&parsed),
                consts::DESIGN_DOC => self.check_design(&parsed),
                consts::AGENTS_DOC => self.check_agents(&parsed),
                consts::README_DOC => self.check_readme(&parsed),
                consts::CONTRIBUTING_DOC => self.check_contributing(&parsed),
                _ => Vec::new(),
            };
            context.stamp(document, &mut own);
            findings.extend(own);
        }
        findings
    }
}

// ─── Block 4: Per-document checks ─────────────────────────

impl TemplateConformanceChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// FRD: `Functional Requirements` entry tables plus `Public Interface` and
    /// `Reference` H2 tables.
    fn check_frd(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = fr_entry_re() else {
            return findings;
        };
        findings.extend(self.check_entry_tables(
            parsed,
            "Functional Requirements",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::FRD_ROW_LABELS,
        ));
        findings.extend(self.check_h2_table(
            parsed,
            "Public Interface",
            consts::ASPECT_DETAIL_COLUMNS,
        ));
        findings.extend(self.check_h2_table(parsed, "Reference", consts::REFERENCE_COLUMNS));
        findings
    }

    /// PRD: `Project Requirements` entry tables plus `Reference` H2 table.
    fn check_prd(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = pr_entry_re() else {
            return findings;
        };
        findings.extend(self.check_entry_tables(
            parsed,
            "Project Requirements",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::PRD_ROW_LABELS,
        ));
        findings.extend(self.check_h2_table(parsed, "Reference", consts::REFERENCE_COLUMNS));
        findings
    }

    /// ROADMAP: `Roadmap Status` entry tables plus `Reference` H2 table.
    fn check_roadmap(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = pr_entry_re() else {
            return findings;
        };
        findings.extend(self.check_entry_tables(
            parsed,
            "Roadmap Status",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::ROADMAP_ROW_LABELS,
        ));
        findings.extend(self.check_h2_table(parsed, "Reference", consts::REFERENCE_COLUMNS));
        findings
    }

    /// BACKLOG: `Backlog Status` entry tables plus `Reference` H2 table.
    fn check_backlog(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = backlog_entry_re() else {
            return findings;
        };
        findings.extend(self.check_entry_tables(
            parsed,
            "Backlog Status",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::BACKLOG_ROW_LABELS,
        ));
        findings.extend(self.check_h2_table(parsed, "Reference", consts::REFERENCE_COLUMNS));
        findings
    }

    /// DATA: `Data Requirements` entry tables (header-only, no fixed rows)
    /// plus `Reference` H2 table.
    fn check_data(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = dt_entry_re() else {
            return findings;
        };
        findings.extend(self.check_entry_tables(
            parsed,
            "Data Requirements",
            re,
            consts::DATA_TYPE_DESCRIPTION_COLUMNS,
            &[],
        ));
        findings.extend(self.check_h2_table(parsed, "Reference", consts::REFERENCE_COLUMNS));
        findings
    }

    /// DESIGN: `Design Requirements` entry tables (header-only, no fixed rows)
    /// plus `Reference` H2 table.
    fn check_design(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = ds_entry_re() else {
            return findings;
        };
        findings.extend(self.check_entry_tables(
            parsed,
            "Design Requirements",
            re,
            consts::DESIGN_VARIANT_VALUE_COLUMNS,
            &[],
        ));
        findings.extend(self.check_h2_table(parsed, "Reference", consts::REFERENCE_COLUMNS));
        findings
    }

    /// AGENTS: `Runtime` H2-level fixed-row table plus H2 tables for Project
    /// Structure, Commands, Documentation Awareness, Reference.
    fn check_agents(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        findings.extend(self.check_h2_fixed_rows(
            parsed,
            "Runtime",
            consts::AGENTS_RUNTIME_ROW_LABELS,
        ));
        findings.extend(self.check_h2_table(
            parsed,
            "Project Structure",
            consts::AGENTS_PATH_PURPOSE_COLUMNS,
        ));
        findings.extend(self.check_h2_table(parsed, "Commands", consts::AGENTS_COMMAND_COLUMNS));
        findings.extend(self.check_h2_table(
            parsed,
            "Documentation Awareness",
            consts::AGENTS_DOCUMENT_AGENT_COLUMNS,
        ));
        findings.extend(self.check_h2_table(parsed, "Reference", consts::REFERENCE_COLUMNS));
        findings
    }

    /// README: `Quick Start` and `Reference` H2 tables.
    fn check_readme(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        findings.extend(self.check_h2_table(
            parsed,
            "Quick Start",
            consts::README_QUICK_START_COLUMNS,
        ));
        findings.extend(self.check_h2_table(parsed, "Reference", consts::REFERENCE_COLUMNS));
        findings
    }

    /// CONTRIBUTING: `Workflow`, `Commit Convention`, and `Reference` H2 tables.
    fn check_contributing(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        findings.extend(self.check_h2_table(
            parsed,
            "Workflow",
            consts::CONTRIBUTING_WORKFLOW_COLUMNS,
        ));
        findings.extend(self.check_h2_table(
            parsed,
            "Commit Convention",
            consts::CONTRIBUTING_COMMIT_COLUMNS,
        ));
        findings.extend(self.check_h2_table(parsed, "Reference", consts::REFERENCE_COLUMNS));
        findings
    }

    // ─── Block 5: Shared helpers ───────────────────────────

    /// Walk every H3 under *parent_h2* whose title matches *entry_re* and verify
    /// each carries a table with *columns* whose first-column row labels equal
    /// *row_labels* (header-only when empty). Findings anchor to the offending H3
    /// line.
    fn check_entry_tables(
        &self,
        parsed: &[Section],
        parent_h2: &str,
        entry_re: &regex::Regex,
        columns: &[&str],
        row_labels: &[&str],
    ) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(parent) = parsed
            .iter()
            .find(|s| s.level == 2 && normalize_heading(&s.title) == normalize_heading(parent_h2))
        else {
            return findings;
        };
        for h3 in h3_subsections(parent) {
            if !entry_re.is_match(&h3.title) {
                continue;
            }
            // Header/shape check anchored to the H3.
            if let Some(finding) = self.check_section_shape(parsed, &h3.title, columns, Some(&h3)) {
                findings.push(finding);
                continue;
            }
            // Row-label check (skipped when the doc has no fixed rows).
            if !row_labels.is_empty() {
                let values = first_column_values(&h3.body);
                let (missing, extra) = row_label_diff(row_labels, &values);
                if !missing.is_empty() || !extra.is_empty() {
                    findings.push(
                        DocFinding::new_with_line(
                            "",
                            h3.line,
                            consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                            consts::TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW,
                            format!(
                                "line {} `### {}` entry table row labels do not match the template; missing {}, extra {}",
                                h3.line,
                                h3.title,
                                missing.join(", "),
                                extra.join(", ")
                            ),
                        )
                        .with_reason(
                            format!(
                                "Every `{}` entry must carry the fixed row labels {} so the table is complete and comparable across items.",
                                parent_h2,
                                row_labels.join(", ")
                            ),
                            format!(
                                "Add the missing row(s) {} and remove the extra row(s) {} from the `### {}` table.",
                                missing.join(", "),
                                extra.join(", "),
                                h3.title
                            ),
                        ),
                    );
                }
            }
        }
        findings
    }

    /// H2-level: the section titled *title* must carry a table whose header
    /// covers *columns*. Finding (if any) anchors to the H2 line.
    fn check_h2_table(&self, parsed: &[Section], title: &str, columns: &[&str]) -> Vec<DocFinding> {
        self.check_section_shape(parsed, title, columns, None)
            .into_iter()
            .collect()
    }

    /// H2-level fixed-row check: the section titled *title* must carry an
    /// `| Aspect | Detail |` table whose first-column row labels equal
    /// *row_labels*. Finding anchors to the H2 line.
    fn check_h2_fixed_rows(
        &self,
        parsed: &[Section],
        title: &str,
        row_labels: &[&str],
    ) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(section) = parsed
            .iter()
            .find(|s| s.level == 2 && normalize_heading(&s.title) == normalize_heading(title))
        else {
            return findings;
        };
        // Shape discrimination (section_not_table / table_missing_columns) is
        // shared with the entry checks; only the fixed-row comparison is
        // specific to this H2-level check.
        if let Some(finding) =
            self.check_section_shape(parsed, title, consts::ASPECT_DETAIL_COLUMNS, None)
        {
            findings.push(finding);
            return findings;
        }
        let values = first_column_values(&section.body);
        let (missing, extra) = row_label_diff(row_labels, &values);
        if !missing.is_empty() || !extra.is_empty() {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                    consts::TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW,
                    format!(
                        "line {} `## {}` table row labels do not match the template; missing {}, extra {}",
                        section.line,
                        section.title,
                        missing.join(", "),
                        extra.join(", ")
                    ),
                )
                .with_reason(
                    format!(
                        "The `{}` section must carry the fixed row labels {} so the table is complete.",
                        title,
                        row_labels.join(", ")
                    ),
                    format!(
                        "Add the missing row(s) {} and remove the extra row(s) {} from the `## {}` table.",
                        missing.join(", "),
                        extra.join(", "),
                        title
                    ),
                ),
            );
        }
        findings
    }

    /// Find the H2 section titled *title* (or use *h3* when given), check that
    /// its body carries a table with every column in *columns*, and return the
    /// finding when the shape is wrong. Returns `None` when the section is
    /// absent (a missing H2 is AES605's question, not this rule's) or when the
    /// shape conforms. When *h3* is `Some`, the check anchors to that H3; the
    /// existing H2-level calls pass `None` and keep their H2 anchor.
    fn check_section_shape(
        &self,
        parsed: &[Section],
        title: &str,
        columns: &[&str],
        h3: Option<&Section>,
    ) -> Option<DocFinding> {
        let section = match h3 {
            Some(h3) => h3,
            None => parsed.iter().find(|s| {
                s.level == 2 && normalize_heading(&s.title) == normalize_heading(title)
            })?,
        };
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
        let marker = if section.level == 2 { "##" } else { "###" };
        Some(
            DocFinding::new_with_line(
                "",
                section.line,
                consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                violation,
                format!(
                    "line {} `{marker} {}` must be a markdown table with columns {}",
                    section.line,
                    section.title,
                    columns.join(" | ")
                ),
            )
            .with_reason(
                format!(
                    "The HOW-TO template mandates `{}` under this section; a body carrying bullets, prose, or a table missing required columns is not a report a reader can verify.",
                    section.title
                ),
                format!(
                    "Replace the body of `{marker} {}` with a table carrying columns {}.",
                    section.title,
                    columns.join(" | ")
                ),
            ),
        )
    }
}

// ─── Block 6: Constructors, Std Traits ────────────────────

impl Default for TemplateConformanceChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// First-column cell of every data row in a markdown table body (header and
/// separator rows skipped).
fn first_column_values(body: &str) -> HashSet<String> {
    let mut values = HashSet::new();
    let mut header_seen = false;
    for raw in body.lines() {
        let trimmed = raw.trim();
        if !trimmed.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = trimmed
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        if !header_seen {
            header_seen = true;
            continue;
        }
        // Separator row: every cell is dashes, colons, or whitespace.
        if cells
            .iter()
            .all(|c| c.chars().all(|c| c == '-' || c == ':' || c.is_whitespace()))
        {
            continue;
        }
        if let Some(first) = cells.first() {
            if !first.is_empty() {
                values.insert(first.to_string());
            }
        }
    }
    values
}

/// The (missing, extra) row labels of *values* against the fixed *row_labels*:
/// labels in *row_labels* absent from *values* and cells in *values* not in
/// *row_labels*. The entry and H2 fixed-row checks share this diff so the two
/// sites cannot drift apart.
fn row_label_diff<'a>(
    row_labels: &'a [&'a str],
    values: &HashSet<String>,
) -> (Vec<&'a str>, Vec<String>) {
    let missing: Vec<&str> = row_labels
        .iter()
        .filter(|label| !values.iter().any(|v| v == *label))
        .copied()
        .collect();
    let extra: Vec<String> = values
        .iter()
        .filter(|v| !row_labels.iter().any(|l| *l == *v))
        .cloned()
        .collect();
    (missing, extra)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn section(level: usize, title: &str, body: &str, line: usize) -> Section {
        Section::new(level, title.to_string(), body.to_string(), line)
    }

    #[test]
    fn entry_tables_accepts_conforming_frd_entry() {
        let text = "# X\n\n## Functional Requirements\n\n### FR-FEAT-001: Do Thing\n\n\
             | Aspect | Detail |\n| --- | --- |\n| Description | x |\n| Business Rules | x |\n| Edge Cases | x |\n| Capability | x |\n| Protocol | x |\n| Method | x |\n| Input | x |\n| Output | x |\n| Error | x |\n| Event | x |\n| Test | x |\n| Metric | x |\n";
        let parsed = sections(text);
        let re = fr_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Functional Requirements",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::FRD_ROW_LABELS,
        );
        assert!(
            findings.is_empty(),
            "a conforming FR entry must pass; got {findings:?}"
        );
    }

    #[test]
    fn entry_tables_rejects_missing_row_label() {
        let text = "# X\n\n## Functional Requirements\n\n### FR-FEAT-001: Do Thing\n\n\
             | Aspect | Detail |\n| --- | --- |\n| Description | x |\n| User Story | x |\n";
        let parsed = sections(text);
        let re = fr_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Functional Requirements",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::FRD_ROW_LABELS,
        );
        assert!(
            findings
                .iter()
                .any(|f| f.violation_type
                    == consts::TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW),
            "missing row labels must fire entry_missing_row; got {findings:?}"
        );
        let message = findings[0].message.as_str();
        assert!(
            message.contains("Business Rules"),
            "message must name at least one missing label; got '{message}'"
        );
        assert!(
            message.contains("User Story"),
            "message must name the extra label; got '{message}'"
        );
    }

    #[test]
    fn entry_tables_rejects_no_table() {
        let text = "# X\n\n## Functional Requirements\n\n### FR-FEAT-001: Do Thing\n\njust prose\n";
        let parsed = sections(text);
        let re = fr_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Functional Requirements",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::FRD_ROW_LABELS,
        );
        assert!(
            findings
                .iter()
                .any(|f| f.violation_type
                    == consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE),
            "no table must fire section_not_table; got {findings:?}"
        );
    }

    #[test]
    fn prd_entry_rejects_missing_table() {
        let text = "# X\n\n## Project Requirements\n\n### PR-PROJ-001: Do Thing\n\njust prose\n";
        let parsed = sections(text);
        let re = pr_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Project Requirements",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::PRD_ROW_LABELS,
        );
        assert!(
            findings
                .iter()
                .any(|f| f.violation_type
                    == consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE),
            "PR entry with no table must fire section_not_table; got {findings:?}"
        );
    }

    #[test]
    fn roadmap_entry_rejects_missing_table() {
        let text = "# X\n\n## Roadmap Status\n\n### PR-PROJ-001: Ship\n\nno table here\n";
        let parsed = sections(text);
        let re = pr_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Roadmap Status",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::ROADMAP_ROW_LABELS,
        );
        assert!(
            findings
                .iter()
                .any(|f| f.violation_type
                    == consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE),
            "roadmap entry with no table must fire section_not_table; got {findings:?}"
        );
    }

    #[test]
    fn backlog_entry_rejects_wrong_row_labels() {
        let text = "# X\n\n## Backlog Status\n\n### {RENDER-01}\n\n\
             | Aspect | Detail |\n| --- | --- |\n| State | x |\n| Done | x |\n| Bogus | x |\n";
        let parsed = sections(text);
        let re = backlog_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Backlog Status",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::BACKLOG_ROW_LABELS,
        );
        assert!(
            findings
                .iter()
                .any(|f| f.violation_type
                    == consts::TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW),
            "backlog entry with wrong row labels must fire entry_missing_row; got {findings:?}"
        );
        let message = findings[0].message.as_str();
        assert!(
            message.contains("In Progress")
                || message.contains("Blocked")
                || message.contains("Risk"),
            "message must name at least one missing label; got '{message}'"
        );
        assert!(
            message.contains("Bogus"),
            "message must name the extra label; got '{message}'"
        );
    }

    #[test]
    fn data_entry_rejects_header_missing_description() {
        let text = "# X\n\n## Data Requirements\n\n### DT-SHAR-001: Severity\n\n\
             | Type | Notes |\n| --- | --- |\n| vo | level |\n";
        let parsed = sections(text);
        let re = dt_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Data Requirements",
            re,
            consts::DATA_TYPE_DESCRIPTION_COLUMNS,
            &[],
        );
        assert!(
            findings.iter().any(|f| f.violation_type
                == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS),
            "DATA header lacking Description must fire table_missing_columns; got {findings:?}"
        );
    }

    #[test]
    fn design_entry_rejects_missing_table() {
        let text = "# X\n\n## Design Requirements\n\n### DS-DESIGN-001: X\n\njust prose\n";
        let parsed = sections(text);
        let re = ds_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Design Requirements",
            re,
            consts::DESIGN_VARIANT_VALUE_COLUMNS,
            &[],
        );
        assert!(
            findings
                .iter()
                .any(|f| f.violation_type
                    == consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE),
            "DESIGN entry with no table must fire section_not_table; got {findings:?}"
        );
    }

    #[test]
    fn data_entry_accepts_header_only() {
        let text = "# X\n\n## Data Requirements\n\n### DT-SHAR-001: Severity\n\n\
             | Type | Description |\n| --- | --- |\n| vo | level |\n";
        let parsed = sections(text);
        let re = dt_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Data Requirements",
            re,
            consts::DATA_TYPE_DESCRIPTION_COLUMNS,
            &[],
        );
        assert!(
            findings.is_empty(),
            "header-only DATA entry must pass; got {findings:?}"
        );
    }

    #[test]
    fn fr_entry_rejects_bad_header_missing_detail() {
        let text = "# X\n\n## Functional Requirements\n\n### FR-FEAT-001: Do Thing\n\n\
             | Aspect |\n| --- | |\n| Description | x |\n";
        let parsed = sections(text);
        let re = fr_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Functional Requirements",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::FRD_ROW_LABELS,
        );
        assert!(
            findings.iter().any(|f| f.violation_type
                == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS),
            "FR table missing Detail column must fire table_missing_columns; got {findings:?}"
        );
    }

    #[test]
    fn agents_project_structure_rejects_missing_columns() {
        let parsed = vec![section(
            2,
            "Project Structure",
            "| Path |\n| --- |\n| crates/ |\n",
            1,
        )];
        let findings = TemplateConformanceChecker::new().check_h2_table(
            &parsed,
            "Project Structure",
            consts::AGENTS_PATH_PURPOSE_COLUMNS,
        );
        assert!(
            !findings.is_empty()
                && findings.iter().any(|f| f.violation_type
                    == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS),
            "AGENTS Project Structure table missing Purpose must fire table_missing_columns; got {findings:?}"
        );
    }

    #[test]
    fn readme_quick_start_rejects_missing_columns() {
        let parsed = vec![section(
            2,
            "Quick Start",
            "| Step | Action |\n| --- | --- |\n| 1 | Do it |\n",
            1,
        )];
        let findings = TemplateConformanceChecker::new().check_h2_table(
            &parsed,
            "Quick Start",
            consts::README_QUICK_START_COLUMNS,
        );
        assert!(
            !findings.is_empty()
                && findings.iter().any(|f| f.violation_type
                    == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS),
            "README Quick Start table missing Command must fire table_missing_columns; got {findings:?}"
        );
    }

    #[test]
    fn contributing_workflow_rejects_missing_columns() {
        let parsed = vec![section(
            2,
            "Workflow",
            "| Step | Action |\n| --- | --- |\n| 1 | Do it |\n",
            1,
        )];
        let findings = TemplateConformanceChecker::new().check_h2_table(
            &parsed,
            "Workflow",
            consts::CONTRIBUTING_WORKFLOW_COLUMNS,
        );
        assert!(
            !findings.is_empty()
                && findings.iter().any(|f| f.violation_type
                    == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS),
            "CONTRIBUTING Workflow table missing Detail must fire table_missing_columns; got {findings:?}"
        );
    }

    #[test]
    fn entry_tables_skips_non_matching_h3() {
        // Non-matching H3 titles are silently skipped (entry-walking scoping);
        // only matching entries are checked. A prose H3 next to a conforming
        // entry must not fire a finding on its own.
        let text = "# X\n\n## Functional Requirements\n\n### Not An Entry At All\n\njust prose\n\n\
             ### FR-FEAT-001: Do Thing\n\n\
             | Aspect | Detail |\n| --- | --- |\n| Description | x |\n| Business Rules | x |\n| Edge Cases | x |\n| Capability | x |\n| Protocol | x |\n| Method | x |\n| Input | x |\n| Output | x |\n| Error | x |\n| Event | x |\n| Test | x |\n| Metric | x |\n";
        let parsed = sections(text);
        let re = fr_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Functional Requirements",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::FRD_ROW_LABELS,
        );
        assert!(
            findings.is_empty(),
            "non-matching H3 skipped and conforming entry passes; got {findings:?}"
        );
    }

    #[test]
    fn entry_tables_empty_parent_no_findings() {
        // Parent H2 exists but has no matching H3 entries → no findings
        // (missing-H2 enforcement is AES605's question; empty body is AES606-silent).
        let text = "# X\n\n## Functional Requirements\n\njust prose, no entries\n";
        let parsed = sections(text);
        let re = fr_entry_re().expect("regex compiles");
        let findings = TemplateConformanceChecker::new().check_entry_tables(
            &parsed,
            "Functional Requirements",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::FRD_ROW_LABELS,
        );
        assert!(
            findings.is_empty(),
            "empty parent H2 yields no findings; got {findings:?}"
        );
    }

    #[test]
    fn first_column_values_only_first_table_header_is_data() {
        // first_column_values aggregates across ALL tables in a body and treats
        // a later table's header row as a data row (single-table assumption,
        // not enforced). Pin that boundary behavior deliberately.
        let body = "| Aspect | Detail |\n| --- | --- |\n| Description | x |\n| Extra Col | y |\n| Second | Header |\n| Data | row |\n";
        let values = first_column_values(body);
        assert!(values.contains("Description"));
        // Current behavior: header_seen skips only the very first `|` line, so
        // a later table's header row first cell leaks into the data values
        // (single-table assumption not enforced). Pin that boundary deliberately.
        assert!(
            values.contains("Second"),
            "later table header first cell leaks in: {values:?}"
        );
    }

    #[test]
    fn backlog_entry_re_matches_single_qualifier() {
        let re = backlog_entry_re().expect("regex compiles");
        assert!(re.is_match("{RENDER-01}"));
        assert!(re.is_match("{FEATURE-NAME-ID}"));
        assert!(!re.is_match("{RENDER-01 | FEATURE-01}"));
    }

    #[test]
    fn agents_runtime_fixed_rows_accepts_conforming() {
        let parsed = vec![section(
            2,
            "Runtime",
            "| Aspect | Detail |\n| --- | --- |\n| Language | Rust |\n| Environment | dev |\n| Artifacts | target/ |\n| Package Manager | cargo |\n| Version Probe | rustc --version |\n| Setup | cargo build |\n| Activation | none |\n",
            1,
        )];
        let findings = TemplateConformanceChecker::new().check_h2_fixed_rows(
            &parsed,
            "Runtime",
            consts::AGENTS_RUNTIME_ROW_LABELS,
        );
        assert!(
            findings.is_empty(),
            "a conforming AGENTS Runtime must pass; got {findings:?}"
        );
    }

    #[test]
    fn agents_runtime_fixed_rows_rejects_missing_row() {
        let parsed = vec![section(
            2,
            "Runtime",
            "| Aspect | Detail |\n| --- | --- |\n| Language | Rust |\n| Environment | dev |\n",
            1,
        )];
        let findings = TemplateConformanceChecker::new().check_h2_fixed_rows(
            &parsed,
            "Runtime",
            consts::AGENTS_RUNTIME_ROW_LABELS,
        );
        assert!(
            findings
                .iter()
                .any(|f| f.violation_type
                    == consts::TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW),
            "missing AGENTS Runtime rows must fire entry_missing_row; got {findings:?}"
        );
    }
}
