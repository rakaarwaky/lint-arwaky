# AES013: Forbidden Inheritance - contract aggregate inherits from protocol
from contract.fake_protocol import FakeProtocol


class ContractBadAggregate(FakeProtocol):
    pass
