// PURPOSE: Per-doc capability for FRD.md — AES609
//
// Answers ALL invariants scoped to FRD.md behind IFrdDocProtocol.
// FR-ID format, protocol parity, API method existence, spec purity,
// crosslinks, heading structure, section order, and entry-table
// conformance are stamped with RULE_CODE_FRD_DOC ("AES609").
// The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IFrdDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_names::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_capabilities_doc_h2_checks::{
    SectionShapeDefect, SectionShapeDefectKind,
};
use shared_doc_rules::utility_doc_common_checks::{
    EntryTableDefect, HeadingDefect, detect_doc_heading, detect_doc_line_drift,
    missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::fr_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::{
    detect_api_method_existence, detect_fr_id_format, detect_fr_protocol_parity,
};
use shared_doc_rules::utility_markdown_scanner::{
    blank_fenced, doc_h2_contract, normalize_heading, sections, source_ext_pattern,
    status_leak_patterns,
};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FrdCapability;

// ─── Block 2: Protocol Implementation ─────────────────────

impl IFrdDocProtocol for FrdCapability {
    fn audit_frd(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::FRD_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_FRD_DOC;
            }
        }
        findings
    }
}

// ─── Block 3: Inherent Implementation and Helpers ─────────

impl Default for FrdCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl FrdCapability {
    pub fn new() -> Self {
        Self
    }
}

impl FrdCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // Template body conformance.
        let drift = detect_doc_line_drift(text, consts::FRD_TEMPLATE_LINES);
        findings.extend(shared_doc_rules::utility_doc_line_drift::line_drift_findings(
            drift,
            consts::FRD_DOC,
            consts::RULE_CODE_FRD_DOC,
        ));

        // FR-ID format (bare FR-NNN without feature prefix).
        for defect in detect_fr_id_format(document) {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    defect.line,
                    consts::RULE_CODE_FRD_DOC,
                    DOC_VIOLATION_NAMES.fr_id_missing_feature_prefix(),
                    format!(
                        "line {} `FR-{}: {}` is a bare FR-ID without its `FR-<FEATURE>` prefix",
                        defect.line, defect.num, defect.title
                    ),
                )
                .with_reason(
                    "An FR-ID must carry its feature prefix so it stays unique across the doc chain.",
                    format!(
                        "Rename to `FR-<FEATURENAME>-{}: {}` in `## Functional Requirements`.",
                        defect.num, defect.title
                    ),
                ),
            );
        }

        // FR protocol parity (requirement count vs contract seams).
        for defect in detect_fr_protocol_parity(document, context) {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    defect.line,
                    consts::RULE_CODE_FRD_DOC,
                    DOC_VIOLATION_NAMES.protocol_count_mismatch(),
                    format!(
                        "FR protocol parity: {} requirement(s) vs {} protocol class(es) — {}",
                        defect.fr_count, defect.protocol_count, defect.direction
                    ),
                )
                .with_reason(
                    "One protocol class is one capability seam and one seam is one requirement; the two counts must match.",
                    format!("Bring the FR count ({}) and protocol class count ({}) into alignment.", defect.fr_count, defect.protocol_count),
                ),
            );
        }

        // Promised API method existence.
        for defect in detect_api_method_existence(document, context) {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    defect.line,
                    consts::RULE_CODE_FRD_DOC,
                    DOC_VIOLATION_NAMES.fr_api_method_not_found(),
                    format!(
                        "`{}` section: promised API method `{}` is not declared by any protocol/aggregate trait",
                        defect.subsection, defect.method
                    ),
                )
                .with_reason(
                    "Every method the FRD promises must exist in the code so the contract seam the feature relies on is real.",
                    format!("Add `{}` to a protocol or aggregate trait, or remove the promise from the FRD.", defect.method),
                ),
            );
        }

        // Spec purity (no source file names, no implementation state).
        Self::check_spec_purity(document, findings);

        // Crosslinks (PRD, BACKLOG, ROADMAP, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_FRD_DOC, findings);

        // Heading structure (exactly one H1, required H2s, no off-template H2s).
        if let Some((required, allowed)) = doc_h2_contract(consts::FRD_DOC) {
            for defect in detect_doc_heading(text, consts::FRD_DOC, required, allowed) {
                let (violation, message, reason, fix) = match &defect {
                    HeadingDefect::H1Count { line: _, count } => (
                        DOC_VIOLATION_NAMES.h1_count_violation(),
                        format!("{} H1 headings; template requires exactly one", count),
                        format!(
                            "{} has {} H1 headings; the template fixes exactly one H1 at the top.",
                            consts::FRD_DOC,
                            count
                        ),
                        format!(
                            "Remove the extra H1 heading(s) so {} keeps exactly one H1.",
                            consts::FRD_DOC
                        ),
                    ),
                    HeadingDefect::H2Missing { line: _, missing } => (
                        DOC_VIOLATION_NAMES.h2_missing(),
                        format!("missing required H2 section(s): {}", missing.join(", ")),
                        format!(
                            "{} is missing required H2 section(s): {}",
                            consts::FRD_DOC,
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
                            consts::FRD_DOC,
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
                        consts::RULE_CODE_FRD_DOC,
                        violation,
                        message,
                    )
                    .with_reason(reason, fix),
                );
            }
        }

        // Section order follows the template.
        Self::check_section_order(
            text,
            consts::FRD_SECTION_ORDER,
            consts::RULE_CODE_FRD_DOC,
            consts::SECTION_STRUCTURE_VIOLATION_ORDER,
            findings,
        );

        // Entry tables under Functional Requirements.
        if let Some(re) = fr_entry_re() {
            for defect in shared_doc_rules::utility_doc_common_checks::detect_entry_tables(
                &parsed,
                "Functional Requirements",
                re,
                consts::ASPECT_DETAIL_COLUMNS,
                consts::FRD_ROW_LABELS,
            ) {
                match defect {
                    EntryTableDefect::Shape(shape) => {
                        findings.push(Self::convert_shape_defect(shape));
                    }
                    EntryTableDefect::RowLabelsMismatch {
                        line,
                        missing,
                        extra,
                        ..
                    } => {
                        let missing_str = missing.join(", ");
                        let extra_str = extra.join(", ");
                        let violation = if missing.is_empty() || extra.is_empty() {
                            DOC_VIOLATION_NAMES.entry_missing_row()
                        } else {
                            DOC_VIOLATION_NAMES.table_missing_columns()
                        };
                        findings.push(
                            DocFinding::new_with_line(
                                "",
                                line,
                                consts::RULE_CODE_FRD_DOC,
                                violation,
                                format!(
                                    "entry table row labels mismatch: missing [{missing_str}] extra [{extra_str}]"
                                ),
                            )
                            .with_reason(
                                "The HOW-TO template fixes the row labels of each entry table; undeclared rows break the fixed shape.",
                                format!("Add missing row(s) [{missing_str}] and remove extra row(s) [{extra_str}]."),
                            ),
                        );
                    }
                }
            }
        }
    }

    fn convert_shape_defect(defect: SectionShapeDefect) -> DocFinding {
        Self::convert_shape_defect_common(defect, consts::RULE_CODE_FRD_DOC)
    }

    fn convert_shape_defect_common(
        defect: SectionShapeDefect,
        rule_code: &'static str,
    ) -> DocFinding {
        let (violation, message, reason, fix) = match defect.kind {
            SectionShapeDefectKind::ExtraColumns => (
                DOC_VIOLATION_NAMES.table_extra_columns(),
                format!(
                    "line {} `{} {}` table carries column(s) outside the template: {}",
                    defect.line,
                    defect.marker,
                    defect.title,
                    defect.extra_columns.join(", ")
                ),
                format!(
                    "The HOW-TO template fixes the header of `{}` to exactly {}; an undeclared column breaks the fixed shape.",
                    defect.title,
                    defect.columns.join(" | ")
                ),
                format!(
                    "Remove the extra column(s) ({}) from `{} {}` or fold the data into an existing column.",
                    defect.extra_columns.join(", "),
                    defect.marker,
                    defect.title,
                ),
            ),
            SectionShapeDefectKind::HeaderRepeated => (
                DOC_VIOLATION_NAMES.table_header_repeated(),
                format!(
                    "line {} `{} {}` repeats its header row as a data row",
                    defect.line, defect.marker, defect.title
                ),
                format!(
                    "The HOW-TO template mandates one header row under `{}`; repeating it in a data-row position ships the column names where filled values belong.",
                    defect.title
                ),
                format!(
                    "Replace the duplicated `| {} |` row under `{} {}` with real values, or delete it if no values exist yet.",
                    defect.columns.join(" | "),
                    defect.marker,
                    defect.title
                ),
            ),
            SectionShapeDefectKind::MissingColumns => (
                DOC_VIOLATION_NAMES.table_missing_columns(),
                format!(
                    "line {} `{} {}` must be a markdown table with columns {}",
                    defect.line,
                    defect.marker,
                    defect.title,
                    defect.columns.join(" | ")
                ),
                format!(
                    "The HOW-TO template mandates `{}` under this section; a body carrying bullets, prose, or a table missing required columns is not a report a reader can verify.",
                    defect.title
                ),
                format!(
                    "Replace the body of `{} {}` with a table carrying columns {}.",
                    defect.marker,
                    defect.title,
                    defect.columns.join(" | ")
                ),
            ),
            SectionShapeDefectKind::NotTable => (
                DOC_VIOLATION_NAMES.section_not_table(),
                format!(
                    "line {} `{} {}` must be a markdown table with columns {}",
                    defect.line,
                    defect.marker,
                    defect.title,
                    defect.columns.join(" | ")
                ),
                format!(
                    "The HOW-TO template mandates `{}` under this section; a body carrying bullets, prose, or a table missing required columns is not a report a reader can verify.",
                    defect.title
                ),
                format!(
                    "Replace the body of `{} {}` with a table carrying columns {}.",
                    defect.marker,
                    defect.title,
                    defect.columns.join(" | ")
                ),
            ),
        };
        DocFinding::new_with_line("", defect.line, rule_code, violation, message)
            .with_reason(reason, fix)
    }

    /// Inline spec-purity check: no concrete source-file names, no
    /// implementation-state claims. Stamped with RULE_CODE_FRD_DOC ("AES609").
    fn check_spec_purity(document: &DocSource, findings: &mut Vec<DocFinding>) {
        let prose = blank_fenced(&document.text);

        // Source file references (.rs, .py, .ts, .tsx).
        if let Some(re) = source_ext_pattern() {
            for caps in re.captures_iter(&prose) {
                let line = document.text[..caps.get(0).map_or(0, |m| m.start())]
                    .lines()
                    .count()
                    .max(1);
                let name = caps.get(0).map_or("", |m| m.as_str());
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        line,
                        consts::RULE_CODE_FRD_DOC,
                        consts::SPEC_PURITY_VIOLATION_SOURCE_FILE_NAMED,
                        format!("{name} names a concrete source file"),
                    )
                    .with_reason(
                        "A specification describes what, not where; naming a source file leaks implementation detail.",
                        format!("Remove the reference to `{name}` and describe the responsibility instead."),
                    ),
                );
            }
        }

        // Status leaks (checkboxes, status fields, implementation state, etc.).
        if let Some(patterns) = status_leak_patterns() {
            for (re, desc) in patterns {
                for caps in re.captures_iter(&prose) {
                    let line = document.text[..caps.get(0).map_or(0, |m| m.start())]
                        .lines()
                        .count()
                        .max(1);
                    findings.push(
                        DocFinding::new_with_line(
                            "",
                            line,
                            consts::RULE_CODE_FRD_DOC,
                            consts::SPEC_PURITY_VIOLATION_STATUS_LEAK,
                            desc.to_string(),
                        )
                        .with_reason(
                            "A spec is a contract, not a status report; implementation state belongs in the backlog.",
                            "Remove the status claim and keep the document specification-only.",
                        ),
                    );
                }
            }
        }
    }

    /// Crosslink check: every required link target must appear as a link in
    /// the document. Stamped with the given rule code.
    fn check_crosslinks(
        document: &DocSource,
        context: &DocAuditContext,
        code: &'static str,
        findings: &mut Vec<DocFinding>,
    ) {
        let name = context.document_name(document);
        for &link in required_links(name) {
            if !has_link(document, link) {
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

    /// Section order: required H2s must appear in template order.
    fn check_section_order(
        text: &str,
        required: &[&str],
        code: &'static str,
        violation_type: &'static str,
        findings: &mut Vec<DocFinding>,
    ) {
        let parsed = sections(text);
        let required_norm: Vec<String> = required
            .iter()
            .map(|title: &&str| normalize_heading(title))
            .collect();
        let mut indices: Vec<usize> = Vec::new();
        for s in &parsed {
            if s.level == 2 {
                let norm = normalize_heading(&s.title);
                for (i, req) in required_norm.iter().enumerate() {
                    if norm == *req || norm.starts_with(req) {
                        indices.push(i);
                        break;
                    }
                }
            }
        }
        for i in 1..indices.len() {
            if indices[i] <= indices[i - 1] {
                findings.push(DocFinding::new_with_line(
                    "",
                    0,
                    code,
                    violation_type,
                    "sections are out of template order".to_string(),
                ));
                return;
            }
        }
    }
}

/// True when *document* contains a markdown link to *target*.
fn has_link(document: &DocSource, target: &str) -> bool {
    document.text.contains(&format!("[{target}"))
}
