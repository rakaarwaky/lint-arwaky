// Fixture: AES204 — dummy function to suppress unused import warning.
use taxonomy::vo::InventoryVO;
use contract::protocol::ContractProtocol;

pub fn process() {
    let _proto = ContractProtocol::new();
}

fn _use_inventory_vo(_i: &InventoryVO) {}
