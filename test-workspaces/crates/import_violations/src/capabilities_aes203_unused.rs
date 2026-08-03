// AES203 ONLY — capabilities with unused import.
// Has BOTH mandatory imports (taxonomy + contract(protocol)), no forbidden, no dummy.
use taxonomy::vo::UserVO;
use taxonomy::vo::CategoryVO;
use contract::protocol::ContractProtocol;

pub fn process() -> UserVO {
    let proto = ContractProtocol::new();
    proto.validate();
    UserVO::new()
}
