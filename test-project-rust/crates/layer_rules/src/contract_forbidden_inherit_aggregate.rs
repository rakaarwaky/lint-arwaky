// AES026: Aggregate implements Port trait directly (forbidden inheritance)
use crate::contract::wrong_name_port::WrongNamePort;

pub struct ForbiddenAggregate;

impl WrongNamePort for ForbiddenAggregate {
    fn execute(&self, input: String) -> bool {
        input.is_empty()
    }
}
