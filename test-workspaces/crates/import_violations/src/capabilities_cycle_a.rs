// Fixture: AES205 — circular dependency A side.
use contract::aggregate::ConfigAggregate;
use taxonomy::vo::UserVO;

pub fn process() -> UserVO {
    let _agg = ConfigAggregate::new();
    UserVO::new()
}
