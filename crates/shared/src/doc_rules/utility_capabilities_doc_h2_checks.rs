// PURPOSE: H2-level table-shape detection for AES606 template conformance.
// Extracted from capabilities_doc_template_conformance_checker.rs to keep
// that file under the AES301 1000-line file-size limit. Pure functions — no
// checker state, no `DocFinding` construction: the calling capability
// formats its own violation message, reason, and fix with its own rule
// code. Lives in the shared utility layer so the capabilities layer can
// consume it without importing itself (AES201).

use crate::taxonomy_doc_section_vo::Section;
use crate::utility_markdown_scanner::normalize_heading;
use std::collections::HashSet;

/// Why a section's table shape failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionShapeDefectKind {
    /// Table ships columns outside the template's fixed header set.
    ExtraColumns,
    /// A data row repeats the header row.
    HeaderRepeated,
    /// Table is present but missing required column(s).
    MissingColumns,
    /// The section body carries no table at all.
    NotTable,
}

/// A detected table-shape defect in a section.
#[derive(Debug, Clone)]
pub struct SectionShapeDefect {
    pub line: usize,
    pub marker: &'static str,
    pub title: String,
    pub columns: Vec<String>,
    pub kind: SectionShapeDefectKind,
    pub extra_columns: Vec<String>,
}

/// A detected fixed-row defect in an H2-level `| Aspect | Detail |` table.
#[derive(Debug, Clone)]
pub struct H2FixedRowsDefect {
    pub line: usize,
    pub title: String,
    pub missing: Vec<String>,
    pub extra: Vec<String>,
}

/// H2-level: the section titled *title* must carry a table whose header
/// covers *columns*. Returns the defect (if any) anchored to the H2 line.
pub fn detect_h2_table(
    parsed: &[Section],
    title: &str,
    columns: &[&str],
) -> Option<SectionShapeDefect> {
    detect_section_shape(parsed, title, columns, None)
}

/// H2-level fixed-row check: the section titled *title* must carry an
/// `| Aspect | Detail |` table whose first-column row labels equal
/// *row_labels*. Returns the defect (if any) anchored to the H2 line.
pub fn detect_h2_fixed_rows(
    parsed: &[Section],
    title: &str,
    row_labels: &[&str],
) -> Option<H2FixedRowsDefect> {
    let Some(section) = parsed
        .iter()
        .find(|s| s.level == 2 && normalize_heading(&s.title) == normalize_heading(title))
    else {
        return None;
    };
    let values = first_column_values(&section.body);
    let (missing, extra) = row_label_diff(row_labels, &values);
    if !missing.is_empty() || !extra.is_empty() {
        return Some(H2FixedRowsDefect {
            line: section.line,
            title: section.title.clone(),
            missing: missing.into_iter().map(String::from).collect(),
            extra,
        });
    }
    None
}

/// Find the H2 section titled *title* (or use *h3* when given), check that
/// its body carries a table with every column in *columns*, and return the
/// defect when the shape is wrong. Returns `None` when the section is
/// absent or when the shape conforms. When *h3* is `Some`, the check anchors
/// to that H3; H2-level calls pass `None`.
pub fn detect_section_shape(
    parsed: &[Section],
    title: &str,
    columns: &[&str],
    h3: Option<&Section>,
) -> Option<SectionShapeDefect> {
    let section = match h3 {
        Some(h3) => h3,
        None => parsed
            .iter()
            .find(|s| s.level == 2 && normalize_heading(&s.title) == normalize_heading(title))?,
    };
    let body = &section.body;
    let marker = if section.level == 2 { "##" } else { "###" };
    let cols_owned: Vec<String> = columns.iter().map(|&s| s.to_string()).collect();

    if has_any_table(body) {
        // Exact-header rule: the template fixes the header set, so a cell
        // outside it is an extra column a reader cannot map to a template
        // column.
        if let Some(extras) = header_extra_columns(body, columns) {
            return Some(SectionShapeDefect {
                line: section.line,
                marker,
                title: section.title.clone(),
                columns: cols_owned,
                kind: SectionShapeDefectKind::ExtraColumns,
                extra_columns: extras,
            });
        }
        // Right columns, but a data row copying the header means the
        // table ships placeholders where the template expects values.
        if table_repeats_header_row(body) {
            return Some(SectionShapeDefect {
                line: section.line,
                marker,
                title: section.title.clone(),
                columns: cols_owned,
                kind: SectionShapeDefectKind::HeaderRepeated,
                extra_columns: Vec::new(),
            });
        }
        return None;
    }
    // The section is present but wrong-shaped. Distinguish "no table at all"
    // from "table missing columns" so the fix message can be specific: a
    // markdown table separator line (`| --- |`) turns a header row into a
    // parseable table.
    let kind = if has_any_table(body) {
        SectionShapeDefectKind::MissingColumns
    } else {
        SectionShapeDefectKind::NotTable
    };
    Some(SectionShapeDefect {
        line: section.line,
        marker,
        title: section.title.clone(),
        columns: cols_owned,
        kind,
        extra_columns: Vec::new(),
    })
}

// ─── Pure markdown-table scanner helpers (inlined; AES201 forbids utility→utility) ───

/// The body carries a markdown table: a `|`-prefixed header row followed by a
/// table separator row.
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
