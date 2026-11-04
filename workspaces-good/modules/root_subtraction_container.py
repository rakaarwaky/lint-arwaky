"""subtraction composition root (AES root layer).

The container wires the subtraction evaluator and operation log behind the
feature's aggregate, the one place that names the concrete classes.
"""

from shared.src.contract_subtraction_aggregate import ISubtractionAggregate
from subtraction.src.agent_subtraction_orchestrator import (
    SubtractionOrchestrator,
    SubtractionOrchestratorDeps,
)
from subtraction.src.capabilities_subtraction_analyzer import SubtractionAnalyzer
from subtraction.src.capabilities_subtraction_log import SubtractionLog

# ─── Block 1: Struct Definition ───────────────────────────

class SubtractionContainer:
    def __init__(self):
        self._orchestrator = SubtractionOrchestrator(
            SubtractionOrchestratorDeps(
                analyzer=SubtractionAnalyzer(),
                log=SubtractionLog(),
            )
        )

    def orchestrator(self) -> ISubtractionAggregate:
        return self._orchestrator
