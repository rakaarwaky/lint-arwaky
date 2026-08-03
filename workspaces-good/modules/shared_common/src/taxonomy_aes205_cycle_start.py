# AES205: Part of circular dependency chain
from ..taxonomy.taxonomy_aes205_cycle_end import CycleEndEntity

class CycleStartEntity:
    def __init__(self):
        self.data = "start"

    def process(self):
        end = CycleEndEntity()
        return self.data
