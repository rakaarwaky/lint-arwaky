pub mod contract_naming_checker_protocol;
pub mod contract_naming_runner_aggregate;
pub mod contract_test_file_prefix_protocol;
pub mod taxonomy_naming_rules_constant;
pub mod taxonomy_naming_rules_request;
pub mod taxonomy_naming_rules_response;
pub mod taxonomy_test_suite_slot_vo;
pub mod utility_naming_checker;
pub mod utility_test_prefix;

// AES103 test/bench prefix vocabulary — shared with AES704 so the naming rule
// and the structure rule cannot drift apart on which prefixes are legal.
pub use utility_test_prefix::{
    ALL_TESTS_DIR_PREFIXES, BENCH_PREFIX, BENCHES_DIR, TEST_SUPPORT_DIRS, TEST_SUPPORT_PREFIXES,
    TEST_TYPE_PREFIXES, TESTS_DIR, has_legal_prefix, leaf_directory_of, matched_prefix,
    nested_under, prefix_list, prefixes_for, slot_of,
};

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_naming_checker_protocol::INamingConventionProtocol;
pub use contract_naming_checker_protocol::ISuffixPolicyProtocol;
pub use contract_naming_runner_aggregate::INamingRunnerAggregate;
pub use contract_test_file_prefix_protocol::ITestFilePrefixProtocol;

// ── Taxonomy types ──
pub use taxonomy_naming_rules_constant::ADAPTER_NAME;
pub use taxonomy_naming_rules_constant::LAYER_PREFIXES;
pub use taxonomy_naming_rules_constant::RULE_CODE_NAMING_CONVENTION;
pub use taxonomy_naming_rules_constant::RULE_CODE_SUFFIX_PREFIX;
pub use taxonomy_naming_rules_constant::RULE_CODE_TEST_FILE_PREFIX;
pub use taxonomy_naming_rules_constant::SNAKE_CASE_SEPARATOR;
pub use taxonomy_naming_rules_constant::SOURCE_EXTENSIONS;
pub use taxonomy_naming_rules_constant::SPECIALIZED_LAYER_MARKER;
pub use taxonomy_naming_rules_constant::SUFFIX_POLICY_STRICT;
pub use taxonomy_naming_rules_request::NamingRequest;
pub use taxonomy_naming_rules_response::NamingResponse;
pub use taxonomy_test_suite_slot_vo::TestSuiteSlot;
