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

use shared_doc_rules::utility_markdown_scanner::{api_contract_methods, fr_id_bare_re};
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
    /// Report every FR-ID, FR-field, and parity violation in the documents of
    /// *context*.
    fn audit_fr_format(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            let name = context.document_name(document);
            // An FRD and a DATA document state requirements under the same ID
            // and field rules; parity is an FRD-only question because only an
            // FRD names a feature's contract module.
            let is_requirement_doc = name == consts::FRD_DOC || name == consts::DATA_DOC;
            if !is_requirement_doc {
                continue;
            }
            let mut own = Vec::new();
            self.check_fr_id_format(document, &mut own);
            self.check_fr_fields(document, &mut own);
            if name == consts::FRD_DOC {
                self.check_fr_protocol_parity(document, context, &mut own);
                self.check_api_method_existence(document, context, &mut own);
            }
            context.stamp(document, &mut own);
            findings.extend(own);
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
            findings.push(DocFinding::new_with_line(
                "",
                line,
                consts::RULE_CODE_FR_FORMAT,
                consts::FR_ID_VIOLATION_MISSING_FEATURE_PREFIX,
                format!(
                    "line {line} heading 'FR-{num}: {title}' is missing its feature prefix; it must be 'FR-<FEATURE>-NNN: <imperative name>'",
                ),
            ));
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
                findings.push(DocFinding::new_with_line(
                    "",
                    line,
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
        findings.push(DocFinding::new_with_line(
            "",
            line,
            consts::RULE_CODE_FR_FORMAT,
            consts::FR_PROTOCOL_PARITY_VIOLATION_COUNT_MISMATCH,
            format!(
                "line {line} FRD declares {fr_count} requirements but the feature's contract \
                 module declares {protocol_count} protocol classes; {direction}"
            ),
        ));
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
            findings.push(DocFinding::new_with_line(
                "",
                line,
                consts::RULE_CODE_FR_FORMAT,
                consts::FR_API_METHOD_VIOLATION_NOT_FOUND,
                format!(
                    "line {line} method `{method}` in the '{subsection}' table is not declared \
                     by any protocol/aggregate trait in the feature's contract module; either \
                     remove `{method}` from the table or create the protocol/aggregate method"
                ),
            ));
        }
    }
}
