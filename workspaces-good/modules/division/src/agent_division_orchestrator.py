"""Division orchestrator (AES `_orchestrator`, AES101 `_aggregate`).

The division feature owns the DIVIDE operation. This orchestrator is the
entry point over the feature: it coordinates the division evaluator with
the division operation log, so one call produces both the arithmetic result
and the durable record of it. An expression the evaluator refuses (division
by zero) is not recorded, so the log holds only results that were actually
produced. It holds no arithmetic of its own.
"""

from shared.src.contract_calculator_protocol import ICalculatorProtocol
from shared.src.contract_division_aggregate import IDivisionAggregate
from shared.src.contract_operation_log_protocol import IOperationLogProtocol
from shared.src.taxonomy_expression_vo import ExpressionVO
from shared.src.taxonomy_result_vo import ResultVO

__all__ = ["DivisionOrchestrator", "DivisionOrchestratorDeps"]


# ─── Block 1: Struct Definition ───────────────────────────

class DivisionOrchestratorDeps:
    def __init__(
        self,
        analyzer: ICalculatorProtocol,
        log: IOperationLogProtocol,
    ):
        self.analyzer = analyzer
        self.log = log


# ─── Block 2: Aggregate Implementation ────────────────────

class DivisionOrchestrator(IDivisionAggregate):
    def __init__(self, deps: DivisionOrchestratorDeps):
        self._analyzer = deps.analyzer
        self._log = deps.log
        self._history: list[ResultVO] = []

    def evaluate(self, expr: ExpressionVO) -> ResultVO | None:
        """Evaluate one division expression and record the result."""
        result = self._analyzer.evaluate(expr)
        if result is not None:
            self._history.append(result)
            self._log.record(result)
        return result


# ─── Block 3: Helpers, Private Methods ────────────────────

    def history(self) -> list[ResultVO]:
        """Every result this feature has produced so far."""
        return list(self._history)
