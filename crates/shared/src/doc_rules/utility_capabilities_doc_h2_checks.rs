// PURPOSE: H2-level table-shape checks for AES606 template conformance.
// Extracted from capabilities_doc_template_conformance_checker.rs to keep
// that file under the AES301 1000-line file-size limit. Pure functions — no
// checker state — so they can be unit-tested independently. Lives in the
// shared utility layer so the capabilities layer can consume it without
// importing itself (AES201).

use crate::taxonomy_doc_rules_constant as consts;
use crate::taxonomy_doc_rules_request::DocFinding;
use crate::taxonomy_doc_rules_violation_names::DocViolationNames;
use crate::taxonomy_doc_section_vo::Section;
use crate::utility_markdown_scanner::normalize_heading;

/// H2-level: the section titled *title* must carry a table whose header
/// covers *columns*. Finding (if any) anchors to the H2 line.
pub fn check_h2_table(
    rule_code: &'static str,
    parsed: &[Section],
    title: &str,
    columns: &[&str],
    names: &DocViolationNames,
) -> Vec<DocFinding> {
    check_section_shape(rule_code, parsed, title, columns, None, names)
        .into_iter()
        .collect()
}

/// H2-level fixed-row check: the section titled *title* must carry an
/// `| Aspect | Detail |` table whose first-column row labels equal
/// *row_labels*. Finding anchors to the H2 line.
pub fn check_h2_fixed_rows(
    rule_code: &'static str,
    parsed: &[Section],
    title: &str,
    row_labels: &[&str],
    names: &DocViolationNames,
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
    if let Some(finding) = check_section_shape(
        rule_code,
        parsed,
        title,
        consts::ASPECT_DETAIL_COLUMNS,
        None,
        names,
    ) {
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
                rule_code,
                names.entry_missing_row(),
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
pub fn check_section_shape(
    rule_code: &'static str,
    parsed: &[Section],
    title: &str,
    columns: &[&str],
    h3: Option<&Section>,
    names: &DocViolationNames,
) -> Option<DocFinding> {
    let section = match h3 {
        Some(h3) => h3,
        None => parsed
            .iter()
            .find(|s| s.level == 2 && normalize_heading(&s.title) == normalize_heading(title))?,
    };
    let body = &section.body;
    if has_any_table(body) {
        // Exact-header rule: the template fixes the header set, so a cell
        // outside it is an extra column a reader cannot map to a template
        // column.
        if let Some(extras) = header_extra_columns(body, columns) {
            let marker = if section.level == 2 { "##" } else { "###" };
            return Some(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    rule_code,
                    names.table_extra_columns(),
                    format!(
                        "line {} `{marker} {}` table carries column(s) outside the template: {}",
                        section.line,
                        section.title,
                        extras.join(", ")
                    ),
                )
                .with_reason(
                    format!(
                        "The HOW-TO template fixes the header of `{}` to exactly {}; an undeclared column breaks the fixed shape.",
                        section.title,
                        columns.join(" | ")
                    ),
                    format!(
                        "Remove the extra column(s) ({}) from `{marker} {}` or fold the data into an existing column.",
                        extras.join(", "),
                        section.title,
                    ),
                ),
            );
        }
        // Right columns, but a data row copying the header means the
        // table ships placeholders where the template expects values.
        if table_repeats_header_row(body) {
            let marker = if section.level == 2 { "##" } else { "###" };
            return Some(
                DocFinding::new_with_line(
                    "",
                    section.line,
                    rule_code,
                    names.table_header_repeated(),
                    format!(
                        "line {} `{marker} {}` repeats its header row as a data row",
                        section.line, section.title
                    ),
                )
                .with_reason(
                    format!(
                        "The HOW-TO template mandates one header row under `{}`; repeating it in a data-row position ships the column names where filled values belong.",
                        section.title
                    ),
                    format!(
                        "Replace the duplicated `| {} |` row under `{marker} {}` with real values, or delete it if no values exist yet.",
                        columns.join(" | "),
                        section.title
                    ),
                ),
            );
        }
        return None;
    }
    // The section is present but wrong-shaped. Distinguish "no table at all"
    // from "table missing columns" so the fix message can be specific: a
    // markdown table separator line (`| --- |`) is what turns a header row
    // into a parseable table.
    let violation = if has_any_table(body) {
        names.table_missing_columns()
    } else {
        names.section_not_table()
    };
    let marker = if section.level == 2 { "##" } else { "###" };
    Some(
        DocFinding::new_with_line(
            "",
            section.line,
            rule_code,
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

// ─── Pure markdown-table scanner helpers (inlined; AES201 forbids utility→utility) ───

// PURPOSE: Pure markdown-table scanner helpers for AES606 template
// conformance. No document-model logic — only `|`-line parsing.
// Lives in the shared utility layer so the capabilities layer can consume it
// without importing itself (AES201).

use std::collections::HashSet;

/// The body carries a markdown table: a `|`-prefixed header row followed by a
/// table separator row. Header rows alone are not tables in markdown.
pub fn has_any_table(body: &str) -> bool {
    let lines = body.lines().collect::<Vec<_>>();
    lines.iter().enumerate().any(|(i, line)| {
        if !line.trim().starts_with('|') || i == 0 || !is_table_separator(line) {
            return false;
        }
        lines[..i].iter().any(|prev| prev.trim().starts_with('|'))
    })
}

/// A line that is a markdown table separator row: `| --- |`, `|:---:|`, etc.
pub fn is_table_separator(line: &str) -> bool {
    let t = line.trim();
    if !t.starts_with('|') {
        return false;
    }
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

/// Does any table in *body* repeat its header row in a data-row position?
pub fn table_repeats_header_row(body: &str) -> bool {
    let mut header: Option<Vec<String>> = None;
    let mut separator_seen = false;
    for line in body.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            header = None;
            separator_seen = false;
            continue;
        }
        let cells: Vec<String> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().to_lowercase())
            .collect();
        if is_table_separator(trimmed) {
            separator_seen = header.is_some();
            continue;
        }
        match &header {
            None => header = Some(cells),
            Some(h) if separator_seen && cells == *h => return true,
            _ => {}
        }
    }
    false
}

/// The header cells of the first table row that carries every *columns* entry
/// but also ships cells outside that fixed set, lowercased.
pub fn header_extra_columns(body: &str, columns: &[&str]) -> Option<Vec<String>> {
    for line in body.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }
        let cells: Vec<String> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().to_lowercase())
            .collect();
        if !columns
            .iter()
            .all(|want| cells.iter().any(|cell| cell.contains(&want.to_lowercase())))
        {
            continue;
        }
        let extras: Vec<String> = cells
            .iter()
            .filter(|cell| !columns.iter().any(|c| cell.contains(&c.to_lowercase())))
            .cloned()
            .collect();
        return (!extras.is_empty()).then_some(extras);
    }
    None
}

/// First-column cell of every data row in a markdown table body
pub fn first_column_values(body: &str) -> HashSet<String> {
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

/// The (missing, extra) row labels of *values* against the fixed *row_labels*.
pub fn row_label_diff<'a>(
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
