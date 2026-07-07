use crate::contract::some_protocol::SomeProtocol;
use crate::contract::other_protocol::OtherProtocol;

pub struct UnusedProtocolProcessor;
impl SomeProtocol for UnusedProtocolProcessor {
    fn required_fn(&self) -> bool { true }
}
