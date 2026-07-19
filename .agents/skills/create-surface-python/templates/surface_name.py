from shared.<domain>.taxonomy_<name>_vo import <VO>
from shared.<domain>.contract_<name>_aggregate import I<Name>Aggregate


class Surface<Name>:
    def __init__(self, aggregate: I<Name>Aggregate):
        self._aggregate = aggregate

    def handle(self, event: TuiEvent) -> Result[UiState, SurfaceError]:
        # orchestration only
        return Ok(UiState.idle())
