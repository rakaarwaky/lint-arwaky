from shared.src.contract_calculator_protocol import CalculatorProtocol
from shared.src.taxonomy_expression_vo import ExpressionVO
from shared.src.taxonomy_result_vo import ResultVO, create_result


class SubtractionAnalyzer(CalculatorProtocol):
    def evaluate(self, expr: ExpressionVO) -> ResultVO:
        return create_result(expr.left, expr.op, expr.right, expr.left - expr.right)
