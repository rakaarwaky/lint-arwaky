"""Addition orchestrator (AES `_orchestrator`, AES101 `_aggregate`).

The addition feature owns the ADD operation. This orchestrator is the entry
point over the feature: it coordinates the addition evaluator with the
addition operation log, so one call produces both the arithmetic result and
the durable record of it. It holds no arithmetic of its own.
"""

from shared.src.contract_addition_aggregate import IAdditionAggregate
from shared.src.contract_calculator_protocol import ICalculatorProtocol
from shared.src.contract_operation_log_protocol import IOperationLogProtocol
from shared.src.taxonomy_expression_vo import ExpressionVO
from shared.src.taxonomy_result_vo import ResultVO

__all__ = ["AdditionOrchestrator", "AdditionOrchestratorDeps"]


# ─── Block 1: Struct Definition ───────────────────────────

class AdditionOrchestratorDeps:
    def __init__(
        self,
        analyzer: ICalculatorProtocol,
        log: IOperationLogProtocol,
    ):
        self.analyzer = analyzer
        self.log = log


# ─── Block 2: Aggregate Implementation ────────────────────

class AdditionOrchestrator(IAdditionAggregate):
    def __init__(self, deps: AdditionOrchestratorDeps):
        self._analyzer = deps.analyzer
        self._log = deps.log
        self._history: list[ResultVO] = []

    def evaluate(self, expr: ExpressionVO) -> ResultVO | None:
        """Evaluate one addition expression and record the result."""
        result = self._analyzer.evaluate(expr)
        if result is not None:
            self._history.append(result)
            self._log.record(result)
        return result


# ─── Block 3: Helpers, Private Methods ────────────────────

    def history(self) -> list[ResultVO]:
        """Every result this feature has produced so far."""
        return list(self._history)
