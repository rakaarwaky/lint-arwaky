"""multiplication composition root (AES root layer).

The container wires the multiplication evaluator and operation log behind the
feature's aggregate, the one place that names the concrete classes.
"""

from multiplication.src.agent_multiplication_orchestrator import (
    MultiplicationOrchestrator,
    MultiplicationOrchestratorDeps,
)
from multiplication.src.capabilities_multiplication_analyzer import MultiplicationAnalyzer
from multiplication.src.capabilities_multiplication_log import MultiplicationLog
from shared.src.contract_multiplication_aggregate import IMultiplicationAggregate

# ─── Block 1: Struct Definition ───────────────────────────

class MultiplicationContainer:
    def __init__(self):
        self._orchestrator = MultiplicationOrchestrator(
            MultiplicationOrchestratorDeps(
                analyzer=MultiplicationAnalyzer(),
                log=MultiplicationLog(),
            )
        )

    def orchestrator(self) -> IMultiplicationAggregate:
        return self._orchestrator
