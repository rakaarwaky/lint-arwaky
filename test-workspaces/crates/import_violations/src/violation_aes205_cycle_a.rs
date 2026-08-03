// Fixture: AES205 — circular dependency A side.
use contract::aggregate::ConfigAggregate;

pub fn process() {
    let _agg = ConfigAggregate::new();
}
