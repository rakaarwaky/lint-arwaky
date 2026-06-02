# Taxonomy module
from infrastructure.utils import deep_nested_complexity # VIOLATION: Taxonomy -> Infrastructure

class DomainModel:
    def __init__(self):
        self.data = deep_nested_complexity([1, 2, 3])
