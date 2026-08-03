from typing import Optional

from shared.src.contract_calculator_protocol import CalculatorProtocol
from shared.src.taxonomy_expression_vo import ExpressionVO
from shared.src.taxonomy_result_vo import ResultVO, create_result


class DivisionAnalyzer(CalculatorProtocol):
    def evaluate(self, expr: ExpressionVO) -> Optional[ResultVO]:
        if expr.right == 0:
            return None
        return create_result(expr.left, expr.op, expr.right, expr.left / expr.right)
