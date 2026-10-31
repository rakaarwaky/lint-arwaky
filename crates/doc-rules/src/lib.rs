// doc-rules — structural invariants for the AES document chain (AES601–AES610)

// ── Capability (stateful check logic) ────────────────────────────────────────
// One file per AES document type: each answers ALL invariants scoped to one
// doc type behind one per-doc protocol trait.
pub mod capabilities_doc_agents;
pub mod capabilities_doc_architecture;
pub mod capabilities_doc_backlog;
pub mod capabilities_doc_contributing;
pub mod capabilities_doc_data;
pub mod capabilities_doc_design;
pub mod capabilities_doc_frd;
pub mod capabilities_doc_prd;
pub mod capabilities_doc_readme;
pub mod capabilities_doc_roadmap;

// ── Agent (orchestration) ─────────────────────────────────────────────────────
pub mod agent_doc_orchestrator;

// ── Root (composition, wiring) ───────────────────────────────────────────────
pub mod root_doc_rules_container;
