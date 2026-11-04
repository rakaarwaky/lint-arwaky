"""division composition root (AES root layer).

The container wires the division evaluator and operation log behind the
feature's aggregate, the one place that names the concrete classes.
"""

from division.src.agent_division_orchestrator import (
    DivisionOrchestrator,
    DivisionOrchestratorDeps,
)
from division.src.capabilities_division_analyzer import DivisionAnalyzer
from division.src.capabilities_division_log import DivisionLog
from shared.src.contract_division_aggregate import IDivisionAggregate

# ─── Block 1: Struct Definition ───────────────────────────

class DivisionContainer:
    def __init__(self):
        self._orchestrator = DivisionOrchestrator(
            DivisionOrchestratorDeps(
                analyzer=DivisionAnalyzer(),
                log=DivisionLog(),
            )
        )

    def orchestrator(self) -> IDivisionAggregate:
        return self._orchestrator
