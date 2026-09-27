// doc-rules — structural invariants for the AES document chain (AES601–AES605)

// ── Capability (stateful check logic) ────────────────────────────────────────
pub mod capabilities_doc_checker;

// ── Agent (orchestration) ─────────────────────────────────────────────────────
pub mod agent_doc_orchestrator;

// ── Root (composition, wiring) ────────────────────────────────────────────────
pub mod root_doc_rules_container;
