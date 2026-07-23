# AES203: capabilities with unused import (entity is imported but never used)
from ..taxonomy.domain_model_base import DomainModelBase

class UnusedImportChecker:
    def check(self):
        # DomainModelBase is imported but never used
        return True
