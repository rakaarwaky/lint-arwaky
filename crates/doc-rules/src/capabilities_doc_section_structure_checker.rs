// PURPOSE: SectionStructureChecker — AES602: template section order and the
// shape of each mandated section.
//
// The one capability that answers AES602. An FRD must read in template order,
// and each section the template mandates must carry the body the template
// promises: a column-complete API Contract pair, an Integration Points table,
// an NFR table, bullets under Test Scenarios and Glossary. A DATA document is
// held to the order rule too — it follows the same template family.
use shared_doc_rules::contract_doc_protocol::ISectionStructureProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
use shared_doc_rules::taxonomy_doc_section_vo::Section;

use shared_doc_rules::utility_markdown_scanner::{
    h3_subsections, has_bullet, has_table_with_columns, normalize_heading, sections,
};

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES602 auditor: section order and the shape of each mandated section.
pub struct SectionStructureChecker {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ISectionStructureProtocol for SectionStructureChecker {
    /// Report every section-order and section-shape violation in the documents
    /// of *context*.
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
            // The section bodies are parsed once per document: six checks read
            // the same tree, and re-parsing per check was the cost this
            // capability exists to avoid.
            if name == consts::FRD_DOC {
                self.check_api_contract(&parsed, &mut own);
                self.check_integration_shape(&parsed, &mut own);
                self.check_nfr_shape(&parsed, &mut own);
                self.check_scenarios(&parsed, &mut own);
                self.check_glossary(&parsed, &mut own);
            }
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

    /// API Contract must carry exactly `### Protocol API` then
    /// `### Aggregate API` — in that order, once each, no others — and each
    /// must own a column-complete table of its own.
    ///
    /// The previous check only asked whether the two headings appeared
    /// somewhere in the section. That let an author answer the seam with one
    /// `Protocol API` table holding every method of every protocol, and then
    /// add a level-3 heading per protocol class to narrate the split that the
    /// single table refused to make. Level-3 headings are now a closed set
    /// with the tables checked underneath them, so the contract cannot be
    /// smuggled through a heading the rule never read.
    fn check_api_contract(&self, parsed: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(api) = parsed
            .iter()
            .find(|s| s.level == 2 && normalize_heading(&s.title).starts_with("api contract"))
        else {
            return;
        };
        let subsections = h3_subsections(api);

        // Every level-3 heading is matched against the fixed pair, so a
        // heading the rule does not know is reported rather than ignored.
        for want in consts::API_CONTRACT_SUBSECTIONS {
            let matches: Vec<&Section> = subsections
                .iter()
                .filter(|s| {
                    let norm = normalize_heading(&s.title);
                    norm == normalize_heading(want) || norm.starts_with(&normalize_heading(want))
                })
                .collect();
            if matches.is_empty() {
                findings.push(DocFinding::new_with_line(
                    "",
                    api.line,
                    consts::RULE_CODE_SECTION_STRUCTURE,
                    consts::SECTION_STRUCTURE_VIOLATION_API_SUBSECTION,
                    format!(
                        "line {} API Contract has no {want} subsection; the template splits the contract into Protocol API and Aggregate API",
                        api.line
                    ),
                ));
            }
            for (index, section) in matches.iter().enumerate() {
                // A table is mandatory per subsection: `Aggregate API` stating
                // its single entry point only in prose is as unverifiable as
                // `Protocol API` with no rows.
                if !has_table_with_columns(&section.body, consts::API_COLUMNS) {
                    findings.push(DocFinding::new_with_line(
                        "",
                        section.line,
                        consts::RULE_CODE_SECTION_STRUCTURE,
                        consts::SECTION_STRUCTURE_VIOLATION_API_SUBSECTION_NO_TABLE,
                        format!(
                            "line {} {want} carries no table with columns {}; every row is a method the caller builds against, so the subsection states them in a table",
                            section.line,
                            consts::API_COLUMNS.join(" | ")
                        ),
                    ));
                }
                if index > 0 {
                    findings.push(DocFinding::new_with_line(
                        "",
                        section.line,
                        consts::RULE_CODE_SECTION_STRUCTURE,
                        consts::SECTION_STRUCTURE_VIOLATION_API_SUBSECTION_DUPLICATED,
                        format!(
                            "line {} {want} appears more than once; the pair Protocol API / Aggregate API is fixed, so each appears exactly once",
                            section.line
                        ),
                    ));
                }
            }
        }

        // The pair is closed: a third heading — one per protocol class, or any
        // other invented subsection — is the author working around the fixed
        // pair rather than satisfying it.
        for section in &subsections {
            let norm = normalize_heading(&section.title);
            let recognized = consts::API_CONTRACT_SUBSECTIONS.iter().any(|want| {
                let want = normalize_heading(want);
                norm == want || norm.starts_with(&want)
            });
            if !recognized {
                findings.push(DocFinding::new_with_line(
                    "",
                    section.line,
                    consts::RULE_CODE_SECTION_STRUCTURE,
                    consts::SECTION_STRUCTURE_VIOLATION_API_H3_UNEXPECTED,
                    format!(
                        "line {} API Contract carries subsection '{}'; the section holds exactly Protocol API and Aggregate API — fold per-protocol detail into the rows of the Protocol API table, never into a heading",
                        section.line, section.title
                    ),
                ));
            }
        }

        // The pair also has an order: readers meet the protocol surface before
        // the composite entry point that delegates to it.
        let order: Vec<usize> = subsections
            .iter()
            .filter_map(|s| {
                let norm = normalize_heading(&s.title);
                consts::API_CONTRACT_SUBSECTIONS.iter().position(|want| {
                    let want = normalize_heading(want);
                    norm == want || norm.starts_with(&want)
                })
            })
            .collect();
        let mut ascending = order.clone();
        ascending.sort_unstable();
        if order != ascending {
            findings.push(DocFinding::new_with_line(
                "",
                api.line,
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_ORDER,
                format!(
                    "line {} API Contract subsections appear out of order; expected: {}",
                    api.line,
                    consts::API_CONTRACT_SUBSECTIONS.join(", ")
                ),
            ));
        }
    }

    /// Integration Points must be a table with the required columns.
    fn check_integration_shape(&self, parsed: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(section) = parsed
            .iter()
            .find(|s| normalize_heading(&s.title).starts_with("integration points"))
        else {
            return;
        };
        if !has_table_with_columns(&section.body, consts::INTEGRATION_COLUMNS) {
            findings.push(DocFinding::new_with_line(
                "",
                section.line,
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_INTEGRATION_NOT_TABLE,
                format!(
                    "line {} Integration Points must be a markdown table with columns {}",
                    section.line,
                    consts::INTEGRATION_COLUMNS.join(" | ")
                ),
            ));
        }
    }

    /// Non-functional Requirements must be a table with the required columns.
    fn check_nfr_shape(&self, parsed: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(section) = parsed.iter().find(|s| {
            let norm = normalize_heading(&s.title);
            norm.contains("non functional") || norm.contains("nonfunctional")
        }) else {
            return;
        };
        if !has_table_with_columns(&section.body, consts::NFR_COLUMNS) {
            findings.push(DocFinding::new_with_line(
                "",
                section.line,
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_NFR_NOT_TABLE,
                format!(
                    "line {} Non-functional Requirements must be a markdown table with columns {}",
                    section.line,
                    consts::NFR_COLUMNS.join(" | ")
                ),
            ));
        }
    }

    /// FRD sections must follow template order.
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
            findings.push(DocFinding::new_with_line(
                "",
                found.first().map_or(0, |(_, line)| *line),
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_ORDER,
                format!(
                    "line {} FRD sections appear out of template order; expected: {}",
                    found.first().map_or(0, |(_, line)| *line),
                    consts::FRD_SECTION_ORDER.join(", ")
                ),
            ));
        }
    }

    /// Test Scenarios must carry bullet items.
    fn check_scenarios(&self, parsed: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(section) = parsed
            .iter()
            .find(|s| normalize_heading(&s.title).starts_with("test scenarios"))
        else {
            return;
        };
        if !has_bullet(&section.body) {
            findings.push(DocFinding::new_with_line(
                "",
                section.line,
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_SCENARIOS_EMPTY,
                format!(
                    "line {} Test Scenarios has no bullet items; the template requires at least one '- scenario' bullet",
                    section.line
                ),
            ));
        }
    }

    /// Glossary must carry bullet items.
    fn check_glossary(&self, parsed: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(section) = parsed
            .iter()
            .find(|s| normalize_heading(&s.title) == "glossary")
        else {
            return;
        };
        if !has_bullet(&section.body) {
            findings.push(DocFinding::new_with_line(
                "",
                section.line,
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_GLOSSARY_EMPTY,
                format!(
                    "line {} Glossary has no bullet items; the template requires at least one '- **Term**: definition' bullet",
                    section.line
                ),
            ));
        }
    }
}
