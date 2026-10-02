// doc_rules — doc-invariant auditors for the AES document chain
pub mod contract_doc_aggregate;
pub mod contract_doc_protocol;
pub mod taxonomy_doc_audit_context_vo;
pub mod taxonomy_doc_rules_constant;
pub mod taxonomy_doc_rules_request;
pub mod taxonomy_doc_rules_response;
pub mod taxonomy_doc_section_vo;
pub mod utility_doc_collector;
pub mod utility_markdown_scanner;
pub mod utility_protocol_counter;

// ─── Re-exports ────────────────────────────────────────────
pub use contract_doc_aggregate::IDocRunnerAggregate;
pub use contract_doc_protocol::{
    ICrosslinkProtocol, IDocHeadingProtocol, IFrFormatProtocol, ISectionStructureProtocol,
    ISpecPurityProtocol,
};
pub use taxonomy_doc_audit_context_vo::DocAuditContext;
pub use taxonomy_doc_rules_constant::*;
pub use taxonomy_doc_rules_request::{DocFinding, DocRequest, DocSource};
pub use taxonomy_doc_rules_response::DocResponse;
pub use taxonomy_doc_section_vo::Section;
