# AES014 — import contract but don't implement it
from ..contract.dead_protocol_aggregate import IDeadProtocol

class BareContractProcessor:
    def run(self):
        return 'no contract implemented'
