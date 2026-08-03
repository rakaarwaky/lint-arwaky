"""
dead_inherit.py — TEST PROJECT ONLY.
Triggers:
- AES016: dead-inheritance-bypass — empty class inheriting from contract protocol
- AES007: contract-barrel-violation — import directly from contract sub-module file
- AES003: single-word filename
"""


# AES007: Import directly from a contract sub-module file instead of __init__
# Contract imports should go through the barrel (__init__.py), not sub-module files.
from contract.fake_protocol import fake_protocol_func  # AES007 violation
from contract.protocol_bad import useless              # AES007 violation


class ILinterPort:
    """Simulated contract protocol interface."""
    def validate(self, code: str) -> bool:
        raise NotImplementedError

    def lint(self, file_path: str) -> list:
        raise NotImplementedError


class EmptyLinterAdapter(ILinterPort):
    """
    AES016: Dead inheritance bypass.
    Empty class inheriting from a contract protocol with no implementation.
    """
    pass


class DeadProtocolStub:
    """
    Another AES016 violation — inherits nothing and does nothing.
    """
    pass


# Also another dead inheritance — inherits contract protocol but has no body
class AnotherDeadAdapter(ILinterPort):
    ...
    # Ellipsis is also dead inheritance bypass


# Unused imports from the AES007 violations above
def check_violations():
    """Function to prevent unused import warnings being the ONLY issue."""
    useless()
    return fake_protocol_func()

