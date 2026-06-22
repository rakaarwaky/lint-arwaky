# AES205: part of circular dependency
from capabilities_cycle_b_processor import BProcessor

class AProtocol:
    def get_a(self) -> str:
        return "A"
