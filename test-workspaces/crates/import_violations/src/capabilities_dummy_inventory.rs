// AES204 only — capabilities with dummy function.
// Has mandatory taxonomy import, no forbidden imports, import is "used" in dummy function.
use taxonomy::vo::InventoryVO;
use contract::protocol::ContractProtocol;

pub fn process() {
    let proto = ContractProtocol::new();
    proto.validate();
}

fn _use_inventory_vo(_i: &InventoryVO) {}
