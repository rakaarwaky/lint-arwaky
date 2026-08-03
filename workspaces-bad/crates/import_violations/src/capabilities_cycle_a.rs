// AES205 side A — capabilities importing contract (creates cycle).
// Has BOTH mandatory imports (taxonomy + contract(protocol)), all imports used, no dummy.
use contract::aggregate::ConfigAggregate;
use contract::protocol::ContractProtocol;
use taxonomy::vo::UserVO;

pub fn process() -> UserVO {
    let agg = ConfigAggregate::new();
    agg.load();
    let proto = ContractProtocol::new();
    proto.validate();
    UserVO::new()
}
