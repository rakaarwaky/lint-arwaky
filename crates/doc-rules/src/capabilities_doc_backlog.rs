// PURPOSE: Per-doc capability for BACKLOG.md — AES607
//
// Answers ALL invariants scoped to BACKLOG.md behind IBacklogDocProtocol.
// ID-level alignment, role-aware crosslinks, heading structure, and
// entry-table conformance are stamped with RULE_CODE_BACKLOG_DOC
// ("AES607"). The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IBacklogDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_names::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_doc_aspect_entry_checks::{
    AspectEntryDefect, detect_aspect_entry_tables,
};
use shared_doc_rules::utility_doc_common_checks::{
    backlog_required_links, collect_union_id_set, detect_doc_heading, detect_doc_line_drift,
    missing_link_violation_type,
};
use shared_doc_rules::utility_doc_entry_regexes::backlog_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::collect_any_id_set;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
pub struct BacklogCapability;

impl BacklogCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IBacklogDocProtocol for BacklogCapability {
    fn audit_backlog(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::BACKLOG_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_BACKLOG_DOC;
            }
        }
        findings
    }
}

impl BacklogCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // Template body conformance.
        let drift = detect_doc_line_drift(text, consts::BACKLOG_TEMPLATE_LINES);
        findings.extend(crate::utility_doc_line_drift::line_drift_findings(
            drift,
            consts::BACKLOG_DOC,
            consts::RULE_CODE_BACKLOG_DOC,
        ));

        // BACKLOG ID-level alignment against its paired spec union.
        Self::check_backlog_alignment(document, context, findings);

        // Role-aware crosslinks.
        Self::check_crosslinks(document, context, consts::RULE_CODE_BACKLOG_DOC, findings);

        // Heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::BACKLOG_DOC) {
            for defect in detect_doc_heading(text, consts::BACKLOG_DOC, required, allowed) {
                let (violation, message, reason, fix) = match &defect {
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H1Count {
                        line: _,
                        count,
                    } => (
                        DOC_VIOLATION_NAMES.h1_count_violation(),
                        format!(
                            "{} opens with {} level-1 headings",
                            consts::BACKLOG_DOC,
                            count
                        ),
                        format!(
                            "{} must open with exactly one H1 naming the project",
                            consts::BACKLOG_DOC
                        ),
                        format!(
                            "Keep exactly one `# {}` line at the top of {}",
                            consts::BACKLOG_DOC,
                            consts::BACKLOG_DOC
                        ),
                    ),
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H2Missing {
                        line: _,
                        missing,
                    } => (
                        DOC_VIOLATION_NAMES.h2_missing(),
                        format!(
                            "{} has no H2 heading for {}",
                            consts::BACKLOG_DOC,
                            missing.join(", ")
                        ),
                        format!(
                            "{} needs a level-2 heading for every template-required section",
                            consts::BACKLOG_DOC
                        ),
                        format!(
                            "Add the missing `## {}` section(s) to {}",
                            missing.join(" `## `"),
                            consts::BACKLOG_DOC
                        ),
                    ),
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H2Unexpected {
                        line: _,
                        unexpected,
                    } => (
                        DOC_VIOLATION_NAMES.h2_unexpected(),
                        format!(
                            "{} carries H2 heading(s) outside the template: {}",
                            consts::BACKLOG_DOC,
                            unexpected.join(", ")
                        ),
                        format!(
                            "{} must not carry H2 headings the template does not declare",
                            consts::BACKLOG_DOC
                        ),
                        format!(
                            "Remove or rename the unexpected heading(s): {}",
                            unexpected.join(", ")
                        ),
                    ),
                };
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        defect.line(),
                        consts::RULE_CODE_BACKLOG_DOC,
                        violation,
                        message,
                    )
                    .with_reason(reason, fix),
                );
            }
        }

        // Entry tables under Backlog Status.
        if let Some(re) = backlog_entry_re() {
            for defect in detect_aspect_entry_tables(
                &parsed,
                "Backlog Status",
                re,
                consts::BACKLOG_ROW_LABELS,
                consts::BACKLOG_ROW_LABELS,
                &[
                    ("State", 1),
                    ("Done", 1),
                    ("In Progress", 1),
                    ("Risk", 1),
                    ("Updated", 1),
                ],
                false,
            ) {
                findings.push(Self::convert_aspect_entry_defect(defect));
            }
        }
    }

    fn convert_aspect_entry_defect(defect: AspectEntryDefect) -> DocFinding {
        Self::convert_aspect_entry_defect_common(defect, consts::RULE_CODE_BACKLOG_DOC)
    }

    fn convert_aspect_entry_defect_common(
        defect: AspectEntryDefect,
        rule_code: &'static str,
    ) -> DocFinding {
        let (violation, message, reason, fix) = match &defect {
            AspectEntryDefect::MissingAspect {
                line,
                h3_title,
                aspect,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
                format!(
                    "line {} `### {}` missing aspect H4 `#### {}`",
                    line, h3_title, aspect
                ),
                format!(
                    "The HOW-TO template requires every entry H3 under `### {}` to carry a `#### {}` subsection.",
                    h3_title, aspect
                ),
                format!("Add `#### {}` under `### {}`.", aspect, h3_title),
            ),
            AspectEntryDefect::OrderMismatch {
                line,
                h3_title,
                aspects,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_H4_ORDER,
                format!(
                    "line {} `### {}` aspect H4s out of canonical order: expected {}",
                    line,
                    h3_title,
                    aspects.join(", ")
                ),
                format!(
                    "Reorder the `####` aspect subsections under `### {}` to match the template order: {}.",
                    h3_title,
                    aspects.join(", ")
                ),
                format!(
                    "Move the aspect H4s under `### {}` into canonical order: {}.",
                    h3_title,
                    aspects.join(", ")
                ),
            ),
            AspectEntryDefect::EmptyBody {
                line,
                aspect,
                h3_title,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
                format!(
                    "line {} `#### {}` under `### {}` carries an empty body",
                    line, aspect, h3_title
                ),
                format!(
                    "Every PRD aspect `#### {}` under `### {}` must carry non-empty content.",
                    aspect, h3_title
                ),
                format!("Add content to `#### {}` under `### {}`.", aspect, h3_title),
            ),
            AspectEntryDefect::NotATable {
                line,
                aspect,
                h3_title,
            } => (
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
            AspectEntryDefect::HeaderExtra {
                line,
                aspect,
                extras,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_EXTRA_COLUMNS,
                format!(
                    "line {} `#### {}` aspect table carries extra column(s): {}",
                    line,
                    aspect,
                    extras.join(", ")
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
            AspectEntryDefect::RowNumbering {
                line,
                aspect,
                h3_title,
            } => (
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
            AspectEntryDefect::RowCount {
                line,
                aspect,
                h3_title,
                total,
                seed_min,
            } => (
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
            AspectEntryDefect::RowMinimum {
                line,
                aspect,
                h3_title,
                total,
                min,
            } => (
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
            AspectEntryDefect::EmptyDetailCell {
                line,
                aspect,
                row_no,
            } => (
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

    /// BACKLOG ID-level alignment: entry H3 tokens must match declared IDs of
    /// the paired spec union, and vice versa.
    fn check_backlog_alignment(
        document: &DocSource,
        context: &DocAuditContext,
        findings: &mut Vec<DocFinding>,
    ) {
        // Build the union from any FRD/DATA/DESIGN present in the same
        // folder; fall back to root PRD/ROADMAP.
        let union = collect_union_id_set(
            collect_any_id_set,
            context.documents().iter().find(|d| {
                context.document_name(d) == consts::FRD_DOC
                    || context.document_name(d) == consts::DATA_DOC
                    || context.document_name(d) == consts::DESIGN_DOC
            }),
            context.documents().iter().find(|d| {
                context.document_name(d) == consts::PRD_DOC
                    || context.document_name(d) == consts::ROADMAP_DOC
            }),
        );

        let defects =
            shared_doc_rules::utility_doc_common_checks::detect_backlog_qualifier_alignment(
                document,
                union.as_ref(),
            );
        for defect in defects {
            let defect_line = defect.line();
            let (violation, message, reason, fix) = match defect {
                shared_doc_rules::utility_doc_common_checks::BacklogQualifierDefect::TokenNotDeclared {
                    line, token, valid,
                } => {
                    let code = if context.is_root_document(document) {
                        consts::PR_BACKLOG_ID_MISSING
                    } else {
                        consts::BACKLOG_QUALIFIER_MISMATCH
                    };
                    (
                        code,
                        format!(
                            "line {line} BACKLOG entry `{{{token}}}` has no matching ID in the paired spec"
                        ),
                        "A BACKLOG entry traces to its spec through the ID".to_string(),
                        format!("Rename the entry to an ID the spec declares: {}.", valid.join(", ")),
                    )
                }
                shared_doc_rules::utility_doc_common_checks::BacklogQualifierDefect::IdNoEntry { line, id } => {
                    let code = if context.is_root_document(document) {
                        consts::BACKLOG_ID_NOT_PR
                    } else {
                        consts::DT_BACKLOG_ID_NOT_DT
                    };
                    (
                        code,
                        format!("Spec ID `{id}` has no BACKLOG entry at line {line}"),
                        "Every declared spec ID needs a matching BACKLOG entry".to_string(),
                        format!("Add a BACKLOG entry `{{{id}}}`"),
                    )
                }
                shared_doc_rules::utility_doc_common_checks::BacklogQualifierDefect::StrayId {
                    line, token, id, valid,
                } => (
                    consts::BACKLOG_QUALIFIER_MISMATCH,
                    format!("line {line} BACKLOG entry `{{{token}}}` cites undeclared id `{id}`"),
                    "An entry can only cite IDs the paired spec declares".to_string(),
                    format!("Remove `{id}` or add it to the spec's declared IDs: {}.", valid.join(", ")),
                ),
            };
            findings.push(
                DocFinding::new_with_line(
                    "",
                    defect_line,
                    consts::RULE_CODE_BACKLOG_DOC,
                    violation,
                    message,
                )
                .with_reason(reason, fix),
            );
        }
    }

    /// Crosslink check: every required link target must appear as a link in
    /// the document, using role-aware link sets.
    fn check_crosslinks(
        document: &DocSource,
        context: &DocAuditContext,
        code: &'static str,
        findings: &mut Vec<DocFinding>,
    ) {
        let name = context.document_name(document);
        let links = backlog_required_links(name, document, context);
        for &link in &links {
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
}
