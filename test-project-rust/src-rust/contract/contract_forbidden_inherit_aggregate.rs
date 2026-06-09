// AES013: Forbidden Inheritance - contract aggregate inherits from protocol
// Contract Aggregate must NOT inherit from Port or Protocol
use crate::contract::wrong_name_port::WrongNamePort;

pub struct ContractForbiddenInheritAggregate;
impl WrongNamePort for ContractForbiddenInheritAggregate {}
