pub use self as layer_rules;
pub use self as contract;
pub use self as capabilities;
pub use ::taxonomy;

pub mod base_aggregate {
    pub struct BaseAggregate;
}
pub mod some_protocol {
    pub trait SomeProtocol {
        fn required_fn(&self) -> bool;
    }
}
pub mod other_protocol {
    pub trait OtherProtocol {}
}
pub mod sub {
    pub mod module {
        pub trait WrongNamePort {
            fn execute(&self, input: String) -> bool;
        }
    }
}

#[path = "contract_forbidden_inherit_aggregate.rs"]
pub mod forbidden_inherit_aggregate;
#[path = "contract_missing_suffix.rs"]
pub mod missing_suffix;

#[path = "capabilities_cycle_violation_processor.rs"]
pub mod cycle_violation_processor;

#[path = "contract_cycle_violation_aggregate.rs"]
pub mod cycle_violation_aggregate;

#[path = "contract_aes201_debug_aggregate.rs"]
pub mod aes201_debug_aggregate;

#[path = "contract_orphan_protocol.rs"]
pub mod orphan_protocol;

#[path = "contract_dead_inherit_aggregate.rs"]
pub mod dead_inherit_aggregate;
