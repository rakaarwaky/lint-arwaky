// PURPOSE: Shared line-drift finding converter and detection for AES601–AES610 capabilities.

use super::taxonomy_doc_rules_defect_vo::LineDriftDefect;
use super::taxonomy_doc_rules_request::DocFinding;
use super::taxonomy_doc_rules_violation_vo::DOC_VIOLATION_NAMES;
use std::collections::HashSet;

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

/// Doc line-drift check against a doc's `*_TEMPLATE_LINES`. Compares the
/// document's H2 sections and their fixed (non-placeholder) body lines
/// against the template. Returns pure defect data; the calling capability
/// stamps with its own rule code.
pub fn detect_doc_line_drift(text: &str, template: &[&str]) -> Vec<LineDriftDefect> {
    let mut defects = Vec::new();
    let doc_lines: Vec<&str> = text.lines().collect();
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
            .filter(|(idx, l)| {
                let t = l.trim();
                !t.is_empty() && !want_placeholders.contains(idx) && !is_table_row(t)
            })
            .map(|(_, l)| *l)
            .collect();
        let have_norm: Vec<String> = have_lines
            .iter()
            .filter(|l| !is_table_row(l.trim()))
            .map(|l| normalize_line(l))
            .collect();
        let mut cursor = 0usize;
        for want in want_strict {
            let start = cursor.min(have_norm.len());
            let want_norm = normalize_line(want);
            match have_norm[start..]
                .iter()
                .position(|have| *have == want_norm)
            {
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
    for line in doc_lines
        .iter()
        .filter(|l| l.trim_start().starts_with("# "))
    {
        let title = line.trim_start().trim_start_matches('#').trim();
        if title.is_empty() {
            defects.push(LineDriftDefect::H1NoTitle { line: 0 });
        }
    }
    defects
}

fn split_sections<'a>(lines: &'a [&'a str]) -> Vec<(String, Vec<&'a str>)> {
    let mut sections_out: Vec<(String, Vec<&str>)> = Vec::new();
    let mut in_fence = false;
    for line in lines {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            match sections_out.last_mut() {
                Some((_, body)) => body.push(line),
                None => sections_out.push((String::new(), vec![line])),
            }
            continue;
        }
        if !in_fence {
            if let Some(title) = line.strip_prefix("## ") {
                sections_out.push((title.trim().to_string(), Vec::new()));
                continue;
            }
        }
        match sections_out.last_mut() {
            Some((_, body)) => body.push(line),
            None => sections_out.push((String::new(), vec![line])),
        }
    }
    sections_out
}

fn normalize_line(line: &str) -> String {
    let trimmed = line.trim();
    trimmed.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_table_row(line: &str) -> bool {
    line.trim_start().starts_with('|')
}

fn placeholder_lines(lines: &[&str]) -> HashSet<usize> {
    let mut inside = HashSet::new();
    let mut depth = 0usize;
    for (idx, line) in lines.iter().enumerate() {
        let opens = line.matches('{').count();
        let closes = line.matches('}').count();
        let has_angle_slot = has_angle_placeholder(line);
        if has_angle_slot || opens > closes {
            inside.insert(idx);
            depth += opens - closes;
        } else if depth > 0 {
            inside.insert(idx);
            depth = depth.saturating_sub(closes);
            depth = depth.saturating_sub(opens);
        } else if closes > opens || (opens > 0 && closes > 0) {
            inside.insert(idx);
        }
    }
    inside
}

fn has_angle_placeholder(line: &str) -> bool {
    let in_code = line.contains('`');
    if in_code {
        return false;
    }
    line.contains('<') && line.contains('>')
}
