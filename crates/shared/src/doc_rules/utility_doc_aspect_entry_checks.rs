// PURPOSE: H4 aspect entry-table checks for AES606 template conformance.
// Shared by the FRD and PRD per-doc capabilities: every item H3 under
// `Functional Requirements` / `Project Requirements` mandates per-aspect H4
// subsections whose `| No | Detail |` tables carry contiguous numbering and
// the bounded/open-ended row count the template fixes. Pure functions — no
// checker state — so both capabilities consume the same decision and cannot
// drift. Lives in the shared utility layer (AES201: capability does not
// import capability).
use crate::taxonomy_doc_rules_constant as consts;
use crate::taxonomy_doc_rules_request::DocFinding;
use crate::taxonomy_doc_section_vo::Section;
use crate::utility_capabilities_doc_h2_checks::{
    first_column_values, has_any_table, header_extra_columns,
};
use crate::utility_markdown_scanner::{h4_subsections, normalize_heading};

/// Walk every entry H3 under *parent_h2* whose title matches *entry_re* and
/// verify the per-aspect H4 subsections: presence and canonical order of
/// *aspects*, each H4's `| No | Detail |` table shape, contiguous `1..N`
/// row numbering, bounded/open-ended row counts against *seed_min*, and
/// non-empty `Detail` cells. Findings anchor to the offending H3 or H4 line.
///
/// When *is_prd* is true, PRD aspects are free-form prose/tables per the
/// HOW-TO-MAKE-PRD template: only presence and non-empty body are enforced.
/// When false (FRD), the strict `| No | Detail |` aspect-table contract is
/// enforced. A legacy entry carrying a single `| Aspect | Detail |` table is
/// routed to `check_entry_tables` instead.
pub fn check_aspect_entry_tables(
    rule_code: &'static str,
    parsed: &[Section],
    parent_h2: &str,
    entry_re: &regex::Regex,
    aspects: &[&str],
    open: &[&str],
    seed_min: &[(&str, usize)],
    is_prd: bool,
) -> Vec<DocFinding> {
    let mut findings = Vec::new();
    let Some(parent) = parsed
        .iter()
        .find(|s| s.level == 2 && normalize_heading(&s.title) == normalize_heading(parent_h2))
    else {
        return findings;
    };
    for h3 in h4_candidate_entries(parsed, parent, entry_re) {
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
                        rule_code,
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
                    rule_code,
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
            if is_prd {
                // PRD aspect: require the H4 to exist (checked above) and
                // carry a non-empty body. No table-shape, numbering, or
                // seed-min count enforcement.
                if h4.body.trim().is_empty() {
                    findings.push(
                        DocFinding::new_with_line(
                            "",
                            h4.line,
                            rule_code,
                            consts::TEMPLATE_CONFORMANCE_VIOLATION_EMPTY_DETAIL_CELL,
                            format!(
                                "line {} `#### {}` under `### {}` has an empty body",
                                h4.line, aspect, h3.title
                            ),
                        )
                        .with_reason(
                            "Each PRD aspect H4 must carry real content.",
                            "Fill `#### {aspect}` with prose, a table, or a numbered list.",
                        ),
                    );
                }
                continue;
            }
            let Some(min) = seed_min.iter().find(|(n, _)| n == aspect).map(|&(_, m)| m) else {
                continue;
            };
            let open_min = open.contains(aspect).then_some(min);
            findings.extend(check_aspect_table(rule_code, &h3, h4, aspect, open_min, min));
        }
    }
    findings
}

/// The entry H3s under *parent* whose titles match *entry_re*, excluding any
/// legacy entry that already carries a single `| Aspect | Detail |` table
/// (that shape is handled by `check_entry_tables` instead of the H4 aspect
/// template). PRD entries never use the legacy shape.
fn h4_candidate_entries(
    _parsed: &[Section],
    parent: &Section,
    entry_re: &regex::Regex,
) -> Vec<Section> {
    let mut out = Vec::new();
    for h3 in crate::utility_markdown_scanner::h3_subsections(parent) {
        if !entry_re.is_match(&h3.title) {
            continue;
        }
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
                continue; // legacy Aspect|Detail table; not an H4-aspect entry
            }
        }
        out.push(h3);
    }
    out
}

/// One aspect H4's `| No | Detail |` table: presence, exact header,
/// contiguous numbering, row count (bounded exact / open-ended minimum),
/// and non-empty `Detail` cells.
///
/// `open`: `None` = bounded exact `seed_min`; `Some(min)` = open-ended,
/// at least `min` rows (numbered rows only — `∞` is not written as a marker row).
pub fn check_aspect_table(
    rule_code: &'static str,
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
                rule_code,
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
                rule_code,
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
                rule_code,
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
                rule_code,
                violation,
                message,
            )
            .with_reason(
                "The open/bounded contract fixes each aspect's minimum or exact seed row count.",
                fix,
            ),
        );
    }

    // Empty or placeholder Detail cells on every numbered row: a lone dash
    // cell carries no content, so it fires like an empty one.
    for (no, detail) in rows.iter() {
        let trimmed = detail.trim();
        if !trimmed.is_empty() && !matches!(trimmed, "—" | "–" | "-") {
            continue;
        }
        if !no.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            continue;
        }
        findings.push(
            DocFinding::new_with_line(
                "",
                h4.line,
                rule_code,
                consts::TEMPLATE_CONFORMANCE_VIOLATION_EMPTY_DETAIL_CELL,
                format!(
                    "line {} `#### {}` row {no} Detail cell is empty or a placeholder dash",
                    h4.line, aspect
                ),
            )
            .with_reason(
                "Every numbered row's Detail cell must carry real content; a lone `—` is a placeholder, not content. A second row belongs in the table only when a second fact exists.",
                "Fill the Detail cell with real content, or remove the row.",
            ),
        );
    }

    findings
}
