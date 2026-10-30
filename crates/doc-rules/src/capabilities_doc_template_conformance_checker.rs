// PURPOSE: TemplateConformanceChecker — AES606: template body-shape conformance
// for the 9-document layout.
//
// Entry-keyed documents (FRD, PRD, ROADMAP, BACKLOG, DATA, DESIGN) mandate
// `| Aspect | Detail |` entry tables under their item H3s; this checker walks
// each item H3 and verifies the table shape and its fixed row labels. H2-level
// sections (AGENTS Runtime, Project Structure, Commands, ...; README Quick
// Start, Reference; CONTRIBUTING Workflow, Commit Convention, Reference; ...)
// carry fixed tables checked directly at H2 level.
//
// Block 1: type + constructor
// Block 2: protocol trait implementation
// Block 3: constructors, std traits, helpers, regexes, entry checks

use shared_doc_rules::contract_doc_protocol::ITemplateConformanceProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
use shared_doc_rules::taxonomy_doc_section_vo::Section;
use shared_doc_rules::utility_capabilities_doc_h2_checks::{
    check_h2_fixed_rows, check_h2_table, check_section_shape, first_column_values, has_any_table,
    header_extra_columns, row_label_diff,
};
use shared_doc_rules::utility_markdown_scanner::{
    blank_fenced, h3_subsections, h4_subsections, normalize_heading, sections,
};
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ─────────────────────────────────────────────

/// The AES606 template-conformance auditor: every mandated section of a
/// recognized document must carry the table shape its template mandates.
pub struct TemplateConformanceChecker {}

// ─── Block 2: Protocol Trait Implementation ─────────────────────────────────

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

// ─── Block 3: Constructors, std traits, helpers, regexes, entry checks ──────

impl TemplateConformanceChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// FRD: `Functional Requirements` per-aspect H4 entry tables plus
    /// `Public Interface` and `Reference` H2 tables.
    fn check_frd(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = Self::fr_entry_re() else {
            return findings;
        };
        findings.extend(self.check_aspect_entry_tables(
            parsed,
            "Functional Requirements",
            re,
            consts::FRD_ASPECTS,
            consts::FRD_ASPECTS_OPEN,
            consts::FRD_ASPECT_SEED_MIN,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Public Interface",
            consts::ASPECT_DETAIL_COLUMNS,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Reference",
            consts::REFERENCE_COLUMNS,
        ));
        findings
    }

    /// PRD: `Project Requirements` per-aspect H4 entry tables plus `Reference`
    /// H2 table.
    fn check_prd(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = Self::pr_entry_re() else {
            return findings;
        };
        findings.extend(self.check_aspect_entry_tables(
            parsed,
            "Project Requirements",
            re,
            consts::PRD_ASPECTS,
            consts::PRD_ASPECTS_OPEN,
            consts::PRD_ASPECT_SEED_MIN,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Reference",
            consts::REFERENCE_COLUMNS,
        ));
        findings
    }

    /// ROADMAP: `Roadmap Status` entry tables plus `Reference` H2 table.
    fn check_roadmap(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = Self::pr_entry_re() else {
            return findings;
        };
        findings.extend(self.check_entry_tables(
            parsed,
            "Roadmap Status",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::ROADMAP_ROW_LABELS,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Reference",
            consts::REFERENCE_COLUMNS,
        ));
        findings
    }

    /// BACKLOG: `Backlog Status` entry tables plus `Reference` H2 table.
    fn check_backlog(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = Self::backlog_entry_re() else {
            return findings;
        };
        findings.extend(self.check_entry_tables(
            parsed,
            "Backlog Status",
            re,
            consts::ASPECT_DETAIL_COLUMNS,
            consts::BACKLOG_ROW_LABELS,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Reference",
            consts::REFERENCE_COLUMNS,
        ));
        findings
    }

    /// DATA: `Data Requirements` entry tables (header-only, no fixed rows)
    /// plus `Reference` H2 table.
    fn check_data(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = Self::dt_entry_re() else {
            return findings;
        };
        findings.extend(self.check_entry_tables(
            parsed,
            "Data Requirements",
            re,
            consts::DATA_TYPE_DESCRIPTION_COLUMNS,
            &[],
        ));
        findings.extend(check_h2_table(
            parsed,
            "Reference",
            consts::REFERENCE_COLUMNS,
        ));
        findings
    }

    /// DESIGN: `Design Requirements` entry tables (header-only, no fixed rows)
    /// plus `Reference` H2 table, then the strict template-shape sweep: no
    /// table where the HOW-TO shows none, and no H3 outside the `DS-` entry
    /// pattern. Public so the test suite can drive the DESIGN branch whole.
    pub fn check_design(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let Some(re) = Self::ds_entry_re() else {
            return findings;
        };
        findings.extend(self.check_entry_tables(
            parsed,
            "Design Requirements",
            re,
            consts::DESIGN_VARIANT_VALUE_COLUMNS,
            &[],
        ));
        findings.extend(check_h2_table(
            parsed,
            "Reference",
            consts::REFERENCE_COLUMNS,
        ));
        // The HOW-TO template shows a prose paragraph under `Design Overview`
        // and DS- entry H3s directly under `Design Requirements`; a table in
        // either spot has no template column set to conform to.
        findings.extend(self.check_no_extra_table(parsed, "Design Overview"));
        if let Some(findings_one) = self.check_requirements_prefix_table(parsed) {
            findings.push(findings_one);
        }
        findings.extend(self.check_design_h3s(parsed, re));
        findings
    }

    /// *title* must not carry a table: the HOW-TO template sanctions either
    /// prose (Design Overview) or entry H3 tables (Design Requirements), never
    /// an H2-level table. Returns `None` when the section is absent — a
    /// missing H2 belongs to AES605.
    fn check_no_extra_table(&self, parsed: &[Section], title: &str) -> Option<DocFinding> {
        let section = parsed
            .iter()
            .find(|s| s.level == 2 && normalize_heading(&s.title) == normalize_heading(title))?;
        if !has_any_table(&section.body) {
            return None;
        }
        Some(
            DocFinding::new_with_line(
                "",
                section.line,
                consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_NOT_IN_TEMPLATE,
                format!(
                    "line {} `## {}` carries a table, but the HOW-TO template prescribes no table here",
                    section.line, section.title
                ),
            )
            .with_reason(
                format!(
                    "The HOW-TO template shows `{}` as prose entries only; an H2-level table has no template column set to conform to.",
                    section.title
                ),
                format!(
                    "Delete the table under `## {}` (or move its content into `### DS-<NAME>-NNN` entry tables, which the template sanctions).",
                    section.title
                ),
            ),
        )
    }

    /// The body text of `## Design Requirements` before its first H3 must not
    /// carry a table: entry tables belong under the `DS-` H3s.
    fn check_requirements_prefix_table(&self, parsed: &[Section]) -> Option<DocFinding> {
        let section = parsed.iter().find(|s| {
            s.level == 2 && normalize_heading(&s.title) == normalize_heading("Design Requirements")
        })?;
        // Blank fenced code first so a table inside an example block cannot
        // read as a real table, and its line structure is preserved.
        let prose = blank_fenced(&section.body);
        let prefix: String = prose
            .split_inclusive('\n')
            .take_while(|line| !line.trim_start().starts_with("### "))
            .collect();
        if !has_any_table(&prefix) {
            return None;
        }
        Some(
            DocFinding::new_with_line(
                "",
                section.line,
                consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_NOT_IN_TEMPLATE,
                format!(
                    "line {} `## Design Requirements` carries a table above its `DS-` entries; the HOW-TO template prescribes no table here",
                    section.line
                ),
            )
            .with_reason(
                "The HOW-TO template places entry tables under each `### DS-<NAME>-NNN` H3; an H2-level overview table has no template column set to conform to.",
                "Delete the table above the first `DS-` entry, or move each row into its own `### DS-<NAME>-NNN` entry table.",
            ),
        )
    }

    /// Every level-3 heading in the DESIGN document must sit under
    /// `## Design Requirements` and match the `DS-<NAME>-NNN:` pattern;
    /// `Design Overview` and `Reference` sanction no H3 at all. H4 and
    /// deeper stay free-form detail per the HOW-TO template.
    fn check_design_h3s(&self, parsed: &[Section], ds_re: &regex::Regex) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for parent_title in ["Design Overview", "Design Requirements", "Reference"] {
            let Some(parent) = parsed.iter().find(|s| {
                s.level == 2 && normalize_heading(&s.title) == normalize_heading(parent_title)
            }) else {
                continue;
            };
            // Blank fences so an example heading inside a code block cannot
            // read as a real H3. Line count is preserved for anchoring.
            let blanked = Section::new(
                parent.level,
                parent.title.clone(),
                blank_fenced(&parent.body),
                parent.line,
            );
            for h3 in h3_subsections(&blanked) {
                let sanctioned = parent_title == "Design Requirements" && ds_re.is_match(&h3.title);
                if sanctioned {
                    continue;
                }
                let detail = if parent_title == "Design Requirements" {
                    "the HOW-TO template reserves `## Design Requirements` H3s for `DS-<NAME>-NNN:` entry items"
                } else {
                    "the HOW-TO template sanctions no `###` heading under this section"
                };
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        h3.line,
                        consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                        consts::TEMPLATE_CONFORMANCE_VIOLATION_H3_UNEXPECTED,
                        format!(
                            "line {} `### {}` sits under `## {}`, which the HOW-TO template does not sanction",
                            h3.line, h3.title, parent_title
                        ),
                    )
                    .with_reason(
                        format!(
                            "{}, so `### {}` is a heading the template cannot recognize.",
                            detail, h3.title
                        ),
                        format!(
                            "Rename `### {}` to `DS-<NAME>-NNN: <title>` or remove it (demote detail to H4 or deeper).",
                            h3.title
                        ),
                    ),
                );
            }
        }
        findings
    }

    /// AGENTS: `Runtime` H2-level fixed-row table plus H2 tables for Project
    /// Structure, Commands, Reference.
    fn check_agents(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        findings.extend(check_h2_fixed_rows(
            parsed,
            "Runtime",
            consts::AGENTS_RUNTIME_ROW_LABELS,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Project Structure",
            consts::AGENTS_PATH_PURPOSE_COLUMNS,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Commands",
            consts::AGENTS_COMMAND_COLUMNS,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Reference",
            consts::REFERENCE_COLUMNS,
        ));
        findings
    }

    /// README: `Quick Start` and `Reference` H2 tables.
    fn check_readme(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        findings.extend(check_h2_table(
            parsed,
            "Quick Start",
            consts::README_QUICK_START_COLUMNS,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Reference",
            consts::REFERENCE_COLUMNS,
        ));
        findings
    }

    /// CONTRIBUTING: `Workflow`, `Commit Convention`, and `Reference` H2 tables.
    fn check_contributing(&self, parsed: &[Section]) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        findings.extend(check_h2_table(
            parsed,
            "Workflow",
            consts::CONTRIBUTING_WORKFLOW_COLUMNS,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Commit Convention",
            consts::CONTRIBUTING_COMMIT_COLUMNS,
        ));
        findings.extend(check_h2_table(
            parsed,
            "Reference",
            consts::REFERENCE_COLUMNS,
        ));
        findings
    }

    // ─── Entry regexes ────────────────────────────────

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
}

// ─── Entry table checks ─────────────────────────────────────────────────────

impl TemplateConformanceChecker {
    /// Walk every H3 under *parent_h2* whose title matches *entry_re* and verify
    /// each carries a table with *columns* whose first-column row labels equal
    /// *row_labels* (header-only when empty). Findings anchor to the offending H3
    /// line.
    pub fn check_entry_tables(
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
            if let Some(finding) = check_section_shape(parsed, &h3.title, columns, Some(&h3)) {
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

    /// Walk every entry H3 under *parent_h2* whose title matches *entry_re*
    /// and verify the per-aspect H4 subsections: presence and canonical order
    /// of *aspects*, each H4's `| No | Detail |` table shape, contiguous
    /// `1..N` row numbering, bounded/open-ended row counts against
    /// *seed_min*, and non-empty `Detail` cells. Findings anchor to the
    /// offending H3 or H4 line.
    pub fn check_aspect_entry_tables(
        &self,
        parsed: &[Section],
        parent_h2: &str,
        entry_re: &regex::Regex,
        aspects: &[&str],
        open: &[&str],
        seed_min: &[(&str, usize)],
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
            // Legacy entry shape: one `| Aspect | Detail |` table on the H3,
            // no per-aspect H4 subsections. Enforced by `check_entry_tables`
            // until the doc migrates to the H4 aspect template.
            if has_any_table(&h3.body)
                && header_extra_columns(&h3.body, &["Aspect", "Detail"]).is_none()
            {
                let columns_match = first_column_values(&h3.body)
                    .iter()
                    .any(|v| v.eq_ignore_ascii_case("aspect"))
                    && first_column_values(&h3.body)
                        .iter()
                        .any(|v| v.eq_ignore_ascii_case("detail"));
                if columns_match {
                    findings.extend(self.check_entry_tables(
                        parsed,
                        parent_h2,
                        entry_re,
                        &["Aspect", "Detail"][..],
                        aspects,
                    ));
                    continue;
                }
            }
            let h4s = h4_subsections(&h3);
            let present: Vec<&Section> = h4s
                .iter()
                .filter(|s| aspects.iter().any(|a| *a == s.title))
                .collect();

            // Presence: every expected aspect H4 must exist under the entry.
            for aspect in aspects {
                if !h4s.iter().any(|s| s.title == *aspect) {
                    findings.push(
                        DocFinding::new_with_line(
                            "",
                            h3.line,
                            consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                            consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
                            format!(
                                "line {} `### {}` is missing H4 aspect `{aspect}`",
                                h3.line, h3.title
                            ),
                        )
                        .with_reason(
                            format!(
                                "The {parent_h2} template requires the per-aspect H4 subsections in order under every entry.",
                            ),
                            format!(
                                "Add `#### {aspect}` in its canonical position with a `| No | Detail |` table carrying at least one numbered row.",
                            ),
                        ),
                    );
                }
            }

            // Order: expected-aspect H4s must follow canonical order.
            let mut order_mismatch = false;
            let mut mismatch_line = h3.line;
            for (expected, h4_sec) in aspects.iter().zip(present.iter()) {
                if *expected != h4_sec.title.as_str() {
                    order_mismatch = true;
                    mismatch_line = h4_sec.line;
                    break;
                }
            }
            if order_mismatch {
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        mismatch_line,
                        consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                        consts::TEMPLATE_CONFORMANCE_VIOLATION_H4_ORDER,
                        format!(
                            "line {} `### {}` H4 aspects are out of canonical order",
                            mismatch_line, h3.title
                        ),
                    )
                        .with_reason(
                            "The entry template fixes the per-aspect H4 order; a reader navigating the entry cannot rely on it otherwise.",
                            format!(
                                "Reorder the aspect H4s of `### {}` to: {}.",
                                h3.title,
                                aspects.join(" → ")
                            ),
                        ),
                );
            }

            // Per-aspect body checks.
            for aspect in aspects {
                let Some(h4) = h4s.iter().find(|s| s.title == *aspect) else {
                    continue;
                };
                let Some(min) = seed_min.iter().find(|(n, _)| n == aspect).map(|&(_, m)| m) else {
                    continue;
                };
                let open_min = open.contains(aspect).then_some(min);
                findings.extend(self.check_aspect_table(&h3, h4, aspect, open_min, min));
            }
        }
        findings
    }

    /// One aspect H4's `| No | Detail |` table: presence, exact header,
    /// contiguous numbering, row count (bounded exact / open-ended minimum),
    /// and non-empty `Detail` cells.
    ///
    /// `open`: `None` = bounded exact `seed_min`; `Some(min)` = open-ended,
    /// at least `min` rows (numbered rows only — `∞` is not written as a marker row).
    fn check_aspect_table(
        &self,
        h3: &Section,
        h4: &Section,
        aspect: &str,
        open: Option<usize>,
        seed_min: usize,
    ) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        if !has_any_table(&h4.body) {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    h4.line,
                    consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                    consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE,
                    format!(
                        "line {} `#### {}` under `### {}` must be a markdown table with columns No | Detail",
                        h4.line, aspect, h3.title
                    ),
                )
                .with_reason(
                    "Each aspect subsection carries a `| No | Detail |` table; a body of prose or bullets is not verifiable.",
                    "Replace the body with a `| No | Detail |` table.",
                ),
            );
            return findings;
        }

        // Header must be exactly `| No | Detail |`.
        let Some(header_cells) = h4
            .body
            .lines()
            .map(str::trim)
            .find(|l| l.starts_with('|'))
            .map(|l| {
                l.trim_matches('|')
                    .split('|')
                    .map(str::trim)
                    .collect::<Vec<_>>()
            })
        else {
            return findings;
        };
        let missing: Vec<&str> = ["No", "Detail"]
            .into_iter()
            .filter(|label| {
                !header_cells
                    .iter()
                    .any(|cell| cell.to_lowercase().contains(&label.to_lowercase()))
            })
            .collect();
        let extras: Vec<&str> = header_cells
            .iter()
            .filter(|cell| {
                let c = cell.to_lowercase();
                !c.contains("no") && !c.contains("detail")
            })
            .copied()
            .collect();
        if !missing.is_empty() || !extras.is_empty() {
            let violation = if missing.is_empty() {
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_EXTRA_COLUMNS
            } else {
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS
            };
            findings.push(
                DocFinding::new_with_line(
                    "",
                    h4.line,
                    consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                    violation,
                    format!(
                        "line {} `#### {}` table header must be exactly `| No | Detail |`",
                        h4.line, aspect
                    ),
                )
                .with_reason(
                    "The aspect table is a fixed two-column shape; a different header breaks the shared row contract.",
                    {
                        let extras_joined = extras.join(", ");
                        format!(
                            "Rename the columns to `No` and `Detail` (remove extras: {}).",
                            if extras.is_empty() {
                                "none"
                            } else {
                                extras_joined.as_str()
                            }
                        )
                    },
                ),
            );
            return findings;
        }

        // Inline row scan: skip header + separator + non-pipe lines;
        // collect (No, Detail) for every data row.
        let mut rows: Vec<(String, &str)> = Vec::new();
        let mut header_seen = false;
        for line in h4.body.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with('|') {
                continue;
            }
            let cells: Vec<&str> = trimmed
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect();
            if cells.len() < 2 {
                continue;
            }
            if !header_seen {
                header_seen = true;
                continue;
            }
            let is_separator = cells
                .iter()
                .all(|c| c.chars().all(|c| c == '-' || c == ':' || c.is_whitespace()));
            if is_separator {
                continue;
            }
            rows.push((cells[0].to_string(), cells[1]));
        }
        let numbered: Vec<&(String, &str)> = rows
            .iter()
            .filter(|(no, _)| no.chars().next().is_some_and(|c| c.is_ascii_digit()))
            .collect();

        // Numbering: 1, 2, … N with no gaps.
        let mut numbering_ok = true;
        for (idx, row) in numbered.iter().enumerate() {
            if row.0 != (idx + 1).to_string() {
                numbering_ok = false;
                break;
            }
        }
        if !numbering_ok {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    h4.line,
                    consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                    consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_NUMBERING,
                    format!(
                        "line {} `#### {}` under `### {}` rows are not numbered 1..N",
                        h4.line, aspect, h3.title
                    ),
                )
                .with_reason(
                    "Each aspect table must carry contiguous numbered rows starting at 1.",
                    "Renumber rows 1, 2, … without gaps.",
                ),
            );
        }

        let total = numbered.len();
        let count_violation = match open {
            Some(min) => total < min,
            None => total != seed_min,
        };
        if count_violation {
            let violation = match open {
                Some(_) => consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_MINIMUM,
                None => consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_COUNT,
            };
            let message = match open {
                Some(min) => format!(
                    "line {} `#### {}` under `### {}` has {} rows; open-ended aspect {aspect} needs at least {min}",
                    h4.line, aspect, h3.title, total
                ),
                None => format!(
                    "line {} `#### {}` under `### {}` has {} rows; bounded aspect {aspect} must carry exactly {seed_min} row{}",
                    h4.line,
                    aspect,
                    h3.title,
                    total,
                    if seed_min == 1 { "" } else { "s" }
                ),
            };
            let fix = match open {
                Some(_) => "Add numbered rows until the aspect carries at least its seed minimum.",
                None => "Adjust the row count to exactly the bounded seed.",
            };
            findings.push(
                DocFinding::new_with_line(
                    "",
                    h4.line,
                    consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                    violation,
                    message,
                )
                .with_reason(
                    "The open/bounded contract fixes each aspect's minimum or exact seed row count.",
                    fix,
                ),
            );
        }

        // Empty Detail cells on every numbered row.
        for (no, detail) in rows.iter() {
            if !detail.is_empty() {
                continue;
            }
            if !no.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                continue;
            }
            findings.push(
                DocFinding::new_with_line(
                    "",
                    h4.line,
                    consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                    consts::TEMPLATE_CONFORMANCE_VIOLATION_EMPTY_DETAIL_CELL,
                    format!(
                        "line {} `#### {}` row {no} Detail cell is empty",
                        h4.line, aspect
                    ),
                )
                .with_reason(
                    "Every numbered row's Detail cell must be filled in a real document.",
                    "Fill the Detail cell or remove the row.",
                ),
            );
        }

        findings
    }
}

impl Default for TemplateConformanceChecker {
    fn default() -> Self {
        Self::new()
    }
}
