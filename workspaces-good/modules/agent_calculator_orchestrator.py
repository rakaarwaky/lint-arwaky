from typing import List, Optional

from shared.src.contract_calculator_aggregate import CalculatorAggregate
from shared.src.contract_calculator_protocol import CalculatorProtocol
from shared.src.taxonomy_expression_vo import ExpressionVO
from shared.src.taxonomy_operation_vo import OperationVO
from shared.src.taxonomy_result_vo import ResultVO


# ─── Block 1: Struct Definition ───────────────────────────

class CalculatorOrchestratorDeps:
    def __init__(
        self,
        addition: CalculatorProtocol,
        subtraction: CalculatorProtocol,
        multiplication: CalculatorProtocol,
        division: CalculatorProtocol,
    ):
        self.addition = addition
        self.subtraction = subtraction
        self.multiplication = multiplication
        self.division = division


# ─── Block 2: Protocol Implementation ─────────────────────

class CalculatorOrchestrator(CalculatorAggregate):
    def __init__(self, deps: CalculatorOrchestratorDeps):
        self._deps = deps
        self._history: List[ResultVO] = []

    def delegate(self, expr: ExpressionVO) -> Optional[ResultVO]:
        analyzer_map = {
            OperationVO.ADD: self._deps.addition,
            OperationVO.SUBTRACT: self._deps.subtraction,
            OperationVO.MULTIPLY: self._deps.multiplication,
            OperationVO.DIVIDE: self._deps.division,
        }
        analyzer = analyzer_map.get(expr.op)
        if analyzer is None:
            return None
        result = analyzer.evaluate(expr)
        if result is not None:
            self._history.append(result)
        return result

    def history(self) -> List[ResultVO]:
        return list(self._history)
