// This capability file violates AES027 (mandatory-inheritance)
// because it imports a contract protocol (removal_protocol) but no struct/class inside implements or inherits it.
use crate::taxonomy::removal_types::RemovalType;
use crate::contract::removal_protocol::IRemovalProtocol; // imported but not implemented!

pub struct UnrelatedStruct {
    pub name: String,
}

impl UnrelatedStruct {
    pub fn do_nothing(&self) {}
}
