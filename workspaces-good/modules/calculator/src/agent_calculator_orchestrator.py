"""Calculator orchestrator (AES `_orchestrator`, AES101 `_aggregate`).

The calculator member coordinates four operation features. This orchestrator
is the aggregate every consumer reaches: it routes a request to the feature
orchestrator that owns the operation, and merges their history. It performs no
arithmetic itself — each operation lives in its own feature folder behind its
own orchestrator, which the root container constructs and injects here.

Alongside the four feature aggregates it holds the four feature operation
logs, so the member owns the cross-feature view: a request answered by the
addition feature is also written to the addition operation log, and the
member reads every operation log when it merges history.
"""

from shared.src.contract_addition_aggregate import IAdditionAggregate
from shared.src.contract_calculator_aggregate import ICalculatorAggregate
from shared.src.contract_division_aggregate import IDivisionAggregate
from shared.src.contract_multiplication_aggregate import IMultiplicationAggregate
from shared.src.contract_operation_log_protocol import IOperationLogProtocol
from shared.src.contract_subtraction_aggregate import ISubtractionAggregate
from shared.src.taxonomy_calculator_request import (
    CalculatorRequest,
    CalculatorVerb,
)
from shared.src.taxonomy_calculator_response import CalculatorResponse
from shared.src.taxonomy_expression_vo import ExpressionVO
from shared.src.taxonomy_operation_vo import OperationVO
from shared.src.taxonomy_result_vo import ResultVO

__all__ = ["CalculatorOrchestrator", "CalculatorOrchestratorDeps"]


# ─── Block 1: Struct Definition ───────────────────────────

class CalculatorOrchestratorDeps:
    def __init__(
        self,
        addition: IAdditionAggregate,
        subtraction: ISubtractionAggregate,
        multiplication: IMultiplicationAggregate,
        division: IDivisionAggregate,
        addition_log: IOperationLogProtocol,
        subtraction_log: IOperationLogProtocol,
        multiplication_log: IOperationLogProtocol,
        history: IOperationLogProtocol,
        division_log: IOperationLogProtocol,
    ):
        self.addition = addition
        self.subtraction = subtraction
        self.multiplication = multiplication
        self.division = division
        self.addition_log = addition_log
        self.subtraction_log = subtraction_log
        self.multiplication_log = multiplication_log
        self.division_log = division_log
        self.history = history


# ─── Block 2: Aggregate Implementation ────────────────────

class CalculatorOrchestrator(ICalculatorAggregate):
    def __init__(self, deps: CalculatorOrchestratorDeps):
        self._features = {
            OperationVO.ADD: deps.addition,
            OperationVO.SUBTRACT: deps.subtraction,
            OperationVO.MULTIPLY: deps.multiplication,
            OperationVO.DIVIDE: deps.division,
        }
        self._history = deps.history

    def execute(self, request: CalculatorRequest) -> CalculatorResponse:
        if request.verb is CalculatorVerb.DELEGATE:
            return CalculatorResponse.delegation(self._delegate(request.expr))
        return CalculatorResponse.history(self._merged_history())


# ─── Block 3: Helpers, Private Methods ────────────────────

    def _delegate(self, expr: ExpressionVO | None) -> ResultVO | None:
        if expr is None:
            return None
        owner = self._features.get(expr.op)
        if owner is None:
            return None
        return owner.evaluate(expr)

    def _merged_history(self) -> list[ResultVO]:
        return self._history.merge([feature.history() for feature in self._features.values()])
