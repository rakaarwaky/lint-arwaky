// AES203 only — capabilities with unused import.
// Has mandatory taxonomy import, no forbidden imports, no dummy functions.
use taxonomy::vo::CategoryVO;
use taxonomy::vo::UserVO;
use contract::protocol::ContractProtocol;

pub fn process() -> UserVO {
    let proto = ContractProtocol::new();
    proto.validate();
    UserVO::new()
}
