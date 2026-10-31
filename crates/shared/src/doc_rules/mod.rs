// doc_rules — doc-invariant auditors for the AES document chain

// Build-time generated doc-linter shape constants (required H2s, table
// column headers, fixed row labels), extracted from the
// `references/*/template.md` files by `build.rs` via Comrak.
mod doc_generated {
    include!(concat!(env!("OUT_DIR"), "/doc_generated.rs"));
}

pub mod contract_doc_aggregate;
pub mod contract_doc_protocol;
pub mod taxonomy_doc_audit_context_vo;
pub mod taxonomy_doc_rules_constant;
pub mod taxonomy_doc_rules_request;
pub mod taxonomy_doc_rules_response;
pub mod taxonomy_doc_rules_template_vo;
pub mod taxonomy_doc_rules_violation_vo;
pub mod taxonomy_doc_section_vo;
pub mod utility_capabilities_doc_h2_checks;
pub mod utility_doc_aspect_entry_checks;
pub mod utility_doc_collector;
pub mod utility_doc_common_checks;
pub mod utility_doc_entry_regexes;
pub mod utility_doc_fr_format_checks;
pub mod utility_doc_line_drift;
pub mod utility_markdown_scanner;
pub mod utility_protocol_counter;

// The generated shape constants are re-exported through
// `taxonomy_doc_rules_constant` (which does `pub use super::doc_generated::*;`),
// so `pub use taxonomy_doc_rules_constant::*;` below is the public surface.

// ─── Re-exports ────────────────────────────────────────────
pub use contract_doc_aggregate::IDocRunnerAggregate;
pub use contract_doc_protocol::{
    IAgentsDocProtocol, IArchitectureDocProtocol, IBacklogDocProtocol, IContributingDocProtocol,
    IDataDocProtocol, IDesignDocProtocol, IPrdDocProtocol, IReadmeDocProtocol, IRoadmapDocProtocol,
};
pub use taxonomy_doc_audit_context_vo::{
    DocAuditContext, DocRulesConfig, MemberDescriptor, MemberRole, StrictModeOverride,
};
pub use taxonomy_doc_rules_constant::*;
pub use taxonomy_doc_rules_request::{DocFinding, DocRequest, DocSource};
pub use taxonomy_doc_rules_response::DocResponse;
pub use taxonomy_doc_rules_template_vo::{TEMPLATE_PROFILES, TemplateProfile};
pub use taxonomy_doc_section_vo::Section;
