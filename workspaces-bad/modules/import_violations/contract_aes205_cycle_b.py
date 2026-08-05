# Fixture: AES205 — circular dependency B side (contract imports capabilities).
from capabilities_aes205_cycle_a import process as cap_process


class ContractCycle:
    def run(self):
        return cap_process()
