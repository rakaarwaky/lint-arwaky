pub mod contract_naming_checker_protocol;
pub mod contract_naming_runner_aggregate;
pub mod taxonomy_naming_rules_constant;
pub mod taxonomy_naming_rules_request;
pub mod taxonomy_naming_rules_response;
pub mod utility_naming_checker;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_naming_checker_protocol::INamingConventionProtocol;
pub use contract_naming_checker_protocol::ISuffixPolicyProtocol;
pub use contract_naming_runner_aggregate::INamingRunnerAggregate;

// ── Taxonomy types ──
pub use taxonomy_naming_rules_constant::ADAPTER_NAME;
pub use taxonomy_naming_rules_constant::LAYER_PREFIXES;
pub use taxonomy_naming_rules_constant::RULE_CODE_NAMING_CONVENTION;
pub use taxonomy_naming_rules_constant::RULE_CODE_SUFFIX_PREFIX;
pub use taxonomy_naming_rules_constant::SNAKE_CASE_SEPARATOR;
pub use taxonomy_naming_rules_constant::SOURCE_EXTENSIONS;
pub use taxonomy_naming_rules_constant::SPECIALIZED_LAYER_MARKER;
pub use taxonomy_naming_rules_constant::SUFFIX_POLICY_STRICT;
pub use taxonomy_naming_rules_request::NamingRequest;
pub use taxonomy_naming_rules_response::NamingResponse;
