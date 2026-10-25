// PURPOSE: MemberDocChecker — AES606: member-folder document presence.
//
// Every workspace member folder must contain its mandated documentation pair
// based on its architectural role, classified by objective crate structure:
//   - Feature: `FRD.md` + `BACKLOG.md`
//   - Kernel: `DATA.md` + `BACKLOG.md`
//   - Surface: `DESIGN.md` + `BACKLOG.md`
//   - None: no documentation required
//
// Capabilities perform zero I/O; member discovery and classification are
// performed once in the shared utility collector (`context.members()`).
// The check is root-gated: it fires only when the audit root is a real
// workspace root (has workspace root markers).
// Config-driven suppression: short-circuits when `effective_strict_member_docs`
// is false, and skips members listed in `exempt_members`.
use shared_doc_rules::contract_doc_protocol::IMemberDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::{DocAuditContext, MemberRole};
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
use std::path::Path;

// ─── Block 1: Struct Definition ────────────────────────────

/// AES606 member-document presence auditor: verifies required document pairs
/// per member folder.
pub struct MemberDocChecker;

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IMemberDocProtocol for MemberDocChecker {
    fn audit_member_docs(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        // When strict_member_docs is effectively disabled, short-circuit.
        if !context.effective_strict_member_docs() {
            return Vec::new();
        }

        // Root-gate: only fire when the audit root is a real workspace root.
        if !context.is_workspace_root() {
            return Vec::new();
        }

        let exempt_members = context
            .config()
            .map(|c| c.exempt_members.as_slice())
            .unwrap_or(&[]);

        let mut findings = Vec::new();

        for member in context.members() {
            if exempt_members.iter().any(|exempt| {
                member.path == Path::new(exempt) || member.path.ends_with(Path::new(exempt))
            }) {
                continue;
            }

            let (required_docs, role_name): (&'static [&'static str], &'static str) =
                match member.role {
                    MemberRole::Feature => (&[consts::FRD_DOC, consts::BACKLOG_DOC], "feature"),
                    MemberRole::Kernel => (&[consts::DATA_DOC, consts::BACKLOG_DOC], "kernel"),
                    MemberRole::Surface => (&[consts::DESIGN_DOC, consts::BACKLOG_DOC], "surface"),
                    // A `None` member owes no doc pair; the first match
                    // already carries the decision, so the tuple below only
                    // ever describes a doc-owing role.
                    MemberRole::None => continue,
                };

            let member_name = member
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            for &doc in required_docs {
                if !member.present_docs.contains(doc) {
                    let doc_rel_path = member
                        .path
                        .strip_prefix(context.root())
                        .unwrap_or(&member.path)
                        .join(doc)
                        .to_string_lossy()
                        .replace('\\', "/");

                    findings.push(
                        DocFinding::new_with_line(
                            doc_rel_path,
                            0,
                            consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                            consts::MEMBER_DOC_VIOLATION,
                            format!(
                                "Member folder `{}` is a {role_name} member but is missing required specification `{doc}`.",
                                member.path.display()
                            ),
                        )
                        .with_reason(
                            format!(
                                "Every {role_name} member must maintain its mandated documentation pair to preserve architectural contract and status traceability."
                            ),
                            format!(
                                "Run `lint-arwaky-cli docs scaffold --member {member_name}` or create `{doc}` in `{}`.",
                                member.path.display()
                            ),
                        ),
                    );
                }
            }
        }

        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for MemberDocChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MemberDocChecker {
    pub fn new() -> Self {
        Self
    }
}
