// AES201 — capabilities member imports segment `agent` (forbidden by the
// capabilities rule row). The target lives in sibling member member_b, so
// only a full-root scan sees the cross-member evidence.
use agent::agent_bar::helper;

pub struct CapabilitiesFoo;

impl CapabilitiesFoo {
    pub fn process() {
        helper();
    }
}
