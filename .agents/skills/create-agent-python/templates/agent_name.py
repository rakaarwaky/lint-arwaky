from shared.<domain>.taxonomy_<name>_vo import <VO>
from shared.<domain>.contract_<name>_aggregate import I<Name>Aggregate


# ─── Block 1: Class Definition & Constructor ──────────────
class Agent<Name>(I<Name>Aggregate):
    def __init__(self, /* DI params */) -> None:
        # DI fields use protocol interfaces
        # Value fields use shared VOs
        ...

    # ─── Block 2: Public Contract (domain aggregate ONLY) ──
    def execute(self, request: ScanRequest) -> list[LintResult]:
        # orchestration only
        return []

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "Agent<Name>()"

    @classmethod
    def create_default(cls) -> "Agent<Name>":
        return cls()
