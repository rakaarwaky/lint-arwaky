// PURPOSE: Per-doc capability for DESIGN.md — AES610
//
// Answers ALL invariants scoped to DESIGN.md behind IDesignDocProtocol.
// DS-ID alignment with BACKLOG (AES601), crosslinks (AES604), heading
// structure (AES605), entry-table conformance + H3 sweep (AES606) are
// stamped with RULE_CODE_DESIGN_DOC ("AES610"). The capability stamps the
// document path on every finding.

use shared_doc_rules::contract_doc_protocol::IDesignDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_names::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_doc_aspect_entry_checks::{
    detect_aspect_entry_tables, AspectEntryDefect,
};
use shared_doc_rules::utility_doc_common_checks::{
    detect_backlog_qualifier_alignment, detect_doc_heading, BacklogQualifierDefect, HeadingDefect,
    missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::ds_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::collect_any_id_set;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};
use shared_doc_rules::utility_protocol_counter::ds_id_heading_re;

#[derive(Default)]
pub struct DesignCapability;

impl DesignCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IDesignDocProtocol for DesignCapability {
    fn audit_design(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::DESIGN_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_DESIGN_DOC;
            }
        }
        findings
    }
}

impl DesignCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES601 — DS-ID alignment with BACKLOG.
        if let Some(backlog) = context
            .documents()
            .iter()
            .find(|d| context.document_name(d) == consts::BACKLOG_DOC)
        {
            let union = shared_doc_rules::utility_doc_common_checks::collect_union_id_set(
                collect_any_id_set,
                Some(document),
                Some(backlog),
            );
            for defect in shared_doc_rules::utility_doc_common_checks::detect_backlog_qualifier_alignment(backlog, union.as_ref()) {
                let (violation, message, reason, fix) = match &defect {
                    BacklogQualifierDefect::TokenNotDeclared { line, token, valid } => (
                        consts::DS_BACKLOG_ID_MISSING,
                        format!("line {} BACKLOG token `{}` is not declared in any spec union", line, token),
                        format!("The BACKLOG entry `{}` has no matching ID in the paired spec union.", token),
                        format!("Declare `{}` in a spec ID (`DS-`, `PR-`, `FR-`, or `DT-` family). Valid: {}", token, valid.join(", ")),
                    ),
                    BacklogQualifierDefect::IdNoEntry { line, id } => (
                        consts::BACKLOG_QUALIFIER_MISMATCH,
                        format!("line {} declared ID `{}` has no BACKLOG entry", line, id),
                        format!("ID `{}` is declared by a spec but has no corresponding BACKLOG entry.", id),
                        format!("Add a BACKLOG entry for `{}`.", id),
                    ),
                    BacklogQualifierDefect::StrayId { line, token, id, valid } => (
                        consts::BACKLOG_QUALIFIER_MISMATCH,
                        format!("line {} BACKLOG entry `{}` cites undeclared id `{}`", line, token, id),
                        format!("BACKLOG entry `{}` references id `{}` which no spec declares.", token, id),
                        format!("Remove the stray id `{}` or declare it in a spec. Valid: {}", id, valid.join(", ")),
                    ),
                };
                findings.push(
                    DocFinding::new_with_line("", defect.line(), consts::RULE_CODE_DESIGN_DOC, violation, message)
                        .with_reason(reason, fix),
                );
            }
        }

        // AES604 — crosslinks (PRD, BACKLOG, ROADMAP, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::DESIGN_DOC) {
            for defect in detect_doc_heading(text, consts::DESIGN_DOC, required, allowed) {
                let (violation, message, reason, fix) = match &defect {
                    HeadingDefect::H1Count { line, count } => (
                        consts::DOC_STRUCTURE_VIOLATION_H1_COUNT,
                        format!("{} H1 headings; template requires exactly one", count),
                        format!("{} has {} H1 headings; the template fixes exactly one H1 at the top.", consts::DESIGN_DOC, count),
                        format!("Remove the extra H1 heading(s) so {} keeps exactly one H1.", consts::DESIGN_DOC),
                    ),
                    HeadingDefect::H2Missing { line, missing } => (
                        consts::DOC_STRUCTURE_VIOLATION_H2_MISSING,
                        format!("missing required H2 section(s): {}", missing.join(", ")),
                        format!("{} is missing required H2 section(s): {}", consts::DESIGN_DOC, missing.join(", ")),
                        format!("Add the missing H2 section(s): {}", missing.join(", ")),
                    ),
                    HeadingDefect::H2Unexpected { line, unexpected } => (
                        consts::DOC_STRUCTURE_VIOLATION_H2_UNEXPECTED,
                        format!("unexpected H2 section(s): {}", unexpected.join(", ")),
                        format!("{} carries H2 section(s) outside the template: {}", consts::DESIGN_DOC, unexpected.join(", ")),
                        format!("Remove the off-template H2 section(s): {}", unexpected.join(", ")),
                    ),
                };
                findings.push(
                    DocFinding::new_with_line("", defect.line(), consts::RULE_CODE_DESIGN_DOC, violation, message)
                        .with_reason(reason, fix),
                );
            }
        }

        // AES606 — entry tables under Design Requirements + H3 sweep.
        if let Some(re) = ds_entry_re() {
            for defect in detect_aspect_entry_tables(
                &parsed,
                "Design Requirements",
                re,
                &[],
                &[],
                &[],
                false,
            ) {
                findings.push(Self::convert_aspect_entry_defect(defect));
            }
        }
        Self::check_design_h3s(text, findings);
    }

    fn convert_aspect_entry_defect(defect: AspectEntryDefect) -> DocFinding {
        Self::convert_aspect_entry_defect_common(defect, consts::RULE_CODE_DESIGN_DOC)
    }

    fn convert_aspect_entry_defect_common(defect: AspectEntryDefect, rule_code: &'static str) -> DocFinding {
        let (violation, message, reason, fix) = match &defect {
            AspectEntryDefect::MissingAspect { line, h3_title, aspect } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
                format!(
                    "line {} `### {}` missing aspect H4 `#### {}`",
                    line, h3_title, aspect
                ),
                format!(
                    "The HOW-TO template requires every entry H3 under `### {}` to carry a `#### {}` subsection.",
                    h3_title, aspect
                ),
                format!(
                    "Add `#### {}` under `### {}`.",
                    aspect, h3_title
                ),
            ),
            AspectEntryDefect::OrderMismatch { line, h3_title, aspects } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_H4_ORDER,
                format!(
                    "line {} `### {}` aspect H4s out of canonical order: expected {}",
                    line, h3_title, aspects.join(", ")
                ),
                format!(
                    "Reorder the `####` aspect subsections under `### {}` to match the template order: {}.",
                    h3_title, aspects.join(", ")
                ),
                format!(
                    "Move the aspect H4s under `### {}` into canonical order: {}.",
                    h3_title, aspects.join(", ")
                ),
            ),
            AspectEntryDefect::EmptyBody { line, aspect, h3_title } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
                format!(
                    "line {} `#### {}` under `### {}` carries an empty body",
                    line, aspect, h3_title
                ),
                format!(
                    "Every PRD aspect `#### {}` under `### {}` must carry non-empty content.",
                    aspect, h3_title
                ),
                format!(
                    "Add content to `#### {}` under `### {}`.",
                    aspect, h3_title
                ),
            ),
            AspectEntryDefect::NotATable { line, aspect, h3_title } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE,
                format!(
                    "line {} `#### {}` under `### {}` must be a markdown table",
                    line, aspect, h3_title
                ),
                format!(
                    "The aspect `#### {}` under `### {}` must carry a `| No | Detail |` table.",
                    aspect, h3_title
                ),
                format!(
                    "Replace the body of `#### {}` under `### {}` with a `| No | Detail |` table.",
                    aspect, h3_title
                ),
            ),
            AspectEntryDefect::HeaderMissing { line, aspect } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS,
                format!(
                    "line {} `#### {}` aspect table header missing required columns",
                    line, aspect
                ),
                format!(
                    "The aspect table must carry columns `| No | Detail |` under `#### {}`; a header missing a required column cannot be parsed.",
                    aspect
                ),
                format!(
                    "Set the aspect table header to `| No | Detail |` under `#### {}`.",
                    aspect
                ),
            ),
            AspectEntryDefect::HeaderExtra { line, aspect, extras } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_EXTRA_COLUMNS,
                format!(
                    "line {} `#### {}` aspect table carries extra column(s): {}",
                    line, aspect, extras.join(", ")
                ),
                format!(
                    "The aspect table header under `#### {}` must be exactly `| No | Detail |`; extra columns break the fixed shape.",
                    aspect
                ),
                format!(
                    "Remove the extra column(s) ({}) from the aspect table header under `#### {}`.",
                    extras.join(", "),
                    aspect
                ),
            ),
            AspectEntryDefect::RowNumbering { line, aspect, h3_title } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_NUMBERING,
                format!(
                    "line {} `#### {}` under `### {}` row numbers are not contiguous 1..N",
                    line, aspect, h3_title
                ),
                format!(
                    "The aspect table rows under `#### {}` must be numbered contiguously starting at 1.",
                    aspect
                ),
                format!(
                    "Renumber the rows of `#### {}` under `### {}` as 1, 2, ... N with no gaps.",
                    aspect, h3_title
                ),
            ),
            AspectEntryDefect::RowCount { line, aspect, h3_title, total, seed_min } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_COUNT,
                format!(
                    "line {} `#### {}` under `### {}` bounded aspect carries {} rows, expected exactly {}",
                    line, aspect, h3_title, total, seed_min
                ),
                format!(
                    "The bounded aspect `#### {}` under `### {}` must carry exactly {} numbered rows.",
                    aspect, h3_title, seed_min
                ),
                format!(
                    "Adjust `#### {}` under `### {}` to carry exactly {} numbered rows.",
                    aspect, h3_title, seed_min
                ),
            ),
            AspectEntryDefect::RowMinimum { line, aspect, h3_title, total, min } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_MINIMUM,
                format!(
                    "line {} `#### {}` under `### {}` open-ended aspect carries {} rows, need at least {}",
                    line, aspect, h3_title, total, min
                ),
                format!(
                    "The open-ended aspect `#### {}` under `### {}` must carry at least {} numbered rows.",
                    aspect, h3_title, min
                ),
                format!(
                    "Add numbered rows to `#### {}` under `### {}` until at least {} are present.",
                    aspect, h3_title, min
                ),
            ),
            AspectEntryDefect::EmptyDetailCell { line, aspect, row_no } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_EMPTY_DETAIL_CELL,
                format!(
                    "line {} `#### {}` row `{}` Detail cell is empty or a placeholder dash",
                    line, aspect, row_no
                ),
                format!(
                    "Every numbered row in `#### {}` must carry real content in its Detail cell; a lone dash cell carries nothing.",
                    aspect
                ),
                format!(
                    "Fill the Detail cell of row `{}` in `#### {}` with real content, or delete the row if no value exists yet.",
                    row_no, aspect
                ),
            ),
        };
        DocFinding::new_with_line("", defect.line(), rule_code, violation, message)
            .with_reason(reason, fix)
    }

    /// Crosslink check: every required link target must appear as a link in
    /// the document.
    fn check_crosslinks(
        document: &DocSource,
        context: &DocAuditContext,
        code: &'static str,
        findings: &mut Vec<DocFinding>,
    ) {
        let name = context.document_name(document);
        for &link in required_links(name) {
            if !document.text.contains(&format!("[{link}")) {
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

    /// H3 sweep: every H3 under Design Requirements must match the DS-ID
    /// pattern; Design Overview and Reference sanction no H3.
    fn check_design_h3s(text: &str, findings: &mut Vec<DocFinding>) {
        let Some(re) = ds_id_heading_re() else {
            return;
        };
        let mut in_design_reqs = false;
        let mut in_design_overview = false;
        let mut in_reference = false;
        for line in text.lines() {
            let trimmed = line.trim();
            if let Some(title) = trimmed.strip_prefix("## ") {
                let norm = title.to_lowercase();
                in_design_reqs = norm == "design requirements";
                in_design_overview = norm == "design overview";
                in_reference = norm == "reference";
                continue;
            }
            if let Some(title) = trimmed.strip_prefix("### ") {
                if in_design_overview || in_reference {
                    findings.push(DocFinding::new_with_line(
                        "",
                        0,
                        consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                        consts::TEMPLATE_CONFORMANCE_VIOLATION_H3_UNEXPECTED,
                        format!("Design has an H3 `{title}` where the template sanctions none"),
                    ));
                    continue;
                }
                if in_design_reqs && !re.is_match(trimmed) {
                    findings.push(DocFinding::new_with_line(
                        "",
                        0,
                        consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                        consts::TEMPLATE_CONFORMANCE_VIOLATION_H3_UNEXPECTED,
                        format!(
                            "Design Requirements H3 `{title}` does not match the DS-ID pattern"
                        ),
                    ));
                }
            }
        }
    }
}
