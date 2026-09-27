// AES403: file-level const declared inside a capabilities file.
// Domain policy constants belong in taxonomy_<domain>_constant.rs.
use crate::contract::some_protocol::SomeProtocol;

const MAX_RETRY_ATTEMPTS: usize = 3;

pub struct LocalConstantProcessor;

impl SomeProtocol for LocalConstantProcessor {
    fn execute(&self, input: String) -> bool {
        input.is_empty() && MAX_RETRY_ATTEMPTS > 0
    }
}
