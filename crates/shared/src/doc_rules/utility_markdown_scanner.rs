// PURPOSE: Markdown text primitives for the AES document chain — heading parsing,
// section slicing, fence blanking, and shape predicates. Every function here is
// stateless over a string, so they belong in the utility layer and the checker
// keeps only the decision of what a finding means.
use crate::taxonomy_doc_rules_constant as consts;
use crate::taxonomy_doc_section_vo::Section;

use std::sync::OnceLock;

/// Status-leak patterns, mirroring the Python checker's `_STATUS_LEAKS`.
pub fn status_leak_patterns() -> Option<&'static [(regex::Regex, &'static str)]> {
    static PATTERNS: OnceLock<Option<Vec<(regex::Regex, &'static str)>>> = OnceLock::new();
    PATTERNS
        .get_or_init(|| {
            let build = || -> Option<Vec<(regex::Regex, &'static str)>> {
                Some(vec![
                    (
                        regex::Regex::new(r"^\s*[-*]\s*\[[ xX]\]").ok()?,
                        "a checkbox task item",
                    ),
                    (
                        regex::Regex::new(r"(?i)^\s*\**\s*status\s*\**\s*:").ok()?,
                        "a Status: field",
                    ),
                    (
                        regex::Regex::new(
                            r"(?i)\b(implemented|unimplemented|partially implemented)\b",
                        )
                        .ok()?,
                        "implementation state",
                    ),
                    (
                        regex::Regex::new(r"(?i)\b(shipped|released|deployed) in v\w*\b").ok()?,
                        "release state",
                    ),
                    (
                        regex::Regex::new(r"^\s*(✅|❌|🟢|🔴|✔|✖)").ok()?,
                        "a status marker",
                    ),
                    (
                        regex::Regex::new(r"(?i)\b\d+\s*%\s*(complete|done)").ok()?,
                        "a progress percentage",
                    ),
                ])
            };
            build()
        })
        .as_ref()
        .map(|v| &**v)
}

/// Pattern matching a concrete source-file reference, per HOW-TO Rule 9.
pub fn source_ext_pattern() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| {
        let exts = consts::SOURCE_EXTENSIONS.join("|");
        regex::Regex::new(&format!(
            r"(?:[A-Za-z0-9_]+/)*[A-Za-z0-9_.<>{{}}*-]+\.(?:{exts})(?:[\w-]{{0}})"
        ))
        .ok()
    })
    .as_ref()
}

/// Heading matcher.
pub fn heading_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"(?m)^(#{1,6})\s+(.*?)\s*$").ok())
        .as_ref()
}

/// Look up the H2 contract for a recognized root document.
pub fn doc_h2_contract(name: &str) -> Option<(&[&str], &[&str])> {
    consts::DOC_HEADING_CONTRACTS
        .iter()
        .find(|(n, _, _)| *n == name)
        .map(|(_, required, allowed)| (*required, *allowed))
}

/// Bullet matcher.
fn bullet_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"(?m)^\s*[-*]\s+\S").ok())
        .as_ref()
}

/// Bare FR-ID matcher (FR-NNN without a feature prefix).
pub fn fr_id_bare_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"(?m)^(#{2,4})\s+FR-(\d+)\s*:\s*(.*)$").ok())
        .as_ref()
}

/// Normalize a heading for comparison: lowercase, drop punctuation.
pub fn normalize_heading(title: &str) -> String {
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
    let collapsed = stripped.split_whitespace().collect::<Vec<_>>().join(" ");
    // Drop a leading list index so "4. Vertical Slicing Folder Structure"
    // still matches the template section name.
    match collapsed.split_once(' ') {
        Some((first, rest)) if first.chars().all(|c| c.is_ascii_digit()) && !first.is_empty() => {
            rest.to_string()
        }
        _ => collapsed,
    }
}

/// Strip fenced code blocks so prose checks ignore examples.
pub fn blank_fenced(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut inside = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            inside = !inside;
            out.push('\n');
            continue;
        }
        out.push_str(if inside { "" } else { line });
        out.push('\n');
    }
    out
}

/// Parse *text* into its level-1 and level-2 sections.
///
/// Headings are collected in a single O(H) pass with a stack-based sweep, and
/// each body runs to the next heading of the same or higher rank.
pub fn sections(text: &str) -> Vec<Section> {
    let Some(re) = heading_re() else {
        return Vec::new();
    };
    // Collect all headings (any level) with their offsets in one pass.
    let mut all: Vec<(usize, usize, String, usize, usize)> = Vec::new();
    for caps in re.captures_iter(text) {
        let full = match caps.get(0) {
            Some(f) => f,
            None => continue,
        };
        // Level is the number of `#` characters (capture group 1), NOT
        // the full match length.
        let level = caps.get(1).map_or(0, |m| m.as_str().len());
        let start = full.start();
        let line = text[..start].lines().count().max(1);
        let title = caps.get(2).map_or("", |m| m.as_str()).to_string();
        let body_start = full.end();
        all.push((level, start, title, line, body_start));
    }
    // Compute next-same-or-higher-rank boundary for each heading.
    let n = all.len();
    let mut boundary: Vec<Option<usize>> = vec![None; n];
    let mut stack: Vec<usize> = Vec::new();
    for i in 0..n {
        let (level, _, _, _, _) = &all[i];
        while let Some(&top) = stack.last() {
            if all[top].0 >= *level {
                boundary[top] = Some(i);
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    // Build sections for H1 and H2 headings only.
    all.iter()
        .enumerate()
        .filter(|(_, (level, _, _, _, _))| *level <= 2)
        .map(|(orig_idx, (level, _start, title, line, body_start))| {
            let body_end = boundary[orig_idx].map_or_else(|| text.len(), |j| all[j].1);
            Section {
                level: *level,
                title: title.clone(),
                body: text
                    .get(*body_start.min(&text.len())..body_end)
                    .unwrap_or_default()
                    .to_string(),
                line: *line,
            }
        })
        .collect()
}

/// Every level-3 heading in *text*, with its title and document-absolute line.
///
/// `sections()` keeps only H1/H2 and `h3_subsections()` reads one section at a
/// time, but the FRD template parity check is a document-wide statement: the
/// template sanctions three level-3 shapes and the document must carry no
/// others, wherever they sit. The line is returned so the caller can ask
/// whether some *other* pattern already accepted the heading on that line —
/// that is how the parity check reuses the shared FR-ID pattern instead of
/// compiling a second one that could drift from it.
///
/// Lines, not byte offsets, are the join key: fence blanking rewrites the text
/// it scans, so offsets from the blanked copy do not address the raw document,
/// while the line count is preserved by construction and does.
///
/// Fenced blocks are blanked first, so a `#` line inside a code fence never
/// reads as a heading.
pub fn h3_headings(text: &str) -> Vec<(String, usize)> {
    let Some(re) = heading_re() else {
        return Vec::new();
    };
    let prose = blank_fenced(text);
    re.captures_iter(&prose)
        .filter_map(|caps| {
            let marker = caps.get(1)?;
            if marker.as_str().len() != 3 {
                return None;
            }
            let full = caps.get(0)?;
            Some((
                caps.get(2).map_or("", |m| m.as_str()).to_string(),
                prose[..full.start()].lines().count().max(1),
            ))
        })
        .collect()
}

/// Level-3 subsections of one level-2 section, with document-absolute lines.
///
/// `sections()` keeps only H1/H2, because every invariant so far reads the top
/// of the document. The API Contract invariant is the exception: it must read
/// one level deeper, so it asks for its own children rather than widening the
/// parse for every check.
pub fn h3_subsections(section: &Section) -> Vec<Section> {
    let Some(re) = heading_re() else {
        return Vec::new();
    };
    // Offsets are relative to the section body, so a heading's document line
    // is the body line plus the number of newlines preceding it.
    let mut all: Vec<(usize, String, usize, usize)> = Vec::new();
    for caps in re.captures_iter(&section.body) {
        let full = match caps.get(0) {
            Some(f) => f,
            None => continue,
        };
        let Some(marker) = caps.get(1) else { continue };
        if marker.as_str().len() != 3 {
            continue;
        }
        let line = section.body[..full.start()].lines().count().max(1);
        let title = caps.get(2).map_or("", |m| m.as_str()).to_string();
        all.push((full.start(), title, line, full.end()));
    }
    all.iter()
        .enumerate()
        .map(|(i, (_start, title, line, body_start))| {
            // A subsection ends where the next one begins, or at the end of
            // the parent body.
            let body_end = all
                .get(i + 1)
                .map_or_else(|| section.body.len(), |(start, _, _, _)| *start);
            Section {
                level: 3,
                title: title.clone(),
                body: section
                    .body
                    .get(*body_start..body_end)
                    .unwrap_or_default()
                    .to_string(),
                // +1 for the parent's own heading line.
                line: section.line + line,
            }
        })
        .collect()
}

/// The methods the FRD promises, one promise at a time.
///
/// Reads the `### Protocol API` and `### Aggregate API` tables under `## API
/// Contract` and yields `(subsection, method, doc_line)` per promised method.
///
/// The method is read from the column its header names, not from the first
/// cell: a table that leads with a `Protocol Trait` column still promises its
/// `Method` column. A table with no `Method` header is skipped rather than
/// guessed at, so a shape the rule cannot read is a parse skip and not a
/// false violation.
///
/// One cell may promise several methods — the shipped tables use both
/// `` `execute` `` and `` `start` / `subscribe` / `stop` `` — so a cell is
/// split on `/` and each name yielded separately. Only table rows count;
/// prose mentioning a method is not a promise.
pub fn api_contract_methods(text: &str) -> Vec<(String, String, usize)> {
    let mut rows = Vec::new();
    // A single sweep over the raw document, tracking the enclosing level-2
    // section and level-3 subsection, so every row keeps its own 1-based
    // document line. Reading the line off a parsed `Section` instead would
    // inherit that struct's body-relative arithmetic, and a finding must name
    // the row a reader will go and look at.
    let mut in_api_contract = false;
    let mut subsection: Option<String> = None;
    let mut method_column: Option<usize> = None;
    // Fenced blocks are blanked first, so a `## API Contract` inside an
    // example cannot open the section and a table inside one cannot
    // contribute rows.
    for (index, raw) in blank_fenced(text).lines().enumerate() {
        let line = index + 1;
        let trimmed = raw.trim();
        if let Some(title) = heading_text(trimmed, 2) {
            in_api_contract = normalize_heading(title).starts_with("api contract");
            subsection = None;
            method_column = None;
            continue;
        }
        if let Some(title) = heading_text(trimmed, 3) {
            if !in_api_contract {
                continue;
            }
            let norm = normalize_heading(title);
            let recognized = consts::API_CONTRACT_SUBSECTIONS.iter().any(|want| {
                let want = normalize_heading(want);
                norm == want || norm.starts_with(&want)
            });
            subsection = recognized.then(|| title.to_string());
            method_column = None;
            continue;
        }
        let Some(title) = subsection.as_deref() else {
            continue;
        };
        if !trimmed.starts_with('|') {
            // Prose between rows ends the table but keeps the subsection: a
            // second table under the same H3 is still an API table, and it
            // brings its own header row.
            method_column = None;
            continue;
        }
        let cells: Vec<&str> = trimmed
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        if method_column.is_none() {
            // The first table row under the heading is the column header.
            method_column = cells
                .iter()
                .position(|cell| normalize_heading(cell) == "method");
            continue;
        }
        let Some(column) = method_column else {
            continue;
        };
        let Some(cell) = cells.get(column).copied() else {
            continue;
        };
        if cell.is_empty()
            || cell
                .chars()
                .all(|c| c == '-' || c == ':' || c.is_whitespace())
        {
            continue;
        }
        for method in promised_methods(cell) {
            rows.push((title.to_string(), method, line));
        }
    }
    rows
}

/// The method names one table cell promises.
///
/// Splits on `/` because a cell may group several methods, then strips
/// backticks, whitespace, and any parenthetical qualifier so
/// `` `execute` `` and `` `IUnusedImportFixProtocol` (FR-001) `` both yield
/// the bare name a trait declares. A name that is not an identifier — a
/// `—` placeholder, or prose — yields nothing, because a table cannot promise
/// a method it never spells.
fn promised_methods(cell: &str) -> Vec<String> {
    cell.split('/')
        .map(|part| {
            part.split('(')
                .next()
                .unwrap_or(part)
                .trim()
                .trim_matches('`')
                .trim()
                .to_string()
        })
        .filter(|name| {
            !name.is_empty()
                && name.chars().all(|c| c.is_alphanumeric() || c == '_')
                && name
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_alphabetic() || c == '_')
        })
        .collect()
}

/// The heading text of a line that is exactly a level-*level* heading, or
/// `None` for any other line.
///
/// Exact rank matters: a level-3 heading must not read as the level-2 section
/// that encloses it.
fn heading_text(trimmed: &str, level: usize) -> Option<&str> {
    let hashes = trimmed.chars().take_while(|c| *c == '#').count();
    if hashes != level {
        return None;
    }
    let rest = trimmed[level..].trim_start();
    if rest.len() == trimmed[level..].len() && !trimmed[level..].is_empty() {
        return None;
    }
    (!rest.is_empty()).then_some(rest.trim_end())
}

/// Does the body hold a table whose header carries every *columns* entry?
pub fn has_table_with_columns(body: &str, columns: &[&str]) -> bool {
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
        if columns
            .iter()
            .all(|want| cells.iter().any(|cell| cell.contains(&want.to_lowercase())))
        {
            return true;
        }
    }
    false
}

/// Does the body carry a bullet item outside any fenced code block?
/// Fenced lines are skipped so a bullet inside a fence is not counted as a
/// meaningful prose bullet.
pub fn has_bullet(body: &str) -> bool {
    let mut inside_fence = false;
    for line in body.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            inside_fence = !inside_fence;
            continue;
        }
        if !inside_fence && bullet_re().is_some_and(|re| re.is_match(line)) {
            return true;
        }
    }
    false
}
