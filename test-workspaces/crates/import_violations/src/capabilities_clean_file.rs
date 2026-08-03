// Clean file — should trigger ZERO import violations.
// Has mandatory taxonomy import, no forbidden imports, all imports used, no dummy functions.
use taxonomy::vo::UserVO;
use contract::protocol::ContractProtocol;

pub fn process() -> UserVO {
    let proto = ContractProtocol::new();
    proto.validate();
    UserVO::new()
}
