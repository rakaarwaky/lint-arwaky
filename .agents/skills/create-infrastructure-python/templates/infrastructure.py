from shared.<domain>.taxonomy_<name>_vo import <VO>
from shared.<domain>.contract_<name>_port import I<Name>Port


# ─── Block 1: Class Definition & Constructor ──────────────
class Infrastructure<Name>(I<Name>Port):
    def __init__(self, /* DI params */) -> None:
        # DI fields use port interfaces
        # Value fields use shared VOs
        ...

    # ─── Block 2: Public Contract (domain port ONLY) ──────
    def method_name(self, param: <VO>) -> Result[<VO>, Error]:
        # port implementation
        ...

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "Infrastructure<Name>()"

    @classmethod
    def create_default(cls) -> "Infrastructure<Name>":
        return cls()
