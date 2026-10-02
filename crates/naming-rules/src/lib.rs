// naming-rules crate — AES101 naming convention + AES102 suffix/prefix
// + AES103 test/bench file-prefix enforcement

// ── Capabilities (stateful check logic) ──
pub mod capabilities_naming_convention_checker;
pub mod capabilities_suffix_policy_checker;
pub mod capabilities_test_file_prefix_checker;

// ── Agent (orchestration) ──
pub mod agent_naming_orchestrator;

// ── Root (composition, wiring) ──
pub mod root_naming_rules_container;
