// PURPOSE: DocChecker — the invariant auditor behind IDocCheckerProtocol
//
// Walks a workspace, collects the documents the chain recognizes, and audits
// each against five document categories (AES601–AES605). Each finding carries
// a machine-readable violation_type so consumers can route on it.
use shared::doc_rules::contract_doc_protocol::IDocCheckerProtocol;
use shared::doc_rules::taxonomy_doc_constant as consts;
use shared::doc_rules::taxonomy_doc_request::{DocFinding, DocRequest, DocSource};
use shared::doc_rules::taxonomy_doc_response::DocResponse;

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

/// A markdown heading with its body and 1-based start line.
struct Section {
    level: usize,
    title: String,
    body: String,
    line: usize,
}

/// The invariant auditor behind the doc checker protocol.
pub struct DocChecker {}

/// Status-leak patterns, mirroring the Python checker's `_STATUS_LEAKS`.
fn status_leak_patterns() -> Option<&'static [(regex::Regex, &'static str)]> {
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
fn source_ext_pattern() -> Option<&'static regex::Regex> {
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
fn heading_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"(?m)^(#{1,6})\s+(.*?)\s*$").ok())
        .as_ref()
}

/// Bullet matcher.
fn bullet_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"(?m)^\s*[-*]\s+\S").ok())
        .as_ref()
}

/// Bare FR-ID matcher (FR-NNN without a feature prefix).
fn fr_id_bare_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"(?m)^(#{2,4})\s+FR-(\d+)\s*:\s*(.*)$").ok())
        .as_ref()
}

/// Well-formed FR-ID matcher (FR-FEATURE-NNN: <name>).
fn fr_id_wellformed_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"(?m)^#{2,4}\s+(FR-[A-Z0-9]+-\d+):").ok())
        .as_ref()
}

/// Normalize a heading for comparison: lowercase, drop punctuation.
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

/// Strip fenced code blocks so prose checks ignore examples.
fn blank_fenced(text: &str) -> String {
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

impl DocChecker {
    /// Collect every document the chain recognizes under *root*.
    pub fn collect_documents(&self, root: &Path) -> Vec<DocSource> {
        let mut out = Vec::new();
        for name in [
            consts::PRD_DOC,
            consts::ROADMAP_DOC,
            consts::FRD_DOC,
            consts::BACKLOG_DOC,
            consts::README_DOC,
            consts::AGENTS_DOC,
        ] {
            let path = root.join(name);
            if let Some(source) = read_source(&path) {
                out.push(source);
            }
        }
        for sub in ["crates", "modules", "packages"] {
            collect_feature_docs(&root.join(sub), &mut out);
        }
        out
    }

    /// Audit one document; *master* is the root master text, used by
    /// cross-cutting invariants that need workspace context.
    fn audit_document(&self, doc: &DocSource, master: Option<&str>) -> Vec<DocFinding> {
        let name = doc
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        let sections = self.sections(&doc.text);
        let mut findings = Vec::new();

        // ── AES601: FR format ──
        if name == consts::FRD_DOC {
            self.check_fr_id_format(doc, &mut findings);
            self.check_fr_fields(doc, &mut findings);
        }

        // ── AES602: Section structure ──
        if name == consts::FRD_DOC {
            self.check_api_contract(&sections, &mut findings);
            self.check_integration_shape(&sections, &mut findings);
            self.check_nfr_shape(&sections, &mut findings);
            self.check_section_order(&sections, &mut findings);
            self.check_scenarios(&sections, &mut findings);
            self.check_glossary(&sections, &mut findings);
        }

        // ── AES603: Spec purity ──
        if is_spec(&name) {
            self.check_status_leak(doc, &mut findings);
            self.check_source_paths(doc, &mut findings);
        }

        // ── AES604: Crosslinks ──
        if name == consts::FRD_DOC {
            self.check_reference_crosslink(&sections, &mut findings);
        }
        if name == consts::BACKLOG_DOC {
            self.check_state_vocab_restated(&sections, master, &mut findings);
        }

        // ── AES605: Feature folder health ──
        // (folder-level checks run after all docs are collected in audit())

        findings
    }

    /// Parse the document into level-1 and level-2 sections.
    fn sections(&self, text: &str) -> Vec<Section> {
        let Some(re) = heading_re() else {
            return Vec::new();
        };
        let mut found = Vec::new();
        for (index, caps) in re.captures_iter(text).enumerate() {
            let level = caps.get(1).map_or(0, |m| m.as_str().len());
            if level > 2 {
                continue;
            }
            let start = caps.get(0).map_or(0, |m| m.start());
            let line = text[..start].lines().count().max(1);
            let title = caps.get(2).map_or("", |m| m.as_str()).to_string();
            let body_start = caps.get(0).map_or(0, |m| m.end());
            let body_end = next_heading_start(text, index);
            found.push(Section {
                level,
                title,
                body: text[body_start.min(text.len())..body_end].to_string(),
                line,
            });
        }
        found
    }

    // ── AES601: FR format ────────────────────────────────────────────────

    /// FR-ID must be `FR-<FEATURENAME>-NNN: <name>`.
    fn check_fr_id_format(&self, doc: &DocSource, findings: &mut Vec<DocFinding>) {
        let Some(re) = fr_id_bare_re() else {
            return;
        };
        for caps in re.captures_iter(&doc.text) {
            let title = caps.get(3).map_or("", |m| m.as_str());
            let line = doc.text[..caps.get(0).map_or(0, |m| m.start())]
                .lines()
                .count()
                .max(1);
            let num = caps.get(2).map_or("", |m| m.as_str());
            findings.push(DocFinding::new(
                consts::RULE_CODE_FR_FORMAT,
                consts::FR_ID_VIOLATION_MISSING_FEATURE_PREFIX,
                format!(
                    "line {line} heading 'FR-{num}: {title}' is missing its feature prefix; it must be 'FR-<FEATURE>-NNN: <imperative name>'",
                ),
            ));
        }
    }

    /// Every requirement states all six FR fields.
    fn check_fr_fields(&self, doc: &DocSource, findings: &mut Vec<DocFinding>) {
        let Some(re) = fr_id_wellformed_re() else {
            return;
        };
        let matches: Vec<_> = re.captures_iter(&doc.text).collect();
        for (index, caps) in matches.iter().enumerate() {
            let id = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let start = caps.get(0).map_or(0, |m| m.end());
            let end = matches
                .get(index + 1)
                .and_then(|n| n.get(0))
                .map_or(doc.text.len(), |m| m.start());
            let body = &doc.text[start.min(doc.text.len())..end.min(doc.text.len())];
            let missing: Vec<&str> = consts::FR_FIELDS
                .iter()
                .copied()
                .filter(|field| !body.contains(&format!("**{field}**")))
                .collect();
            if !missing.is_empty() {
                let line = doc.text[..caps.get(0).map_or(0, |m| m.start())]
                    .lines()
                    .count()
                    .max(1);
                findings.push(DocFinding::new(
                    consts::RULE_CODE_FR_FORMAT,
                    consts::FR_FIELDS_VIOLATION_FIELD_MISSING,
                    format!(
                        "line {line} {id} is missing required field(s): {}; the contract requires all of {}",
                        missing.join(", "),
                        consts::FR_FIELDS.join(", ")
                    ),
                ));
            }
        }
    }

    // ── AES602: Section structure ────────────────────────────────────────

    /// API Contract must carry Protocol API and Aggregate API subsections.
    fn check_api_contract(&self, sections: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(api) = sections
            .iter()
            .find(|s| normalize_heading(&s.title).starts_with("api contract"))
        else {
            return;
        };
        for required in ["Protocol API", "Aggregate API"] {
            let present = api
                .body
                .lines()
                .any(|line| line.trim_start().starts_with("###") && line.contains(required));
            if !present {
                findings.push(DocFinding::new(
                    consts::RULE_CODE_SECTION_STRUCTURE,
                    consts::SECTION_STRUCTURE_VIOLATION_API_SUBSECTION,
                    format!(
                        "line {} API Contract has no {required} subsection; the template splits the contract into Protocol API and Aggregate API",
                        api.line
                    ),
                ));
            }
        }
    }

    /// Integration Points must be a table with the required columns.
    fn check_integration_shape(&self, sections: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(section) = sections
            .iter()
            .find(|s| normalize_heading(&s.title).starts_with("integration points"))
        else {
            return;
        };
        if !has_table_with_columns(&section.body, consts::INTEGRATION_COLUMNS) {
            findings.push(DocFinding::new(
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_INTEGRATION_NOT_TABLE,
                format!(
                    "line {} Integration Points must be a markdown table with columns {}",
                    section.line,
                    consts::INTEGRATION_COLUMNS.join(" | ")
                ),
            ));
        }
    }

    /// Non-functional Requirements must be a table with the required columns.
    fn check_nfr_shape(&self, sections: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(section) = sections.iter().find(|s| {
            let norm = normalize_heading(&s.title);
            norm.contains("non functional") || norm.contains("nonfunctional")
        }) else {
            return;
        };
        if !has_table_with_columns(&section.body, consts::NFR_COLUMNS) {
            findings.push(DocFinding::new(
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_NFR_NOT_TABLE,
                format!(
                    "line {} Non-functional Requirements must be a markdown table with columns {}",
                    section.line,
                    consts::NFR_COLUMNS.join(" | ")
                ),
            ));
        }
    }

    /// FRD sections must follow template order.
    fn check_section_order(&self, sections: &[Section], findings: &mut Vec<DocFinding>) {
        let mut found: Vec<(usize, usize)> = Vec::new();
        for section in sections {
            if section.level != 2 {
                continue;
            }
            let normalized = normalize_heading(&section.title);
            for (index, want) in consts::FRD_SECTION_ORDER.iter().enumerate() {
                let want_norm = normalize_heading(want);
                if normalized == want_norm || normalized.starts_with(&want_norm) {
                    found.push((index, section.line));
                    break;
                }
            }
        }
        let ordered: Vec<usize> = found.iter().map(|(index, _)| *index).collect();
        let mut sorted = ordered.clone();
        sorted.sort_unstable();
        if ordered != sorted {
            findings.push(DocFinding::new(
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_ORDER,
                format!(
                    "line {} FRD sections appear out of template order; expected: {}",
                    found.first().map_or(0, |(_, line)| *line),
                    consts::FRD_SECTION_ORDER.join(", ")
                ),
            ));
        }
    }

    /// Test Scenarios must carry bullet items.
    fn check_scenarios(&self, sections: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(section) = sections
            .iter()
            .find(|s| normalize_heading(&s.title).starts_with("test scenarios"))
        else {
            return;
        };
        if !has_bullet(&blank_fenced(&section.body)) {
            findings.push(DocFinding::new(
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_SCENARIOS_EMPTY,
                format!(
                    "line {} Test Scenarios has no bullet items; the template requires at least one '- scenario' bullet",
                    section.line
                ),
            ));
        }
    }

    /// Glossary must carry bullet items.
    fn check_glossary(&self, sections: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(section) = sections
            .iter()
            .find(|s| normalize_heading(&s.title) == "glossary")
        else {
            return;
        };
        if !has_bullet(&blank_fenced(&section.body)) {
            findings.push(DocFinding::new(
                consts::RULE_CODE_SECTION_STRUCTURE,
                consts::SECTION_STRUCTURE_VIOLATION_GLOSSARY_EMPTY,
                format!(
                    "line {} Glossary has no bullet items; the template requires at least one '- **Term**: definition' bullet",
                    section.line
                ),
            ));
        }
    }

    // ── AES603: Spec purity ──────────────────────────────────────────────

    /// A spec must not carry implementation state.
    fn check_status_leak(&self, doc: &DocSource, findings: &mut Vec<DocFinding>) {
        let Some(patterns) = status_leak_patterns() else {
            return;
        };
        for (number, line) in doc.text.lines().enumerate() {
            for (pattern, what) in patterns {
                if pattern.is_match(line) {
                    findings.push(DocFinding::new(
                        consts::RULE_CODE_SPEC_PURITY,
                        consts::SPEC_PURITY_VIOLATION_STATUS_LEAK,
                        format!(
                            "line {} carries {what} ('{}'); specs promise, backlogs report; move the claim to BACKLOG.md",
                            number + 1,
                            line.trim()
                        ),
                    ));
                }
            }
        }
    }

    /// A spec must not name source files.
    fn check_source_paths(&self, doc: &DocSource, findings: &mut Vec<DocFinding>) {
        let Some(re) = source_ext_pattern() else {
            return;
        };
        for (number, line) in doc.text.lines().enumerate() {
            if let Some(matched) = re.find(line) {
                findings.push(DocFinding::new(
                    consts::RULE_CODE_SPEC_PURITY,
                    consts::SPEC_PURITY_VIOLATION_SOURCE_FILE_NAMED,
                    format!(
                        "line {} names source file '{}' — specs are stateless: refer to roles and behaviour, never source files",
                        number + 1,
                        matched.as_str()
                    ),
                ));
            }
        }
    }

    // ── AES604: Crosslinks ───────────────────────────────────────────────

    /// An FRD must link its BACKLOG.md and PRD.md in the Reference section.
    fn check_reference_crosslink(&self, sections: &[Section], findings: &mut Vec<DocFinding>) {
        let Some(section) = sections
            .iter()
            .find(|s| normalize_heading(&s.title) == "reference")
        else {
            return;
        };
        if !section.body.contains(consts::BACKLOG_DOC) {
            findings.push(DocFinding::new(
                consts::RULE_CODE_CROSSLINKS,
                consts::CROSSLINKS_VIOLATION_NO_BACKLOG_LINK,
                format!(
                    "line {} Reference section must link {}",
                    section.line,
                    consts::BACKLOG_DOC
                ),
            ));
        }
        if !section.body.contains(consts::PRD_DOC) {
            findings.push(DocFinding::new(
                consts::RULE_CODE_CROSSLINKS,
                consts::CROSSLINKS_VIOLATION_NO_PRD_LINK,
                format!(
                    "line {} Reference section must link {}",
                    section.line,
                    consts::PRD_DOC
                ),
            ));
        }
    }

    /// State vocabulary must live only in the root master, not in sub-docs.
    fn check_state_vocab_restated(
        &self,
        sections: &[Section],
        master: Option<&str>,
        findings: &mut Vec<DocFinding>,
    ) {
        let Some(section) = sections
            .iter()
            .find(|s| normalize_heading(&s.title) == "current condition")
        else {
            return;
        };
        let restated: Vec<&str> = consts::MASTER_ONLY_SECTIONS
            .iter()
            .copied()
            .filter(|title| {
                sections
                    .iter()
                    .any(|s| normalize_heading(&s.title) == normalize_heading(title))
            })
            .collect();
        if restated.is_empty() {
            return;
        }
        if master.is_some() {
            findings.push(DocFinding::new(
                consts::RULE_CODE_CROSSLINKS,
                consts::CROSSLINKS_VIOLATION_STATE_VOCAB_RESTATED,
                format!(
                    "line {} feature backlog carries {} section(s); those live once, in the root master",
                    section.line,
                    restated.join(", ")
                ),
            ));
        }
    }

    // ── AES605: Feature folder health ────────────────────────────────────

    /// A folder carrying a doc pair must hold an orchestrator (feature folder).
    fn check_feature_folder(&self, root: &Path, findings: &mut Vec<DocFinding>) {
        for sub in ["crates", "modules", "packages"] {
            let dir = root.join(sub);
            let Ok(entries) = fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let feature = entry.path();
                if !feature.is_dir() {
                    continue;
                }
                let has_frd = feature.join(consts::FRD_DOC).is_file();
                let has_backlog = feature.join(consts::BACKLOG_DOC).is_file();
                if !has_frd && !has_backlog {
                    continue;
                }
                let is_shared = feature.file_name().is_some_and(|n| n == consts::KERNEL_DIR);
                if is_shared {
                    findings.push(DocFinding::new(
                        consts::RULE_CODE_FEATURE_FOLDER,
                        consts::FEATURE_FOLDER_VIOLATION_SHARED_HAS_DOCS,
                        format!(
                            "kernel folder '{}' must not carry a doc pair; move it to a feature folder",
                            sub
                        ),
                    ));
                    continue;
                }
                let has_orchestrator = has_orchestrator(&feature);
                if !has_orchestrator {
                    findings.push(DocFinding::new(
                        consts::RULE_CODE_FEATURE_FOLDER,
                        consts::FEATURE_FOLDER_VIOLATION_NO_ORCHESTRATOR,
                        format!(
                            "feature folder '{}/{}' carries a doc pair but holds no orchestrator; a feature folder must hold a *_orchestrator file",
                            sub,
                            entry.file_name().to_string_lossy()
                        ),
                    ));
                }
            }
        }
    }
}

/// Does the folder hold an orchestrator file, directly or under *src*?
fn has_orchestrator(dir: &Path) -> bool {
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if has_orchestrator(&path) {
                return true;
            }
        } else if path
            .file_stem()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.ends_with(consts::ORCHESTRATOR_SUFFIX))
        {
            return true;
        }
    }
    false
}

/// Does the body hold a table whose header carries every *columns* entry?
fn has_table_with_columns(body: &str, columns: &[&str]) -> bool {
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

/// Does the body carry a bullet item?
fn has_bullet(body: &str) -> bool {
    body.lines()
        .any(|line| bullet_re().is_some_and(|re| re.is_match(line)))
}

/// Read a document, returning `None` when it is absent or unreadable.
fn read_source(path: &Path) -> Option<DocSource> {
    let text = fs::read_to_string(path).ok()?;
    Some(DocSource {
        path: path.to_path_buf(),
        text,
    })
}

/// Collect the FRD/BACKLOG pair from every feature folder under *dir*.
fn collect_feature_docs(dir: &Path, out: &mut Vec<DocSource>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let feature = entry.path();
        if !feature.is_dir() {
            continue;
        }
        for doc in [consts::FRD_DOC, consts::BACKLOG_DOC] {
            if let Some(source) = read_source(&feature.join(doc)) {
                out.push(source);
            }
        }
    }
}

/// Byte offset of the next heading at the same or higher rank, or text length.
fn next_heading_start(text: &str, from_index: usize) -> usize {
    let Some(re) = heading_re() else {
        return text.len();
    };
    let all: Vec<_> = re.captures_iter(text).collect();
    let Some(current) = all.get(from_index) else {
        return text.len();
    };
    let rank = current.get(1).map_or(0, |m| m.as_str().len());
    for candidate in all.iter().skip(from_index + 1) {
        let candidate_rank = candidate.get(1).map_or(0, |m| m.as_str().len());
        if candidate_rank <= rank {
            return candidate.get(0).map_or(text.len(), |m| m.start());
        }
    }
    text.len()
}

/// Is this document a spec (promise-bearing) rather than a status report?
fn is_spec(name: &str) -> bool {
    matches!(
        name,
        consts::FRD_DOC
            | consts::PRD_DOC
            | consts::ROADMAP_DOC
            | "ARCHITECTURE.md"
            | "CONTRIBUTING.md"
    )
}

/// Collect unique findings, sorted for stable output.
fn sorted(findings: Vec<DocFinding>) -> Vec<DocFinding> {
    let mut seen = BTreeSet::new();
    let mut out: Vec<DocFinding> = findings
        .into_iter()
        .filter(|f| seen.insert((f.code, f.violation_type, f.message.clone())))
        .collect();
    out.sort_by(|a, b| {
        (a.code, a.violation_type, a.message.clone()).cmp(&(
            b.code,
            b.violation_type,
            b.message.clone(),
        ))
    });
    out
}

impl IDocCheckerProtocol for DocChecker {
    /// Walk the document chain under the request's root and audit every
    /// recognized document against all invariants.
    fn audit(&self, request: DocRequest) -> DocResponse {
        let DocRequest::AuditAll { root } = request;
        let documents = self.collect_documents(&root);
        let master = documents
            .iter()
            .find(|d| {
                d.path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| consts::MASTER_DOC_CANDIDATES.contains(&n))
            })
            .map(|d| d.text.as_str());

        let mut findings = Vec::new();
        for doc in &documents {
            findings.extend(self.audit_document(doc, master));
        }
        // Folder-level checks (AES605) need the workspace root directly.
        self.check_feature_folder(&root, &mut findings);

        DocResponse::Findings {
            findings: sorted(findings),
        }
    }
}
