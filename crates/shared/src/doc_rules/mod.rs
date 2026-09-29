// doc_rules — doc-invariant auditors for the AES document chain
pub mod contract_doc_aggregate;
pub mod contract_doc_protocol;
pub mod taxonomy_doc_rules_constant;
pub mod taxonomy_doc_rules_request;
pub mod taxonomy_doc_rules_response;
pub mod utility_protocol_counter;

// ─── Re-exports ────────────────────────────────────────────
pub use contract_doc_aggregate::IDocRunnerAggregate;
pub use contract_doc_protocol::IDocCheckerProtocol;
pub use taxonomy_doc_rules_constant::*;
pub use taxonomy_doc_rules_request::{DocFinding, DocRequest};
pub use taxonomy_doc_rules_response::DocResponse;
