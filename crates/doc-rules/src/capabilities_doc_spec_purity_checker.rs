// PURPOSE: SpecPurityChecker — AES603: a specification carries no
// implementation state and names no source file.
//
// The one capability that answers AES603. It inspects the promise-bearing
// documents — the specs, not the backlogs — and reports two violation
// families: a line that leaks implementation state (a checkbox, a status
// field, a progress percentage), and a line that names a concrete source file.
// The document set is a decision this capability owns: a backlog reports
// progress, so purity is a property of specs alone.
use shared_doc_rules::contract_doc_protocol::ISpecPurityProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};

use shared_doc_rules::utility_markdown_scanner::{source_ext_pattern, status_leak_patterns};

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES603 auditor: no implementation state, no source-file names.
pub struct SpecPurityChecker {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ISpecPurityProtocol for SpecPurityChecker {
    /// Report every status leak and source-file reference in the documents of
    /// *context*.
    fn audit_spec_purity(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if !Self::is_spec(context.document_name(document)) {
                continue;
            }
            let mut own = Vec::new();
            self.check_status_leak(document, &mut own);
            self.check_source_paths(document, &mut own);
            context.stamp(document, &mut own);
            findings.extend(own);
        }
        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for SpecPurityChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SpecPurityChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Is this document a spec (promise-bearing) rather than a status report?
    ///
    /// The distinction is the whole rule: a BACKLOG records that work shipped,
    /// which is its job, so only the specs are held to purity.
    fn is_spec(name: &str) -> bool {
        matches!(
            name,
            consts::FRD_DOC | consts::DATA_DOC | consts::PRD_DOC | consts::ROADMAP_DOC
        )
    }

    /// A spec must not carry implementation state.
    fn check_status_leak(&self, document: &DocSource, findings: &mut Vec<DocFinding>) {
        let Some(patterns) = status_leak_patterns() else {
            return;
        };
        for (number, line) in document.text.lines().enumerate() {
            // Table rows define status vocabulary (e.g. the "Implemented"
            // glossary in ROADMAP.md); they are definitions, not claims.
            if line.trim_start().starts_with('|') {
                continue;
            }
            for (pattern, what) in patterns {
                // A macro invocation such as `unimplemented!` names a language
                // token rather than making a claim about this feature's
                // progress, so a match immediately followed by `!` is skipped.
                if !pattern
                    .find_iter(line)
                    .any(|m| !line[m.end()..].starts_with('!'))
                {
                    continue;
                }
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        number + 1,
                        consts::RULE_CODE_SPEC_PURITY,
                        consts::SPEC_PURITY_VIOLATION_STATUS_LEAK,
                        format!("line {} carries {what} ('{}')", number + 1, line.trim()),
                    )
                    .with_reason(
                        "Specs state what the system promises; implementation state belongs in the backlog, so a spec carrying it no longer reads as a stable promise.",
                        format!("Move the {what} claim on line {} to BACKLOG.md.", number + 1),
                    ),
                );
            }
        }
    }

    /// A spec must not name source files.
    fn check_source_paths(&self, document: &DocSource, findings: &mut Vec<DocFinding>) {
        let Some(re) = source_ext_pattern() else {
            return;
        };
        for (number, line) in document.text.lines().enumerate() {
            if let Some(matched) = re.find(line) {
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        number + 1,
                        consts::RULE_CODE_SPEC_PURITY,
                        consts::SPEC_PURITY_VIOLATION_SOURCE_FILE_NAMED,
                        format!(
                            "line {} names source file '{}'",
                            number + 1,
                            matched.as_str()
                        ),
                    )
                    .with_reason(
                        "Specs are stateless: naming a source file ties the promise to a current file layout that will move.",
                        format!(
                            "Replace the reference to '{}' on line {} with the role or behaviour it describes.",
                            matched.as_str(),
                            number + 1
                        ),
                    ),
                );
            }
        }
    }
}
