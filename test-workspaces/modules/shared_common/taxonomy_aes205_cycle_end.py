# AES205: Part of circular dependency chain
from ..taxonomy.taxonomy_aes205_cycle_start import CycleStartEntity

class CycleEndEntity:
    def __init__(self):
        self.data = "end"

    def process(self):
        start = CycleStartEntity()
        return self.data
