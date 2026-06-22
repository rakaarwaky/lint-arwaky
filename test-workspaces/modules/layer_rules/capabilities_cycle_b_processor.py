# AES205: part of circular dependency
from contract_cycle_a_protocol import AProtocol

class BProcessor:
    def get_b(self) -> str:
        return "B"
