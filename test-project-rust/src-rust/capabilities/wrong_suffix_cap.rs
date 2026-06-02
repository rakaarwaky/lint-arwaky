// This capability file violates AES011 (suffix-mismatch)
// because "cap" is not one of the allowed capability suffixes (like analyzer, executor, processor, etc.).
use crate::taxonomy::removal_types::RemovalType;

pub struct WrongSuffixCap {
    pub value: String,
}

impl WrongSuffixCap {
    pub fn do_nothing(&self) {}
}
