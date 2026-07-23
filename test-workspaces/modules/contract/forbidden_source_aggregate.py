# AES013 — forbidden inheritance test
# This contract aggregate file inherits from contract protocol (forbidden)
from contract.dead_protocol_aggregate import IDeadProtocol

class ForbiddenSourceAggregate(IDeadProtocol):
    def aggregate(self):
        return 'forbidden'
