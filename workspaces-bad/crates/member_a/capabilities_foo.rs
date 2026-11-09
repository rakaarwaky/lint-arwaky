// AES201 ONLY — capabilities importing agent from member_b (forbidden).
// Import segment `agent` maps to the agent layer; capabilities forbids agent
// under the AES201 rule row (capabilities → surface,agent,capabilities,root).
use agent::agent_bar::helper;

pub struct CapabilitiesFoo;

impl CapabilitiesFoo {
    pub fn process() {
        helper();
    }
}
