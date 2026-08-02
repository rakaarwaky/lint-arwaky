// naming-rules crate — AES101 naming convention + AES102 suffix/prefix enforcement

// ── Capabilities (stateful check logic) ──
pub mod capabilities_naming_convention_checker;
pub mod capabilities_suffix_prefix_checker;

// ── Utility (stateless helpers) ──
pub mod utility_naming_checker;

// ── Agent (orchestration) ──
pub mod agent_naming_orchestrator;

// ── Root (composition, wiring) ──
pub mod root_naming_rules_container;
