// PURPOSE: H4 aspect entry-table detection for AES606 template conformance.
// Shared by the FRD and PRD per-doc capabilities: every item H3 under
// `Functional Requirements` / `Project Requirements` mandates per-aspect H4
// subsections whose `| No | Detail |` tables carry contiguous numbering and
// the bounded/open-ended row count the template fixes. Pure functions — no
// checker state — so both capabilities consume the same decision and cannot
// drift. Lives in the shared utility layer (AES201: capability does not
// import capability). Pure defect data — no DocFinding construction or
// rule_code parameter.

use crate::taxonomy_doc_section_vo::Section;
use crate::utility_capabilities_doc_h2_checks::{
    first_column_values, has_any_table, header_extra_columns,
};
use crate::utility_markdown_scanner::{h4_subsections, normalize_heading};

// ─── Defect data ─────────────────────────────────────────────────────────────

/// A defect detected by `detect_aspect_entry_tables` / `detect_aspect_table`.
#[derive(Debug, Clone)]
pub enum AspectEntryDefect {
    /// Expected aspect H4 is missing under the entry H3.
    MissingAspect {
        line: usize,
        h3_title: String,
        aspect: String,
    },
    /// H4 aspect subsections are out of canonical order under the entry H3.
    OrderMismatch {
        line: usize,
        h3_title: String,
        aspects: Vec<String>,
    },
    /// PRD aspect H4 carries an empty body.
    EmptyBody {
        line: usize,
        aspect: String,
        h3_title: String,
    },
    /// Aspect H4 body is not a markdown table.
    NotATable {
        line: usize,
        aspect: String,
        h3_title: String,
    },
    /// Aspect H4 table header is missing required columns.
    HeaderMissing { line: usize, aspect: String },
    /// Aspect H4 table header carries extra columns.
    HeaderExtra {
        line: usize,
        aspect: String,
        extras: Vec<String>,
    },
    /// Rows are not numbered 1..N contiguously.
    RowNumbering {
        line: usize,
        aspect: String,
        h3_title: String,
    },
    /// Bounded aspect row count differs from seed_min.
    RowCount {
        line: usize,
        aspect: String,
        h3_title: String,
        total: usize,
        seed_min: usize,
    },
    /// Open-ended aspect row count is below its minimum.
    RowMinimum {
        line: usize,
        aspect: String,
        h3_title: String,
        total: usize,
        min: usize,
    },
    /// A numbered row's Detail cell is empty or a placeholder dash.
    EmptyDetailCell {
        line: usize,
        aspect: String,
        row_no: String,
    },
}

impl AspectEntryDefect {
    /// The anchor line of this defect, for `DocFinding` construction by the
    /// calling capability.
    pub fn line(&self) -> usize {
        match self {
            AspectEntryDefect::MissingAspect { line, .. }
            | AspectEntryDefect::OrderMismatch { line, .. }
            | AspectEntryDefect::EmptyBody { line, .. }
            | AspectEntryDefect::NotATable { line, .. }
            | AspectEntryDefect::HeaderMissing { line, .. }
            | AspectEntryDefect::HeaderExtra { line, .. }
            | AspectEntryDefect::RowNumbering { line, .. }
            | AspectEntryDefect::RowCount { line, .. }
            | AspectEntryDefect::RowMinimum { line, .. }
            | AspectEntryDefect::EmptyDetailCell { line, .. } => *line,
        }
    }
}

// ─── detect functions ────────────────────────────────────────────────────────

/// Walk every entry H3 under *parent_h2* whose title matches *entry_re* and
/// detect per-aspect H4 subsection defects: presence and canonical order of
/// *aspects*, each H4's `| No | Detail |` table shape, contiguous `1..N`
/// row numbering, bounded/open-ended row counts against *seed_min*, and
/// non-empty `Detail` cells.
///
/// When *is_prd* is true, PRD aspects are free-form prose/tables per the
/// HOW-TO-MAKE-PRD template: only presence and non-empty body are enforced.
/// When false (FRD), the strict `| No | Detail |` aspect-table contract is
/// enforced. A legacy entry carrying a single `| Aspect | Detail |` table is
/// routed to `detect_entry_tables` instead.
pub fn detect_aspect_entry_tables(
    parsed: &[Section],
    parent_h2: &str,
    entry_re: &regex::Regex,
    aspects: &[&str],
    open: &[&str],
    seed_min: &[(&str, usize)],
    is_prd: bool,
) -> Vec<AspectEntryDefect> {
    let mut defects = Vec::new();
    let Some(parent) = parsed
        .iter()
        .find(|s| s.level == 2 && normalize_heading(&s.title) == normalize_heading(parent_h2))
    else {
        return defects;
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
                defects.push(AspectEntryDefect::MissingAspect {
                    line: h3.line,
                    h3_title: h3.title.clone(),
                    aspect: aspect.to_string(),
                });
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
            defects.push(AspectEntryDefect::OrderMismatch {
                line: mismatch_line,
                h3_title: h3.title.clone(),
                aspects: aspects.iter().map(|&a| a.to_string()).collect(),
            });
        }

        // Per-aspect body checks.
        for aspect in aspects {
            let Some(h4) = h4s.iter().find(|s| s.title == *aspect) else {
                continue;
            };
            if is_prd {
                if h4.body.trim().is_empty() {
                    defects.push(AspectEntryDefect::EmptyBody {
                        line: h4.line,
                        aspect: aspect.to_string(),
                        h3_title: h3.title.clone(),
                    });
                }
                continue;
            }
            let Some(min) = seed_min.iter().find(|(n, _)| n == aspect).map(|&(_, m)| m) else {
                continue;
            };
            let open_min = open.contains(aspect).then_some(min);
            defects.extend(detect_aspect_table(&h3, h4, aspect, open_min, min));
        }
    }
    defects
}

/// The entry H3s under *parent* whose titles match *entry_re*, excluding any
/// legacy entry that already carries a single `| Aspect | Detail |` table
/// (that shape is handled by `detect_entry_tables` instead of the H4 aspect
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
pub fn detect_aspect_table(
    h3: &Section,
    h4: &Section,
    aspect: &str,
    open: Option<usize>,
    seed_min: usize,
) -> Vec<AspectEntryDefect> {
    let mut defects = Vec::new();
    if !has_any_table(&h4.body) {
        defects.push(AspectEntryDefect::NotATable {
            line: h4.line,
            aspect: aspect.to_string(),
            h3_title: h3.title.clone(),
        });
        return defects;
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
        return defects;
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
    if !missing.is_empty() {
        defects.push(AspectEntryDefect::HeaderMissing {
            line: h4.line,
            aspect: aspect.to_string(),
        });
        return defects;
    }
    if !extras.is_empty() {
        defects.push(AspectEntryDefect::HeaderExtra {
            line: h4.line,
            aspect: aspect.to_string(),
            extras: extras.iter().map(|s| s.to_string()).collect(),
        });
        return defects;
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
        defects.push(AspectEntryDefect::RowNumbering {
            line: h4.line,
            aspect: aspect.to_string(),
            h3_title: h3.title.clone(),
        });
    }

    let total = numbered.len();
    let count_violation = match open {
        Some(min) => total < min,
        None => total != seed_min,
    };
    if count_violation {
        match open {
            Some(min) => defects.push(AspectEntryDefect::RowMinimum {
                line: h4.line,
                aspect: aspect.to_string(),
                h3_title: h3.title.clone(),
                total,
                min,
            }),
            None => defects.push(AspectEntryDefect::RowCount {
                line: h4.line,
                aspect: aspect.to_string(),
                h3_title: h3.title.clone(),
                total,
                seed_min,
            }),
        }
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
        defects.push(AspectEntryDefect::EmptyDetailCell {
            line: h4.line,
            aspect: aspect.to_string(),
            row_no: no.clone(),
        });
    }

    defects
}
