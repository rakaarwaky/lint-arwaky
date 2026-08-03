// AES202 ONLY — capabilities missing mandatory taxonomy import.
// Has contract(protocol) but NO taxonomy, no forbidden imports, all imports used.
use contract::protocol::ContractProtocol;

pub fn process() {
    let proto = ContractProtocol::new();
    proto.validate();
}
