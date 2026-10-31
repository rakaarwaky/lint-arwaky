// PURPOSE: Shared FR-format mechanics for AES601 family rules used by more
// than one per-doc capability (FRD, PRD). Pure functions over documents and
// context; the calling capability stamps findings with its own AES code.
// Lives in crates/shared/src/doc_rules so the FR-format decision cannot drift
// between the FRD and PRD capabilities. No DocFinding construction, no
// format! messages here.

use crate::taxonomy_doc_audit_context_vo::DocAuditContext;
use crate::taxonomy_doc_rules_constant as consts;
use crate::taxonomy_doc_rules_request::DocSource;
use crate::utility_markdown_scanner::{
    api_contract_methods, fr_id_bare_re, h3_subsections, normalize_heading, sections,
};
use crate::utility_protocol_counter::{
    collect_ds_id_set, collect_dt_id_set, collect_pr_id_set, contract_method_names,
    count_fr_headings, count_protocol_traits, fr_id_heading_re, locate_kernel_srcs,
    pr_id_heading_re,
};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::PathBuf;

// ─── Defect data ─────────────────────────────────────────────────────────

/// A defect found by `detect_fr_id_format`.
#[derive(Debug, Clone)]
pub struct FrIdDefect {
    pub line: usize,
    pub num: String,
    pub title: String,
}

impl FrIdDefect {
    pub fn line(&self) -> usize {
        self.line
    }
}

/// A defect found by `detect_pr_id_parity`.
#[derive(Debug, Clone)]
pub enum PrIdParityDefect {
    /// PR-ID declared in the PRD has no ROADMAP `Roadmap Status` entry.
    PrMissingFromRoadmap { line: usize, id: String },
    /// ROADMAP `PR-` item is not declared in the PRD.
    RoadmapNotInPrd {
        line: usize,
        id: String,
        valid: Vec<String>,
    },
}

impl PrIdParityDefect {
    pub fn line(&self) -> usize {
        match self {
            PrIdParityDefect::PrMissingFromRoadmap { line, .. } => *line,
            PrIdParityDefect::RoadmapNotInPrd { line, .. } => *line,
        }
    }
}

/// A defect found by `detect_api_method_existence`.
#[derive(Debug, Clone)]
pub struct ApiMethodDefect {
    pub line: usize,
    pub subsection: String,
    pub method: String,
}

impl ApiMethodDefect {
    pub fn line(&self) -> usize {
        self.line
    }
}

/// A defect found by `detect_fr_protocol_parity`.
#[derive(Debug, Clone)]
pub struct ProtocolParityDefect {
    pub line: usize,
    pub fr_count: usize,
    pub protocol_count: usize,
    pub direction: String,
}

impl ProtocolParityDefect {
    pub fn line(&self) -> usize {
        self.line
    }
}

// ─── FR-ID format ────────────────────────────────────────────────────────

/// FR-ID must be `FR-<FEATURENAME>-NNN: <name>`. A bare `FR-NNN` (no feature
/// prefix) is a violation; the heading itself is reported at its own line.
/// Returns pure defect data; the calling capability stamps with its own rule
/// code and violation type.
pub fn detect_fr_id_format(document: &DocSource) -> Vec<FrIdDefect> {
    let Some(re) = fr_id_bare_re() else {
        return Vec::new();
    };
    let mut defects = Vec::new();
    for caps in re.captures_iter(&document.text) {
        let title = caps.get(3).map_or("", |m| m.as_str());
        let line = document.text[..caps.get(0).map_or(0, |m| m.start())]
            .lines()
            .count()
            .max(1);
        let num = caps.get(2).map_or("", |m| m.as_str());
        defects.push(FrIdDefect {
            line,
            num: num.to_string(),
            title: title.to_string(),
        });
    }
    defects
}

/// Accepted FR-ID set from an FRD's `FR-<feature>-NNN:` headings.
pub fn collect_fr_id_set(frd: &str) -> Option<Vec<String>> {
    let re = fr_id_heading_re()?;
    let ids: Vec<String> = re
        .captures_iter(frd)
        .filter_map(|caps| caps.name("id").map(|m| m.as_str().to_string()))
        .collect();
    if ids.is_empty() { None } else { Some(ids) }
}

/// Accepted PR/FR/DS/DT ID set from a document, whichever families its
/// headings carry. A document may declare several families, so the collector
/// unions the ones that are present.
pub fn collect_any_id_set(text: &str) -> Option<Vec<String>> {
    let mut ids = collect_pr_id_set(text).unwrap_or_default();
    ids.extend(collect_fr_id_set(text).unwrap_or_default());
    ids.extend(collect_ds_id_set(text).unwrap_or_default());
    ids.extend(collect_dt_id_set(text).unwrap_or_default());
    if ids.is_empty() { None } else { Some(ids) }
}

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

/// `(1-based heading line, id)` for every PR-ID heading in *text*.
pub fn pr_id_heading_lines(text: &str) -> Vec<(usize, String)> {
    let Some(re) = pr_id_heading_re() else {
        return Vec::new();
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

// ─── PR-ID parity (PRD ↔ ROADMAP) ─────────────────────────────────────────

/// PRD ↔ ROADMAP PR-ID parity. Every `PR-<PROJECT>-NNN` declared in the PRD's
/// `Project Requirements` must have a matching ROADMAP `Roadmap Status`
/// entry, and vice versa. Returns pure defect data; the calling capability
/// stamps with its own rule code and violation types.
pub fn detect_pr_id_parity(prd: &DocSource, roadmap: &DocSource) -> Vec<PrIdParityDefect> {
    let mut defects = Vec::new();
    let pr_ids = collect_pr_id_set(&prd.text).unwrap_or_default();
    let roadmap_ids = collect_pr_id_set(&roadmap.text).unwrap_or_default();
    let pr_set: HashSet<&str> = pr_ids.iter().map(String::as_str).collect();
    let roadmap_set: HashSet<&str> = roadmap_ids.iter().map(String::as_str).collect();

    let roadmap_heading_line = section_line(&roadmap.text, "roadmap status").unwrap_or(1);
    for id in &pr_ids {
        if !roadmap_set.contains(id.as_str()) {
            defects.push(PrIdParityDefect::PrMissingFromRoadmap {
                line: roadmap_heading_line,
                id: id.clone(),
            });
        }
    }

    for (line, id) in pr_id_heading_lines(&roadmap.text) {
        if !pr_set.contains(id.as_str()) {
            let mut valid: Vec<&str> = pr_ids.iter().map(String::as_str).collect();
            valid.sort();
            defects.push(PrIdParityDefect::RoadmapNotInPrd {
                line,
                id,
                valid: valid.into_iter().map(|s| s.to_string()).collect(),
            });
        }
    }
    defects
}

// ─── Promised API method existence ────────────────────────────────────────

/// Every method the FRD promises must be declared in the code. The `Protocol
/// API` and `Aggregate API` tables are a promise to the integrator; a row
/// whose method no protocol/aggregate trait declares is a promise the code
/// does not keep. Returns pure defect data; the calling capability stamps
/// with its own rule code and violation type.
pub fn detect_api_method_existence(
    document: &DocSource,
    context: &DocAuditContext,
) -> Vec<ApiMethodDefect> {
    let mut defects = Vec::new();
    let Some(feature) = document
        .path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
    else {
        return defects;
    };
    let module = feature.replace('-', "_");
    let mut declared = None;
    for kernel_src in locate_kernel_srcs(context.root()) {
        if let Some(names) = contract_method_names(&kernel_src.join(&module)) {
            declared = Some(names);
            break;
        }
    }
    let Some(declared) = declared else {
        return defects;
    };
    for (subsection, method, line) in api_contract_methods(&document.text) {
        if declared.contains(&method) {
            continue;
        }
        defects.push(ApiMethodDefect {
            line,
            subsection: subsection.to_string(),
            method: method.to_string(),
        });
    }
    defects
}

// ─── FR protocol parity (requirement count vs contract seam count) ─────────

/// The FRD declares a different number of requirements than the feature's
/// contract module declares protocol classes. One protocol class is one
/// capability seam and one seam is one requirement, so the two counts must
/// be equal. Aggregate traits are not capability seams and are excluded.
/// Returns pure defect data; the calling capability stamps with its own rule
/// code and violation type.
pub fn detect_fr_protocol_parity(
    document: &DocSource,
    context: &DocAuditContext,
) -> Vec<ProtocolParityDefect> {
    let mut defects = Vec::new();
    let Some(fr_count) = count_fr_headings(&document.text) else {
        return defects;
    };
    let Some(feature) = document
        .path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
    else {
        return defects;
    };
    let module = feature.replace('-', "_");
    let mut protocol_count = None;
    for kernel_src in locate_kernel_srcs(context.root()) {
        if let Some(count) = count_protocol_traits(&kernel_src.join(&module)) {
            protocol_count = Some(count);
            break;
        }
    }
    let Some(protocol_count) = protocol_count else {
        return defects;
    };
    if fr_count == protocol_count {
        return defects;
    }
    let direction = if protocol_count > fr_count {
        "the code has more protocol classes than requirements; either split the requirements \
         up to match, or merge the classes down — see the 4-direction table in \
         references/frd/template.md"
    } else {
        "the code has fewer protocol classes than requirements; either merge the requirements \
         down to match, or split the methods into more classes — see the 4-direction table in \
         references/frd/template.md"
    };
    let line = document
        .text
        .lines()
        .position(|l| {
            let l = l.trim_start();
            l.starts_with("## ") && l.contains("Requirements")
        })
        .map_or(1, |index| index + 1);
    defects.push(ProtocolParityDefect {
        line,
        fr_count,
        protocol_count,
        direction: direction.to_string(),
    });
    defects
}

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
        let Some(token) = extract_qualifier(&sub.title) else {
            continue;
        };
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

/// Group documents by parent folder so the FRD/BACKLOG alignment sub-check
/// can read both members of a feature folder together. The root PRD/ROADMAP
/// pair and any root BACKLOG are read together so the PR-parity and root
/// ID-alignment sub-checks can pair them.
pub fn group_documents_by_folder<'a>(
    documents: &'a [DocSource],
    _context: &DocAuditContext,
) -> BTreeMap<PathBuf, Vec<&'a DocSource>> {
    let mut by_folder: BTreeMap<PathBuf, Vec<&DocSource>> = BTreeMap::new();
    for document in documents {
        by_folder
            .entry(
                document
                    .path
                    .parent()
                    .map_or_else(PathBuf::new, |p| p.to_path_buf()),
            )
            .or_default()
            .push(document);
    }
    by_folder
}

/// Is the given folder the audit root itself?
pub fn is_root_document(context: &DocAuditContext, document: &DocSource) -> bool {
    context.is_root_document(document)
}
