// AES205 side A — capabilities importing contract (creates cycle with cycle_end).
// Has mandatory taxonomy import, no forbidden imports (contract is allowed), all imports used.
use contract::aggregate::ConfigAggregate;
use taxonomy::vo::UserVO;

pub fn process() -> UserVO {
    let agg = ConfigAggregate::new();
    agg.load();
    UserVO::new()
}
