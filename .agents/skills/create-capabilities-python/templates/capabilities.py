from shared.<domain>.taxonomy_<name>_vo import <VO>
from shared.<domain>.contract_<name>_protocol import I<Name>Protocol


# ─── Block 1: Class Definition & Constructor ──────────────
class Capabilities<Name>(I<Name>Protocol):
    def __init__(self, /* DI params */) -> None:
        # DI fields use protocol interfaces
        # Value fields use shared VOs
        ...

    # ─── Block 2: Public Contract (domain protocol ONLY) ──
    def method_name(self, param: <VO>) -> None:
        # domain behavior
        ...

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "Capabilities<Name>()"

    @classmethod
    def create_default(cls) -> "Capabilities<Name>":
        return cls()
