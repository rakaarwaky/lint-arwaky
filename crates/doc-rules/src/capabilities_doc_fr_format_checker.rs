// PURPOSE: FrFormatChecker — AES601: requirement-ID shape, required FR fields,
// FR/protocol-class parity, and API-table method existence.
//
// The one capability that answers AES601. It reads the requirement documents
// (an FRD and a DATA document state requirements under the same ID rules) and
// reports four violation families: an ID that lost its feature prefix, a
// requirement block missing one of the six contract fields, a requirement
// count that has drifted from the feature's count of capability seams, and a
// promised Protocol/Aggregate API method that the contract module never
// declares.
use shared_doc_rules::contract_doc_protocol::IFrFormatProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use shared_doc_rules::utility_markdown_scanner::{
    api_contract_methods, fr_id_bare_re, normalize_heading, sections,
};
use shared_doc_rules::utility_protocol_counter::{
    contract_method_names, count_fr_headings, count_protocol_traits, fr_id_heading_re,
    locate_kernel_srcs,
};

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES601 auditor: FR-ID format, FR-field completeness, FR/protocol class
/// parity, and API-table method existence.
pub struct FrFormatChecker {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IFrFormatProtocol for FrFormatChecker {
    /// Report every FR-ID, FR-field, parity, and promised-method violation in
    /// the documents of *context*.
    ///
    /// Documents are grouped by parent folder so the FRD/BACKLOG alignment
    /// sub-check can read both members of a feature folder together. DATA
    /// documents keep their own FR-ID and field checks outside pairing.
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
            let frd = documents
                .iter()
                .find(|d| context.document_name(d) == consts::FRD_DOC);
            let backlog = documents
                .iter()
                .find(|d| context.document_name(d) == consts::BACKLOG_DOC);
            if let Some(frd) = frd {
                let mut own = Vec::new();
                self.check_fr_id_format(frd, &mut own);
                self.check_fr_fields(frd, &mut own);
                self.check_fr_protocol_parity(frd, context, &mut own);
                self.check_api_method_existence(frd, context, &mut own);
                if let Some(backlog) = backlog {
                    self.check_backlog_fr_id_alignment(frd, backlog, &mut own);
                }
                context.stamp(frd, &mut own);
                findings.extend(own);
            }
            for document in documents {
                if context.document_name(document) == consts::DATA_DOC {
                    let mut own = Vec::new();
                    self.check_fr_id_format(document, &mut own);
                    self.check_fr_fields(document, &mut own);
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

    /// Every requirement states all six FR fields.
    ///
    /// Uses the shared FR-ID pattern so the field check and the parity
    /// counter anchor on the same accepted ID set; a heading the counter
    /// skips can never produce a missing-field finding.
    fn check_fr_fields(&self, document: &DocSource, findings: &mut Vec<DocFinding>) {
        let Some(re) = fr_id_heading_re() else {
            return;
        };
        let matches: Vec<_> = re.captures_iter(&document.text).collect();
        for (index, caps) in matches.iter().enumerate() {
            let id = caps.name("id").map_or("", |m| m.as_str()).to_string();
            let start = caps.get(0).map_or(0, |m| m.end());
            let end = matches
                .get(index + 1)
                .and_then(|n| n.get(0))
                .map_or(document.text.len(), |m| m.start());
            let body = &document.text[start.min(document.text.len())..end.min(document.text.len())];
            let missing: Vec<&str> = consts::FR_FIELDS
                .iter()
                .copied()
                .filter(|field| !body.contains(&format!("**{field}**")))
                .collect();
            if !missing.is_empty() {
                let line = document.text[..caps.get(0).map_or(0, |m| m.start())]
                    .lines()
                    .count()
                    .max(1);
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        line,
                        consts::RULE_CODE_FR_FORMAT,
                        consts::FR_FIELDS_VIOLATION_FIELD_MISSING,
                        format!(
                            "line {line} {id} is missing required field(s): {}",
                            missing.join(", ")
                        ),
                    )
                    .with_reason(
                        format!(
                            "The contract requires all of {} so a reader can evaluate the requirement against the full specification.",
                            consts::FR_FIELDS.join(", ")
                        ),
                        format!("Add the missing field(s) {} to the {id} block.", missing.join(", ")),
                    ),
                );
            }
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
             HOW-TO-MAKE-FRD.md"
        } else {
            "the code has fewer protocol classes than requirements; either merge the requirements \
             down to match, or split the methods into more classes — see the 4-direction table in \
             HOW-TO-MAKE-FRD.md"
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

    /// FR-ID ↔ BACKLOG ID alignment for one feature folder.
    ///
    /// Groups the FRD's accepted FR-ID set against the BACKLOG `## Backlog`
    /// table's ID column and reports each mismatch in both directions. Silent
    /// when the FRD declares no FR-IDs or the BACKLOG has no `## Backlog`
    /// section.
    fn check_backlog_fr_id_alignment(
        &self,
        frd: &DocSource,
        backlog: &DocSource,
        findings: &mut Vec<DocFinding>,
    ) {
        let Some(fr_ids) = Self::collect_fr_id_set(frd) else {
            return;
        };
        let (backlog_rows, section_line) = Self::parse_backlog_id_column(backlog);
        if section_line == 0 {
            return;
        }

        // (a) FR-ID present in FRD, absent from BACKLOG → backlog_id_missing.
        let backlog_id_set: HashSet<&str> =
            backlog_rows.iter().map(|(id, _)| id.as_str()).collect();
        for fr_id in &fr_ids {
            if !backlog_id_set.contains(fr_id.as_str()) {
                Self::push_backlog_id_missing(section_line, fr_id, findings);
            }
        }

        // (b) BACKLOG ID not a valid FRD FR-ID → backlog_id_not_fr.
        let mut valid: Vec<&str> = fr_ids.iter().map(String::as_str).collect();
        valid.sort();
        for (id, line) in &backlog_rows {
            if !fr_ids.contains(id.as_str()) {
                Self::push_backlog_id_not_fr(*line, id, &valid, findings);
            }
        }
    }

    /// Accepted FR-ID set from an FRD's `FR-<feature>-NNN:` headings.
    ///
    /// Returns `None` when the FRD declares no FR-IDs, leaving the alignment
    /// check silent rather than flagging every BACKLOG row.
    fn collect_fr_id_set(frd: &DocSource) -> Option<HashSet<String>> {
        let re = fr_id_heading_re()?;
        let ids: HashSet<String> = re
            .captures_iter(&frd.text)
            .filter_map(|caps| caps.name("id").map(|m| m.as_str().to_string()))
            .collect();
        if ids.is_empty() { None } else { Some(ids) }
    }

    /// ID column of the `## Backlog` table, scoped to that section.
    ///
    /// Walks the section's body lines, keeping data rows only: skips the
    /// header row, the separator row, and non-pipe lines. Returns
    /// `((id_cell, 1-based row line) rows, 1-based heading line)`; the
    /// heading line is `0` only when no `## Backlog` section exists.
    fn parse_backlog_id_column(backlog: &DocSource) -> (Vec<(String, usize)>, usize) {
        let parsed = sections(&backlog.text);
        let Some(section) = parsed
            .iter()
            .find(|s| s.level == 2 && normalize_heading(&s.title) == "backlog")
        else {
            return (Vec::new(), 0);
        };

        let mut rows = Vec::new();
        let mut header_seen = false;
        for (offset, raw) in section.body.lines().enumerate() {
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
            let id = cells.first().copied().unwrap_or("").to_string();
            if id.is_empty() {
                continue;
            }
            // `+2` because the body skips the heading line and the offset is 0-based.
            rows.push((id, section.line + offset + 2));
        }
        // `+1`: `section.line` is 0-based, so the heading's 1-based document
        // line matches the 1-based row lines above.
        (rows, section.line + 1)
    }

    /// Anchored to the bad BACKLOG row; lists the FRD's valid FR-IDs as the remedy.
    fn push_backlog_id_not_fr(
        line: usize,
        id: &str,
        fr_ids: &[&str],
        findings: &mut Vec<DocFinding>,
    ) {
        findings.push(
            DocFinding::new_with_line(
                "",
                line,
                consts::RULE_CODE_FR_FORMAT,
                consts::FR_BACKLOG_ID_NOT_FR,
                format!(
                    "line {line} BACKLOG row id `{id}` is not a valid FRD FR-ID"
                ),
            )
            .with_reason(
                "Every BACKLOG `## Backlog` row must carry an FRD FR-ID so the backlog stays traceable to a requirement.",
                format!("Replace `{id}` with one of the FRD FR-IDs: {}.", fr_ids.join(", ")),
            ),
        );
    }

    /// Anchored to the `## Backlog` section heading.
    fn push_backlog_id_missing(line: usize, fr_id: &str, findings: &mut Vec<DocFinding>) {
        findings.push(
            DocFinding::new_with_line(
                "",
                line,
                consts::RULE_CODE_FR_FORMAT,
                consts::FR_BACKLOG_ID_MISSING,
                format!(
                    "FR-ID `{fr_id}` declared in the FRD has no BACKLOG `## Backlog` row"
                ),
            )
            .with_reason(
                "Every FRD FR-ID must appear in the BACKLOG `## Backlog` table so the backlog tracks every requirement.",
                format!(
                    "Add a BACKLOG `## Backlog` row whose ID cell is `{fr_id}`."
                ),
            ),
        );
    }
}
