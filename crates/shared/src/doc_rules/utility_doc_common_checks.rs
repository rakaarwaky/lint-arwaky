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
use crate::utility_capabilities_doc_h2_checks::{
    first_column_values, row_label_diff, SectionShapeDefect, SectionShapeDefectKind,
};
use crate::utility_doc_entry_regexes::fr_entry_re;
use crate::utility_markdown_scanner::{
    h3_subsections, h4_subsections, normalize_heading, sections,
};
use std::collections::{BTreeSet, HashSet};
use std::path::Path;

// ─── Defect data ─────────────────────────────────────────────────────────

/// A defect found by `detect_entry_tables`.
#[derive(Debug, Clone)]
pub enum EntryTableDefect {
    /// The entry's table shape is wrong (extra columns, repeated header,
    /// missing columns, or no table at all).
    Shape(SectionShapeDefect),
    /// The entry's first-column row labels do not match the template.
    RowLabelsMismatch {
        line: usize,
        title: String,
        missing: Vec<String>,
        extra: Vec<String>,
    },
}

impl EntryTableDefect {
    /// The anchor line of this defect, for `DocFinding` construction by the
    /// calling capability.
    pub fn line(&self) -> usize {
        match self {
            EntryTableDefect::Shape(d) => d.line,
            EntryTableDefect::RowLabelsMismatch { line, .. } => *line,
        }
    }
}

/// A defect found by `detect_doc_heading`.
#[derive(Debug, Clone)]
pub enum HeadingDefect {
    /// The document opens with a number of H1 headings other than one.
    H1Count { line: usize, count: usize },
    /// A required H2 heading is missing.
    H2Missing { line: usize, missing: Vec<String> },
    /// An H2 heading appears outside the template's required+allowed set.
    H2Unexpected { line: usize, unexpected: Vec<String> },
}

impl HeadingDefect {
    pub fn line(&self) -> usize {
        match self {
            HeadingDefect::H1Count { line, .. } => *line,
            HeadingDefect::H2Missing { line, .. } => *line,
            HeadingDefect::H2Unexpected { line, .. } => *line,
        }
    }
}

/// A defect found by `detect_agents_line_drift`.
#[derive(Debug, Clone)]
pub enum LineDriftDefect {
    /// AGENTS.md has no YAML frontmatter block.
    FrontmatterMissing { line: usize, doc_name: String },
    /// AGENTS.md has a section the template does not declare.
    SectionNotInTemplate { line: usize, section: String },
    /// AGENTS.md is missing a template section.
    MissingTemplateSection { line: usize, section: String },
    /// A template-fixed line does not match.
    LineMismatch {
        line: usize,
        section: String,
        expected: String,
        found: String,
    },
    /// AGENTS.md H1 has no title.
    H1NoTitle { line: usize },
}

impl LineDriftDefect {
    pub fn line(&self) -> usize {
        match self {
            LineDriftDefect::FrontmatterMissing { line, .. } => *line,
            LineDriftDefect::SectionNotInTemplate { line, .. } => *line,
            LineDriftDefect::MissingTemplateSection { line, .. } => *line,
            LineDriftDefect::LineMismatch { line, .. } => *line,
            LineDriftDefect::H1NoTitle { line } => *line,
        }
    }
}

/// A defect found by `detect_backlog_qualifier_alignment`.
#[derive(Debug, Clone)]
pub enum BacklogQualifierDefect {
    /// BACKLOG entry token has no matching ID in the paired spec.
    TokenNotDeclared {
        line: usize,
        token: String,
        valid: Vec<String>,
    },
    /// Spec-declared ID has no BACKLOG entry.
    IdNoEntry { line: usize, id: String },
    /// BACKLOG row cites an id the paired spec does not declare.
    StrayId {
        line: usize,
        token: String,
        id: String,
        valid: Vec<String>,
    },
}

impl BacklogQualifierDefect {
    pub fn line(&self) -> usize {
        match self {
            BacklogQualifierDefect::TokenNotDeclared { line, .. } => *line,
            BacklogQualifierDefect::IdNoEntry { line, .. } => *line,
            BacklogQualifierDefect::StrayId { line, .. } => *line,
        }
    }
}

/// A defect found by `detect_member_docs`.
#[derive(Debug, Clone)]
pub enum MemberDocDefect {
    /// Member folder is missing a required specification doc.
    MissingRequiredDoc {
        doc_rel_path: String,
        line: usize,
        role_name: String,
        doc: String,
    },
    /// Member folder carries a forbidden specification doc.
    ForbiddenDoc {
        doc_rel_path: String,
        line: usize,
        role_name: String,
        doc: String,
    },
}

impl MemberDocDefect {
    pub fn line(&self) -> usize {
        match self {
            MemberDocDefect::MissingRequiredDoc { line, .. } => *line,
            MemberDocDefect::ForbiddenDoc { line, .. } => *line,
        }
    }
}

// ─── Entry tables (AES606) ───────────────────────────────────────────────

/// Walk every H3 under *parent_h2* whose title matches *entry_re* and detect
/// table-shape defects and row-label mismatches. Findings anchor to the
/// offending H3 line. The calling capability builds its own DocFinding with
/// its own AES code.
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
        // H4-aspect entries carry per-aspect `####` subsections; the H4
        // contract is enforced by `detect_aspect_entry_tables`, so
        // `detect_entry_tables` skips them to avoid double-processing.
        if !h4_subsections(&h3).is_empty() {
            continue;
        }
        // Header/shape check anchored to the H3.
        if let Some(shape) = crate::utility_capabilities_doc_h2_checks::detect_section_shape(
            parsed,
            &h3.title,
            columns,
            Some(&h3),
        ) {
            defects.push(EntryTableDefect::Shape(shape));
            continue;
        }
        // Row-label check (skipped when the doc has no fixed rows).
        if !row_labels.is_empty() {
            let values = first_column_values(&h3.body);
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
    if ids.is_empty() {
        None
    } else {
        Some(ids)
    }
}

/// Accepted FR-ID set from an FRD's `FR-<feature>-NNN:` headings.
pub fn collect_fr_id_set(frd: &str) -> Option<Vec<String>> {
    let re = fr_entry_re()?;
    let ids: Vec<String> = re
        .captures_iter(frd)
        .filter_map(|caps| caps.get(0).map(|m| m.as_str().to_string()))
        .collect();
    if ids.is_empty() {
        None
    } else {
        Some(ids)
    }
}

/// `(1-based heading line, id)` for every PR-ID heading in *text*.
pub fn pr_id_heading_lines(text: &str) -> Vec<(usize, String)> {
    let re = match crate::utility_protocol_counter::pr_id_heading_re() {
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
    let rel = context.relative_path(document);
    let parent_folder = rel.rsplit('/').nth(1).map(|s| s.to_owned());
    let role = context.members().iter().find_map(|m| {
        m.path
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|folder| Some(folder.to_string()) == parent_folder.clone())
            .map(|_| m.role.clone())
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
        MemberRole::Kernel => Some((&[consts::DATA_DOC, consts::BACKLOG_DOC][..], "kernel")),
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
    name: &str,
    required: &[&str],
    allowed: &[&str],
) -> Vec<HeadingDefect> {
    let mut defects = Vec::new();
    let Some(re) = crate::utility_markdown_scanner::heading_re() else {
        return defects;
    };
    let prose = crate::utility_markdown_scanner::blank_fenced(text);
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

// ─── AGENTS line-drift mechanics (AES605) ────────────────────────────────

/// AGENTS line-drift check against `AGENTS_TEMPLATE`. Returns pure defect
/// data; the calling capability stamps with its own rule code.
pub fn detect_agents_line_drift(
    text: &str,
    template: &[&str],
    doc_name: &str,
) -> Vec<LineDriftDefect> {
    let mut defects = Vec::new();
    let doc_lines: Vec<&str> = text.lines().collect();
    let has_frontmatter = doc_lines.first().is_some_and(|l| l.trim() == "---");
    if !has_frontmatter {
        defects.push(LineDriftDefect::FrontmatterMissing {
            line: 1,
            doc_name: doc_name.to_string(),
        });
    }
    let first_line = doc_lines.first().map_or("", |v| *v);
    let looks_like_agents = first_line.trim() == "---" || first_line.starts_with("# ");
    if !looks_like_agents {
        return defects;
    }
    if template.is_empty() {
        return defects;
    }
    let doc_sections = split_sections(&doc_lines);
    let template_sections = split_sections(template);
    for (name, _) in &doc_sections {
        if !template_sections.iter().any(|(key, _)| key == name) {
            defects.push(LineDriftDefect::SectionNotInTemplate {
                line: 0,
                section: name.clone(),
            });
        }
    }
    for (name, want_lines) in &template_sections {
        let Some((_, have_lines)) = doc_sections.iter().find(|(key, _)| key == name) else {
            defects.push(LineDriftDefect::MissingTemplateSection {
                line: 0,
                section: name.clone(),
            });
            continue;
        };
        let want_placeholders = placeholder_lines(want_lines);
        let want_strict: Vec<&str> = want_lines
            .iter()
            .enumerate()
            .filter(|(idx, l)| !l.trim().is_empty() && !want_placeholders.contains(idx))
            .map(|(_, l)| *l)
            .collect();
        let mut cursor = 0usize;
        for want in want_strict {
            let start = cursor.min(have_lines.len());
            match have_lines[start..].iter().position(|have| *have == want) {
                Some(offset) => cursor = start + offset + 1,
                None => {
                    let offending = have_lines
                        .get(start)
                        .copied()
                        .or_else(|| have_lines.last().copied())
                        .unwrap_or("");
                    defects.push(LineDriftDefect::LineMismatch {
                        line: 0,
                        section: name.clone(),
                        expected: want.to_string(),
                        found: offending.to_string(),
                    });
                    cursor = start + 1;
                }
            }
        }
    }
    if !doc_lines.iter().any(|l| l.trim_start().starts_with("# ")) {
        defects.push(LineDriftDefect::H1NoTitle { line: 0 });
    }
    for line in doc_lines.iter().filter(|l| l.trim_start().starts_with("# ")) {
        let title = line.trim_start().trim_start_matches('#').trim();
        if title.is_empty() {
            defects.push(LineDriftDefect::H1NoTitle { line: 0 });
        }
    }
    defects
}

/// Split a document into `H2 heading -> body lines` sections.
fn split_sections<'a>(lines: &'a [&'a str]) -> Vec<(String, Vec<&'a str>)> {
    let mut sections_out: Vec<(String, Vec<&str>)> = Vec::new();
    for line in lines {
        if let Some(title) = line.strip_prefix("## ") {
            sections_out.push((title.trim().to_string(), Vec::new()));
            continue;
        }
        match sections_out.last_mut() {
            Some((_, body)) => body.push(line),
            None => sections_out.push((String::new(), vec![line])),
        }
    }
    sections_out
}

/// Every line that belongs to a `{...}` slot, including slots that wrap across
/// physical lines.
fn placeholder_lines(lines: &[&str]) -> HashSet<usize> {
    let mut inside = HashSet::new();
    let mut depth = 0usize;
    for (idx, line) in lines.iter().enumerate() {
        let opens = line.matches('{').count();
        let closes = line.matches('}').count();
        if opens > closes {
            inside.insert(idx);
            depth += opens - closes;
        } else if depth > 0 {
            inside.insert(idx);
            depth = depth.saturating_sub(closes);
            depth = depth.saturating_sub(opens);
        } else if closes > opens || (opens > 0 && closes > 0) {
            inside.insert(idx);
        }
        if depth == 0 {}
    }
    inside
}
