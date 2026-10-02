from shared.src.contract_calculator_protocol import ICalculatorProtocol
from shared.src.taxonomy_expression_vo import ExpressionVO
from shared.src.taxonomy_result_vo import ResultVO, create_result

# ─── Block 1: Struct Definition ─────────────────────────────

class SubtractionAnalyzer(ICalculatorProtocol):
    """Subtracts the right operand of an expression from the left one."""

    # ─── Block 2: Protocol Trait Implementation ────────────────

    def evaluate(self, expr: ExpressionVO) -> ResultVO:
        """Evaluate one subtraction expression."""
        return create_result(expr.left, expr.op, expr.right, expr.left - expr.right)

    # ─── Block 3: Constructors, Std Traits, Helpers ─────────────

    def __repr__(self) -> str:
        return "SubtractionAnalyzer()"
