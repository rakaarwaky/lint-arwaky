use calculator_shared::contract_calculator_protocol::ICalculatorProtocol;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_result_vo::ResultVO;

// ─── Block 1: Struct Definition ────────────────────────────

/// Multiplies the two operands of an expression.
pub struct MultiplicationAnalyzer;

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ICalculatorProtocol for MultiplicationAnalyzer {
    fn evaluate(&self, expr: &ExpressionVO) -> Option<ResultVO> {
        let value = expr.left * expr.right;
        Some(ResultVO::new(expr.left, &expr.op, expr.right, value))
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl MultiplicationAnalyzer {
    /// The analyzer holds no state, so construction takes no arguments.
    pub fn new() -> Self {
        Self
    }
}
