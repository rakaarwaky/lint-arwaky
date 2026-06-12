pub use self as layer_rules;
pub use self as contract;
pub use ::taxonomy;

pub mod base_aggregate {
    pub struct BaseAggregate;
}
pub mod base_port {
    pub trait BasePort {}
}
pub mod some_protocol {
    pub trait SomeProtocol {
        fn required_fn(&self) -> bool;
    }
}
pub mod other_protocol {
    pub trait OtherProtocol {}
}
pub mod removal_port {
    pub trait IRemovalPort {
        fn remove_background(&self, img: Vec<u8>) -> Vec<u8>;
    }
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
#[path = "contract_wrong_name_port.rs"]
pub mod wrong_name_port;
