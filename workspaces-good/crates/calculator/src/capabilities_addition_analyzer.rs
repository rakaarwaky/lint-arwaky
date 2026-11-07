use calculator_shared::contract_calculator_protocol::ICalculatorProtocol;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_result_vo::ResultVO;

// ─── Block 1: Module Declaration ────────────────────────────

/// Adds the two operands of an expression.
pub struct AdditionAnalyzer;

// ─── Block 2: Module Contract ────────────────

impl ICalculatorProtocol for AdditionAnalyzer {
    fn evaluate(&self, expr: &ExpressionVO) -> Option<ResultVO> {
        let value = expr.left + expr.right;
        Some(ResultVO::new(expr.left, &expr.op, expr.right, value))
    }
}

// ─── Block 3: Module Helpers ────────────

impl Default for AdditionAnalyzer {
    /// The analyzer holds no state, so the default carries nothing.
    fn default() -> Self {
        Self
    }
}
