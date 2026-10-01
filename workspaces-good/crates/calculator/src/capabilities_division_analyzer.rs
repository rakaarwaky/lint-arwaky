use calculator_shared::contract_calculator_protocol::ICalculatorProtocol;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_result_vo::ResultVO;

// ─── Block 1: Struct Definition ────────────────────────────

/// Divides the left operand of an expression by the right one.
pub struct DivisionAnalyzer;

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ICalculatorProtocol for DivisionAnalyzer {
    fn evaluate(&self, expr: &ExpressionVO) -> Option<ResultVO> {
        if expr.right == 0.0 {
            return None;
        }
        let value = expr.left / expr.right;
        Some(ResultVO::new(expr.left, &expr.op, expr.right, value))
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl DivisionAnalyzer {
    /// The analyzer holds no state, so construction takes no arguments.
    pub fn new() -> Self {
        Self
    }
}
