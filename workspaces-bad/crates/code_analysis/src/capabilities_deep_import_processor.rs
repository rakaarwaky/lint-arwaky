// AES007: import with 5+ segments
use crate::contract::some_protocol::SomeProtocol;

pub struct DeepImportProcessor;

impl SomeProtocol for DeepImportProcessor {
    fn required_fn(&self) -> bool {
        true
    }
}
