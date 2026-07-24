from shared.<domain>.taxonomy_<name>_vo import <VO>
from shared.<domain>.contract_<name>_aggregate import I<Name>Aggregate
from shared.<domain>.contract_<protocol>_protocol import I<Protocol>Protocol


# ─── Block 1: Struct Definition & Constructor ──────────────
class <Name>Orchestrator(I<Name>Aggregate):
    def __init__(self, service: I<Protocol>Protocol) -> None:
        # DI fields use protocol interfaces
        # Value fields use shared VOs
        self._service = service

    # ─── Block 2: Public Contract (domain aggregate ONLY) ──
    def execute(self, request: <RequestVO>) -> <ResultVO>:
        # orchestration only - delegate to protocol
        return self._service.process(request)

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "<Name>Orchestrator()"

    @classmethod
    def create_default(cls) -> "<Name>Orchestrator":
        return cls(service=Default<Protocol>())
