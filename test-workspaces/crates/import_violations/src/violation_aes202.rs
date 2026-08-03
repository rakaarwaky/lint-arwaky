// Fixture: AES202 — capabilities missing mandatory taxonomy import.
// AES202 HIGH: capabilities scope requires at least one taxonomy import.
use contract::protocol::ContractProtocol;

pub fn process() {
    let _proto = ContractProtocol::new();
}
