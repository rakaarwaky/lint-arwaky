// AES403: inline test code in a capabilities file.
// Tests belong in tests/, not inside the capability source.
use crate::contract::some_protocol::SomeProtocol;

pub struct InlineTestProcessor;

impl SomeProtocol for InlineTestProcessor {
    fn execute(&self, input: String) -> bool {
        input.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_empty() {
        let p = InlineTestProcessor;
        assert!(p.execute("".to_string()));
    }
}
