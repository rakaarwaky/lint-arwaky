// Fixture: AES205 — circular dependency A side.
use contract::aggregate::ContractAggregate;

pub fn process() {
    let _agg = ContractAggregate::new();
}
