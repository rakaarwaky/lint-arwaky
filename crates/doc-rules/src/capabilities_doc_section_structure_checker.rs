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
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_section_vo::Section;

use shared_doc_rules::utility_markdown_scanner::{
    h3_headings, h3_subsections, has_bullet, has_table_with_columns, normalize_heading, sections,
};
use shared_doc_rules::utility_protocol_counter::fr_id_heading_re;

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
                // Document-wide, not section-scoped: the point is that the
                // parent section cannot be chosen to dodge the rule.
                self.check_h3_template_parity(document, &mut own);
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
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        api.line,
                        consts::RULE_CODE_SECTION_STRUCTURE,
                        consts::SECTION_STRUCTURE_VIOLATION_API_SUBSECTION,
                        format!("line {} API Contract has no {want} subsection", api.line),
                    )
                    .with_reason(
                        "The template splits the API Contract into Protocol API and Aggregate API, so a missing subsection leaves one half of the contract unstateable.",
                        format!("Add the missing `{want}` subsection under `## API Contract`."),
                    ),
                );
            }
            for (index, section) in matches.iter().enumerate() {
                // A table is mandatory per subsection: `Aggregate API` stating
                // its single entry point only in prose is as unverifiable as
                // `Protocol API` with no rows.
                if !has_table_with_columns(&section.body, consts::API_COLUMNS) {
                    findings.push(
                        DocFinding::new_with_line(
                            "",
                            section.line,
                            consts::RULE_CODE_SECTION_STRUCTURE,
                            consts::SECTION_STRUCTURE_VIOLATION_API_SUBSECTION_NO_TABLE,
                            format!(
                                "line {} {want} carries no table with columns {}",
                                section.line,
                                consts::API_COLUMNS.join(" | ")
                            ),
                        )
                        .with_reason(
                            "Every row is a method the caller builds against, so the subsection states them in a table to stay verifiable.",
                            format!(
                                "Add a table with columns {} under the {want} subsection.",
                                consts::API_COLUMNS.join(" | ")
                            ),
                        ),
                    );
                }
                if index > 0 {
                    findings.push(
                        DocFinding::new_with_line(
                            "",
                            section.line,
                            consts::RULE_CODE_SECTION_STRUCTURE,
                            consts::SECTION_STRUCTURE_VIOLATION_API_SUBSECTION_DUPLICATED,
                            format!("line {} {want} appears more than once", section.line),
                        )
                        .with_reason(
                            "The pair Protocol API / Aggregate API is fixed, so each appears exactly once.",
                            "Remove the duplicate {want} subsection from the API Contract.",
                        ),
                    );
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
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        section.line,
                        consts::RULE_CODE_SECTION_STRUCTURE,
                        consts::SECTION_STRUCTURE_VIOLATION_API_H3_UNEXPECTED,
                        format!(
                            "line {} API Contract carries subsection '{}'",
                            section.line, section.title
                        ),
                    )
                    .with_reason(
                        "The section holds exactly Protocol API and Aggregate API; an invented subsection is the author working around the fixed pair.",
                        "Fold the per-protocol detail into the rows of the Protocol API table and remove the heading.",
                    ),
                );
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
            findings.push(
                DocFinding::new_with_line(
                    "",
                    api.line,
                    consts::RULE_CODE_SECTION_STRUCTURE,
                    consts::SECTION_STRUCTURE_VIOLATION_ORDER,
                    format!(
                        "line {} API Contract subsections appear out of order",
                        api.line
                    ),
                )
                .with_reason(
                    "Readers meet the protocol surface before the composite entry point that delegates to it, so the pair has a fixed order.",
                    format!(
                        "Reorder the API Contract subsections to the expected order: {}.",
                        consts::API_CONTRACT_SUBSECTIONS.join(", ")
                    ),
                ),
            );
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
            findings.push(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    consts::RULE_CODE_SECTION_STRUCTURE,
                    consts::SECTION_STRUCTURE_VIOLATION_INTEGRATION_NOT_TABLE,
                    format!(
                        "line {} Integration Points must be a markdown table with columns {}",
                        section.line,
                        consts::INTEGRATION_COLUMNS.join(" | ")
                    ),
                )
                .with_reason(
                    "The table shape names the component, the data that flows, and the mechanism, so a reader can trace each integration without reading the code.",
                    format!(
                        "Rewrite Integration Points as a markdown table with columns {}.",
                        consts::INTEGRATION_COLUMNS.join(" | ")
                    ),
                ),
            );
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
            findings.push(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    consts::RULE_CODE_SECTION_STRUCTURE,
                    consts::SECTION_STRUCTURE_VIOLATION_NFR_NOT_TABLE,
                    format!(
                        "line {} Non-functional Requirements must be a markdown table with columns {}",
                        section.line,
                        consts::NFR_COLUMNS.join(" | ")
                    ),
                )
                .with_reason(
                    "The table shape names each requirement, its scope, and its budget, so a reader can evaluate it without reading the code.",
                    format!(
                        "Rewrite Non-functional Requirements as a markdown table with columns {}.",
                        consts::NFR_COLUMNS.join(" | ")
                    ),
                ),
            );
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

    /// Every level-3 heading in the FRD must be a shape the template
    /// sanctions: a requirement heading, `Protocol API`, or `Aggregate API`.
    ///
    /// `check_api_contract` reads the level-3 headings *under* `## API
    /// Contract` only, so a document that moves a level-3 section into any
    /// other parent section escapes it. That is how `crates/filesystem/FRD.md`
    /// came to carry six `### I*Protocol (N operations)` tables under
    /// `## Assumptions & Constraints` while `docs` reported 0 violations: the
    /// content was a 100% duplicate of the `Protocol API` table (63 rows on
    /// both sides, zero-name symmetric difference), yet nothing inspected it.
    ///
    /// This check reads the whole document instead of one section, so the
    /// section cannot be chosen to dodge the rule. The requirement-heading
    /// exemption reuses the shared FR-ID pattern over the raw text rather than
    /// a second pattern, so the set AES602 accepts and the set AES601 accepts
    /// are the same set by construction.
    fn check_h3_template_parity(&self, document: &DocSource, findings: &mut Vec<DocFinding>) {
        let sanctioned = |title: &str| {
            let norm = normalize_heading(title);
            consts::FRD_H3_TITLES.iter().any(|want| {
                let want = normalize_heading(want);
                norm == want || norm.starts_with(&want)
            })
        };
        // A requirement heading is sanctioned by shape, not by title, so the
        // lines the shared FR-ID pattern accepts are collected up front and a
        // heading on one of them is exempt. Reusing that pattern is what keeps
        // the set AES602 accepts and the set AES601 accepts identical.
        let requirement_lines: Vec<usize> = fr_id_heading_re()
            .map(|re| {
                re.captures_iter(&document.text)
                    .filter_map(|caps| {
                        let start = caps.get(0)?.start();
                        Some(document.text[..start].lines().count().max(1))
                    })
                    .collect()
            })
            .unwrap_or_default();
        for (title, line) in h3_headings(&document.text) {
            if sanctioned(&title) || requirement_lines.contains(&line) {
                continue;
            }
            findings.push(
                DocFinding::new_with_line(
                    "",
                    line,
                    consts::RULE_CODE_SECTION_STRUCTURE,
                    consts::SECTION_STRUCTURE_VIOLATION_H3_OFF_TEMPLATE,
                    format!("line {line} '{title}' is a level-3 heading the FRD template does not sanction"),
                )
                .with_reason(
                    "The template's only level-3 headings are '### FR-<FEATURENAME>-NNN: <name>', '### Protocol API', and '### Aggregate API', so an invented H3 drifts from the reference shape.",
                    "State this content inside the section that owns it, or demote the heading to a level-4 heading.",
                ),
            );
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
            findings.push(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    consts::RULE_CODE_SECTION_STRUCTURE,
                    consts::SECTION_STRUCTURE_VIOLATION_SCENARIOS_EMPTY,
                    format!("line {} Test Scenarios has no bullet items", section.line),
                )
                .with_reason(
                    "The template requires at least one bullet so the scenarios list them one per line instead of one undifferentiated block.",
                    "Add at least one '- scenario' bullet to the Test Scenarios section.",
                ),
            );
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
            findings.push(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    consts::RULE_CODE_SECTION_STRUCTURE,
                    consts::SECTION_STRUCTURE_VIOLATION_GLOSSARY_EMPTY,
                    format!("line {} Glossary has no bullet items", section.line),
                )
                .with_reason(
                    "The template requires at least one bullet so the glossary lists terms one per line instead of one undifferentiated block.",
                    "Add at least one '- **Term**: definition' bullet to the Glossary section.",
                ),
            );
        }
    }
}
