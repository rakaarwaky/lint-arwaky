from shared.<name_feature>.contract_<name>_protocol import I<Name>Protocol
from shared.<name_feature>.contract_<name>_aggregate import I<Name>Aggregate
from shared.<name_feature>.taxonomy_<name>_vo import <Name>VO
from shared.<name_feature>.taxonomy_<result>_vo import <ResultVO>

class <Name>Orchestrator(I<Name>Aggregate):
    def __init__(self, deps: <Name>Deps):
        self._deps = deps

    def execute(self, request: <RequestVO>) -> <ResultVO>:
        formatter: I<Name>Protocol = self._get_formatter(request)
        return formatter.process(request)

    def __repr__(self) -> str:
        return "<Name>Orchestrator()"

    def _get_formatter(self, request: <RequestVO>) -> I<Name>Protocol:
        match request.type:
            case RequestType.A: return self._deps.a
            case RequestType.B: return self._deps.b
