// PURPOSE: Per-doc capability for FRD.md — AES609
//
// Answers ALL invariants scoped to FRD.md behind IFrdDocProtocol.
// FR-format family violations carry RULE_CODE_FR_FORMAT ("AES601");
// heading/structure/spec/crosslink/template findings carry their own rule
// codes. The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IFrdDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_doc_aspect_entry_checks::check_aspect_entry_tables;
use shared_doc_rules::utility_doc_common_checks::{
    check_doc_heading, check_entry_tables, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::fr_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::{
    check_api_method_existence, check_fr_id_format, check_fr_protocol_parity,
};
use shared_doc_rules::utility_markdown_scanner::{
    blank_fenced, doc_h2_contract, normalize_heading, sections, source_ext_pattern,
    status_leak_patterns,
};

pub struct FrdCapability;

impl FrdCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IFrdDocProtocol for FrdCapability {
    fn audit_frd(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::FRD_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
        }
        findings
    }
}

impl FrdCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES601 — FR-ID format (bare FR-NNN without feature prefix).
        check_fr_id_format(document, findings);

        // AES601 — FR protocol parity (requirement count vs contract seams).
        check_fr_protocol_parity(document, context, findings);

        // AES601 — promised API method existence.
        check_api_method_existence(document, context, findings);

        // AES603 — spec purity (no source file names, no implementation state).
        Self::check_spec_purity(document, findings);

        // AES604 — crosslinks (PRD, BACKLOG, ROADMAP, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure (exactly one H1, required H2s, no off-template H2s).
        if let Some((required, allowed)) = doc_h2_contract(consts::FRD_DOC) {
            check_doc_heading(consts::RULE_CODE_FRD_DOC, text, consts::FRD_DOC, required, allowed, findings);
        }

        // AES602 — section order follows the template.
        Self::check_section_order(
            text,
            consts::FRD_SECTION_ORDER,
            consts::RULE_CODE_SECTION_STRUCTURE,
            consts::SECTION_STRUCTURE_VIOLATION_ORDER,
            findings,
        );

        // AES606 — entry tables under Functional Requirements.
        if let Some(re) = fr_entry_re() {
            findings.extend(check_entry_tables(
                consts::RULE_CODE_FRD_DOC,
                &parsed,
                "Functional Requirements",
                re,
                consts::ASPECT_DETAIL_COLUMNS,
                consts::FRD_ROW_LABELS,
            ));
        }

        // AES606 — H4 aspect entry tables under Functional Requirements.
        if let Some(re) = fr_entry_re() {
            findings.extend(check_aspect_entry_tables(
                consts::RULE_CODE_FRD_DOC,
                &parsed,
                "Functional Requirements",
                re,
                consts::FRD_ASPECTS,
                consts::FRD_ASPECTS_OPEN,
                consts::FRD_ASPECT_SEED_MIN,
                false,
            ));
        }
    }

    /// Inline spec-purity check: no concrete source-file names, no
    /// implementation-state claims. Stamped with RULE_CODE_SPEC_PURITY ("AES603").
    fn check_spec_purity(document: &DocSource, findings: &mut Vec<DocFinding>) {
        let prose = blank_fenced(&document.text);

        // Source file references (.rs, .py, .ts, .tsx).
        if let Some(re) = source_ext_pattern() {
            for caps in re.captures_iter(&prose) {
                let line = document.text[..caps.get(0).map_or(0, |m| m.start())]
                    .lines()
                    .count()
                    .max(1);
                let name = caps.get(0).map_or("", |m| m.as_str());
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        line,
                        consts::RULE_CODE_SPEC_PURITY,
                        consts::SPEC_PURITY_VIOLATION_SOURCE_FILE_NAMED,
                        format!("{name} names a concrete source file"),
                    )
                    .with_reason(
                        "A specification describes what, not where; naming a source file leaks implementation detail.",
                        format!("Remove the reference to `{name}` and describe the responsibility instead."),
                    ),
                );
            }
        }

        // Status leaks (checkboxes, status fields, implementation state, etc.).
        if let Some(patterns) = status_leak_patterns() {
            for (re, desc) in patterns {
                for caps in re.captures_iter(&prose) {
                    let line = document.text[..caps.get(0).map_or(0, |m| m.start())]
                        .lines()
                        .count()
                        .max(1);
                    findings.push(
                        DocFinding::new_with_line(
                            "",
                            line,
                            consts::RULE_CODE_SPEC_PURITY,
                            consts::SPEC_PURITY_VIOLATION_STATUS_LEAK,
                            format!("{desc}"),
                        )
                        .with_reason(
                            "A spec is a contract, not a status report; implementation state belongs in the backlog.",
                            "Remove the status claim and keep the document specification-only.",
                        ),
                    );
                }
            }
        }
    }

    /// Crosslink check: every required link target must appear as a link in
    /// the document. Stamped with the given rule code.
    fn check_crosslinks(
        document: &DocSource,
        context: &DocAuditContext,
        code: &'static str,
        findings: &mut Vec<DocFinding>,
    ) {
        let name = context.document_name(document);
        for &link in required_links(name) {
            if !has_link(document, link) {
                let violation_type = missing_link_violation_type(link);
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        0,
                        code,
                        violation_type,
                        format!("{name} does not link its required {link}"),
                    )
                    .with_reason(
                        "Every document must cross-link its required artifacts so the chain stays navigable.",
                        format!("Add a link to {link} in the Reference section."),
                    ),
                );
            }
        }
    }

    /// Section order: required H2s must appear in template order.
    fn check_section_order(
        text: &str,
        required: &[&str],
        code: &'static str,
        violation_type: &'static str,
        findings: &mut Vec<DocFinding>,
    ) {
        let parsed = sections(text);
        let required_norm: Vec<String> = required.iter().map(|title: &&str| normalize_heading(title)).collect();
        let mut indices: Vec<usize> = Vec::new();
        for s in &parsed {
            if s.level == 2 {
                let norm = normalize_heading(&s.title);
                for (i, req) in required_norm.iter().enumerate() {
                    if norm == *req || norm.starts_with(req) {
                        indices.push(i);
                        break;
                    }
                }
            }
        }
        for i in 1..indices.len() {
            if indices[i] <= indices[i - 1] {
                findings.push(DocFinding::new_with_line(
                    "",
                    0,
                    code,
                    violation_type,
                    "sections are out of template order".to_string(),
                ));
                return;
            }
        }
    }
}

/// True when *document* contains a markdown link to *target*.
fn has_link(document: &DocSource, target: &str) -> bool {
    document.text.contains(&format!("[{target}"))
}
