// AES204 ONLY — capabilities with dummy function.
// Has BOTH mandatory imports (taxonomy + contract(protocol)), no forbidden, import used in dummy.
use taxonomy::vo::InventoryVO;
use contract::protocol::ContractProtocol;

pub fn process() {
    let proto = ContractProtocol::new();
    proto.validate();
    let inv = InventoryVO::new();
    inv.count();
}

fn _use_inventory_vo(_i: &InventoryVO) {}
