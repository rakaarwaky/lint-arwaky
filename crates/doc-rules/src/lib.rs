// doc-rules — structural invariants for the AES document chain (AES601–AES605)

// ── Capability (stateful check logic) ────────────────────────────────────────
// One file per AES doc rule: each answers exactly one rule behind one
// protocol trait.
pub mod capabilities_doc_crosslink_checker;
pub mod capabilities_doc_fr_format_checker;
pub mod capabilities_doc_heading_structure_checker;
pub mod capabilities_doc_section_structure_checker;
pub mod capabilities_doc_spec_purity_checker;

// ── Agent (orchestration) ─────────────────────────────────────────────────────
pub mod agent_doc_orchestrator;

// ── Root (composition, wiring) ───────────────────────────────────────────────
pub mod root_doc_rules_container;
