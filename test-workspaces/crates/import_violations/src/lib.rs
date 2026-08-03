// Fixture: AES201 violation — capabilities layer importing agent (forbidden).
// This file is in capabilities layer (prefix: capabilities_*) but imports agent layer.
// AES201 CRITICAL: capabilities must NOT import agent.
use agent::orchestrator::AgentOrchestrator;

pub fn process() {
    let _agent = AgentOrchestrator::new();
}
