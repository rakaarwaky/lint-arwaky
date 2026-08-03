// AES201 ONLY — capabilities importing agent (forbidden).
// Has BOTH mandatory imports (taxonomy + contract(protocol)), all imports used, no dummy.
use agent::orchestrator::AgentOrchestrator;
use taxonomy::vo::UserVO;
use contract::protocol::ContractProtocol;

pub fn process() -> UserVO {
    let agent = AgentOrchestrator::new();
    agent.run();
    let proto = ContractProtocol::new();
    proto.validate();
    UserVO::new()
}
