// AES202 only — capabilities missing mandatory taxonomy import.
// No forbidden imports, all imports used, no dummy functions.
use contract::protocol::ContractProtocol;

pub fn process() {
    let proto = ContractProtocol::new();
    proto.validate();
}
