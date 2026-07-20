from shared.<name-feature>.taxonomy_<name-policy>_vo import (
    <NamePolicy>VO,
)
from shared.<name-feature>.contract_<name-store>_protocol import I<NameStore>Protocol
from shared.<name-feature>.contract_<name-collaborator>_protocol import (
    I<NameCollaborator>Protocol,
)
from shared.<name-feature>.contract_<name-capability>_protocol import (
    I<NameCapability>Protocol,
)


# ─── Block 1: Class Definition & Constructor ──────────────
class Capabilities<NameCapability>(I<NameCapability>Protocol):
    def __init__(
        self,
        collaborator: I<NameCollaborator>Protocol,
        store: I<NameStore>Protocol,
        policy: <NamePolicy>VO,
    ):
        self._collaborator = collaborator
        self._store = store
        self._policy = policy

    # ─── Block 2: Public Contract (domain protocol ONLY) ──
    def execute(self, input: <DomainVO>) -> list[<ResultVO>]:
        violations: list[<ResultVO>] = []
        # domain logic using injected dependencies
        return violations

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "Capabilities<NameCapability>()"
