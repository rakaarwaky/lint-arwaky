"""addition composition root (AES root layer).

The container wires the addition evaluator and operation log behind the
feature's aggregate, the one place that names the concrete classes.
"""

from addition.src.agent_addition_orchestrator import (
    AdditionOrchestrator,
    AdditionOrchestratorDeps,
)
from addition.src.capabilities_addition_analyzer import AdditionAnalyzer
from addition.src.capabilities_addition_log import AdditionLog
from shared.src.contract_addition_aggregate import IAdditionAggregate

# ─── Block 1: Struct Definition ───────────────────────────

class AdditionContainer:
    def __init__(self):
        self._orchestrator = AdditionOrchestrator(
            AdditionOrchestratorDeps(
                analyzer=AdditionAnalyzer(),
                log=AdditionLog(),
            )
        )

    def orchestrator(self) -> IAdditionAggregate:
        return self._orchestrator
