// PURPOSE: Shared line-drift finding converter for AES601–AES610 capabilities.

use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
use shared_doc_rules::taxonomy_doc_rules_violation_names::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_doc_common_checks::LineDriftDefect;

/// Convert line-drift defects into `DocFinding`s stamped with `doc_name` and
/// `code`. Every doc capability calls this so the message text names the doc.
pub fn line_drift_findings(
    defects: Vec<LineDriftDefect>,
    doc_name: &str,
    code: &'static str,
) -> Vec<DocFinding> {
    defects
        .into_iter()
        .map(|defect| {
            let (violation, message, reason, fix) = match &defect {
                LineDriftDefect::SectionNotInTemplate { line: _, section } => (
                    DOC_VIOLATION_NAMES.line_drift(),
                    format!("{doc_name} section `{section}` is not in the template"),
                    format!("{doc_name} carries a section the template does not declare."),
                    format!("Remove the off-template section `{section}`."),
                ),
                LineDriftDefect::MissingTemplateSection { line: _, section } => (
                    DOC_VIOLATION_NAMES.line_drift(),
                    format!("{doc_name} is missing the template section `{section}`"),
                    format!("{doc_name} must keep every template section."),
                    format!("Add the `{section}` section to {doc_name}."),
                ),
                LineDriftDefect::LineMismatch {
                    line: _,
                    section,
                    expected,
                    found,
                } => (
                    DOC_VIOLATION_NAMES.line_drift(),
                    format!(
                        "{doc_name} section `{section}` line drift: expected `{expected}` found `{found}`"
                    ),
                    format!("{doc_name} section `{section}` must match the template line-by-line."),
                    format!("Align `{section}` in {doc_name} with the template."),
                ),
                LineDriftDefect::H1NoTitle { line: _ } => (
                    DOC_VIOLATION_NAMES.line_drift(),
                    format!("{doc_name} H1 has no title"),
                    format!("{doc_name} H1 heading carries no title."),
                    format!("Give the {doc_name} H1 a title."),
                ),
            };
            DocFinding::new_with_line("", defect.line(), code, violation, message)
                .with_reason(reason, fix)
        })
        .collect()
}
