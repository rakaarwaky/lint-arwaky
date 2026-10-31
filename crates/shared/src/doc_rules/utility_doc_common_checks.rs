// PURPOSE: Shared doc-invariant mechanics used by more than one per-doc
// capability. Every function here is stateless over its inputs and returns
// pure defect data — no DocFinding construction, no format! messages.
// The calling capability stamps findings with its own AES code.
// Lives in crates/shared/src/doc_rules so no per-doc capability re-implements
// a table/shape/crosslink/mechanic a sibling also uses.

use crate::taxonomy_doc_audit_context_vo::{DocAuditContext, MemberRole};
use crate::taxonomy_doc_rules_constant as consts;
use crate::taxonomy_doc_rules_request::DocSource;
use crate::taxonomy_doc_section_vo::Section;
use std::collections::{BTreeSet, HashSet};
use std::path::Path;
use std::sync::OnceLock;

// ─── Defect data ─────────────────────────────────────────────────────────

pub use crate::taxonomy_doc_rules_defect_vo::{
    BacklogQualifierDefect, EntryTableDefect, HeadingDefect, LineDriftDefect, MemberDocDefect,
};

fn normalize_heading(title: &str) -> String {
    let lowered = title.to_lowercase();
    let stripped: String = lowered
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect();
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn heading_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(consts::HEADING_PATTERN).ok())
        .as_ref()
}

fn pr_id_heading_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(consts::PR_ID_HEADING_PATTERN).ok())
        .as_ref()
}

fn fr_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(consts::FR_ENTRY_PATTERN).ok())
        .as_ref()
}

fn blank_fenced(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_fence = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            out.push('\n');
            continue;
        }
        if in_fence {
            out.push('\n');
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn sections(text: &str) -> Vec<Section> {
    let Some(re) = heading_re() else {
        return Vec::new();
    };
    let prose = blank_fenced(text);
    let mut matches: Vec<(usize, usize, String, usize)> = Vec::new();
    for (line_idx, line) in prose.lines().enumerate() {
        if let Some(caps) = re.captures(line) {
            let hashes = caps.get(1).map_or("", |m| m.as_str());
            let title = caps.get(2).map_or("", |m| m.as_str()).trim().to_string();
            matches.push((line_idx, hashes.len(), title, line_idx));
        }
    }

    let mut result = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    for (i, &(start_line, level, ref title, _)) in matches.iter().enumerate() {
        let next_start = matches
            .iter()
            .skip(i + 1)
            .find(|(_, l, _, _)| *l <= level)
            .map_or(lines.len(), |(s, _, _, _)| *s);
        let body = lines[start_line + 1..next_start].join("\n");
        result.push(Section {
            level,
            title: title.clone(),
            body,
            line: start_line,
        });
    }
    result
}

fn h3_subsections(section: &Section) -> Vec<Section> {
    let mut out = Vec::new();
    let lines: Vec<&str> = section.body.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("### ") && !line.starts_with("#### ") {
            let title = line.trim_start_matches('#').trim().to_string();
            let start_line = section.line + i + 1;
            let mut body_lines = Vec::new();
            i += 1;
            while i < lines.len() {
                let inner = lines[i];
                if inner.starts_with('#') {
                    let level = inner.chars().take_while(|&c| c == '#').count();
                    if level <= 3 {
                        break;
                    }
                }
                body_lines.push(inner);
                i += 1;
            }
            out.push(Section {
                level: 3,
                title,
                body: body_lines.join("\n"),
                line: start_line,
            });
            continue;
        }
        i += 1;
    }
    out
}

fn h4_subsections(section: &Section) -> Vec<Section> {
    let mut out = Vec::new();
    let lines: Vec<&str> = section.body.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("#### ") {
            let title = line.trim_start_matches('#').trim().to_string();
            let start_line = section.line + i + 1;
            let mut body_lines = Vec::new();
            i += 1;
            while i < lines.len() {
                let inner = lines[i];
                if inner.starts_with('#') {
                    let level = inner.chars().take_while(|&c| c == '#').count();
                    if level <= 4 {
                        break;
                    }
                }
                body_lines.push(inner);
                i += 1;
            }
            out.push(Section {
                level: 4,
                title,
                body: body_lines.join("\n"),
                line: start_line,
            });
            continue;
        }
        i += 1;
    }
    out
}

fn is_table_separator(line: &str) -> bool {
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

fn has_any_table(body: &str) -> bool {
    let lines = body.lines().collect::<Vec<_>>();
    lines.iter().enumerate().any(|(i, line)| {
        if !line.trim().starts_with('|') || i == 0 || !is_table_separator(line) {
            return false;
        }
        lines[..i].iter().any(|prev| prev.trim().starts_with('|'))
    })
}

fn header_extra_columns(body: &str, columns: &[&str]) -> Option<Vec<String>> {
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

fn table_repeats_header_row(body: &str) -> bool {
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

fn detect_section_shape(
    parsed: &[Section],
    title: &str,
    columns: &[&str],
    h3: Option<&Section>,
) -> Option<crate::taxonomy_doc_rules_defect_vo::SectionShapeDefect> {
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
        if let Some(extras) = header_extra_columns(body, columns) {
            return Some(crate::taxonomy_doc_rules_defect_vo::SectionShapeDefect {
                line: section.line,
                marker,
                title: section.title.clone(),
                columns: cols_owned,
                kind: crate::taxonomy_doc_rules_defect_vo::SectionShapeDefectKind::ExtraColumns,
                extra_columns: extras,
            });
        }
        if table_repeats_header_row(body) {
            return Some(crate::taxonomy_doc_rules_defect_vo::SectionShapeDefect {
                line: section.line,
                marker,
                title: section.title.clone(),
                columns: cols_owned,
                kind: crate::taxonomy_doc_rules_defect_vo::SectionShapeDefectKind::HeaderRepeated,
                extra_columns: Vec::new(),
            });
        }
        return None;
    }
    let kind = if has_any_table(body) {
        crate::taxonomy_doc_rules_defect_vo::SectionShapeDefectKind::MissingColumns
    } else {
        crate::taxonomy_doc_rules_defect_vo::SectionShapeDefectKind::NotTable
    };
    Some(crate::taxonomy_doc_rules_defect_vo::SectionShapeDefect {
        line: section.line,
        marker,
        title: section.title.clone(),
        columns: cols_owned,
        kind,
        extra_columns: Vec::new(),
    })
}

fn first_column_values(body: &str) -> HashSet<String> {
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
        if is_table_separator(trimmed) {
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

fn row_label_diff(want: &[&str], have: &HashSet<String>) -> (Vec<String>, Vec<String>) {
    let mut missing = Vec::new();
    for &label in want {
        let label_norm = normalize_heading(label);
        let found = have.iter().any(|v| normalize_heading(v) == label_norm);
        if !found {
            missing.push(label.to_string());
        }
    }
    let mut extra = Vec::new();
    for label in have {
        let label_norm = normalize_heading(label);
        let matched = want.iter().any(|w| normalize_heading(w) == label_norm);
        if !matched {
            extra.push(label.clone());
        }
    }
    (missing, extra)
}

/// so the gate follows the template without hardcoding the H4 title.
pub fn detect_entry_tables(
    parsed: &[Section],
    parent_h2: &str,
    entry_re: &regex::Regex,
    columns: &[&str],
    row_labels: &[&str],
) -> Vec<EntryTableDefect> {
    let mut defects = Vec::new();
    let Some(parent) = parsed
        .iter()
        .find(|s| s.level == 2 && normalize_heading(&s.title) == normalize_heading(parent_h2))
    else {
        return defects;
    };
    for h3 in h3_subsections(parent) {
        if !entry_re.is_match(&h3.title) {
            continue;
        }
        let h4s = h4_subsections(&h3);

        // Identify the H4 (if any) that carries the template table.
        // The H4 is discovered by its table header matching *columns*, so
        // the gate is template-adaptive: renaming the H4 in the template
        // requires no code change.
        let scope_h4: Option<&Section> = if h4s.is_empty() {
            None
        } else {
            h4s.iter().find(|h4| {
                header_extra_columns(&h4.body, columns).is_none() && has_any_table(&h4.body)
            })
        };

        // Shape check: run against the H4 body when scoped, the H3 body otherwise.
        if let Some(h4) = scope_h4 {
            let scoped = Section {
                level: 3,
                title: h3.title.clone(),
                body: h4.body.clone(),
                line: h3.line + h4.line,
            };
            if let Some(shape) = detect_section_shape(parsed, &scoped.title, columns, Some(&scoped))
            {
                defects.push(EntryTableDefect::Shape(shape));
                continue;
            }
        } else if let Some(shape) = detect_section_shape(parsed, &h3.title, columns, Some(&h3)) {
            defects.push(EntryTableDefect::Shape(shape));
            continue;
        }

        // Row-label check: scoped to the matching H4 body when H4 subsections
        // exist; the H3 body otherwise.
        if !row_labels.is_empty() {
            let body = match scope_h4 {
                Some(h4) => &h4.body,
                None => &h3.body,
            };
            let values = first_column_values(body);
            let (missing, extra) = row_label_diff(row_labels, &values);
            if !missing.is_empty() || !extra.is_empty() {
                defects.push(EntryTableDefect::RowLabelsMismatch {
                    line: h3.line,
                    title: h3.title.clone(),
                    missing: missing.into_iter().map(|s| s.to_string()).collect(),
                    extra,
                });
            }
        }
    }
    defects
}

// ─── BACKLOG entries ─────────────────────────────────────────────────────

/// BACKLOG entry H3s under `## Backlog Status`: `(token, 1-based heading line,
/// stray ids in the entry's Next/Done/Blocked rows)` — one tuple per entry.
/// Returns `(entries, section_line)`; `section_line` is `0` only when no
/// `## Backlog Status` section exists.
pub fn parse_backlog_entries(backlog: &DocSource) -> (Vec<(String, usize, Vec<String>)>, usize) {
    let parsed = sections(&backlog.text);
    let Some(section) = parsed
        .iter()
        .find(|s| s.level == 2 && normalize_heading(&s.title) == "backlog status")
    else {
        return (Vec::new(), 0);
    };
    let Some(stray_re) = regex::Regex::new(r"\b((?:PR|FR|DS|DT)-[A-Za-z0-9_]+-\d+)").ok() else {
        return (Vec::new(), section.line + 1);
    };
    let mut entries: Vec<(String, usize, Vec<String>)> = Vec::new();
    for sub in h3_subsections(section) {
        let token = match extract_qualifier(&sub.title) {
            Some(t) => t,
            None => continue,
        };
        if token.is_empty() {
            continue;
        }
        let mut stray_ids = Vec::new();
        let label_row: BTreeSet<&str> = consts::BACKLOG_ROW_LABELS
            .iter()
            .filter(|l| **l != "State")
            .cloned()
            .collect();
        for raw in sub.body.lines() {
            let trimmed = raw.trim();
            if !trimmed.starts_with('|') {
                continue;
            }
            let cells: Vec<&str> = trimmed
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect();
            let Some(label) = cells.first().copied() else {
                continue;
            };
            if !label_row.contains(label) {
                continue;
            }
            let detail = cells.get(1).copied().unwrap_or("");
            for caps in stray_re.captures_iter(detail) {
                if let Some(m) = caps.get(1) {
                    stray_ids.push(m.as_str().to_string());
                }
            }
        }
        entries.push((token, sub.line + 1, stray_ids));
    }
    (entries, section.line + 1)
}

/// The single brace-wrapped ID token of a BACKLOG entry H3 title
/// (`{FR-SAMPLE-001}` → `FR-SAMPLE-001`), or `None` for any other shape.
fn extract_qualifier(title: &str) -> Option<String> {
    let inner = title.trim();
    let id = inner.split(':').next()?.trim();
    let id = id.chars().filter(|c| *c != '|').collect::<String>();
    if id.is_empty() {
        return None;
    }
    Some(id.trim().to_string())
}

/// Detect BACKLOG ID-level alignment against a paired spec union. Returns
/// pure defect data; the calling capability stamps with its own rule code
/// and chooses violation types (root vs non-root).
pub fn detect_backlog_qualifier_alignment(
    backlog: &DocSource,
    union: Option<&Vec<String>>,
) -> Vec<BacklogQualifierDefect> {
    let mut defects = Vec::new();
    let ids: Vec<String> = union.cloned().unwrap_or_default();
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let (entries, section_line) = parse_backlog_entries(backlog);
    if section_line == 0 {
        return defects;
    }

    // (a) entry token not equal to any declared ID.
    if !ids.is_empty() {
        for (token, line, _) in &entries {
            if !id_set.contains(token.as_str()) {
                let mut valid: Vec<&str> = ids.iter().map(String::as_str).collect();
                valid.sort();
                defects.push(BacklogQualifierDefect::TokenNotDeclared {
                    line: *line,
                    token: token.clone(),
                    valid: valid.into_iter().map(|s| s.to_string()).collect(),
                });
            }
        }
    }

    // (b) declared ID with no BACKLOG entry, once per ID.
    if !ids.is_empty() {
        let entry_set: HashSet<&str> = entries.iter().map(|(t, _, _)| t.as_str()).collect();
        for id in &ids {
            if !entry_set.contains(id.as_str()) {
                defects.push(BacklogQualifierDefect::IdNoEntry {
                    line: section_line,
                    id: id.clone(),
                });
            }
        }
    }

    // (c) stray id token in an entry's Next/Done/Blocked row.
    for (token, line, stray_ids) in &entries {
        for id in stray_ids {
            if id_set.contains(id.as_str()) {
                continue;
            }
            let mut valid: Vec<&str> = ids.iter().map(String::as_str).collect();
            valid.sort();
            defects.push(BacklogQualifierDefect::StrayId {
                line: *line,
                token: token.clone(),
                id: id.clone(),
                valid: valid.into_iter().map(|s| s.to_string()).collect(),
            });
        }
    }
    defects
}

// ─── ID-set helpers ──────────────────────────────────────────────────────

/// Union of the accepted ID sets of up to two documents under a shared
/// collector. `None` documents are skipped; the result is `None` when neither
/// document declares any IDs.
pub fn collect_union_id_set(
    collect: fn(&str) -> Option<Vec<String>>,
    first: Option<&DocSource>,
    second: Option<&DocSource>,
) -> Option<Vec<String>> {
    let mut ids: Vec<String> = Vec::new();
    if let Some(document) = first {
        ids.extend(collect(&document.text).unwrap_or_default());
    }
    if let Some(document) = second {
        ids.extend(collect(&document.text).unwrap_or_default());
    }
    if ids.is_empty() { None } else { Some(ids) }
}

/// Accepted FR-ID set from an FRD's `FR-<feature>-NNN:` headings.
pub fn collect_fr_id_set(frd: &str) -> Option<Vec<String>> {
    let re = fr_entry_re()?;
    let ids: Vec<String> = re
        .captures_iter(frd)
        .filter_map(|caps| caps.get(0).map(|m| m.as_str().to_string()))
        .collect();
    if ids.is_empty() { None } else { Some(ids) }
}

/// `(1-based heading line, id)` for every PR-ID heading in *text*.
pub fn pr_id_heading_lines(text: &str) -> Vec<(usize, String)> {
    let re = match pr_id_heading_re() {
        Some(r) => r,
        None => return Vec::new(),
    };
    re.captures_iter(text)
        .filter_map(|caps| {
            let id = caps.name("id")?.as_str().to_string();
            let start = caps.get(0)?.start();
            let line = text[..start].matches('\n').count() + 1;
            Some((line, id))
        })
        .collect()
}

/// 1-based line of the H2 whose normalized title is *title*, or `None` when
/// no such section exists.
pub fn section_line(text: &str, title: &str) -> Option<usize> {
    sections(text)
        .iter()
        .find(|s| s.level == 2 && normalize_heading(&s.title) == title)
        .map(|s| s.line + 1)
}

/// Extract the project identifier segment from `PR-<PROJECT>-NNN`, e.g.
/// `PR-LA-001` → `LA`. Returns `None` for any other shape.
pub fn project_identifier(id: &str) -> Option<String> {
    id.strip_prefix("PR-")?
        .split_once('-')
        .map(|(project, _)| project.to_string())
}

// ─── Cross-capability crosslink mechanics (AES604) ───────────────────────

/// The mandatory reference links per document, one entry per the AES604
/// target state; empty for README, which carries a placeholder table.
pub fn required_links(name: &str) -> &'static [&'static str] {
    match name {
        consts::AGENTS_DOC => &[
            consts::README_DOC,
            consts::CONTRIBUTING_DOC,
            consts::ARCHITECTURE_DOC,
            consts::ROADMAP_DOC,
            consts::PRD_DOC,
        ],
        consts::PRD_DOC => &[consts::ROADMAP_DOC, consts::ARCHITECTURE_DOC],
        consts::ROADMAP_DOC => &[consts::PRD_DOC, consts::ARCHITECTURE_DOC],
        consts::BACKLOG_DOC => &[
            consts::PRD_DOC,
            consts::FRD_DOC,
            consts::ROADMAP_DOC,
            consts::ARCHITECTURE_DOC,
        ],
        consts::FRD_DOC => &[
            consts::PRD_DOC,
            consts::BACKLOG_DOC,
            consts::ROADMAP_DOC,
            consts::ARCHITECTURE_DOC,
        ],
        consts::DATA_DOC => &[
            consts::PRD_DOC,
            consts::BACKLOG_DOC,
            consts::ROADMAP_DOC,
            consts::ARCHITECTURE_DOC,
        ],
        consts::DESIGN_DOC => &[
            consts::PRD_DOC,
            consts::BACKLOG_DOC,
            consts::ROADMAP_DOC,
            consts::ARCHITECTURE_DOC,
        ],
        consts::CONTRIBUTING_DOC => &[consts::PRD_DOC, consts::ARCHITECTURE_DOC],
        // README.md mandates no links; its Reference section is a
        // placeholder table. ARCHITECTURE.md is out of scope for AES604.
        _ => &[],
    }
}

/// Map a required link to its violation type.
pub fn missing_link_violation_type(link: &str) -> &'static str {
    match link {
        consts::BACKLOG_DOC => consts::CROSSLINKS_VIOLATION_NO_BACKLOG_LINK,
        consts::PRD_DOC => consts::CROSSLINKS_VIOLATION_NO_PRD_LINK,
        consts::ROADMAP_DOC => "no_roadmap_link",
        consts::ARCHITECTURE_DOC => "no_architecture_link",
        consts::README_DOC => "no_readme_link",
        consts::CONTRIBUTING_DOC => "no_contributing_link",
        consts::FRD_DOC => "no_frd_link",
        consts::DATA_DOC => "no_data_link",
        consts::DESIGN_DOC => "no_design_link",
        _ => "no_required_link",
    }
}

/// Role-aware required link set for a non-root BACKLOG doc: Surface owes
/// DESIGN instead of FRD; Kernel owes DATA instead of FRD; everything else
/// uses the legacy set.
pub fn backlog_required_links(
    name: &str,
    document: &DocSource,
    context: &DocAuditContext,
) -> Vec<&'static str> {
    if name != consts::BACKLOG_DOC {
        return required_links(name).to_vec();
    }
    // Resolve the member role for this BACKLOG's parent folder.
    let parent_folder = document
        .path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str());
    let role = context.members().iter().find_map(|m| {
        let member_name = m.path.file_name().and_then(|n| n.to_str());
        if member_name == parent_folder || document.path.parent().is_some_and(|p| p == m.path) {
            Some(m.role.clone())
        } else {
            None
        }
    });
    match role {
        Some(MemberRole::Surface) => vec![
            consts::PRD_DOC,
            consts::DESIGN_DOC,
            consts::ROADMAP_DOC,
            consts::ARCHITECTURE_DOC,
        ],
        Some(MemberRole::Kernel) => vec![
            consts::PRD_DOC,
            consts::DATA_DOC,
            consts::ROADMAP_DOC,
            consts::ARCHITECTURE_DOC,
        ],
        _ => required_links(consts::BACKLOG_DOC).to_vec(),
    }
}

// ─── Cross-capability member-doc mechanics (AES606) ──────────────────────

/// Required doc pair per member role.
fn required_docs_for_role(role: MemberRole) -> Option<(&'static [&'static str], &'static str)> {
    match role {
        MemberRole::Feature => Some((&[consts::FRD_DOC, consts::BACKLOG_DOC][..], "feature")),
        MemberRole::Kernel => Some((&[consts::DATA_DOC, consts::BACKLOG_DOC][..], "layer")),
        MemberRole::Surface => Some((&[consts::DESIGN_DOC, consts::BACKLOG_DOC][..], "surface")),
        MemberRole::None => None,
    }
}

/// Forbidden docs per member role.
fn forbidden_docs_for_role(role: MemberRole) -> &'static [&'static str] {
    match role {
        MemberRole::Feature => &[consts::DATA_DOC],
        MemberRole::Surface => &[consts::FRD_DOC, consts::DATA_DOC],
        MemberRole::Kernel => &[consts::FRD_DOC, consts::DESIGN_DOC],
        MemberRole::None => &[],
    }
}

/// Detect member-doc presence and forbidden-doc defects for every member
/// folder. Root-gated: silent when the audit root is not a real workspace
/// root. Gated on `effective_strict_member_docs`. Returns pure defect data;
/// the calling capability stamps with `RULE_CODE_TEMPLATE_CONFORMANCE`.
pub fn detect_member_docs(context: &DocAuditContext) -> Vec<MemberDocDefect> {
    let mut defects = Vec::new();
    if !context.effective_strict_member_docs() {
        return defects;
    }
    if !context.is_workspace_root() {
        return defects;
    }
    let exempt_members = context
        .config()
        .map(|c| c.exempt_members.as_slice())
        .unwrap_or(&[]);

    for member in context.members() {
        if exempt_members.iter().any(|exempt| {
            member.path == Path::new(exempt) || member.path.ends_with(Path::new(exempt))
        }) {
            continue;
        }

        let Some((required_docs, role_name)) = required_docs_for_role(member.role.clone()) else {
            continue;
        };

        for &doc in required_docs {
            if !member.present_docs.contains(doc) {
                let doc_rel_path = member
                    .path
                    .strip_prefix(context.root())
                    .unwrap_or(&member.path)
                    .join(doc)
                    .to_string_lossy()
                    .replace('\\', "/");
                defects.push(MemberDocDefect::MissingRequiredDoc {
                    doc_rel_path: doc_rel_path.to_string(),
                    line: 0,
                    role_name: role_name.to_string(),
                    doc: doc.to_string(),
                });
            }
        }

        for &doc in forbidden_docs_for_role(member.role.clone()) {
            if member.present_docs.contains(doc) {
                let doc_rel_path = member
                    .path
                    .strip_prefix(context.root())
                    .unwrap_or(&member.path)
                    .join(doc)
                    .to_string_lossy()
                    .replace('\\', "/");
                defects.push(MemberDocDefect::ForbiddenDoc {
                    doc_rel_path: doc_rel_path.to_string(),
                    line: 0,
                    role_name: role_name.to_string(),
                    doc: doc.to_string(),
                });
            }
        }
    }
    defects
}

// ─── Cross-capability heading mechanics (AES605) ─────────────────────────

/// H1/H2 heading contract check: exactly one H1, every required H2, no H2
/// outside required+allowed. Returns pure defect data anchored to line 0.
pub fn detect_doc_heading(
    text: &str,
    _name: &str,
    required: &[&str],
    allowed: &[&str],
) -> Vec<HeadingDefect> {
    let mut defects = Vec::new();
    let Some(re) = heading_re() else {
        return defects;
    };
    let prose = blank_fenced(text);
    let captures: Vec<_> = re.captures_iter(&prose).collect();
    let h1_count: usize = captures
        .iter()
        .filter(|c| c.get(1).is_some_and(|m| m.as_str().len() == 1))
        .count();
    if h1_count != 1 {
        defects.push(HeadingDefect::H1Count {
            line: 0,
            count: h1_count,
        });
    }
    let h2: Vec<String> = captures
        .iter()
        .filter(|c| c.get(1).is_some_and(|m| m.as_str().len() == 2))
        .map(|c| normalize_heading(c.get(2).map_or("", |m| m.as_str())))
        .collect();
    let missing: Vec<&str> = required
        .iter()
        .copied()
        .filter(|want| {
            let want = normalize_heading(want);
            !h2.iter()
                .any(|title| title == &want || title.starts_with(&want))
        })
        .collect();
    if !missing.is_empty() {
        defects.push(HeadingDefect::H2Missing {
            line: 0,
            missing: missing.into_iter().map(|s| s.to_string()).collect(),
        });
    }
    let permitted: Vec<String> = required
        .iter()
        .copied()
        .chain(allowed.iter().copied())
        .map(normalize_heading)
        .collect();
    let unexpected_h2: Vec<String> = h2
        .into_iter()
        .filter(|title| !permitted.iter().any(|a| title == a || title.starts_with(a)))
        .collect();
    if !unexpected_h2.is_empty() {
        defects.push(HeadingDefect::H2Unexpected {
            line: 0,
            unexpected: unexpected_h2,
        });
    }
    defects
}
