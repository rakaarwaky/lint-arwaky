// PURPOSE: FrFormatChecker — AES601: requirement-ID shape, PR/DS/DT/FR parity,
// and BACKLOG ID-level alignment.
//
// The one capability that answers AES601. It reads the requirement documents
// (an FRD and a DATA document state requirements under the same ID rules) and
// reports the violation families: an ID that lost its feature prefix, a
// requirement count that has drifted from the feature's count of capability
// seams, a promised Protocol/Aggregate API method that the contract module
// never declares, PR-ID parity between the PRD and ROADMAP, and BACKLOG
// ID-level alignment against the paired spec.
use shared_doc_rules::contract_doc_protocol::IFrFormatProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::PathBuf;

use shared_doc_rules::utility_markdown_scanner::{
    api_contract_methods, fr_id_bare_re, h3_subsections, normalize_heading, sections,
};
use shared_doc_rules::utility_protocol_counter::{
    collect_ds_id_set, collect_dt_id_set, collect_pr_id_set, contract_method_names,
    count_fr_headings, count_protocol_traits, fr_id_heading_re, locate_kernel_srcs,
    pr_id_heading_re,
};

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES601 auditor: FR-ID format, PR/DS/DT/FR parity, and BACKLOG
/// ID-level alignment.
pub struct FrFormatChecker {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IFrFormatProtocol for FrFormatChecker {
    /// Report every FR-ID, PR-parity, BACKLOG-alignment, parity, and
    /// promised-method violation in the documents of *context*.
    ///
    /// Documents are grouped by parent folder so the FRD/BACKLOG alignment
    /// sub-check can read both members of a feature folder together. The
    /// root PRD/ROADMAP pair and any root BACKLOG are read together so the
    /// PR-parity and root ID-alignment sub-checks can pair them.
    fn audit_fr_format(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        let mut by_folder: BTreeMap<PathBuf, Vec<&DocSource>> = BTreeMap::new();
        for document in context.documents() {
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
        for documents in by_folder.values() {
            let is_root = documents
                .first()
                .is_some_and(|d| context.is_root_document(d));
            let find = |name: &str| documents.iter().find(|d| context.document_name(d) == name);
            let frd = find(consts::FRD_DOC);
            let backlog = find(consts::BACKLOG_DOC);
            let design = find(consts::DESIGN_DOC);
            let data = find(consts::DATA_DOC);
            let prd = find(consts::PRD_DOC);
            let roadmap = find(consts::ROADMAP_DOC);

            if let Some(frd) = frd {
                let mut own = Vec::new();
                self.check_fr_id_format(frd, &mut own);
                self.check_fr_protocol_parity(frd, context, &mut own);
                self.check_api_method_existence(frd, context, &mut own);
                context.stamp(frd, &mut own);
                findings.extend(own);
            }
            // Root PRD ↔ ROADMAP PR-ID parity (Decision 6).
            if is_root {
                if let (Some(prd), Some(roadmap)) = (prd, roadmap) {
                    let mut own = Vec::new();
                    self.check_pr_id_parity(prd, roadmap, &mut own);
                    context.stamp(prd, &mut own);
                    findings.extend(own);
                }
                // Root/legacy BACKLOG pairs with the root ROADMAP/PRD union
                // (Decision 9 pairing table).
                if let Some(backlog) = backlog {
                    let union = Self::collect_union_id_set(
                        collect_pr_id_set,
                        roadmap.copied(),
                        prd.copied(),
                    );
                    let mut own = Vec::new();
                    self.check_backlog_qualifier_alignment(
                        backlog,
                        union.as_ref(),
                        consts::PR_BACKLOG_ID_MISSING,
                        consts::BACKLOG_ID_NOT_PR,
                        &mut own,
                    );
                    context.stamp(backlog, &mut own);
                    findings.extend(own);
                }
            }
            // Feature BACKLOG pairs with its FRD (+ DESIGN.md if present);
            // kernel BACKLOG with DATA.md; surface BACKLOG with DESIGN.md.
            // Alignment is ID-level over the paired spec union.
            if !is_root {
                if let Some(backlog) = backlog {
                    // One doc per spec family: FRD declares FR-IDs, DATA.md
                    // declares DT-IDs, DESIGN.md declares DS-IDs. A folder
                    // that holds two of these unions both declared ID sets.
                    let mut specs: Vec<&DocSource> = Vec::new();
                    if let Some(frd) = frd {
                        specs.push(frd);
                    }
                    if let Some(data) = data {
                        specs.push(data);
                    }
                    if let Some(design) = design {
                        specs.push(design);
                    }
                    let union = Self::collect_union_id_set(
                        Self::collect_any_id_set,
                        specs.first().copied(),
                        specs.get(1).copied(),
                    );
                    // The not-code / missing-code pair is chosen by the
                    // pairing table (Decision 9): feature BACKLOG aligns with
                    // its FRD (+ DESIGN if present) → FR codes; kernel → DATA
                    // → DT codes; surface → DESIGN → DS codes. FRD takes
                    // priority so a feature folder carrying both FRD and
                    // DESIGN still reports FR codes.
                    let (missing_code, not_code) = if specs
                        .iter()
                        .any(|d| context.document_name(d) == consts::FRD_DOC)
                    {
                        (consts::FR_BACKLOG_ID_MISSING, consts::FR_BACKLOG_ID_NOT_FR)
                    } else if specs
                        .iter()
                        .any(|d| context.document_name(d) == consts::DATA_DOC)
                    {
                        (consts::DT_BACKLOG_ID_MISSING, consts::DT_BACKLOG_ID_NOT_DT)
                    } else {
                        (consts::DS_BACKLOG_ID_MISSING, consts::DS_BACKLOG_ID_NOT_DS)
                    };
                    let mut own = Vec::new();
                    self.check_backlog_qualifier_alignment(
                        backlog,
                        union.as_ref(),
                        missing_code,
                        not_code,
                        &mut own,
                    );
                    context.stamp(backlog, &mut own);
                    findings.extend(own);
                }
            }
            for document in documents {
                if context.document_name(document) == consts::DATA_DOC {
                    let mut own = Vec::new();
                    self.check_fr_id_format(document, &mut own);
                    context.stamp(document, &mut own);
                    findings.extend(own);
                }
            }
        }
        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for FrFormatChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl FrFormatChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// FR-ID must be `FR-<FEATURENAME>-NNN: <name>`.
    fn check_fr_id_format(&self, document: &DocSource, findings: &mut Vec<DocFinding>) {
        let Some(re) = fr_id_bare_re() else {
            return;
        };
        for caps in re.captures_iter(&document.text) {
            let title = caps.get(3).map_or("", |m| m.as_str());
            let line = document.text[..caps.get(0).map_or(0, |m| m.start())]
                .lines()
                .count()
                .max(1);
            let num = caps.get(2).map_or("", |m| m.as_str());
            findings.push(
                DocFinding::new_with_line(
                    "",
                    line,
                    consts::RULE_CODE_FR_FORMAT,
                    consts::FR_ID_VIOLATION_MISSING_FEATURE_PREFIX,
                    format!("line {line} heading 'FR-{num}: {title}' is missing its feature prefix"),
                )
                .with_reason(
                    "A requirement without its feature prefix cannot be traced back to the feature it belongs to.",
                    "Rename the heading to 'FR-<FEATURE>-NNN: <imperative name>'.",
                ),
            );
        }
    }

    /// The FRD declares a different number of requirements than the feature's
    /// contract module declares protocol classes.
    ///
    /// One protocol class is one capability seam, and one seam is one
    /// requirement, so the two counts must be equal. A single protocol file
    /// may hold many classes — count the classes, never the files. Aggregate
    /// traits are not capability seams and are excluded.
    fn check_fr_protocol_parity(
        &self,
        document: &DocSource,
        context: &DocAuditContext,
        findings: &mut Vec<DocFinding>,
    ) {
        let Some(fr_count) = count_fr_headings(&document.text) else {
            return;
        };
        let Some(feature) = document
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
        else {
            return;
        };
        // The feature crate uses `-` where the shared module uses `_`.
        let module = feature.replace('-', "_");
        // A contract module can live in any documented layout — crates/,
        // modules/, or packages/ — so try each kernel root until one
        // holds the feature's module instead of skipping non-crates
        // layouts. A layout that carries no shared kernel at all keeps the
        // check out of the way.
        let mut protocol_count = None;
        for kernel_src in locate_kernel_srcs(context.root()) {
            if let Some(count) = count_protocol_traits(&kernel_src.join(&module)) {
                protocol_count = Some(count);
                break;
            }
        }
        let Some(protocol_count) = protocol_count else {
            return;
        };
        if fr_count == protocol_count {
            return;
        }
        let direction = if protocol_count > fr_count {
            "the code has more protocol classes than requirements; either split the requirements \
             up to match, or merge the classes down — see the 4-direction table in \
             HOW-TO-MAKE-FRD/template.md"
        } else {
            "the code has fewer protocol classes than requirements; either merge the requirements \
             down to match, or split the methods into more classes — see the 4-direction table in \
             HOW-TO-MAKE-FRD/template.md"
        };
        // Every doc finding names a line, so the mismatch is anchored to the
        // requirements section it is about; a document without one falls back
        // to the first line.
        let line = document
            .text
            .lines()
            .position(|l| {
                let l = l.trim_start();
                l.starts_with("## ") && l.contains("Requirements")
            })
            .map_or(1, |index| index + 1);
        findings.push(
            DocFinding::new_with_line(
                "",
                line,
                consts::RULE_CODE_FR_FORMAT,
                consts::FR_PROTOCOL_PARITY_VIOLATION_COUNT_MISMATCH,
                format!(
                    "line {line} FRD declares {fr_count} requirements but the feature's contract \
                     module declares {protocol_count} protocol classes"
                ),
            )
            .with_reason(
                "One protocol class is one capability seam and one seam is one requirement, so the two counts must be equal.",
                direction,
            ),
        );
    }

    /// Every method the FRD promises must be declared in the code.
    ///
    /// The `Protocol API` and `Aggregate API` tables are a promise to the
    /// integrator; a row whose method no protocol/aggregate trait declares
    /// is a promise the code does not keep. The finding names both remedies:
    /// drop the promise from the table, or declare the method. A feature
    /// whose contract module cannot be read keeps the check silent, the same
    /// error-handling contract as the parity count.
    fn check_api_method_existence(
        &self,
        document: &DocSource,
        context: &DocAuditContext,
        findings: &mut Vec<DocFinding>,
    ) {
        let Some(feature) = document
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
        else {
            return;
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
            return;
        };
        for (subsection, method, line) in api_contract_methods(&document.text) {
            if declared.contains(&method) {
                continue;
            }
            findings.push(
                DocFinding::new_with_line(
                    "",
                    line,
                    consts::RULE_CODE_FR_FORMAT,
                    consts::FR_API_METHOD_VIOLATION_NOT_FOUND,
                    format!(
                        "line {line} method `{method}` in the '{subsection}' table is not declared \
                         by any protocol/aggregate trait in the feature's contract module"
                    ),
                )
                .with_reason(
                    "The `Protocol API` and `Aggregate API` tables are a promise to the integrator; \
                     a row whose method no protocol/aggregate trait declares is a promise the code \
                     does not keep.",
                    format!(
                        "Remove `{method}` from the '{subsection}' table or create the \
                         protocol/aggregate method."
                    ),
                ),
            );
        }
    }

    /// Accepted PR/FR/DS/DT ID set from a document, whichever family its
    /// headings carry. A document may declare several families (a feature
    /// folder's FRD declares FR-IDs and a paired DESIGN.md declares
    /// DS-IDs), so the collector unions the two that are present.
    pub(crate) fn collect_any_id_set(text: &str) -> Option<Vec<String>> {
        let mut ids = collect_pr_id_set(text).unwrap_or_default();
        ids.extend(Self::collect_fr_id_set(text).unwrap_or_default());
        ids.extend(collect_ds_id_set(text).unwrap_or_default());
        ids.extend(collect_dt_id_set(text).unwrap_or_default());
        if ids.is_empty() { None } else { Some(ids) }
    }

    /// Accepted FR-ID set from an FRD's `FR-<feature>-NNN:` headings.
    ///
    /// Returns `None` when the FRD declares no FR-IDs, leaving the alignment
    /// check silent rather than flagging every BACKLOG row.
    pub(crate) fn collect_fr_id_set(frd: &str) -> Option<Vec<String>> {
        let re = fr_id_heading_re()?;
        let ids: Vec<String> = re
            .captures_iter(frd)
            .filter_map(|caps| caps.name("id").map(|m| m.as_str().to_string()))
            .collect();
        if ids.is_empty() { None } else { Some(ids) }
    }

    /// PRD ↔ ROADMAP PR-ID parity (Decision 6).
    ///
    /// Every `PR-<PROJECT>-NNN` item declared in the PRD's `Project
    /// Requirements` must have a matching ROADMAP `Roadmap Status` entry,
    /// and vice versa. The two directions fire separate codes so a reader
    /// can tell which document drifted.
    fn check_pr_id_parity(
        &self,
        prd: &DocSource,
        roadmap: &DocSource,
        findings: &mut Vec<DocFinding>,
    ) {
        let pr_ids = collect_pr_id_set(&prd.text).unwrap_or_default();
        let roadmap_ids = collect_pr_id_set(&roadmap.text).unwrap_or_default();
        let pr_set: HashSet<&str> = pr_ids.iter().map(String::as_str).collect();
        let roadmap_set: HashSet<&str> = roadmap_ids.iter().map(String::as_str).collect();

        // A PRD item missing from the ROADMAP → pr_roadmap_id_missing,
        // anchored to the item's heading line in the ROADMAP's `## Roadmap
        // Status` section (the document that owes the entry).
        let roadmap_heading_line = Self::section_line(&roadmap.text, "roadmap status").unwrap_or(1);
        for id in &pr_ids {
            if !roadmap_set.contains(id.as_str()) {
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        roadmap_heading_line,
                        consts::RULE_CODE_FR_FORMAT,
                        consts::PR_ROADMAP_ID_MISSING,
                        format!("PR-ID `{id}` declared in the PRD has no ROADMAP `Roadmap Status` entry"),
                    )
                    .with_reason(
                        "Every PRD `PR-` item must appear in the ROADMAP so the master tracks the project's whole promise.",
                        format!("Add a ROADMAP `Roadmap Status` entry `### {id}: <name>`."),
                    ),
                );
            }
        }

        // A ROADMAP item not declared in the PRD → roadmap_pr_id_not_pr,
        // anchored to the item's own heading line in the ROADMAP.
        let roadmap_pr_lines = Self::pr_id_heading_lines(&roadmap.text);
        for (line, id) in &roadmap_pr_lines {
            if !pr_set.contains(id.as_str()) {
                let mut valid: Vec<&str> = pr_ids.iter().map(String::as_str).collect();
                valid.sort();
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        *line,
                        consts::RULE_CODE_FR_FORMAT,
                        consts::ROADMAP_PR_ID_NOT_PR,
                        format!("line {line} ROADMAP item `{id}` is not declared in the PRD"),
                    )
                    .with_reason(
                        "Every ROADMAP `PR-` item must be a PRD-declared requirement so the master does not invent promises.",
                        format!("Declare `{id}` in the PRD, or remove it from the ROADMAP. Valid PR-IDs: {}.", valid.join(", ")),
                    ),
                );
            }
        }
    }
    /// 1-based line of the H2 whose normalized title is *title*, or `None`
    /// when no such section exists.
    fn section_line(text: &str, title: &str) -> Option<usize> {
        sections(text)
            .iter()
            .find(|s| s.level == 2 && normalize_heading(&s.title) == title)
            .map(|s| s.line + 1)
    }

    /// `(1-based heading line, id)` for every PR-ID heading in *text*.
    fn pr_id_heading_lines(text: &str) -> Vec<(usize, String)> {
        let Some(re) = pr_id_heading_re() else {
            return Vec::new();
        };
        re.captures_iter(text)
            .filter_map(|caps| {
                let id = caps.name("id")?.as_str().to_string();
                let start = caps.get(0)?.start();
                let line = text[..start].lines().count().max(1);
                Some((line, id))
            })
            .collect()
    }

    /// BACKLOG entry H3s under `## Backlog Status`: `(token, 1-based
    /// heading line, stray ids in the entry's Next/Done/Blocked rows)` —
    /// one tuple per entry.
    ///
    /// A qualifying H3 title is exactly one brace-wrapped token
    /// (`{FR-SAMPLE-001}`); a title that does not read as one is skipped, so a
    /// BACKLOG with malformed entries is silent on alignment. Stray ids are
    /// every `PR-`/`FR-`/`DS-`/`DT-` token in the entry's `Next`, `Done`,
    /// and `Blocked` data rows (row labels come from `BACKLOG_ROW_LABELS`).
    ///
    /// Returns `(entries, section_line)`; `section_line` is `0` only when no
    /// `## Backlog Status` section exists, which makes the caller silent.
    pub(crate) fn parse_backlog_entries(
        backlog: &DocSource,
    ) -> (Vec<(String, usize, Vec<String>)>, usize) {
        let parsed = sections(&backlog.text);
        let Some(section) = parsed
            .iter()
            .find(|s| s.level == 2 && normalize_heading(&s.title) == "backlog status")
        else {
            return (Vec::new(), 0);
        };
        let Some(stray_re) = regex::Regex::new(r"\b((?:PR|FR|DS|DT)-[A-Za-z0-9_]+-\d+)").ok()
        else {
            return (Vec::new(), section.line + 1);
        };
        let mut entries: Vec<(String, usize, Vec<String>)> = Vec::new();
        for sub in h3_subsections(section) {
            let Some(token) = Self::extract_qualifier(&sub.title) else {
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
        // `+1` converts the 0-based heading index to the 1-based document
        // line of the `## Backlog Status` heading.
        (entries, section.line + 1)
    }

    /// The single brace-wrapped ID token of a BACKLOG entry H3 title
    /// (`{FR-SAMPLE-001}` → `FR-SAMPLE-001`), or `None` for any other shape.
    /// `<ID>: <title>` single-qualifier heading (BACKLOG `Backlog Status` entries).
    /// Strips the `:` title suffix to yield the raw ID token
    /// (`FR-ECOMMERCE-001: Checkout flow` → `FR-ECOMMERCE-001`), per Decision 9.
    fn extract_qualifier(title: &str) -> Option<String> {
        let inner = title.trim();
        let id = inner.split(':').next()?.trim();
        let id = id.chars().filter(|c| *c != '|').collect::<String>();
        if id.is_empty() {
            return None;
        }
        Some(id.trim().to_string())
    }

    /// Union of the accepted ID sets of up to two documents under a shared
    /// collector. `None` documents are skipped, and the result is the empty
    /// set when neither document declares any IDs — a union the caller can
    /// test for silence.
    pub(crate) fn collect_union_id_set(
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

    /// BACKLOG ID-level alignment against a paired spec union
    /// (Decision 9), parameterized by the family's missing-code and not-code.
    ///
    /// (a) An entry token that no declared ID equals verbatim fires
    /// `backlog_qualifier_mismatch`, anchored to the entry's H3 line.
    /// (b) A declared ID with no entry fires the *missing_code*
    /// (analog of `backlog_id_missing`), anchored to `## Backlog Status`.
    /// (c) A stray `PR-`/`FR-`/`DS-`/`DT-` token in an entry's
    /// `Next`/`Done`/`Blocked` row that no declared ID is fires the
    /// *not_code* (analog of `backlog_id_not_fr`), anchored to the
    /// entry's H3 line.
    ///
    /// Silent when the BACKLOG has no `## Backlog Status` section.
    /// Directions (a) and (b) are silent when the paired spec declares no
    /// IDs; direction (c) stray-ID still fires against an empty declared set.
    fn check_backlog_qualifier_alignment(
        &self,
        backlog: &DocSource,
        union: Option<&Vec<String>>,
        missing_code: &'static str,
        not_code: &'static str,
        findings: &mut Vec<DocFinding>,
    ) {
        let ids: Vec<String> = union.cloned().unwrap_or_default();
        let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
        let (entries, section_line) = Self::parse_backlog_entries(backlog);
        if section_line == 0 {
            return;
        }

        // (a) entry token not equal to any declared ID.
        // Silent when the union declares no IDs — nothing can carry a token.
        if !ids.is_empty() {
            for (token, line, _) in &entries {
                if !id_set.contains(token.as_str()) {
                    let mut valid: Vec<&str> = ids.iter().map(String::as_str).collect();
                    valid.sort();
                    findings.push(
                        DocFinding::new_with_line(
                            "",
                            *line,
                            consts::RULE_CODE_FR_FORMAT,
                            consts::BACKLOG_QUALIFIER_MISMATCH,
                            format!(
                                "line {line} BACKLOG entry `{{{token}}}` has no matching ID in the paired spec"
                            ),
                        )
                        .with_reason(
                            "A BACKLOG entry traces to its spec through the ID, so a token no spec ID equals cannot be aligned.",
                            format!(
                                "Rename the entry to an ID the spec declares: {}.",
                                valid.join(", ")
                            ),
                        ),
                    );
                }
            }
        }

        // (b) declared ID with no BACKLOG entry, once per ID.
        // Silent when the union declares no IDs — nothing is declared to track.
        if !ids.is_empty() {
            let entry_set: HashSet<&str> = entries.iter().map(|(t, _, _)| t.as_str()).collect();
            for id in &ids {
                if !entry_set.contains(id.as_str()) {
                    findings.push(
                        DocFinding::new_with_line(
                            "",
                            section_line,
                            consts::RULE_CODE_FR_FORMAT,
                            missing_code,
                            format!(
                                "spec-declared ID `{id}` has no BACKLOG `## Backlog Status` entry"
                            ),
                        )
                        .with_reason(
                            "Every spec-declared ID must have a BACKLOG entry so the backlog tracks every requirement.",
                            format!("Add a BACKLOG entry `### {{{id}}}` under `## Backlog Status`."),
                        ),
                    );
                }
            }
        }

        // (c) stray id token in an entry's Next/Done/Blocked row.
        // Runs even when the union declares no IDs: every cited token is then
        // untraceable because nothing is declared.
        for (token, line, stray_ids) in &entries {
            for id in stray_ids {
                if id_set.contains(id.as_str()) {
                    continue;
                }
                let mut valid: Vec<&str> = ids.iter().map(String::as_str).collect();
                valid.sort();
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        *line,
                        consts::RULE_CODE_FR_FORMAT,
                        not_code,
                        format!(
                            "line {line} BACKLOG entry `{{{token}}}` cites `{id}`, which the paired spec does not declare"
                        ),
                    )
                    .with_reason(
                        "A BACKLOG row cites an id its paired spec does not declare, so the citation is untraceable.",
                        format!("Replace `{id}` with one of: {}.", valid.join(", ")),
                    ),
                );
            }
        }
    }
}
