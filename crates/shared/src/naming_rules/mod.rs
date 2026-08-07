pub mod contract_naming_checker_protocol;
pub mod contract_naming_runner_aggregate;
pub mod taxonomy_naming_constant;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_naming_checker_protocol::INamingConventionChecker;
pub use contract_naming_checker_protocol::ISuffixPrefixChecker;
pub use contract_naming_runner_aggregate::INamingRunnerAggregate;

// ── Taxonomy types ──
pub use taxonomy_naming_constant::ADAPTER_NAME;
pub use taxonomy_naming_constant::LAYER_PREFIXES;
pub use taxonomy_naming_constant::RULE_CODE_NAMING_CONVENTION;
pub use taxonomy_naming_constant::RULE_CODE_SUFFIX_PREFIX;
pub use taxonomy_naming_constant::SNAKE_CASE_SEPARATOR;
pub use taxonomy_naming_constant::SOURCE_EXTENSIONS;
pub use taxonomy_naming_constant::SPECIALIZED_LAYER_MARKER;
pub use taxonomy_naming_constant::SUFFIX_POLICY_STRICT;
