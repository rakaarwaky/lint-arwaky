// PURPOSE: The six capability seams of the doc-rules feature — one protocol
// trait per AES document rule (AES601–AES606).
//
// Each trait answers exactly one rule and carries exactly one method, so a
// requirement maps to a seam one-for-one and the parity check can hold. The
// agent behind the aggregate injects all five, hands each the same audit
// context, and merges what they report; no consumer ever sees a protocol.
use crate::taxonomy_doc_audit_context_vo::DocAuditContext;
use crate::taxonomy_doc_rules_request::DocFinding;

/// AES601 — requirement identifiers, required FR fields, FR/protocol-class
/// parity, and the API methods the FRD promises the code declares.
pub trait IFrFormatProtocol: Send + Sync {
    /// Report every FR-ID, FR-field, parity, and promised-method violation in
    /// the documents of *context*.
    fn audit_fr_format(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// AES602 — the template section order and the shape of each mandated section.
pub trait ISectionStructureProtocol: Send + Sync {
    /// Report every section-order and section-shape violation in the documents
    /// of *context*.
    fn audit_section_structure(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// AES603 — a specification carries no implementation state and names no
/// source file.
pub trait ISpecPurityProtocol: Send + Sync {
    /// Report every status leak and source-file reference in the documents of
    /// *context*.
    fn audit_spec_purity(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// AES604 — the Reference crosslinks a document owes, and the single home the
/// state vocabulary may live in.
pub trait ICrosslinkProtocol: Send + Sync {
    /// Report every missing crosslink and every restated master-only section in
    /// the documents of *context*.
    fn audit_crosslinks(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// AES605 — the H1/H2 heading structure each recognised document must hold.
pub trait IDocHeadingProtocol: Send + Sync {
    /// Report every heading-count and heading-contract violation in the
    /// documents of *context*.
    fn audit_doc_heading(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// AES606 (first half) — template body-shape conformance: every mandated
/// section that carries a table in the HOW-TO template must actually be a
/// table with the required columns.
pub trait ITemplateConformanceProtocol: Send + Sync {
    /// Report every section that is present but carries the wrong body shape.
    fn audit_template_conformance(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// AES606 (second half) — required-document presence: every document the
/// template requires at the workspace root must exist.
pub trait IRequiredDocProtocol: Send + Sync {
    /// Report one finding per required root document that does not exist.
    fn audit_required_docs(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}
