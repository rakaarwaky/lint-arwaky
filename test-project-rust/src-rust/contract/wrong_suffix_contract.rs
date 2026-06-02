// This contract file violates AES008 (contract-suffix-mismatch)
// because it is located in the contract layer but its name lacks the required contract suffixes (_port, _protocol, _aggregate).
use crate::taxonomy::removal_types::RemovalType;

pub trait IWrongSuffixContract {
    fn execute_something(&self) -> bool;
}

pub struct HelperData {
    pub active: bool,
}
