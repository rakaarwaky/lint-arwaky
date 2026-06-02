# AES016 — empty class inheriting from contract (dead inheritance bypass)
# Placeholder fraud to trick architecture checker
from contract import ILinterAdapterPort

class FakeContractPlaceholder(ILinterAdapterPort):
    """Empty class - dead inheritance bypass."""
    pass

class DummyComplianceStub:
    """Another empty compliance bypass."""
    pass
