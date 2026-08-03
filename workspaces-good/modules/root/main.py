import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from shared.src.contract_calculator_aggregate import CalculatorAggregate
from shared.src.contract_calculator_protocol import CalculatorProtocol
from shared.src.taxonomy_expression_vo import ExpressionVO
from shared.src.taxonomy_operation_vo import OperationVO
from shared.src.taxonomy_result_vo import ResultVO
from addition.src.capability_addition_analyzer import AdditionAnalyzer
from subtraction.src.capability_subtraction_analyzer import SubtractionAnalyzer
from multiplication.src.capability_multiplication_analyzer import MultiplicationAnalyzer
from division.src.capability_division_analyzer import DivisionAnalyzer
from cli.src.surface_calculator_command import run


class Calculator(CalculatorAggregate):
    def __init__(self) -> None:
        self._history: list[ResultVO] = []
        self._capabilities: dict[OperationVO, CalculatorProtocol] = {
            OperationVO.ADD: AdditionAnalyzer(),
            OperationVO.SUBTRACT: SubtractionAnalyzer(),
            OperationVO.MULTIPLY: MultiplicationAnalyzer(),
            OperationVO.DIVIDE: DivisionAnalyzer(),
        }

    def delegate(self, expr: ExpressionVO) -> ResultVO | None:
        protocol = self._capabilities.get(expr.op)
        if protocol is None:
            return None
        result = protocol.evaluate(expr)
        if result is not None:
            self._history.append(result)
        return result

    def history(self) -> list[ResultVO]:
        return list(self._history)


if __name__ == "__main__":
    calc = Calculator()
    run(calc)
